#!/usr/bin/env node
import crypto from "node:crypto";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);
const web3 = require("@solana/web3.js");
const {
  ComputeBudgetProgram,
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} = web3;

const SCRIPT_ROOT = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(SCRIPT_ROOT, "../..");
const VECTOR_ROOT = process.env.SOLMATH_VECTOR_ROOT || path.join(ROOT, "benchmark");
const TEST_DATA_ROOT = process.env.SOLMATH_TEST_DATA_ROOT || path.join(ROOT, "test_data");
const RPC = process.env.ANCHOR_PROVIDER_URL || "http://127.0.0.1:8899";
const WALLET = process.env.ANCHOR_WALLET || path.join(os.homedir(), ".config/solana/id.json");

function positiveInteger(name, fallback) {
  const value = Number.parseInt(process.env[name] || fallback, 10);
  if (!Number.isSafeInteger(value) || value <= 0) {
    throw new Error(`${name} must be a positive integer`);
  }
  return value;
}

const LIMIT = positiveInteger("BENCH_LIMIT", "2000");
const CONCURRENCY = positiveInteger("BENCH_CONCURRENCY", "32");
const SIMULATE_ONLY = process.env.BENCH_SIMULATE_ONLY === "1";
const OUTPUT_PATH = path.resolve(
  ROOT,
  process.env.BENCH_OUTPUT || ".superstack/composite-cu-revalidation-2026-07-12.json",
);
const ARTIFACT = process.env.SBF_ARTIFACT || null;
const ARTIFACT_PATH = ARTIFACT === null ? null : path.resolve(ROOT, ARTIFACT);
const PROGRAM_ID = new PublicKey(
  process.env.SBF_PROGRAM_ID || "BdR4cSgZGQgXNo33SZSYQXy7XgEK61sHT4NQaAkc3PBm",
);

const connection = new Connection(RPC, "confirmed");
const payer = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync(WALLET, "utf8"))));

function discriminator(name) {
  return crypto.createHash("sha256").update(`global:${name}`).digest().subarray(0, 8);
}

function unsignedLE(value, bytes) {
  let v = BigInt(value);
  if (v < 0n) throw new Error(`negative unsigned value ${value}`);
  const out = Buffer.alloc(bytes);
  for (let i = 0; i < bytes; i += 1) {
    out[i] = Number(v & 255n);
    v >>= 8n;
  }
  if (v !== 0n) throw new Error(`value ${value} does not fit u${bytes * 8}`);
  return out;
}

function signedLE(value, bytes) {
  let v = BigInt(value);
  const modulus = 1n << BigInt(bytes * 8);
  if (v < 0n) v += modulus;
  return unsignedLE(v, bytes);
}

const U128 = (value) => unsignedLE(value, 16);
const I128 = (value) => signedLE(value, 16);
const I64 = (value) => signedLE(value, 8);

function instruction(name, encodedArgs) {
  return new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [],
    data: Buffer.concat([discriminator(name), ...encodedArgs]),
  });
}

function load(filename, directory = VECTOR_ROOT) {
  return () => {
    const raw = JSON.parse(fs.readFileSync(path.join(directory, filename), "utf8"));
    return Array.isArray(raw) ? raw : raw.vectors;
  };
}

function stratified(values, count = LIMIT) {
  if (values.length <= count) return values;
  return Array.from({ length: count }, (_, index) => values[Math.floor(index * values.length / count)]);
}

function percentile(sorted, percent) {
  if (sorted.length === 0) return 0;
  return sorted[Math.min(sorted.length - 1, Math.ceil(percent / 100 * sorted.length) - 1)];
}

