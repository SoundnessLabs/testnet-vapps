#!/usr/bin/env node
const fs = require("fs");
const { execSync } = require("child_process");
const path = require("path");

const LOG_FILE = path.resolve(__dirname, "cli_usage_log.json");
const PROOF_ON_EACH_RUN = process.env.PROOF_ON_EACH_RUN === "1"; // optional

function readLogs() {
  if (!fs.existsSync(LOG_FILE)) return [];
  try { return JSON.parse(fs.readFileSync(LOG_FILE, "utf8")); }
  catch { return []; }
}

function writeLogs(logs) {
  fs.writeFileSync(LOG_FILE, JSON.stringify(logs, null, 2));
}

function logUsage(command, status, durationMs) {
  const entry = {
    command,
    status,            // "success" | "error"
    duration_ms: durationMs,
    timestamp: new Date().toISOString(),
  };
  const logs = readLogs();
  logs.push(entry);
  writeLogs(logs);
  return entry;
}

function maybeGenerateProof() {
  try {
    execSync(`soundness-cli proof generate --input ${LOG_FILE} --output proof.json`, {
      stdio: "inherit",
      cwd: __dirname,
    });
    return true;
  } catch (e) {
    console.error("Failed to generate proof:", e?.message || e);
    return false;
  }
}

async function main() {
  const args = process.argv.slice(2);
  if (args.length === 0) {
    console.error("Usage: node tracker.js <soundness-cli ...args>");
    process.exit(1);
  }

  // Ensure command starts with 'soundness-cli'
  const cmd = args[0] === "soundness-cli" ? args.join(" ") : ["soundness-cli", ...args].join(" ");

  const start = Date.now();
  let status = "success";
  try {
    execSync(cmd, { stdio: "inherit", cwd: process.cwd() });
  } catch (e) {
    status = "error";
  } finally {
    const duration = Date.now() - start;
    const entry = logUsage(cmd, status, duration);
    console.log("\n[tracker] Logged:", entry);
  }

  if (PROOF_ON_EACH_RUN) {
    console.log("[tracker] Generating proof for latest logs…");
    maybeGenerateProof();
  }
}
main();
