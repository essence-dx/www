import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("source-build image metadata receipt records route references without optimization overclaim", () => {
  const imageSource = read("dx-www/src/build/source_engine/image.rs");
  const sourceEngineModule = read("dx-www/src/build/source_engine/mod.rs");
  const imageBmffSource = read("dx-www/src/build/source_engine/image_bmff.rs");
  const imagePlaceholderSource = read("dx-www/src/build/source_engine/image_placeholder.rs");
  const nextRustSource = read("dx-www/src/next_rust.rs");
  const graphSource = read("dx-www/src/build/source_engine/graph.rs");
  const graphSnapshotSource = read("dx-www/src/build/source_engine/ecosystem_graph.rs");
  const ecosystemSource = read("dx-www/src/build/source_engine/ecosystem.rs");
  const readinessSource = read("dx-www/src/build/source_engine/readiness.rs");
  const handoffSource = read("dx-www/src/build/source_engine/ecosystem_handoff.rs");
  const receiptSource = read("dx-www/src/build/source_engine/receipt.rs");
  const rustFixture = read("dx-www/tests/source_build_engine.rs");
  const pngFixture = read("dx-www/tests/source_build_image_metadata.rs");

  assert.match(imageSource, /struct SourceBuildImageRouteReference/);
  assert.match(imageSource, /struct SourceBuildImageStyleReference/);
  assert.match(imageSource, /pub dimension_source: String/);
  assert.match(imageSource, /pub optimizer_invoked: bool/);
  assert.match(imageSource, /struct ImageDimensionEvidence/);
  assert.match(imageSource, /pub fn image_format_counts/);
  assert.match(imageSource, /pub fn image_dimension_source_counts/);
  assert.match(imageSource, /pub fn image_placeholder_count/);
  assert.match(imageSource, /pub fn image_placeholder_artifact_count/);
  assert.match(imageSource, /pub fn image_placeholder_artifact_bytes/);
  assert.match(imageSource, /pub fn image_placeholder_artifact_outputs/);
  assert.match(imageSource, /pub fn image_style_reference_count/);
  assert.match(imageSource, /pub fn route_references_for_asset/);
  assert.match(imageSource, /pub fn style_references_for_asset/);
  assert.match(imageSource, /"svg-root-attributes"/);
  assert.match(imageSource, /"raster-svg-placeholder-artifact"/);
  assert.match(imageSource, /"png-ihdr"/);
  assert.match(imageSource, /fn webp_dimensions/);
  assert.match(imageSource, /"webp-vp8x"/);
  assert.match(imageSource, /fn avif_dimensions/);
  assert.match(imageSource, /"avif-ispe"/);
  assert.match(imageSource, /avif_dimensions_from_bmff/);
  assert.match(imageSource, /fn ico_dimensions/);
  assert.match(imageSource, /"ico-directory-entry"/);
  assert.match(imageSource, /records_webp_vp8x_dimensions_with_raster_placeholder_without_optimizer/);
  assert.match(imageSource, /"static-image-url"/);
  assert.doesNotMatch(imageSource, /next\/image/);
  assert.doesNotMatch(imageSource, /"turbopack-image::process::optimize"\.to_string\(\)/);
  assert.doesNotMatch(imageSource, /fn svg_placeholder_markup/);
  assert.doesNotMatch(imageSource, /fn write_image_placeholder_artifact/);

  assert.match(sourceEngineModule, /mod image_bmff;/);
  assert.match(sourceEngineModule, /mod image_placeholder;/);
  assert.match(imageBmffSource, /pub\(super\) fn avif_dimensions_from_bmff/);
  assert.match(imageBmffSource, /find_box_payload/);
  assert.match(imageBmffSource, /ispe_dimensions/);
  assert.match(imageBmffSource, /has_zero_full_box_header/);
  assert.doesNotMatch(imageBmffSource, /sharp|squoosh|node_modules|next\/image/i);

  assert.match(imagePlaceholderSource, /pub struct SourceBuildImagePlaceholder/);
  assert.match(imagePlaceholderSource, /pub\(super\) fn svg_placeholder_data_url/);
  assert.match(imagePlaceholderSource, /pub\(super\) fn raster_placeholder_data_url/);
  assert.match(imagePlaceholderSource, /pub\(super\) fn write_image_placeholder_artifact/);
  assert.match(imagePlaceholderSource, /fn svg_placeholder_markup/);
  assert.match(imagePlaceholderSource, /pub output: Option<String>/);
  assert.match(imagePlaceholderSource, /pub hash: Option<String>/);
  assert.match(imagePlaceholderSource, /pub artifact_bytes: Option<usize>/);
  assert.doesNotMatch(imagePlaceholderSource, /sharp|squoosh|node_modules|next\/image/i);

  assert.match(nextRustSource, /upstream: "turbopack\/crates\/turbopack-image"/);
  assert.match(nextRustSource, /dx_role: "DX-WWW image asset metadata reference"/);
  assert.match(
    nextRustSource,
    /boundary: "reference-only: metadata receipts before pipeline claims"/,
  );
  assert.match(
    nextRustSource,
    /upstream: "turbopack\/crates\/turbopack-image"[\s\S]*?reference_only: true/,
  );

  assert.match(graphSource, /referenced_by_routes: Vec<SourceBuildImageRouteReference>/);
  assert.match(graphSource, /referenced_by_styles: Vec<SourceBuildImageStyleReference>/);
  assert.match(graphSource, /pub node_modules_required: bool/);
  assert.match(graphSource, /pub lifecycle_scripts_executed: bool/);
  assert.match(graphSource, /pub source_owned_contract: bool/);
  assert.match(graphSource, /pub external_runtime_required: bool/);
  assert.match(graphSource, /pub external_runtime_executed: bool/);
  assert.match(graphSource, /assets\.iter\(\)\.any\(\|asset\| asset\.node_modules_required\)/);
  assert.match(graphSource, /node_modules_required: false/);
  assert.match(graphSource, /lifecycle_scripts_executed: false/);
  assert.match(graphSource, /source_owned_contract: true/);
  assert.match(graphSource, /external_runtime_required: false/);
  assert.match(graphSource, /external_runtime_executed: false/);
  assert.match(graphSource, /route_references_for_asset\(project_root, &relative, (?:&)?routes\)/);
  assert.match(graphSource, /style_references_for_asset\(&relative, (?:&)?styles\)/);
  assert.match(graphSource, /write_image_placeholder_artifact/);
  assert.match(graphSource, /metadata\.optimization\.placeholder\.as_mut\(\)/);

  assert.match(graphSnapshotSource, /let image_optimization = image_optimization_summary\(&nodes, &edges\)/);
  assert.match(graphSnapshotSource, /let mut format_counts = BTreeMap::new\(\)/);
  assert.match(graphSnapshotSource, /let mut dimension_source_counts = BTreeMap::new\(\)/);
  assert.match(graphSnapshotSource, /let mut placeholder_count = 0usize/);
  assert.match(graphSnapshotSource, /let mut placeholder_artifact_count = 0usize/);
  assert.match(graphSnapshotSource, /let mut placeholder_artifact_bytes = 0u64/);
  assert.match(graphSnapshotSource, /let mut placeholder_artifact_outputs = Vec::new\(\)/);
  assert.match(graphSnapshotSource, /let placeholder_artifact_edge_count = edges/);
  assert.match(graphSnapshotSource, /let mut style_reference_count = 0usize/);
  assert.match(graphSnapshotSource, /"image-placeholder-asset"/);
  assert.match(graphSnapshotSource, /"emits-placeholder"/);
  assert.match(graphSnapshotSource, /"source_image": asset\.path/);
  assert.match(graphSnapshotSource, /let status = if placeholder_count > 0/);
  assert.match(graphSnapshotSource, /"formatCounts": format_counts/);
  assert.match(graphSnapshotSource, /"dimensionSourceCounts": dimension_source_counts/);
  assert.match(graphSnapshotSource, /"placeholderCount": placeholder_count/);
  assert.match(graphSnapshotSource, /"placeholderArtifactCount": placeholder_artifact_count/);
  assert.match(graphSnapshotSource, /"placeholderArtifactBytes": placeholder_artifact_bytes/);
  assert.match(graphSnapshotSource, /"placeholderArtifactOutputs": placeholder_artifact_outputs/);
  assert.match(graphSnapshotSource, /"placeholderArtifactEdgeCount": placeholder_artifact_edge_count/);
  assert.match(graphSnapshotSource, /"styleReferenceCount": style_reference_count/);
  assert.match(graphSnapshotSource, /"node_modules_required": asset\.node_modules_required/);
  assert.match(
    graphSnapshotSource,
    /"lifecycle_scripts_executed": asset\.lifecycle_scripts_executed/,
  );
  assert.match(graphSnapshotSource, /"source_owned_contract": asset\.source_owned_contract/);
  assert.match(
    graphSnapshotSource,
    /"external_runtime_required": asset\.external_runtime_required/,
  );
  assert.match(
    graphSnapshotSource,
    /"external_runtime_executed": asset\.external_runtime_executed/,
  );

  assert.match(ecosystemSource, /"route_references": route_references/);
  assert.match(ecosystemSource, /let style_references = image_style_reference_count\(&manifest\.assets\)/);
  assert.match(ecosystemSource, /"style_references": style_references/);
  assert.match(ecosystemSource, /let placeholders_emitted = image_placeholder_count\(&manifest\.assets\)/);
  assert.match(ecosystemSource, /"placeholders_emitted": placeholders_emitted/);
  assert.match(ecosystemSource, /"formats": image_format_counts\(&manifest\.assets\)/);
  assert.match(ecosystemSource, /"dimension_sources": image_dimension_source_counts\(&manifest\.assets\)/);
  assert.match(ecosystemSource, /"referenced_by_routes": asset\.referenced_by_routes/);
  assert.match(ecosystemSource, /"referenced_by_styles": asset\.referenced_by_styles/);
  assert.match(ecosystemSource, /"full_pipeline_parity": false/);

  assert.match(readinessSource, /"image_formats": image_format_counts\(&manifest\.assets\)/);
  assert.match(readinessSource, /"image_placeholders": image_placeholders/);
  assert.match(readinessSource, /let image_placeholder_artifacts = image_placeholder_artifact_count\(&manifest\.assets\)/);
  assert.match(readinessSource, /let image_placeholder_artifact_bytes = image_placeholder_artifact_bytes\(&manifest\.assets\)/);
  assert.match(readinessSource, /let image_placeholder_artifact_outputs = image_placeholder_artifact_outputs\(&manifest\.assets\)/);
  assert.match(readinessSource, /"image_placeholder_artifacts": image_placeholder_artifacts/);
  assert.match(readinessSource, /"image_placeholder_artifact_bytes": image_placeholder_artifact_bytes/);
  assert.match(readinessSource, /"image_placeholder_artifact_outputs": image_placeholder_artifact_outputs/);
  assert.match(
    readinessSource,
    /"image_dimension_sources": image_dimension_source_counts\(&manifest\.assets\)/,
  );

  assert.match(handoffSource, /"formats": image_format_counts\(&manifest\.assets\)/);
  assert.match(handoffSource, /"placeholder_count": image_placeholders/);
  assert.match(handoffSource, /let image_placeholder_artifacts = image_placeholder_artifact_count\(&manifest\.assets\)/);
  assert.match(handoffSource, /let image_placeholder_artifact_bytes = image_placeholder_artifact_bytes\(&manifest\.assets\)/);
  assert.match(handoffSource, /let image_placeholder_artifact_outputs = image_placeholder_artifact_outputs\(&manifest\.assets\)/);
  assert.match(handoffSource, /"placeholder_artifact_count": image_placeholder_artifacts/);
  assert.match(handoffSource, /"placeholder_artifact_bytes": image_placeholder_artifact_bytes/);
  assert.match(handoffSource, /"placeholder_artifact_outputs": image_placeholder_artifact_outputs/);
  assert.match(handoffSource, /let image_pipeline_status = if image_placeholders > 0/);
  assert.match(
    handoffSource,
    /"dimension_sources": image_dimension_source_counts\(&manifest\.assets\)/,
  );

  assert.match(receiptSource, /pub image_placeholders: usize/);
  assert.match(receiptSource, /image_placeholders = image_placeholder_count\(assets\)/);
  assert.match(receiptSource, /assets\.iter\(\)\.any\(\|asset\| asset\.node_modules_required\)/);
  assert.match(
    receiptSource,
    /records-metadata-and-placeholder-artifacts-no-image-transforms-emitted/,
  );

  assert.match(rustFixture, /image_receipt\["summary"\]\["route_references"\]/);
  assert.match(rustFixture, /image_receipt\["summary"\]\["formats"\]\["svg"\]/);
  assert.match(rustFixture, /image_receipt\["summary"\]\["dimension_sources"\]\["svg-root-attributes"\]/);
  assert.match(rustFixture, /manifest\["assets"\]\[0\]\["node_modules_required"\]/);
  assert.match(rustFixture, /manifest\["assets"\]\[0\]\["source_owned_contract"\]/);
  assert.match(rustFixture, /build_readiness\["graph"\]\["image_formats"\]\["svg"\]/);
  assert.match(rustFixture, /zed_handoff\["image_pipeline"\]\["formats"\]\["svg"\]/);
  assert.match(rustFixture, /asset\["image_metadata"\]\["dimension_source"\]/);
  assert.match(rustFixture, /image_receipt\["assets"\]\[0\]\["referenced_by_routes"\]\[0\]\["route"\]/);
  assert.match(rustFixture, /image_receipt\["assets"\]\[0\]\["image_metadata"\]\["dimension_source"\]/);
  assert.match(rustFixture, /graph_snapshot\["graph"\]\["imageOptimization"\]\["routeReferenceCount"\]/);
  assert.match(rustFixture, /graph_snapshot\["graph"\]\["imageOptimization"\]\["formatCounts"\]\["svg"\]/);
  assert.match(
    rustFixture,
    /graph_snapshot\["graph"\]\["imageOptimization"\]\["dimensionSourceCounts"\]\s*\["svg-root-attributes"\]/,
  );
  assert.match(rustFixture, /image_node\["node_modules_required"\]/);
  assert.match(rustFixture, /image_node\["lifecycle_scripts_executed"\]/);
  assert.match(rustFixture, /image_node\["source_owned_contract"\]/);
  assert.match(rustFixture, /image_node\["external_runtime_required"\]/);
  assert.match(rustFixture, /image_node\["external_runtime_executed"\]/);

  assert.match(
    pngFixture,
    /source_build_image_metadata_records_png_header_evidence_in_consumer_surfaces/,
  );
  assert.match(pngFixture, /const PNG_BYTES: &\[u8\]/);
  assert.match(pngFixture, /"png-ihdr"/);
  assert.match(pngFixture, /\["optimizer_invoked"\]/);
  assert.match(pngFixture, /"metadata-plus-raster-svg-placeholder"/);
  assert.match(pngFixture, /"raster-svg-placeholder-artifact"/);
  assert.match(pngFixture, /public\/images\/hero-/);
  assert.match(pngFixture, /root\.join\(placeholder_output\)\.is_file\(\)/);
  assert.match(pngFixture, /"image-placeholder-asset"/);
  assert.match(pngFixture, /"emits-placeholder"/);
  assert.match(pngFixture, /placeholder_node_id/);
  assert.match(pngFixture, /"placeholderArtifactCount"/);
  assert.match(pngFixture, /"placeholderArtifactBytes"/);
  assert.match(pngFixture, /"placeholderArtifactOutputs"/);
  assert.match(pngFixture, /"placeholderArtifactEdgeCount"/);
  assert.match(pngFixture, /build_readiness\["graph"\]\["image_placeholder_artifacts"\]/);
  assert.match(pngFixture, /build_readiness\["graph"\]\["image_placeholder_artifact_bytes"\]/);
  assert.match(pngFixture, /build_readiness\["graph"\]\["image_placeholder_artifact_outputs"\]/);
  assert.match(pngFixture, /zed_handoff\["image_pipeline"\]\["placeholder_artifact_count"\]/);
  assert.match(pngFixture, /zed_handoff\["image_pipeline"\]\["placeholder_artifact_bytes"\]/);
  assert.match(pngFixture, /zed_handoff\["image_pipeline"\]\["placeholder_artifact_outputs"\]/);
  assert.match(pngFixture, /graph_snapshot\["graph"\]\["imageOptimization"\]\["formatCounts"\]\["png"\]/);
  assert.match(
    pngFixture,
    /source_build_image_metadata_records_css_url_style_references_without_route_reference/,
  );
  assert.match(pngFixture, /"referenced_by_styles"/);
  assert.match(pngFixture, /"style_references"/);
  assert.match(pngFixture, /"styleReferenceCount"/);
  assert.match(pngFixture, /"css-url"/);
  assert.match(pngFixture, /public\/images\/background\.png/);
  assert.match(
    pngFixture,
    /graph_snapshot\["graph"\]\["imageOptimization"\]\["dimensionSourceCounts"\]\["png-ihdr"\]/,
  );
  assert.match(
    pngFixture,
    /source_build_image_metadata_records_avif_ispe_evidence_without_optimizer/,
  );
  assert.match(pngFixture, /const AVIF_BYTES: &\[u8\]/);
  assert.match(pngFixture, /"image\/avif"/);
  assert.match(pngFixture, /"avif-ispe"/);
  assert.match(pngFixture, /public\/images\/hero-avif-/);
  assert.match(
    pngFixture,
    /source_build_image_metadata_records_ico_directory_dimensions_without_optimizer/,
  );
  assert.match(pngFixture, /const ICO_BYTES: &\[u8\]/);
  assert.match(pngFixture, /"image\/x-icon"/);
  assert.match(pngFixture, /"ico-directory-entry"/);
  assert.match(pngFixture, /public\/images\/favicon-/);
  assert.match(
    pngFixture,
    /source_build_image_metadata_keeps_malformed_avif_format_only_without_placeholder/,
  );
  assert.match(pngFixture, /const MALFORMED_AVIF_BYTES: &\[u8\]/);
  assert.match(pngFixture, /"format-only-no-dimensions"/);
  assert.match(pngFixture, /"\/images\/broken-avif.avif"/);
  assert.doesNotMatch(pngFixture, /next\/image/);
  assert.doesNotMatch(pngFixture, /sharp|squoosh/i);

  assert.match(
    pngFixture,
    /source_build_image_metadata_keeps_corrupt_png_format_only_without_dimensions/,
  );
  assert.match(pngFixture, /const CORRUPT_PNG_BYTES: &\[u8\]/);
  assert.match(pngFixture, /"metadata-format-only"/);
  assert.match(pngFixture, /"format-only-no-dimensions"/);
  assert.match(pngFixture, /asset\["image_metadata"\]\.get\("width"\)\.is_none\(\)/);
  assert.match(pngFixture, /asset\["image_metadata"\]\.get\("height"\)\.is_none\(\)/);
  assert.match(pngFixture, /graph_snapshot\["graph"\]\["imageOptimization"\]\["metadataAssetCount"\]/);
  assert.match(
    pngFixture,
    /source_build_image_metadata_emits_svg_placeholder_data_url_without_optimizer/,
  );
  assert.match(pngFixture, /"svg-placeholder-data-url"/);
  assert.match(pngFixture, /\["placeholder"\]\["data_url"\]/);
  assert.match(pngFixture, /\["placeholder"\]\["output"\]/);
  assert.match(pngFixture, /\["placeholder"\]\["hash"\]/);
  assert.match(pngFixture, /root\.join\(placeholder_output\)\.is_file\(\)/);
  assert.match(pngFixture, /image_receipt\["summary"\]\["placeholders_emitted"\]/);
  assert.match(pngFixture, /graph_snapshot\["graph"\]\["imageOptimization"\]\["placeholderCount"\]/);
});