function summarize(name, rows) {
  const measured = rows.filter((row) => Number.isInteger(row.cu));
  const values = measured.map((row) => row.cu).sort((a, b) => a - b);
  const transactionValues = measured
    .map((row) => row.transactionCu)
    .filter(Number.isInteger)
    .sort((a, b) => a - b);
  const accepted = measured.filter((row) => row.succeeded).length;
  const acceptedValues = measured
    .filter((row) => row.succeeded)
    .map((row) => row.cu)
    .sort((a, b) => a - b);
  const average = values.length === 0 ? 0 : Math.round(values.reduce((sum, value) => sum + value, 0) / values.length);
  const acceptedAverage = acceptedValues.length === 0
    ? 0
    : Math.round(acceptedValues.reduce((sum, value) => sum + value, 0) / acceptedValues.length);
  const transactionAverage = transactionValues.length === 0
    ? 0
    : Math.round(transactionValues.reduce((sum, value) => sum + value, 0) / transactionValues.length);
  return {
    name,
    total: rows.length,
    measured: measured.length,
    accepted,
    rejected: measured.length - accepted,
    errors: rows.length - measured.length,
    min: values[0] || 0,
    average,
    median: percentile(values, 50),
    p95: percentile(values, 95),
    p99: percentile(values, 99),
    max: values.at(-1) || 0,
    accepted_average: acceptedAverage,
    accepted_median: percentile(acceptedValues, 50),
    accepted_p95: percentile(acceptedValues, 95),
    accepted_p99: percentile(acceptedValues, 99),
    accepted_max: acceptedValues.at(-1) || 0,
    transaction_average: transactionAverage,
    transaction_p99: percentile(transactionValues, 99),
    transaction_max: transactionValues.at(-1) || 0,
  };
}

async function getTransaction(signature) {
  for (let attempt = 0; attempt < 10; attempt += 1) {
    const transaction = await connection.getTransaction(signature, {
      commitment: "confirmed",
      maxSupportedTransactionVersion: 0,
    });
    if (transaction) return transaction;
    await new Promise((resolve) => setTimeout(resolve, 100 * (attempt + 1)));
  }
  return null;
}

async function measureOne(ix) {
  for (let attempt = 0; attempt < 4; attempt += 1) {
    try {
      const transaction = new Transaction().add(
        ComputeBudgetProgram.setComputeUnitLimit({ units: 1_400_000 }),
        ix,
      );
      if (SIMULATE_ONLY) {
        transaction.feePayer = payer.publicKey;
        const simulation = await connection.simulateTransaction(transaction);
        const logs = simulation.value.logs || [];
        const remaining = logs.flatMap((line) => {
          const match = line.match(/consumption:\s*(\d+)\s*units remaining/);
          return match ? [Number.parseInt(match[1], 10)] : [];
        });
        const successLine = logs.find((line) => line.includes("succeeded ="));
        return {
          signature: null,
          cu: remaining.length >= 2 ? remaining[0] - remaining[1] : null,
          transactionCu: Number(simulation.value.unitsConsumed ?? NaN),
          succeeded: successLine?.includes("true") || false,
          error: simulation.value.err === null && remaining.length >= 2
            ? null
            : JSON.stringify(simulation.value.err) || "missing CU markers",
        };
      }
      const signature = await sendAndConfirmTransaction(connection, transaction, [payer], {
        commitment: "confirmed",
        skipPreflight: false,
      });
      const details = await getTransaction(signature);
      const logs = details?.meta?.logMessages || [];
      const remaining = logs.flatMap((line) => {
        const match = line.match(/consumption:\s*(\d+)\s*units remaining/);
        return match ? [Number.parseInt(match[1], 10)] : [];
      });
      const successLine = logs.find((line) => line.includes("succeeded ="));
      return {
        signature,
        cu: remaining.length >= 2 ? remaining[0] - remaining[1] : null,
        transactionCu: Number(details?.meta?.computeUnitsConsumed ?? NaN),
        succeeded: successLine?.includes("true") || false,
        error: remaining.length >= 2 ? null : "missing CU markers",
      };
    } catch (error) {
      if (attempt === 3) return { cu: null, succeeded: false, error: String(error) };
      await new Promise((resolve) => setTimeout(resolve, 250 * (attempt + 1)));
    }
  }
  return { cu: null, succeeded: false, error: "unreachable" };
}

async function runSuite(name, vectorSource, encode) {
  const vectors = typeof vectorSource === "function" ? vectorSource() : vectorSource;
  const sampled = stratified(vectors);
  const rows = [];
  for (let offset = 0; offset < sampled.length; offset += CONCURRENCY) {
    const batch = sampled.slice(offset, offset + CONCURRENCY);
    rows.push(...await Promise.all(batch.map((value) => measureOne(encode(value)))));
    if ((offset + batch.length) % 500 === 0 || offset + batch.length === sampled.length) {
      process.stdout.write(`${name}: ${offset + batch.length}/${sampled.length}\n`);
    }
  }
  const result = summarize(name, rows);
  const sampleError = rows.find((row) => row.error)?.error;
  if (sampleError) process.stdout.write(`${name} sample error: ${sampleError}\n`);
  process.stdout.write(`${JSON.stringify(result)}\n`);
  return { result, rows };
}

