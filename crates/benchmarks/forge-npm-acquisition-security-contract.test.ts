import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = process.cwd();
const acquisitionSource = readFileSync(
  join(repoRoot, "dx-www", "src", "cli", "forge_npm_acquisition.rs"),
  "utf8",
);
const archiveSource = readFileSync(
  join(repoRoot, "dx-www", "src", "cli", "forge_npm_archive.rs"),
  "utf8",
);

test("npm acquisition rejects mismatched registry integrity", () => {
  assert.match(acquisitionSource, /fn verify_npm_integrity\(/);
  assert.match(
    acquisitionSource,
    /if digest != expected\s*\{[\s\S]*?bail!\(\s*"npm dist\.integrity did not match downloaded tarball/,
  );
  assert.doesNotMatch(
    acquisitionSource,
    /Ok\(digest == expected\)/,
    "integrity mismatch must not become a soft false report",
  );
});

test("npm acquisition constrains registry and tarball network boundaries", () => {
  for (const marker of [
    "validate_npm_registry_url",
    "validate_npm_tarball_url",
    "validate_allowed_npm_url",
    "url_origin",
    "is_loopback_host",
    "content_length",
  ]) {
    assert.match(acquisitionSource, new RegExp(marker));
  }

  assert.match(acquisitionSource, /must use https, or http for loopback test registries/);
  assert.match(acquisitionSource, /must not contain credentials/);
  assert.match(acquisitionSource, /tarball URL origin must match/);
  assert.match(acquisitionSource, /declares \{\} bytes, above Forge acquire limit/);
});

test("npm acquisition receipts expose declared lifecycle scripts without executing them", () => {
  for (const field of [
    "lifecycle_scripts_declared",
    "lifecycle_script_names",
    "lifecycle_script_status",
    "inspect_npm_lifecycle_scripts",
  ]) {
    assert.match(acquisitionSource, new RegExp(field));
  }

  assert.match(acquisitionSource, /package_manager_execution_allowed:\s*false/);
  assert.match(acquisitionSource, /package_installs_run:\s*false/);
  assert.match(acquisitionSource, /lifecycle_scripts_executed:\s*false/);
  assert.match(acquisitionSource, /"lifecycle_scripts_declared"/);
  assert.match(acquisitionSource, /"lifecycle_script_names"/);
});

test("npm evidence markers are sanitized and do not overclaim provenance", () => {
  assert.match(acquisitionSource, /fn evidence_marker_value\(/);
  assert.match(acquisitionSource, /let provenance_verified = integrity_verified/);
  assert.match(acquisitionSource, /registry_integrity_verified=\{integrity_verified\}/);
  assert.doesNotMatch(
    acquisitionSource,
    /let license = selection\.license[\s\S]*license=\{license\}/,
    "package-controlled license text must be sanitized before evidence markers are written",
  );
  assert.doesNotMatch(
    acquisitionSource,
    /provenance_verified=true/,
    "npm acquisition must not mark provenance verified when registry integrity is missing",
  );
});

test("npm acquisition reuses the core Forge acquisition slug helper", () => {
  assert.match(acquisitionSource, /acquisition_package_slug/);
  assert.doesNotMatch(acquisitionSource, /fn forge_acquire_package_slug/);
});

test("npm archive extraction keeps traversal and symlink-like entries non-materialized", () => {
  for (const guard of ["ParentDir", "RootDir", "Prefix", "is_absolute", "node_modules"]) {
    assert.match(archiveSource, new RegExp(guard));
  }

  assert.match(archiveSource, /strip_prefix\("package\/"\)/);
  assert.match(archiveSource, /outside npm package root/);
  assert.match(archiveSource, /Component::Normal/);
  assert.match(archiveSource, /typeflag,\s*b'0'\s*\|\s*0/);
  assert.doesNotMatch(
    archiveSource,
    /typeflag == b'2'[\s\S]*?fs::/,
    "npm symlink entries must not be materialized",
  );
});
