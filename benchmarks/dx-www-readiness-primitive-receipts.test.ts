import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");

function read(relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function expectMarkers(source: string, markers: string[], label: string): void {
  for (const marker of markers) {
    assert.match(
      source,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `${label} should include ${marker}`,
    );
  }
}

function markerCount(source: string, marker: string): number {
  return source.split(marker).length - 1;
}

function sliceBetween(source: string, start: string, end: string): string {
  const startIndex = source.indexOf(start);
  assert.notEqual(startIndex, -1, `expected source to include ${start}`);
  const endIndex = source.indexOf(end, startIndex + start.length);
  assert.notEqual(endIndex, -1, `expected source after ${start} to include ${end}`);
  return source.slice(startIndex, endIndex);
}

function primitiveBlock(primitiveProof: string, id: string, nextId?: string): string {
  return sliceBetween(
    primitiveProof,
    `"id": "${id}"`,
    nextId ? `"id": "${nextId}"` : '"rule": "Primitive claims must stay tied',
  );
}

test("release-readiness primitive receipts stay source-owned and do not claim hosted behavior", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const staticMarkup = read(
    "dx-www/src/cli/app_router_execution/source_render_parts/static_markup.rs",
  );
  const staticExpression = read(
    "dx-www/src/cli/app_router_execution/source_render_parts/static_expression.rs",
  );
  const deployAdapter = read("dx-www/src/cli/deploy_adapter_contract.rs");
  const templateShell = read("benchmarks/template-shell.test.ts");

  expectMarkers(
    readiness,
    [
      "READINESS_PRIMITIVE_PROOF_SCHEMA",
      "READINESS_PRIMITIVE_PROOF_RECEIPT_CONTRACT",
      "READINESS_PRIMITIVE_PROOF_RECEIPT",
      "READINESS_PRIMITIVE_PROOF_RECEIPT_SR",
      "READINESS_PRIMITIVE_PROOF_RECEIPT_MACHINE",
      "dx.www.readiness.primitive_proof",
      "dx.www.readiness.primitive_proof_receipt_contract",
      "write_readiness_primitive_proof_receipt",
      "readiness_primitive_proof_sr_fields",
      "readiness_primitive_proof_stale_reason",
      "primitive_proof_stale_reason_from_receipt",
      "primitive_proof_missing_primitives",
      "primitive-proof-receipt-missing",
      "primitive-proof-source-coverage-incomplete",
      "primitive-hosted-browser-proof-missing",
      "primitive-proof-project-receipt-missing-root-current",
      "primitive_proof_root_current_project_missing",
      "source-owned-primitive-root-current-project-receipt-needed",
      "readiness_parent_primitive_proof_receipt",
      "primitive_proof_stale_reason",
      '"stale_reason": primitive_proof_stale_reason',
      "source-owned-primitive-foundation-current",
      "hosted Image, Font, Script, and Wasm primitive behavior proof with receipts",
      "Regenerate project-local receipts before treating this app as current.",
      "source-owned-foundation-not-full-framework-parity",
      "static-safe next/image lowers to <img>",
      "data-dx-framework-component and data-dx-image-boundary markers",
      "module-scope font loader detection",
      "CSS variable/class metadata receipts",
      "static-safe next/script lowers to <script>",
      "strategy is converted to data-dx-next-script-strategy instead of invalid HTML",
      ".wasm and .wasm.gz are immutable runtime assets",
      "wasm/bindgen source-guard receipts expose app-owned generated-Wasm boundaries",
      "not_yet_claimed",
      "local-source-owned-primitive-foundation",
      "hosted optimizer service",
      "cross-provider hosted font cache proof",
      "browser proof for app-owned wasm modules",
      "Primitive claims must stay tied to source-owned receipts",
    ],
    "release-readiness primitive proof",
  );

  expectMarkers(
    staticMarkup,
    [
      "framework_static_component_name",
      "data-dx-framework-component",
      "apply_next_image_static_attributes",
      "apply_next_script_static_attributes",
      "apply_next_font_static_attributes",
      "static_html_tag_name",
    ],
    "static markup primitive lowering",
  );
  assert.ok(
    markerCount(staticMarkup, "apply_next_image_static_attributes") >= 2,
    "image lowering should be applied in both static element render paths",
  );
  assert.ok(
    markerCount(staticMarkup, "apply_next_script_static_attributes") >= 2,
    "script lowering should be applied in both static element render paths",
  );
  assert.ok(
    markerCount(staticMarkup, "apply_next_font_static_attributes") >= 2,
    "font metadata should be applied in both static element render paths",
  );

  expectMarkers(
    staticExpression,
    [
      "data-dx-image-boundary",
      "next-image-static-optimized-metadata",
      "data-dx-next-script-strategy",
      "data-dx-script-boundary",
      "next-script-static-script-metadata",
      "next_font_binding_for_expression",
      "collect_font_loader_detections",
      "data-dx-next-font",
    ],
    "static expression primitive receipt markers",
  );

  expectMarkers(
    `${deployAdapter}\n${templateShell}`,
    [
      "content_encoding",
      "content_type",
      "encoded_from",
      "Content-Type",
      "chunks/app.wasm.gz",
      "application/wasm",
      "wasm/bindgen",
      "data-dx-wasm-interaction",
      "data-dx-wasm-action",
    ],
    "Wasm deploy/template proof markers",
  );
});