function bvnVectors() {
  let state = 0x9e3779b9n;
  const next = () => {
    state = (1664525n * state + 1013904223n) & 0xffffffffn;
    return Number(state) / 0x100000000;
  };
  return Array.from({ length: Math.max(LIMIT, 2000) }, (_, index) => ({
    a: BigInt(Math.round((-4 + 8 * next()) * 1e12)).toString(),
    b: BigInt(Math.round((-4 + 8 * next()) * 1e12)).toString(),
    rho: index % 20 === 0
      ? (index % 40 === 0 ? "990000000000" : "-990000000000")
      : BigInt(Math.round((-0.95 + 1.9 * next()) * 1e12)).toString(),
  }));
}

function americanKbiVectors() {
  const strike = 100_000_000_000_000n;
  let state = 0xa11ce55n;
  const next = () => {
    state = (1664525n * state + 1013904223n) & 0xffffffffn;
    return Number(state) / 0x100000000;
  };
  return Array.from({ length: Math.max(LIMIT, 2000) }, (_, index) => {
    const logMoneyness = -0.75 + 1.5 * index / (Math.max(LIMIT, 2000) - 1);
    const rate = index % 41 === 0 ? 0 : 0.12 * next();
    const dividendYield = index % 43 === 0 ? 0.12 : 0.12 * next();
    const sigma = index % 47 === 0 ? 0.10 : (index % 53 === 0 ? 1.20 : 0.10 + 1.10 * next());
    const maturity = index % 59 === 0 ? 30 / 365 : (index % 61 === 0 ? 2 : 30 / 365 + (2 - 30 / 365) * next());
    return {
      spot: BigInt(Math.round(Number(strike) * Math.exp(logMoneyness))).toString(),
      strike: strike.toString(),
      rate: BigInt(Math.round(rate * 1e12)).toString(),
      dividendYield: BigInt(Math.round(dividendYield * 1e12)).toString(),
      sigma: BigInt(Math.round(sigma * 1e12)).toString(),
      maturity: BigInt(Math.round(maturity * 1e12)).toString(),
    };
  });
}

function nigRuntimeVectors() {
  let state = 0x4e494720n;
  const next = () => {
    state = (1664525n * state + 1013904223n) & 0xffffffffn;
    return Number(state) / 0x100000000;
  };
  const raw = (value) => BigInt(Math.round(value * 1e12)).toString();
  return Array.from({ length: Math.max(LIMIT, 2000) }, (_, index) => {
    const time = 0.01 + 4.99 * next();
    const alpha = 2.05 + 97.95 * next();
    const low = Math.max(-0.64 * alpha, -1 - 0.64 * alpha);
    const high = Math.min(0.64 * alpha, -1 + 0.64 * alpha);
    const beta = low + (high - low) * next();
    const minDelta = 0.00101 / time;
    const maxDelta = Math.min(14.99, 14.99 / time);
    const delta = minDelta * Math.pow(maxDelta / minDelta, next());
    const rate = -0.249 + 0.498 * next();
    const dividend = -0.249 + 0.498 * next();
    const logForward = -1.99 + 3.98 * next();
    const spot = 100;
    const strike = spot / Math.exp(logForward - (rate - dividend) * time);
    const notional = Math.max(spot, strike);
    return {
      s: raw(spot),
      k: raw(strike),
      r: raw(rate),
      q: raw(dividend),
      t: raw(time),
      alpha: raw(alpha),
      beta: raw(beta),
      delta: raw(delta),
      requested: raw(notional * 5e-5),
      index,
    };
  });
}

