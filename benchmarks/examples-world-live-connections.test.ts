import { mkdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { describe, it } from "node:test";
import assert from "node:assert/strict";

import {
  runWorldConnectionProbes,
  writeWorldConnectionReceipt,
  worldConnectionReceiptPath,
} from "../examples/world/lib/world/connections/runner.ts";
import { worldConnectionProbes } from "../examples/world/lib/world/connections/providers/index.ts";
import { hasLeakedEnvValue } from "../examples/world/lib/world/connections/redaction.ts";

const repoRoot = process.cwd();

function providerEnvValues(env: Record<string, string | undefined>): Record<string, string | undefined> {
  const names = new Set<string>();

  for (const probe of worldConnectionProbes) {
    for (const name of probe.requiredEnv) {
      names.add(name);
    }

    for (const name of probe.optionalEnv ?? []) {
      names.add(name);
    }
  }

  return Object.fromEntries([...names].map((name) => [name, env[name]]));
}

describe("examples/world live connection runner", () => {
  it("runs real probes when credentials are present and writes a redacted receipt", async () => {
    mkdirSync(dirname(join(repoRoot, worldConnectionReceiptPath)), { recursive: true });

    const receipt = await runWorldConnectionProbes({
      env: process.env,
      allowNetwork: true,
      includeCli: true,
      timeoutMs: 5000,
    });

    await writeWorldConnectionReceipt(receipt);

    const written = JSON.parse(readFileSync(join(repoRoot, worldConnectionReceiptPath), "utf8"));
    const vercelCli = receipt.results.find((result) => result.id === "vercel-cli-whoami");

    assert.equal(receipt.schema, "dx.examples.world.live-connections");
    assert.equal(receipt.redaction, "secret-values-never-included");
    assert.equal(written.schema, receipt.schema);
    assert.ok(receipt.results.length >= 15);
    assert.ok(vercelCli, "Vercel CLI probe should be part of the live runner");
    assert.equal(hasLeakedEnvValue(receipt, providerEnvValues(process.env)), false);
  });

  it("does not leak raw provider env values in configured-readiness mode", async () => {
    const fakeEnv = {
      OPENAI_API_KEY: "sk-test-openai-secret-never-print",
      STRIPE_SECRET_KEY: "sk_test_stripe_secret_never_print",
      VERCEL_TOKEN: "vercel-token-never-print",
      TURSO_DATABASE_URL: "libsql://example.turso.io",
      TURSO_AUTH_TOKEN: "turso-token-never-print",
    };

    const receipt = await runWorldConnectionProbes({
      env: fakeEnv,
      allowNetwork: false,
      includeCli: false,
      timeoutMs: 10,
    });

    assert.equal(receipt.totals.configuredReadiness > 0, true);
    assert.equal(hasLeakedEnvValue(receipt, fakeEnv), false);
    assert.doesNotMatch(JSON.stringify(receipt), /never-print/);
  });
});
