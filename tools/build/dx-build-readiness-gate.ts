#!/usr/bin/env node

const { main } = require("./readiness-gate/cli.ts");

main(process.argv.slice(2));
