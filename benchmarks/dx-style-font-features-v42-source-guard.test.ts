import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

function readRequiredFile(path: string): string {
  return readFileSync(path, "utf8");
}

function escaped(marker: string): RegExp {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

test("dx-style Tailwind v4.2 font-feature utilities generate CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    "fn font_feature_settings_utility(class_name: &str) -> Option<String>",
    ".or_else(|| font_feature_settings_utility(class_name))",
    'generate_utility_css("font-features-[\'smcp\',\'onum\']").unwrap()',
    'generate_utility_css("font-features-(--dx-font-features)").unwrap()',
    'assert!(generate_utility_css("font-features-[bad;value]").is_none())',
  ]) {
    assert.match(utility, escaped(marker));
  }

  for (const marker of [
    'class_name: "font-features-[\'smcp\',\'onum\']"',
    'class_name: "font-features-(--dx-font-features)"',
    'area: "tailwind-v4.2-font-features"',
  ]) {
    assert.match(parity, escaped(marker));
  }

  assert.match(matrix, /Tailwind v4\.2 font-feature utilities now have targeted guards/i);
  assert.match(`${dx}\n${todo}\n${changelog}`, /Tailwind v4\.2 font-feature utilities/i);
});