function asianVectors() {
  let state = 0xa51a2026n;
  const next = () => {
    state = (1664525n * state + 1013904223n) & 0xffffffffn;
    return Number(state) / 0x100000000;
  };
  const raw = (value) => BigInt(Math.round(value * 1e12)).toString();
  return Array.from({ length: Math.max(LIMIT, 2000) }, (_, index) => {
    const spot = 20 + 480 * next();
    const time = 1 / 365 + (2 - 1 / 365) * next();
    const averagingTime = index % 5 === 0
      ? Math.min(time, 30 / (365 * 24 * 60))
      : Math.max(1 / (365 * 24), time * next());
    const fixedWeight = index % 3 === 0 ? 0 : 0.01 + 0.94 * next();
    return {
      s: raw(spot),
      k: raw(spot * (0.5 + next())),
      r: raw(0.2 * next()),
      q: raw(0.2 * next()),
      sigma: raw(0.05 + 1.95 * next()),
      t: raw(time),
      averagingTime: raw(averagingTime),
      fixedAverage: fixedWeight === 0 ? "0" : raw(spot * (0.7 + 0.6 * next())),
      fixedWeight: raw(fixedWeight),
    };
  });
}

function asianAdversarialVectors() {
  let state = 0x41534941n;
  const next = () => {
    state = (1664525n * state + 1013904223n) & 0xffffffffn;
    return Number(state) / 0x100000000;
  };
  const raw = (value) => BigInt(Math.max(1, Math.round(value * 1e12))).toString();
  const count = Math.max(LIMIT, 2000);
  return Array.from({ length: count }, (_, index) => {
    const mode = index % 12;
    const spot = 0.001 + 99_999.999 * next();
    const strike = 0.001 + 99_999.999 * next();
    let rate = 10 * next();
    let q = 10 * next();
    let sigma = 0.000001 + 99.999999 * next();
    let time = 0.000001 + 99.999999 * next();
    let averagingTime = time * (0.000001 + 0.999999 * next());
    let fixedWeight = next();

    if (mode === 0) {
      // One-raw-unit maturity/window: smallest structurally valid contract.
      time = 1e-12; averagingTime = time; sigma = 0.05; rate = 0; q = 0;
    } else if (mode === 1) {
      // Minute-scale in-progress TWAP with almost all observations fixed.
      time = 1 / (365 * 24 * 60); averagingTime = time; sigma = 2;
      rate = 0.2; q = 0.2; fixedWeight = 0.999999999999;
    } else if (mode === 2) {
      // Exercise the degree-8 series immediately below its |A|+|B| seam.
      sigma = 0.7; rate = 0.11; q = 0.03;
      averagingTime = 0.249999 / (sigma * sigma + 2 * Math.abs(rate - q));
      time = averagingTime;
    } else if (mode === 3) {
      // Exercise the closed form immediately above the same seam.
      sigma = 0.7; rate = 0.11; q = 0.03;
      averagingTime = 0.250001 / (sigma * sigma + 2 * Math.abs(rate - q));
      time = averagingTime;
    } else if (mode === 4) {
      // Long future start followed by a very short averaging window.
      sigma = 0.6; rate = 0.08; q = 0.02; time = 50;
      averagingTime = 30 / (365 * 24 * 60); fixedWeight = 0;
    } else if (mode === 5) {
      // Zero-carry second-moment branch.
      rate = 0.12; q = 0.12; sigma = 1.2; time = 2;
      averagingTime = 2;
    } else if (mode === 6) {
      // Tiny non-zero carry on each side of zero.
      rate = index % 24 === 6 ? 0.050000000001 : 0.05;
      q = index % 24 === 6 ? 0.05 : 0.050000000001;
      // V > 0.25 forces the dedicated small-carry expansion rather than the
      // origin bivariate series.
      sigma = 1.9; time = 0.5; averagingTime = 0.1;
    } else if (mode === 7) {
      // Near the accepted HP exponential boundary.
      rate = 0.39; q = 0; sigma = 0.01; time = 99.9;
      averagingTime = 0.01;
    } else if (mode === 8) {
      // Deep normal-CDF tail and small variance.
      sigma = 0.000001; rate = 0.01; q = 0; time = 1;
      averagingTime = 1; fixedWeight = 0;
    } else if (mode === 9) {
      // Broad high-volatility path, often expected to fail closed quickly.
      sigma = 100; rate = 10; q = 0; time = 100; averagingTime = 100;
    } else if (mode === 10) {
      // Fully fixed discounted-intrinsic fast path.
      fixedWeight = 1; averagingTime = 0; sigma = 1;
    } else {
      // Ordinary partial fixing with future weight still material.
      sigma = 0.05 + 1.95 * next(); rate = 0.2 * next(); q = 0.2 * next();
      time = 1 / 365 + (2 - 1 / 365) * next();
      averagingTime = time * (0.001 + 0.999 * next());
      fixedWeight = 0.01 + 0.98 * next();
    }

    const fullyFixed = fixedWeight >= 1;
    const noFixedPart = !fullyFixed && fixedWeight === 0;
    return {
      s: raw(spot),
      k: raw(strike),
      r: raw(rate),
      q: raw(q),
      sigma: raw(sigma),
      t: raw(time),
      averagingTime: fullyFixed ? "0" : raw(Math.min(time, averagingTime)),
      fixedAverage: noFixedPart ? "0" : raw(0.001 + 99_999.999 * next()),
      fixedWeight: fullyFixed ? "1000000000000" : raw(Math.min(fixedWeight, 0.999999999999)),
    };
  });
}