test("release-readiness primitive implemented lists keep hosted and browser gaps out", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const primitiveProof = sliceBetween(
    readiness,
    "fn readiness_primitive_proof() -> Value",
    "fn readiness_route_handler_server_action_gaps",
  );
  const primitives = [
    {
      id: "image",
      nextId: "font",
      implemented: [
        "static-safe next/image lowers to <img>",
        "data-dx-framework-component and data-dx-image-boundary markers",
      ],
      notYet: [
        "hosted optimizer service",
        "remote image loader parity",
        "responsive srcset generation for every loader",
      ],
    },
    {
      id: "font",
      nextId: "script",
      implemented: [
        "module-scope font loader detection",
        "CSS variable/class metadata receipts",
        "static fallback font-family attributes without remote font requests",
      ],
      notYet: [
        "font binary downloading",
        "full Next font manifest parity",
        "cross-provider hosted font cache proof",
      ],
    },
    {
      id: "script",
      nextId: "wasm",
      implemented: [
        "static-safe next/script lowers to <script>",
        "strategy is converted to data-dx-next-script-strategy instead of invalid HTML",
        "afterInteractive/lazyOnload default to defer when async/defer is absent",
      ],
      notYet: [
        "full Next script lifecycle ordering",
        "worker strategy runtime",
        "onReady/onLoad callback execution without a client runtime",
      ],
    },
    {
      id: "wasm",
      implemented: [
        ".wasm and .wasm.gz are immutable runtime assets",
        "precompressed wasm metadata carries Content-Encoding, encoded_from, and decoded application/wasm content type",
        "wasm/bindgen source-guard receipts expose app-owned generated-Wasm boundaries",
      ],
      notYet: [
        "automatic Rust-to-wasm app build pipeline",
        "generated wasm-bindgen glue execution for arbitrary apps",
        "browser proof for app-owned wasm modules",
      ],
    },
  ];

  for (const primitive of primitives) {
    const block = primitiveBlock(primitiveProof, primitive.id, primitive.nextId);
    const implemented = sliceBetween(block, '"implemented": [', '"not_yet_claimed": [');
    const notYet = block.slice(block.indexOf('"not_yet_claimed": ['));

    expectMarkers(implemented, primitive.implemented, `${primitive.id} implemented proof`);
    expectMarkers(notYet, primitive.notYet, `${primitive.id} not-yet-claimed proof`);
    assert.doesNotMatch(
      implemented,
      /hosted|remote image loader parity|font binary downloading|full Next .*parity|full Next script lifecycle ordering|worker strategy runtime|\bonReady\b|\bonLoad\b|automatic Rust-to-wasm|arbitrary apps|browser proof/i,
      `${primitive.id} implemented proof must stay source-owned`,
    );
  }
});

