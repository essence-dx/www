import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = process.cwd();
const forgeSecurity = readFileSync(
  join(repoRoot, "core", "src", "ecosystem", "forge_security.rs"),
  "utf8",
);

test("Forge package contract policies traffic from reviewed security evidence", () => {
  assert.match(
    forgeSecurity,
    /traffic: package_provenance_policy_traffic\(package\)/,
  );
  assert.match(
    forgeSecurity,
    /traffic: package_advisory_policy_traffic\(&package\.advisory_review\)/,
  );
  assert.match(
    forgeSecurity,
    /traffic: package_license_policy_traffic\(&package\.license_review\)/,
  );
});

test("Forge package contract helpers yellow unverified security metadata", () => {
  assert.match(
    forgeSecurity,
    /if package\.provenance\.verified \{[\s\S]*?DxUpdateTraffic::Green[\s\S]*?\} else \{[\s\S]*?DxUpdateTraffic::Yellow/,
  );
  assert.match(
    forgeSecurity,
    /let live_feed =[\s\S]*?DxForgeAdvisoryCoverageKind::LiveFeed[\s\S]*?review\.live_coverage/,
  );
  assert.match(
    forgeSecurity,
    /if reviewed_offline \|\| live_feed \{[\s\S]*?DxUpdateTraffic::Green[\s\S]*?\} else \{[\s\S]*?DxUpdateTraffic::Yellow/,
  );
  assert.match(
    forgeSecurity,
    /if review\.reviewed \{[\s\S]*?DxUpdateTraffic::Green[\s\S]*?\} else \{[\s\S]*?DxUpdateTraffic::Yellow/,
  );
});

test("Forge package contract tests cover yellow and green security metadata", () => {
  assert.match(
    forgeSecurity,
    /package_contract_policies_are_yellow_for_declared_but_unverified_security_metadata/,
  );
  assert.match(
    forgeSecurity,
    /package_contract_policies_are_green_for_reviewed_security_metadata/,
  );
});
