import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");
const demoServerPath = path.join(root, "demo", "server.rs");

test("fair-counter benchmark route uses the lean tiny response path", () => {
  const source = fs.readFileSync(demoServerPath, "utf8");

  assert.match(source, /async fn fair_counter\(\) -> impl IntoResponse \{\s*tiny_benchmark_html\(fair_counter_html\(\)\)\s*\}/);
  assert.match(source, /fn tiny_benchmark_html\(route: &'static DxOptimizedHtml\) -> impl IntoResponse/);
  assert.match(source, /"x-dx-delivery-mode", route\.profile\(\)\.delivery_mode\.as_str\(\)/);
  assert.match(source, /async fn medium_docs\(\) -> impl IntoResponse \{\s*benchmark_html\(real_docs_html\(\)\)\s*\}/);
  assert.match(source, /async fn big_dashboard\(\) -> impl IntoResponse \{\s*benchmark_html\(real_dashboard_html\(\)\)\s*\}/);
});