const suites = [
  ["ln_fixed_i", load("prod_ln_vectors.json"), (v) => instruction("compute_ln", [U128(v.x)])],
  ["exp_fixed_i", load("prod_exp_vectors.json"), (v) => instruction("compute_exp", [I128(v.x)])],
  ["norm_cdf_poly", load("prod_norm_cdf_vectors.json"), (v) => instruction("compute_cdf", [I128(v.x)])],
  ["ln_fixed_hp", load("prod_ln_vectors.json"), (v) => instruction("compute_ln_hp", [I128(BigInt(v.x) * 1000n)])],
  ["exp_fixed_hp", load("prod_exp_vectors.json"), (v) => instruction("compute_exp_hp", [I128(BigInt(v.x) * 1000n)])],
  ["norm_cdf_poly_hp", load("prod_norm_cdf_vectors.json"), (v) => instruction("compute_cdf_hp", [I128(BigInt(v.x) * 1000n)])],
  ["fp_div_hp_safe", load("prod_norm_cdf_vectors.json"), (v) => instruction("compute_div_hp", [I128(BigInt(v.x) * 1000n + 2_000_000_000_000_000n), I128(3_000_000_000_000_000n)])],
  ["norm_pdf", load("prod_norm_pdf_vectors.json"), (v) => instruction("compute_pdf", [I128(v.x)])],
  ["pow_fixed", load("prod_pow_fixed_vectors.json"), (v) => instruction("compute_pow", [U128(v.base), U128(v.exp)])],
  ["pow_fixed_i", load("prod_pow_fixed_i_vectors.json"), (v) => instruction("compute_pow_i", [I128(v.base), I128(v.exp)])],
  ["pow_int", load("prod_pow_int_vectors.json"), (v) => instruction("compute_pow_int", [U128(v.base), U128(v.n)])],
  ["inverse_norm_cdf", load("prod_inverse_norm_cdf_vectors.json"), (v) => instruction("compute_inverse_cdf", [I128(v.p)])],
  ["norm_cdf_and_pdf", load("prod_cdf_pdf_vectors.json"), (v) => instruction("compute_cdf_pdf", [I128(v.x)])],
  ["norm_cdf_and_pdf_poly", load("prod_cdf_pdf_vectors.json"), (v) => instruction("compute_cdf_pdf_poly", [I128(v.x)])],
  ["fp_mul_i_round", bvnVectors(), (v) => instruction("compute_mul_i_round", [I128(v.a), I128(v.b)])],
  ["fp_mul_i_fast", bvnVectors(), (v) => instruction("compute_mul_i_fast", [I128(v.a), I128(v.b)])],
  ["fp_mul_i_fast_round", bvnVectors(), (v) => instruction("compute_mul_i_fast_round", [I128(v.a), I128(v.b)])],
  ["fp_div_i", bvnVectors(), (v) => instruction("compute_div_i", [I128(v.a), I128(v.b === "0" ? "1000000000000" : v.b)])],
  ["black_scholes_price", load("prod_black_scholes_price_vectors.json"), (v) => instruction("compute_bs_price", [U128(v.s), U128(v.k), U128(v.r), U128(v.sigma), U128(v.t)])],
  ["bs_full", load("prod_bs_full_vectors.json"), (v) => instruction("compute_bs_full", [U128(v.s), U128(v.k), U128(v.r), U128(v.sigma), U128(v.t)])],
  ["bs_delta", load("prod_greeks_vectors.json"), (v) => instruction("compute_bs_delta", [U128(v.s), U128(v.k), U128(v.r), U128(v.sigma), U128(v.t)])],
  ["bs_gamma", load("prod_greeks_vectors.json"), (v) => instruction("compute_bs_gamma", [U128(v.s), U128(v.k), U128(v.r), U128(v.sigma), U128(v.t)])],
  ["bs_vega", load("prod_greeks_vectors.json"), (v) => instruction("compute_bs_vega", [U128(v.s), U128(v.k), U128(v.r), U128(v.sigma), U128(v.t)])],
  ["bs_theta", load("prod_greeks_vectors.json"), (v) => instruction("compute_bs_theta", [U128(v.s), U128(v.k), U128(v.r), U128(v.sigma), U128(v.t)])],
  ["bs_rho", load("prod_greeks_vectors.json"), (v) => instruction("compute_bs_rho", [U128(v.s), U128(v.k), U128(v.r), U128(v.sigma), U128(v.t)])],
  ["implied_vol", load("prod_implied_vol_vectors.json"), (v) => instruction("compute_iv", [U128(v.call_price), U128(v.s), U128(v.k), U128(v.r), U128(v.t)])],
  ["arithmetic_asian_price", asianVectors(), (v) => instruction("compute_asian", [U128(v.s), U128(v.k), U128(v.r), U128(v.q), U128(v.sigma), U128(v.t), U128(v.averagingTime), U128(v.fixedAverage), U128(v.fixedWeight)])],
  ["arithmetic_asian_price_adversarial", asianAdversarialVectors(), (v) => instruction("compute_asian", [U128(v.s), U128(v.k), U128(v.r), U128(v.q), U128(v.sigma), U128(v.t), U128(v.averagingTime), U128(v.fixedAverage), U128(v.fixedWeight)])],
  ["sabr_implied_vol", load("sabr_vectors.json", TEST_DATA_ROOT), (v) => instruction("compute_sabr_vol", [U128(v.F_fp), U128(v.K_fp), U128(v.T_fp), U128(v.alpha_fp), U128(v.beta_fp), I128(v.rho_fp), U128(v.nu_fp)])],
  ["sabr_price", load("sabr_vectors.json", TEST_DATA_ROOT), (v) => instruction("compute_sabr_price", [U128(v.F_fp), U128(v.K_fp), U128(0), U128(v.T_fp), U128(v.alpha_fp), U128(v.beta_fp), I128(v.rho_fp), U128(v.nu_fp)])],
  ["sabr_greeks", load("sabr_vectors.json", TEST_DATA_ROOT), (v) => instruction("compute_sabr_greeks", [U128(v.F_fp), U128(v.K_fp), U128(0), U128(v.T_fp), U128(v.alpha_fp), U128(v.beta_fp), I128(v.rho_fp), U128(v.nu_fp)])],
  ["sabr_precompute_and_vol_at", load("sabr_vectors.json", TEST_DATA_ROOT), (v) => instruction("compute_sabr_precomputed_vol", [U128(v.F_fp), U128(v.K_fp), U128(v.T_fp), U128(v.alpha_fp), U128(v.beta_fp), I128(v.rho_fp), U128(v.nu_fp)])],
  ["sabr_precompute", load("sabr_vectors.json", TEST_DATA_ROOT), (v) => instruction("compute_sabr_precompute", [U128(v.F_fp), U128(v.T_fp), U128(v.alpha_fp), U128(v.beta_fp), I128(v.rho_fp), U128(v.nu_fp)])],
  ["sabr_vol_at", load("sabr_vectors.json", TEST_DATA_ROOT), (v) => instruction("compute_sabr_vol_at", [U128(v.F_fp), U128(v.K_fp), U128(v.T_fp), U128(v.alpha_fp), U128(v.beta_fp), I128(v.rho_fp), U128(v.nu_fp)])],
  ["sabr_z_over_chi_pade", load("prod_sabr_z_over_chi_vectors.json"), (v) => instruction("compute_sabr_z_over_chi", [I128(v.z), I128(v.rho)])],
  ["bvn_cdf", bvnVectors(), (v) => instruction("compute_bvn", [I128(v.a), I128(v.b), I128(v.rho)])],
  ["bvn_cdf_hp", bvnVectors(), (v) => instruction("compute_bvn_hp", [I128(v.a), I128(v.b), I128(v.rho)])],
  ["heston_deterministic", () => load("heston_vectors.json", TEST_DATA_ROOT)().map((v) => ({ ...v, xi_fp: "0" })), (v) => instruction("compute_heston", [U128(v.S_fp), U128(v.K_fp), U128(v.r_fp), U128(v.T_fp), U128(v.v0_fp), U128(v.kappa_fp), U128(v.theta_fp), U128(v.xi_fp), I128(v.rho_fp)])],
  ["heston_stochastic_reject", load("heston_vectors.json", TEST_DATA_ROOT), (v) => instruction("compute_heston", [U128(v.S_fp), U128(v.K_fp), U128(v.r_fp), U128(v.T_fp), U128(v.v0_fp), U128(v.kappa_fp), U128(v.theta_fp), U128(v.xi_fp), I128(v.rho_fp)])],
  ["nig_i128_reject", load("nig_call_price_scale_vectors.json"), (v) => instruction("compute_nig", [U128(v.s), U128(v.k), U128(v.r), U128(v.t), U128(v.alpha), I128(v.beta), U128(v.delta)])],
  ["nig_price_certified", nigRuntimeVectors(), (v) => instruction("compute_nig_certified", [U128(v.s), U128(v.k), I128(v.r), I128(v.q), U128(v.t), U128(v.alpha), I128(v.beta), U128(v.delta), U128(v.requested)])],
  ["nig_i64_call_reject", load("nig_cos_vectors.json"), (v) => instruction("compute_nig_64", [I64(v.s), I64(v.k), I64(v.r), I64(v.t), I64(v.alpha), I64(v.beta), I64(v.delta_param)])],
  ["nig_i64_put_reject", load("nig_cos_vectors.json"), (v) => instruction("compute_nig_put_64", [I64(v.s), I64(v.k), I64(v.r), I64(v.t), I64(v.alpha), I64(v.beta), I64(v.delta_param)])],
  ["Phi2Table.eval", bvnVectors(), (v) => instruction("compute_phi2_eval", [I128(v.a), I128(v.b)])],
  ["american_kbi_call", americanKbiVectors(), (v) => instruction("compute_american_kbi_call", [U128(v.spot), U128(v.strike), U128(v.rate), U128(v.dividendYield), U128(v.sigma), U128(v.maturity)])],
  ["american_kbi_put", americanKbiVectors(), (v) => instruction("compute_american_kbi_put", [U128(v.spot), U128(v.strike), U128(v.rate), U128(v.dividendYield), U128(v.sigma), U128(v.maturity)])],
];