test("release-readiness primitive and plain HTML proof words cannot be read as hosted browser receipts", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const primitiveProof = sliceBetween(
    readiness,
    "fn readiness_primitive_proof() -> Value",
    "fn readiness_route_handler_server_action_gaps",
  );
  const implementedEvidence = Array.from(
    primitiveProof.matchAll(/"implemented": \[([\s\S]*?)\]\s*,\s*"not_yet_claimed": \[/g),
    (match) => match[1],
  ).join("\n");
  const notYetClaimedEvidence = Array.from(
    primitiveProof.matchAll(/"not_yet_claimed": \[([\s\S]*?)\]\s*(?:\}|\])/g),
    (match) => match[1],
  ).join("\n");
  const noJsReceipt = sliceBetween(
    readiness,
    "fn write_readiness_no_js_artifact_receipt",
    "fn readiness_no_js_artifact_paths",
  );

  assert.equal(
    markerCount(primitiveProof, '"implemented": ['),
    4,
    "Image, Font, Script, and Wasm should each have an implemented source-owned list",
  );
  assert.equal(
    markerCount(primitiveProof, '"not_yet_claimed": ['),
    4,
    "Image, Font, Script, and Wasm should each keep hosted/browser gaps explicit",
  );
  assert.doesNotMatch(
    implementedEvidence,
    /hosted|provider|remote image loader parity|font binary downloading|full Next .*parity|full Next script lifecycle ordering|worker strategy runtime|\bonReady\b|\bonLoad\b|automatic Rust-to-wasm|browser proof|browser execution|release ready/i,
    "implemented primitive evidence must not read like hosted/browser proof",
  );
  expectMarkers(
    notYetClaimedEvidence,
    [
      "hosted optimizer service",
      "remote image loader parity",
      "font binary downloading",
      "cross-provider hosted font cache proof",
      "full Next script lifecycle ordering",
      "worker strategy runtime",
      "browser proof for app-owned wasm modules",
    ],
    "release-readiness primitive hosted/browser gap wording",
  );
  expectMarkers(
    noJsReceipt,
    [
      '"meaningful_html_without_js": passed',
      '"live_browser_executed": false',
      '"javascript_disabled_browser": false',
      '"live_astro_parity_receipt": "missing"',
      '"rule": "This receipt validates already-produced no-JS build artifacts only; it does not run a browser or claim Astro payload/paint/throughput parity."',
    ],
    "release-readiness plain HTML receipt boundary",
  );
  assert.doesNotMatch(
    noJsReceipt,
    /"live_browser_executed": true|"javascript_disabled_browser": true|"live_astro_parity_receipt": "(?!missing)|"hosted_provider_proof": true|"release_ready": true|"fastest_world_claim": true/,
    "plain HTML receipt must not claim hosted/browser execution",
  );
});

test("release-readiness primitive and plain HTML receipt gates stay blocked until hosted proof exists", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const primitiveGate = sliceBetween(
    readiness,
    '"id": "primitive-proof"',
    '"id": "route-handler-server-action-proof-gaps"',
  );
  const primitiveProof = sliceBetween(
    readiness,
    "fn readiness_primitive_proof() -> Value",
    "fn readiness_route_handler_server_action_gaps",
  );
  const noJsSrFields = sliceBetween(
    readiness,
    "fn readiness_no_js_artifact_sr_fields",
    "fn readiness_proof_graph_sr_fields",
  );

  expectMarkers(
    primitiveGate,
    [
      '"status": primitive_proof_status',
      '"blocks_release": true',
      '"primitive_proof_receipt": READINESS_PRIMITIVE_PROOF_RECEIPT',
      '"stale_reason": primitive_proof_stale_reason',
      '"next_proof": primitive_proof_next_proof',
    ],
    "release-readiness primitive gate",
  );
  expectMarkers(
    readiness,
    [
      "source-owned-primitive-foundation",
      "source-owned-primitive-receipt-current-hosted-proof-needed",
      "durable source-owned primitive foundation receipt, then hosted Image, Font, Script, and Wasm primitive behavior proof with receipts",
      "hosted Image optimizer, hosted Font cache, Script lifecycle browser matrix, and app-owned Wasm browser execution receipts",
    ],
    "release-readiness primitive gate statuses",
  );
  assert.doesNotMatch(
    primitiveGate,
    /"blocks_release": false|"release_ready": true|"fastest_world_claim": true|"hosted_provider_proof": true/,
  );

  expectMarkers(
    primitiveProof,
    [
      '"schema": READINESS_PRIMITIVE_PROOF_SCHEMA',
      '"receipt_contract": READINESS_PRIMITIVE_PROOF_RECEIPT_CONTRACT',
      '"receipt_path": READINESS_PRIMITIVE_PROOF_RECEIPT',
      '"serializer_receipt_path": READINESS_PRIMITIVE_PROOF_RECEIPT_SR',
      '"machine_contract_path": READINESS_PRIMITIVE_PROOF_RECEIPT_MACHINE',
      '"status": "source-owned-foundation-not-full-framework-parity"',
      '"id": "image"',
      '"id": "font"',
      '"id": "script"',
      '"id": "wasm"',
      "hosted optimizer service",
      "cross-provider hosted font cache proof",
      "full Next script lifecycle ordering",
      "browser proof for app-owned wasm modules",
    ],
    "release-readiness primitive proof body",
  );
  assert.equal(
    markerCount(primitiveProof, '"not_yet_claimed": ['),
    4,
    "each primitive should keep an explicit not_yet_claimed list",
  );
  assert.doesNotMatch(
    primitiveProof,
    /"release_ready": true|"fastest_world_claim": true|"hosted_provider_proof": true/,
  );

  const primitiveReceiptWriter = sliceBetween(
    readiness,
    "fn write_readiness_primitive_proof_receipt",
    "fn write_readiness_bundle_partition_receipt",
  );
  expectMarkers(
    primitiveReceiptWriter,
    [
      '"schema": READINESS_PRIMITIVE_PROOF_RECEIPT_CONTRACT',
      '"primitive_proof_schema": READINESS_PRIMITIVE_PROOF_SCHEMA',
      '"status": status',
      '"source_owned": true',
      '"browser_runtime_executed": false',
      '"hosted_provider_proof": false',
      '"proof_scope": "local-source-owned-primitive-foundation"',
      "readiness_primitive_proof_sr_fields",
      "write_readiness_json_receipt",
      "does not run a browser, execute generated Wasm, fetch remote fonts, optimize remote images, or claim hosted provider parity",
    ],
    "release-readiness primitive receipt writer",
  );

  expectMarkers(
    noJsSrFields,
    [
      '("release_ready", sr_bool(false))',
      '("fastest_world_claim", sr_bool(false))',
      '("live_browser_executed", sr_bool(false))',
      '("javascript_disabled_browser", sr_bool(false))',
      '("astro_parity_claimed", sr_bool(false))',
      "artifact-only no-JS proof; live browser and Astro parity remain separate gates",
    ],
    "release-readiness no-JS plain HTML SR fields",
  );
});

