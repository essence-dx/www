import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

function readRequiredFile(path: string): string {
  return readFileSync(path, "utf8");
}

function escaped(marker: string): RegExp {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

test("dx-style Tailwind v4.3 container-size utilities generate CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    "fn container_utility(class_name: &str) -> Option<String>",
    ".or_else(|| container_utility(class_name))",
    'generate_utility_css("@container-normal").unwrap()',
    'generate_utility_css("@container-normal/sidebar").unwrap()',
    'generate_utility_css("@container-size").unwrap()',
    'generate_utility_css("@container-size/main").unwrap()',
    'generate_utility_css("@container/main").unwrap()',
    'assert!(generate_utility_css("@container-size/[bad]").is_none())',
  ]) {
    assert.match(utility, escaped(marker));
  }

  for (const marker of [
    'class_name: "@container-normal"',
    'class_name: "@container-normal/sidebar"',
    'class_name: "@container-size"',
    'class_name: "@container-size/main"',
    'area: "tailwind-v4.3-container-type"',
  ]) {
    assert.match(parity, escaped(marker));
  }

  assert.match(matrix, /Tailwind v4\.3 container marker utilities now have targeted guards/i);
  assert.match(`${dx}\n${todo}\n${changelog}`, /Tailwind v4\.3 container-size utilities/i);
});