const output = {
  run_date: new Date().toISOString(),
  target: process.env.BENCH_TARGET || `Agave local validator at ${RPC}`,
  measurement: SIMULATE_ONLY
    ? "unsigned simulateTransaction; math CU is the difference between sol_log_compute_units markers and full CU is unitsConsumed"
    : "signed transaction; math CU is the difference between sol_log_compute_units markers and full CU is unitsConsumed",
  program_id: PROGRAM_ID.toBase58(),
  artifact: ARTIFACT,
  artifact_sha256: ARTIFACT_PATH === null
    ? null
    : crypto.createHash("sha256").update(fs.readFileSync(ARTIFACT_PATH)).digest("hex"),
  artifact_bytes: ARTIFACT_PATH === null ? null : fs.statSync(ARTIFACT_PATH).size,
  limit_per_suite: LIMIT,
  concurrency: CONCURRENCY,
  results: [],
};

const requestedSuites = new Set(
  (process.env.BENCH_SUITES || "").split(",").map((name) => name.trim()).filter(Boolean),
);
const selectedSuites = requestedSuites.size === 0
  ? suites
  : suites.filter(([name]) => requestedSuites.has(name));

for (const [name, vectors, encode] of selectedSuites) {
  const { result } = await runSuite(name, vectors, encode);
  output.results.push(result);
  fs.writeFileSync(OUTPUT_PATH, `${JSON.stringify(output, null, 2)}\n`);
}

process.stdout.write(`Wrote ${OUTPUT_PATH}\n`);