test("plain HTML public tooling keeps Style, Icon, and Check usable outside WWW", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const defaultTemplateSources = read("dx-www/src/cli/default_template_sources.rs");
  const staticExpression = read(
    "dx-www/src/cli/app_router_execution/source_render_parts/static_expression.rs",
  );
  const publicTools = read("dx-www/src/cli/public_framework_tools.rs");
  const publicToolsBenchmark = read("benchmarks/public-framework-tools.test.ts");

  expectMarkers(
    readiness,
    [
      "READINESS_NO_JS_ARTIFACT_RECEIPT_CONTRACT",
      "READINESS_NO_JS_OUTPUT_HTML_SUFFIX",
      "meaningful_html_without_js",
      "live_browser_executed",
      "javascript_disabled_browser",
      "live_astro_parity_receipt",
      "artifact-only no-JS proof; live browser and Astro parity remain separate gates",
    ],
    "release-readiness no-JS receipt honesty",
  );

  expectMarkers(
    defaultTemplateSources,
    [
      "examples/template/styles/theme.css",
      "examples/template/styles/generated.css",
      "examples/template/styles/globals.css",
      "examples/template/components/icons/icon.tsx",
      "examples/template/public/logo.svg",
      "examples/template/public/icon.svg",
      "examples/template/public/favicon.svg",
    ],
    "default plain HTML-compatible template sources",
  );

  expectMarkers(
    staticExpression,
    [
      "static_dx_icon_element_html",
      "data-icon-source=\"dx-icons\"",
      "data-dx-icon",
      "data-dx-icon-set",
      "data-dx-icon-name",
    ],
    "plain HTML dx-icon render path",
  );

  expectMarkers(
    publicTools,
    [
      'collect_files(&root, &["tsx", "jsx", "ts", "js", "mdx", "html"])',
      "data-dx-icon",
      "normalized_public_artifact_path(\"app/index.html\")",
      "copy_public_runtime_artifacts",
      "materialize_vercel_build_output",
      "copy_public_runtime_artifacts_leaves_receipts_outside_vercel_static",
      "application/wasm",
      "\"static_dir\": \".vercel/output/static\"",
      "\"evidence_excluded_from_public_output\": true",
      "\"dx check web-perf\"",
    ],
    "public framework tool plain HTML path",
  );

  expectMarkers(
    publicToolsBenchmark,
    [
      "dx-style CSS-first directive compatibility is source-owned and diagnosed",
      "public CLI advertises React TSX, dx-style, imports, web-perf, and Vercel deploy lanes",
      "dx deploy vercel performs a guarded local preflight pipeline",
      "materialize_vercel_build_output_keeps_tiny_static_public_and_evidence_private",
    ],
    "public framework benchmark coverage",
  );
});
