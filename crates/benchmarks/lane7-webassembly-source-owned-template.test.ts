import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

const wasmSourceFiles = [
  "examples/template/wasm/bindgen/loader.ts",
  "examples/template/wasm/bindgen/react.tsx",
  "examples/template/wasm/bindgen/readiness.ts",
  "examples/template/wasm/bindgen/metadata.ts",
  "examples/template/wasm/bindgen/README.md",
] as const;

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath: string) {
  return JSON.parse(read(relativePath));
}

test("WebAssembly Bridge materializes source-owned runtime helpers for the default template", () => {
  for (const relativePath of wasmSourceFiles) {
    assert.ok(
      fs.existsSync(path.join(root, relativePath)),
      `${relativePath} should be materialized source, not a black-box import`,
    );
  }

  const loader = read("examples/template/wasm/bindgen/loader.ts");
  assert.match(loader, /export async function loadWasmBindgenModule/);
  assert.match(loader, /export function inspectWasmBindgenResponse/);
  assert.match(loader, /WebAssembly\.instantiateStreaming/);
  assert.match(loader, /WebAssembly\.instantiate/);
  assert.doesNotMatch(loader, /\bwasm-pack\b/);
  assert.doesNotMatch(loader, /\bcargo\s+build\b/);
  assert.doesNotMatch(loader, /\b(?:npm install|pnpm install|yarn add)\b/);

  const react = read("examples/template/wasm/bindgen/react.tsx");
  assert.match(react, /export function useWasmBindgenModule/);
  assert.match(react, /useEffect/);
  assert.doesNotMatch(react, /node_modules/);

  const readiness = read("examples/template/wasm/bindgen/readiness.ts");
  assert.match(readiness, /export function createWasmBindgenTemplateReadiness/);
  assert.match(readiness, /runtimeProof: false/);
  assert.doesNotMatch(readiness, /WebAssembly\.instantiate/);
  assert.doesNotMatch(readiness, /\bfetch\(/);

  const metadata = read("examples/template/wasm/bindgen/metadata.ts");
  assert.match(metadata, /packageId: "wasm\/bindgen"/);
  assert.match(metadata, /officialPackageName: "WebAssembly Bridge"/);
  assert.match(metadata, /sourceMirror: "G:\/WWW\/inspirations\/wasm-bindgen"/);
  assert.match(metadata, /runtimeProof: false/);

  const interopStatus = read("examples/template/wasm-interop-status.tsx");
  assert.match(interopStatus, /@\/wasm\/bindgen\/loader/);
  assert.match(interopStatus, /@\/wasm\/bindgen\/react/);

  const dashboard = read(
    "examples/template/components/template-app/dashboard-page.tsx",
  );
  assert.match(dashboard, /createWasmBindgenTemplateReadiness/);
  assert.match(dashboard, /data-dx-wasm-readiness/);

  const materializer = read("tools/launch/materialize-www-template.ts");
  assert.match(materializer, /createWasmBindgenTemplateReadiness/);
  assert.match(materializer, /data-dx-wasm-readiness/);

  const receipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
  );
  for (const relativePath of wasmSourceFiles) {
    assert.ok(
      receipt.files.includes(relativePath),
      `${relativePath} should be tracked by the WebAssembly Bridge receipt`,
    );
    assert.match(receipt.file_hashes[relativePath], /^[a-f0-9]{64}$/);
  }

  const packageStatus = readJson(
    "examples/template/.dx/forge/package-status.json",
  );
  const visibility = packageStatus.package_lane_visibility.find(
    (entry: { package_id?: string }) => entry.package_id === "wasm/bindgen",
  );
  assert.ok(visibility, "WebAssembly Bridge package visibility should exist");
  const helperSurface = visibility.selected_surfaces.find(
    (surface: { surface_id?: string }) =>
      surface.surface_id === "webassembly-bridge-source-owned-runtime-helpers",
  );
  assert.ok(helperSurface, "source-owned runtime helper surface should be visible");
  assert.deepEqual(helperSurface.files, [...wasmSourceFiles]);
  for (const relativePath of wasmSourceFiles) {
    assert.match(helperSurface.file_hashes[relativePath], /^[a-f0-9]{64}$/);
    assert.ok(visibility.receipt_hash_refresh.tracked_files.includes(relativePath));
  }
});
