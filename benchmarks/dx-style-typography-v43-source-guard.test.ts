import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

function readRequiredFile(path: string): string {
  return readFileSync(path, "utf8");
}

test("dx-style Tailwind v4.3 typography detail utilities generate CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    "fn typography_detail_utility(class_name: &str) -> Option<String>",
    '"wrap-anywhere" => Some("overflow-wrap: anywhere")',
    '"align-middle" => Some("vertical-align: middle")',
    'generate_utility_css("wrap-anywhere").unwrap()',
    'generate_utility_css("indent-8").unwrap()',
    'generate_utility_css("-indent-8").unwrap()',
    'generate_utility_css("indent-(--dx-indent)").unwrap()',
    'generate_utility_css("align-(--dx-vertical-align)").unwrap()',
    'generate_utility_css("decoration-4").unwrap()',
    'generate_utility_css("decoration-(length:--dx-decoration-thickness)").unwrap()',
    'generate_utility_css("underline-offset-4").unwrap()',
    'generate_utility_css("-underline-offset-2").unwrap()',
  ]) {
    assert.match(utility, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    'class_name: "wrap-anywhere"',
    'class_name: "indent-8"',
    'class_name: "align-middle"',
    'class_name: "decoration-4"',
    'class_name: "underline-offset-4"',
    'area: "tailwind-v4.3-typography"',
  ]) {
    assert.match(parity, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(matrix, /Tailwind v4\.3 typography detail utilities now have targeted guards/i);
  assert.match(`${dx}\n${todo}\n${changelog}`, /Tailwind v4\.3 typography detail utilities/i);
});
