import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import test from "node:test";

const root = resolve(import.meta.dirname, "..");
const matrixPath = join(
  root,
  "related-crates/style/fixtures/tailwind-v43-official-fixture-matrix.json",
);
const dxStyleSupportPath = join(root, "dx-www/src/cli/dx_style_support.rs");
const publicToolsPath = join(root, "dx-www/src/cli/public_framework_tools.rs");
const dxStyleToolsPath = join(
  root,
  "dx-www/src/cli/public_framework_tools/dx_style.rs",
);
const styleParserPath = join(root, "related-crates/style/src/parser/mod.rs");
const plainTextScannerPath = join(root, "related-crates/style/src/parser/plain_text.rs");

function readRequiredFile(filePath: string): string {
  assert.ok(existsSync(filePath), `expected ${relative(root, filePath)} to exist`);
  return readFileSync(filePath, "utf8");
}

function escaped(marker: string): RegExp {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

test("dx-style source scanner canaries separate static strings from dynamic fragments", () => {
  const matrix = JSON.parse(readRequiredFile(matrixPath));
  const canaries = new Map(
    (matrix.sourceScannerCanaries?.canaries ?? []).map((entry: { id: string }) => [
      entry.id,
      entry,
    ]),
  );

  const objectMap = canaries.get("tsx-static-object-map") as
    | { tokens?: string[] }
    | undefined;
  assert.ok(objectMap, "source scanner matrix should keep static object-map canary");
  for (const token of ["bg-blue-600", "hover:bg-blue-500", "text-white"]) {
    assert.ok(objectMap.tokens?.includes(token), `static object-map should include ${token}`);
  }

  const dynamicDiagnostic = canaries.get("dynamic-fragment-diagnostic") as
    | { rejectedTokens?: string[]; description?: string }
    | undefined;
  assert.ok(
    dynamicDiagnostic,
    "source scanner matrix should carry dynamic-fragment diagnostic canary",
  );
  assert.match(
    dynamicDiagnostic.description ?? "",
    /source scanner boundary/i,
    "dynamic fragments should be classified as source scanner boundaries",
  );
  for (const token of [
    "bg-${color}-600",
    "hover:${state}:opacity-100",
    "p-${size}",
  ]) {
    assert.ok(
      dynamicDiagnostic.rejectedTokens?.includes(token),
      `dynamic-fragment diagnostic should reject ${token}`,
    );
  }

  const staticLiteralScan = canaries.get("tsx-static-array-and-helper-literals") as
    | { tokens?: string[]; rejectedTokens?: string[]; description?: string }
    | undefined;
  assert.ok(
    staticLiteralScan,
    "source scanner matrix should carry static array/helper literal canary",
  );
  assert.match(
    staticLiteralScan.description ?? "",
    /arrays, object maps, and helper calls/i,
  );
  for (const token of [
    "grid",
    "grid-cols-2",
    "disabled:opacity-50",
    "data-[state=open]:block",
  ]) {
    assert.ok(
      staticLiteralScan.tokens?.includes(token),
      `static literal scan should include ${token}`,
    );
  }
  for (const token of ["text-${tone}-600", "card shell copy"]) {
    assert.ok(
      staticLiteralScan.rejectedTokens?.includes(token),
      `static literal scan should reject ${token}`,
    );
  }

  const diagnosticReceipt = canaries.get("source-scan-diagnostic-receipt") as
    | { diagnostics?: string[]; description?: string }
    | undefined;
  assert.ok(
    diagnosticReceipt,
    "source scanner matrix should carry structured diagnostic receipt canary",
  );
  assert.match(diagnosticReceipt.description ?? "", /line and column/i);
  for (const diagnostic of [
    "dynamic-fragment",
    "object-key",
    "unsafe-candidate",
    "non-utility-candidate",
    "duplicate-candidate",
  ]) {
    assert.ok(
      diagnosticReceipt.diagnostics?.includes(diagnostic),
      `diagnostic receipt should include ${diagnostic}`,
    );
  }
});

test("dx-style unsupported scanned-class diagnostics name dynamic fragments explicitly", () => {
  const dxStyleSupport = readRequiredFile(dxStyleSupportPath);

  for (const marker of [
    "fn has_dynamic_class_fragment(class_name: &str) -> bool",
    "dynamic class fragment was scanned but cannot be generated statically",
    "unsupported_scan_reports_dynamic_class_fragments_as_source_boundaries",
    '"bg-${color}-600"',
    '"hover:${state}:opacity-100"',
    '"p-${size}"',
  ]) {
    assert.match(dxStyleSupport, escaped(marker));
  }

  assert.match(
    dxStyleSupport,
    /has_dynamic_class_fragment\(class_name\)[\s\S]{0,260}return Some\(\s*"dynamic class fragment was scanned but cannot be generated statically/,
    "dynamic fragment diagnostics should run before generic missing-utility diagnostics",
  );
});

test("dx-style style parser owns safe static plain-text source literal scanning", () => {
  const styleParser = readRequiredFile(styleParserPath);
  const plainTextScanner = readRequiredFile(plainTextScannerPath);

  assert.match(styleParser, /mod plain_text;/);
  assert.match(styleParser, /plain_text::scan_plain_text_class_tokens/);
  assert.match(styleParser, /source_diagnostics:/);
  assert.match(styleParser, /pub struct SourceScanDiagnostic/);
  assert.match(styleParser, /pub enum SourceScanDiagnosticKind/);
  assert.match(styleParser, /extracts_plain_text_static_class_literals_from_tsx_sources/);
  assert.match(styleParser, /reports_plain_text_source_scan_diagnostics_with_locations/);

  for (const marker of [
    "pub(crate) fn extract_plain_text_class_tokens(source: &str) -> Vec<String>",
    "pub(crate) fn scan_plain_text_class_tokens(source: &str) -> PlainTextScanReport",
    "fn is_plain_text_class_candidate(token: &str) -> bool",
    "plain_text_extraction_reads_static_arrays_object_maps_and_helpers",
    "plain_text_extraction_rejects_dynamic_fragments_and_prose",
    "plain_text_diagnostics_report_dynamic_object_key_unsafe_prose_and_duplicates",
    "SourceScanDiagnosticKind::DynamicFragment",
    "SourceScanDiagnosticKind::DuplicateCandidate",
    '"data-[state=open]:block"',
    '"disabled:opacity-50"',
    "text-${tone}-600",
    '"card shell copy"',
  ]) {
    assert.match(plainTextScanner, escaped(marker));
  }
});

test("dx style build and check receipts surface source-scan diagnostics with locations", () => {
  const dxStyleSupport = readRequiredFile(dxStyleSupportPath);
  const publicTools = readRequiredFile(publicToolsPath);
  const dxStyleTools = readRequiredFile(dxStyleToolsPath);

  assert.match(publicTools, /mod dx_style;/);

  for (const marker of [
    "pub(super) struct SourceScanDiagnosticFinding",
    "pub(super) fn source_scan_diagnostic_findings_for_source(",
    "source_scan_diagnostic_receipts_include_locations",
    "SourceScanDiagnosticKind::DynamicFragment",
    "SourceScanDiagnosticKind::UnsafeCandidate",
    "source_file",
    "line",
    "column",
  ]) {
    assert.match(dxStyleSupport, escaped(marker));
  }

  for (const marker of [
    "const DX_STYLE_SOURCE_SCAN_DIAGNOSTIC_FINDING_LIMIT",
    "fn collect_source_scan_diagnostic_findings(",
    "fn source_scan_diagnostic_counts_by_kind(",
    "source_scan_diagnostic_count",
    "source_scan_diagnostic_counts_by_kind",
    "source_scan_diagnostic_findings",
    "dx-style-source-scan-diagnostic",
  ]) {
    assert.match(dxStyleTools, escaped(marker));
  }

  assert.match(
    dxStyleTools,
    /"source_scan_diagnostic_count": source_scan_diagnostic_findings\.len\(\)/,
  );
});
