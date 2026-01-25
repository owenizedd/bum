#!/usr/bin/env node

const { run } = require("./index");
const { execSync } = require("child_process");
const { join } = require("path");
const { homedir } = require("os");
const { existsSync } = require("fs");

const args = process.argv.slice(2);

function ensureBun() {
  const bunBinName = process.platform === "win32" ? "bun.exe" : "bun";
  const bunDir = join(homedir(), ".bun", "bin");
  const bunPath = join(bunDir, bunBinName);
  
  // Fast path: If Bun exists in the default location, prepending it to PATH is cheap and safe.
  if (existsSync(bunPath)) {
    process.env.PATH = `${bunDir}${require("path").delimiter}${process.env.PATH}`;
    return;
  }

  try {
    // Fallback: Check if bun is in PATH (e.g. installed via brew/system)
    execSync("bun --version", { stdio: "ignore" });
  } catch (e) {
    // Not found anywhere -> install
    installBun();
    // Add to PATH after install
    process.env.PATH = `${bunDir}${require("path").delimiter}${process.env.PATH}`;
  }
}

function installBun() {
  console.log("Bun not found. Installing Bun...");
  try {
    if (process.platform === "win32") {
      execSync('powershell -c "irm bun.sh/install.ps1 | iex"', {
        stdio: "inherit",
      });
    } else {
      execSync("curl -fsSL https://bun.sh/install | bash", {
        stdio: "inherit",
      });
    }
    console.log("Bun installed successfully.");
  } catch (e) {
    console.error("Failed to install Bun. Please install it manually.");
    process.exit(1);
  }
}

ensureBun();

try {
  run(args);
} catch (e) {
  console.error(e);
  process.exit(1);
}

try {
  run(args);
} catch (e) {
  console.error(e);
  process.exit(1);
}
