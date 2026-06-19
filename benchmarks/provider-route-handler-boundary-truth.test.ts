import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

function sourceBetween(source, startNeedle, endNeedle) {
  const start = source.indexOf(startNeedle);
  const end = source.indexOf(endNeedle, start + startNeedle.length);
  assert.notEqual(start, -1, `missing source marker: ${startNeedle}`);
  assert.notEqual(end, -1, `missing source marker: ${endNeedle}`);
  return source.slice(start, end);
}

test("Payments route handlers keep provider boundary separate from local config readiness", () => {
  const payments = read("core/src/delivery/route_handler_payments.rs");
  const readiness = sourceBetween(
    payments,
    "fn stripe_get_readiness_body",
    "fn stripe_post_readiness_body",
  );
  const postReadiness = sourceBetween(
    payments,
    "fn stripe_post_readiness_body",
    "fn stripe_checkout_boundary_body",
  );
  const checkout = sourceBetween(
    payments,
    "fn stripe_checkout_boundary_body",
    "fn stripe_bad_request_body",
  );
  const webhook = sourceBetween(
    payments,
    "fn stripe_webhook_boundary_body",
    "fn stripe_get_readiness_body",
  );

  for (const [name, source] of [
    ["stripe GET readiness", readiness],
    ["stripe POST readiness", postReadiness],
    ["stripe checkout", checkout],
    ["stripe webhook", webhook],
  ]) {
    assert.match(source, /providerBoundary": true/, `${name} must keep live provider boundary visible`);
    assert.match(source, /providerConfigured":/, `${name} must expose local provider config separately`);
    assert.match(source, /liveProviderExecution": false/, `${name} must keep live provider execution false`);
  }

  assert.doesNotMatch(payments, /providerBoundary": !configured/);
  assert.doesNotMatch(payments, /providerBoundary": !credentials_configured/);
});

test("Instant route handlers keep hosted provider boundary separate from local app-id readiness", () => {
  const compat = read("core/src/delivery/route_handler_compat.rs");
  const readiness = sourceBetween(
    compat,
    "fn instant_route_readiness_body",
    "fn instant_route_handler_post_body",
  );
  const postRoute = sourceBetween(
    compat,
    "fn instant_route_handler_post_body",
    "fn instant_app_id_configured",
  );

  for (const [name, source] of [
    ["Instant GET readiness", readiness],
    ["Instant POST route", postRoute],
  ]) {
    assert.match(source, /providerBoundary": true/, `${name} must keep hosted provider boundary visible`);
    assert.match(source, /providerConfigured": app_id_configured/, `${name} must expose app-id readiness separately`);
    assert.match(source, /liveProviderExecution": false/, `${name} must keep hosted execution false`);
  }

  assert.doesNotMatch(compat, /providerBoundary": !app_id_configured/);
});
