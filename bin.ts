#!/usr/bin/env node

const { run } = require("./index");

const args = process.argv.slice(2);

try {
  run(args);
} catch (e) {
  console.error(e);
  process.exit(1);
}

