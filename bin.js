#!/usr/bin/env node

// bin.ts
var { run } = require("./index");
var args = process.argv.slice(2);
try {
  run(args);
} catch (e) {
  console.error(e);
  process.exit(1);
}
