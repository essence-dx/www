import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

test("media-icon manifest cache uses the shared dx-serializer machine API", () => {
  const cargoToml = read("related-crates/media-icon/Cargo.toml");
  const lib = read("related-crates/media-icon/src/lib.rs");
  const manifest = read("related-crates/media-icon/src/machine_manifest.rs");

  assert.match(cargoToml, /dx-serializer = \{ path = "\.\.\/\.\.\/\.\.\/serializer", default-features = false, features = \["converters", "mmap"\] \}/);
  assert.match(cargoToml, /blake3 = \{ workspace = true \}/);
  assert.match(cargoToml, /rkyv = \{ version = "0\.8", features = \["bytecheck"\] \}/);
  assert.match(lib, /pub mod machine_manifest;/);

  assert.match(manifest, /MachineCacheKind::Index/);
  assert.match(manifest, /write_typed_machine_cache/);
  assert.match(manifest, /open_typed_machine_cache::<IconManifestMachineV1>/);
  assert.match(manifest, /icon_manifest_source_fingerprint/);
  assert.match(manifest, /\.dx\/icon\/machine\/v1/);
  assert.match(manifest, /DX_ICON_DATA/);
  assert.match(manifest, /"full_icon_search_speed_claimed": false/);
  assert.match(manifest, /"faster_than_upstream_claimed": false/);
  assert.match(manifest, /"upstream_baseline_measured": false/);
  assert.match(manifest, /"upstream_baseline_command": null/);
  assert.match(manifest, /"upstream_baseline_checkout": null/);
  assert.match(manifest, /"same_machine_benchmark_required": true/);
  assert.match(manifest, /"test_command": null/);
  assert.match(manifest, /"test_command_recorded": false/);
});

test("generated icon and media machine cache roots stay local-only", () => {
  const gitignore = read(".gitignore");

  assert.match(gitignore, /^\.dx\/icon\/$/m);
  assert.match(gitignore, /^\.dx\/media\/$/m);
  assert.match(gitignore, /^\.dx\/performance\/$/m);
});
