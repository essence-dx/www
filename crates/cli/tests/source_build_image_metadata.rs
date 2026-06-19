use std::fs;
use std::path::Path;

use dx_www::build::{SourceBuildEngine, SourceBuildOptions};
use serde_json::Value;

const PNG_BYTES: &[u8] = &[
    0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, b'I', b'H', b'D', b'R',
    0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x20, 0x08, 0x06, 0x00, 0x00, 0x00, 0xa9, 0xf0, 0x7e,
    0x1e, 0x00, 0x00, 0x00, 0x00, b'I', b'E', b'N', b'D', 0xae, 0x42, 0x60, 0x82,
];
const AVIF_BYTES: &[u8] = &[
    0x00, 0x00, 0x00, 0x1c, b'f', b't', b'y', b'p', b'a', b'v', b'i', b'f', 0x00, 0x00, 0x00, 0x00,
    b'a', b'v', b'i', b'f', b'm', b'i', b'f', b'1', b'm', b'i', b'a', b'f', 0x00, 0x00, 0x00, 0x30,
    b'm', b'e', b't', b'a', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, b'i', b'p', b'r', b'p',
    0x00, 0x00, 0x00, 0x1c, b'i', b'p', b'c', b'o', 0x00, 0x00, 0x00, 0x14, b'i', b's', b'p', b'e',
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x40, 0x00, 0x00, 0x00, 0xb4,
];
const MALFORMED_AVIF_BYTES: &[u8] = &[
    0x00, 0x00, 0x00, 0x1c, b'f', b't', b'y', b'p', b'a', b'v', b'i', b'f', 0x00, 0x00, 0x00, 0x00,
    b'a', b'v', b'i', b'f', b'm', b'i', b'f', b'1', b'm', b'i', b'a', b'f', 0x00, 0x00, 0x00, 0x30,
    b'm', b'e', b't', b'a', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, b'i', b'p', b'r', b'p',
    0x00, 0x00, 0x00, 0x1c, b'i', b'p', b'c', b'o', 0x00, 0x00, 0x00, 0x14, b'i', b's', b'p', b'e',
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x40, 0x00, 0x00, 0x00, 0xb4,
];
const ICO_BYTES: &[u8] = &[
    0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x20, 0x20, 0x00, 0x00, 0x01, 0x00, 0x20, 0x00, 0x04, 0x00,
    0x00, 0x00, 0x16, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
const CORRUPT_PNG_BYTES: &[u8] = b"not a png header";

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).expect("json file")).expect("parse json")
}

#[test]
fn source_build_image_metadata_records_png_header_evidence_in_consumer_surfaces() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("public/images")).expect("public images dir");
    fs::write(
        root.join("app/page.tsx"),
        r#"export default function Page() {
  return <main><img src="/images/hero.png" alt="DX" /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(root.join("public/images/hero.png"), PNG_BYTES).expect("png source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    let manifest = read_json(&report.manifest_path);
    let asset = &manifest["assets"][0];
    assert_eq!(asset["path"], "public/images/hero.png");
    assert_eq!(asset["image_metadata"]["format"], "png");
    assert_eq!(asset["image_metadata"]["mime_type"], "image/png");
    assert_eq!(asset["image_metadata"]["width"], 64);
    assert_eq!(asset["image_metadata"]["height"], 32);
    assert_eq!(asset["image_metadata"]["dimension_source"], "png-ihdr");
    assert_eq!(
        asset["image_metadata"]["optimization"]["status"],
        "metadata-plus-raster-svg-placeholder"
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["variants_emitted"],
        0
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["optimizer_invoked"],
        false
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["resize_emitted"],
        false
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["encoding_emitted"],
        false
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["placeholder"]["kind"],
        "raster-svg-placeholder-artifact"
    );
    let placeholder_output = asset["image_metadata"]["optimization"]["placeholder"]["output"]
        .as_str()
        .expect("png placeholder output");
    assert!(
        placeholder_output.starts_with(".dx/www/output/image-placeholders/public/images/hero-")
    );
    assert!(placeholder_output.ends_with(".placeholder.svg"));
    assert!(root.join(placeholder_output).is_file());
    let placeholder_artifact =
        fs::read_to_string(root.join(placeholder_output)).expect("png placeholder artifact");
    assert!(placeholder_artifact.contains(r#"width="64""#));
    assert!(placeholder_artifact.contains(r#"height="32""#));
    assert_eq!(
        asset["image_metadata"]["optimization"]["placeholder"]["artifact_bytes"],
        placeholder_artifact.as_bytes().len()
    );

    let image_receipt = read_json(&root.join(".dx/receipts/build/image-metadata.json"));
    assert_eq!(image_receipt["summary"]["formats"]["png"], 1);
    assert_eq!(image_receipt["summary"]["dimension_sources"]["png-ihdr"], 1);
    assert_eq!(image_receipt["summary"]["optimized_variants_emitted"], 0);
    assert_eq!(image_receipt["summary"]["placeholders_emitted"], 1);
    assert_eq!(
        image_receipt["boundary"]["optimization"],
        "metadata-plus-placeholder-artifacts-no-resize-or-encoding"
    );
    assert_eq!(
        image_receipt["assets"][0]["image_metadata"]["optimization"]["placeholder"]["output"],
        placeholder_output
    );
    assert_eq!(
        image_receipt["assets"][0]["referenced_by_routes"][0]["specifier"],
        "/images/hero.png"
    );

    let graph_receipt = read_json(&report.graph_receipt_path);
    let nodes = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes");
    let image_node = nodes
        .iter()
        .find(|node| node["path"] == "public/images/hero.png")
        .expect("public image node");
    assert_eq!(image_node["image_metadata"]["format"], "png");
    assert_eq!(image_node["image_metadata"]["dimension_source"], "png-ihdr");
    let placeholder_node_id = format!("image-placeholder-asset:{placeholder_output}");
    let placeholder_node = nodes
        .iter()
        .find(|node| node["id"].as_str() == Some(placeholder_node_id.as_str()))
        .expect("placeholder graph node");
    assert_eq!(placeholder_node["kind"], "image-placeholder-asset");
    assert_eq!(placeholder_node["path"], placeholder_output);
    assert_eq!(placeholder_node["source_image"], "public/images/hero.png");
    assert_eq!(
        placeholder_node["hash"],
        asset["image_metadata"]["optimization"]["placeholder"]["hash"]
    );
    assert_eq!(
        placeholder_node["bytes"].as_u64(),
        Some(placeholder_artifact.as_bytes().len() as u64)
    );
    assert_eq!(placeholder_node["optimizer_invoked"], false);
    assert_eq!(placeholder_node["resize_emitted"], false);
    assert_eq!(placeholder_node["encoding_emitted"], false);
    let edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(edges.iter().any(|edge| {
        edge["from"] == "public-asset:public/images/hero.png"
            && edge["to"].as_str() == Some(placeholder_node_id.as_str())
            && edge["kind"] == "emits-placeholder"
            && edge["optimizer_invoked"] == false
            && edge["resize_emitted"] == false
            && edge["encoding_emitted"] == false
    }));

    let graph_snapshot = read_json(&report.graph_snapshot_path);
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["formatCounts"]["png"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["dimensionSourceCounts"]["png-ihdr"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["optimizedVariantCount"],
        0
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["placeholderCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["placeholderArtifactCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["placeholderArtifactBytes"].as_u64(),
        Some(placeholder_artifact.as_bytes().len() as u64)
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["placeholderArtifactOutputs"][0],
        placeholder_output
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["placeholderArtifactEdgeCount"],
        1
    );

    let build_readiness = read_json(&report.build_readiness_path);
    assert_eq!(build_readiness["graph"]["image_formats"]["png"], 1);
    assert_eq!(
        build_readiness["graph"]["image_dimension_sources"]["png-ihdr"],
        1
    );
    assert_eq!(build_readiness["graph"]["image_placeholder_artifacts"], 1);
    assert_eq!(
        build_readiness["graph"]["image_placeholder_artifact_bytes"].as_u64(),
        Some(placeholder_artifact.as_bytes().len() as u64)
    );
    assert_eq!(
        build_readiness["graph"]["image_placeholder_artifact_outputs"][0],
        placeholder_output
    );

    let zed_handoff = read_json(&report.zed_handoff_path);
    assert_eq!(
        zed_handoff["image_pipeline"]["placeholder_artifact_count"],
        1
    );
    assert_eq!(
        zed_handoff["image_pipeline"]["placeholder_artifact_bytes"].as_u64(),
        Some(placeholder_artifact.as_bytes().len() as u64)
    );
    assert_eq!(
        zed_handoff["image_pipeline"]["placeholder_artifact_outputs"][0],
        placeholder_output
    );
}

#[test]
fn source_build_image_metadata_records_css_url_style_references_without_route_reference() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("styles")).expect("styles dir");
    fs::create_dir_all(root.join("public/images")).expect("public images dir");
    fs::write(
        root.join("app/page.tsx"),
        r#"import "../styles/app.css";

export default function Page() {
  return <main className="hero">DX</main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("styles/app.css"),
        r#".hero { background-image: url("../public/images/background.png"); }
"#,
    )
    .expect("style source");
    fs::write(root.join("public/images/background.png"), PNG_BYTES).expect("png source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    let manifest = read_json(&report.manifest_path);
    let asset = &manifest["assets"][0];
    assert_eq!(asset["path"], "public/images/background.png");
    assert_eq!(asset["image_metadata"]["format"], "png");
    assert_eq!(asset["image_metadata"]["dimension_source"], "png-ihdr");
    assert!(asset.get("referenced_by_routes").is_none());
    let style_references = asset["referenced_by_styles"]
        .as_array()
        .expect("style references");
    assert_eq!(style_references.len(), 1);
    assert_eq!(style_references[0]["style_path"], "styles/app.css");
    assert_eq!(
        style_references[0]["specifier"],
        "../public/images/background.png"
    );
    assert_eq!(style_references[0]["kind"], "css-url");
    assert_eq!(style_references[0]["node_modules_required"], false);

    let image_receipt = read_json(&root.join(".dx/receipts/build/image-metadata.json"));
    assert_eq!(image_receipt["summary"]["route_references"], 0);
    assert_eq!(image_receipt["summary"]["style_references"], 1);
    assert_eq!(
        image_receipt["assets"][0]["referenced_by_styles"][0]["style_path"],
        "styles/app.css"
    );

    let graph_receipt = read_json(&report.graph_receipt_path);
    let image_node = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes")
        .iter()
        .find(|node| node["path"] == "public/images/background.png")
        .expect("public image node");
    assert_eq!(
        image_node["referenced_by_styles"][0]["specifier"],
        "../public/images/background.png"
    );
    let edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(edges.iter().any(|edge| {
        edge["from"] == "dx-style-css:styles/app.css"
            && edge["to"] == "public-asset:public/images/background.png"
            && edge["reference_source"] == "css-url"
    }));

    let graph_snapshot = read_json(&report.graph_snapshot_path);
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["routeReferenceCount"],
        0
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["styleReferenceCount"],
        1
    );
}

#[test]
fn source_build_image_metadata_records_avif_ispe_evidence_without_optimizer() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("public/images")).expect("public images dir");
    fs::write(
        root.join("app/page.tsx"),
        r#"export default function Page() {
  return <main><img src="/images/hero-avif.avif" alt="DX" /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(root.join("public/images/hero-avif.avif"), AVIF_BYTES).expect("avif source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    let manifest = read_json(&report.manifest_path);
    let asset = &manifest["assets"][0];
    assert_eq!(asset["path"], "public/images/hero-avif.avif");
    assert_eq!(asset["image_metadata"]["format"], "avif");
    assert_eq!(asset["image_metadata"]["mime_type"], "image/avif");
    assert_eq!(asset["image_metadata"]["width"], 320);
    assert_eq!(asset["image_metadata"]["height"], 180);
    assert_eq!(asset["image_metadata"]["dimension_source"], "avif-ispe");
    assert_eq!(
        asset["image_metadata"]["optimization"]["status"],
        "metadata-plus-raster-svg-placeholder"
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["optimizer_invoked"],
        false
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["resize_emitted"],
        false
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["encoding_emitted"],
        false
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["placeholder"]["kind"],
        "raster-svg-placeholder-artifact"
    );
    let placeholder_output = asset["image_metadata"]["optimization"]["placeholder"]["output"]
        .as_str()
        .expect("avif placeholder output");
    assert!(
        placeholder_output
            .starts_with(".dx/www/output/image-placeholders/public/images/hero-avif-")
    );
    assert!(placeholder_output.ends_with(".placeholder.svg"));
    assert!(root.join(placeholder_output).is_file());

    let image_receipt = read_json(&root.join(".dx/receipts/build/image-metadata.json"));
    assert_eq!(image_receipt["summary"]["formats"]["avif"], 1);
    assert_eq!(
        image_receipt["summary"]["dimension_sources"]["avif-ispe"],
        1
    );
    assert_eq!(image_receipt["summary"]["optimized_variants_emitted"], 0);
    assert_eq!(image_receipt["summary"]["placeholders_emitted"], 1);
    assert_eq!(
        image_receipt["assets"][0]["referenced_by_routes"][0]["specifier"],
        "/images/hero-avif.avif"
    );

    let graph_snapshot = read_json(&report.graph_snapshot_path);
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["formatCounts"]["avif"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["dimensionSourceCounts"]["avif-ispe"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["placeholderCount"],
        1
    );
}

#[test]
fn source_build_image_metadata_records_ico_directory_dimensions_without_optimizer() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("public/images")).expect("public images dir");
    fs::write(
        root.join("app/page.tsx"),
        r#"export default function Page() {
  return <main><img src="/images/favicon.ico" alt="DX" /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(root.join("public/images/favicon.ico"), ICO_BYTES).expect("ico source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    let manifest = read_json(&report.manifest_path);
    let asset = &manifest["assets"][0];
    assert_eq!(asset["path"], "public/images/favicon.ico");
    assert_eq!(asset["image_metadata"]["format"], "ico");
    assert_eq!(asset["image_metadata"]["mime_type"], "image/x-icon");
    assert_eq!(asset["image_metadata"]["width"], 32);
    assert_eq!(asset["image_metadata"]["height"], 32);
    assert_eq!(
        asset["image_metadata"]["dimension_source"],
        "ico-directory-entry"
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["status"],
        "metadata-plus-raster-svg-placeholder"
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["optimizer_invoked"],
        false
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["resize_emitted"],
        false
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["encoding_emitted"],
        false
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["placeholder"]["kind"],
        "raster-svg-placeholder-artifact"
    );
    let placeholder_output = asset["image_metadata"]["optimization"]["placeholder"]["output"]
        .as_str()
        .expect("ico placeholder output");
    assert!(
        placeholder_output.starts_with(".dx/www/output/image-placeholders/public/images/favicon-")
    );
    assert!(placeholder_output.ends_with(".placeholder.svg"));
    assert!(root.join(placeholder_output).is_file());

    let image_receipt = read_json(&root.join(".dx/receipts/build/image-metadata.json"));
    assert_eq!(image_receipt["summary"]["formats"]["ico"], 1);
    assert_eq!(
        image_receipt["summary"]["dimension_sources"]["ico-directory-entry"],
        1
    );
    assert_eq!(image_receipt["summary"]["optimized_variants_emitted"], 0);
    assert_eq!(image_receipt["summary"]["placeholders_emitted"], 1);
    assert_eq!(
        image_receipt["assets"][0]["referenced_by_routes"][0]["specifier"],
        "/images/favicon.ico"
    );

    let graph_snapshot = read_json(&report.graph_snapshot_path);
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["formatCounts"]["ico"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["dimensionSourceCounts"]["ico-directory-entry"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["placeholderCount"],
        1
    );
}

#[test]
fn source_build_image_metadata_keeps_malformed_avif_format_only_without_placeholder() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("public/images")).expect("public images dir");
    fs::write(
        root.join("app/page.tsx"),
        r#"export default function Page() {
  return <main><img src="/images/broken-avif.avif" alt="DX" /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("public/images/broken-avif.avif"),
        MALFORMED_AVIF_BYTES,
    )
    .expect("malformed avif source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    let manifest = read_json(&report.manifest_path);
    let asset = &manifest["assets"][0];
    assert_eq!(asset["path"], "public/images/broken-avif.avif");
    assert_eq!(asset["image_metadata"]["format"], "avif");
    assert_eq!(asset["image_metadata"]["mime_type"], "image/avif");
    assert_eq!(asset["image_metadata"]["status"], "metadata-format-only");
    assert_eq!(
        asset["image_metadata"]["dimension_source"],
        "format-only-no-dimensions"
    );
    assert!(asset["image_metadata"].get("width").is_none());
    assert!(asset["image_metadata"].get("height").is_none());
    assert!(
        asset["image_metadata"]["optimization"]
            .get("placeholder")
            .is_none()
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["optimizer_invoked"],
        false
    );

    let image_receipt = read_json(&root.join(".dx/receipts/build/image-metadata.json"));
    assert_eq!(image_receipt["summary"]["formats"]["avif"], 1);
    assert_eq!(
        image_receipt["summary"]["dimension_sources"]["format-only-no-dimensions"],
        1
    );
    assert_eq!(image_receipt["summary"]["placeholders_emitted"], 0);
    assert_eq!(image_receipt["summary"]["optimized_variants_emitted"], 0);
    assert_eq!(
        image_receipt["assets"][0]["referenced_by_routes"][0]["specifier"],
        "/images/broken-avif.avif"
    );

    let graph_snapshot = read_json(&report.graph_snapshot_path);
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["formatCounts"]["avif"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["metadataAssetCount"],
        0
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["placeholderCount"],
        0
    );
}

#[test]
fn source_build_image_metadata_keeps_corrupt_png_format_only_without_dimensions() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("public/images")).expect("public images dir");
    fs::write(
        root.join("app/page.tsx"),
        r#"export default function Page() {
  return <main><img src="/images/broken.png" alt="DX" /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(root.join("public/images/broken.png"), CORRUPT_PNG_BYTES).expect("png source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    let manifest = read_json(&report.manifest_path);
    let asset = &manifest["assets"][0];
    assert_eq!(asset["path"], "public/images/broken.png");
    assert_eq!(asset["image_metadata"]["format"], "png");
    assert_eq!(asset["image_metadata"]["mime_type"], "image/png");
    assert_eq!(asset["image_metadata"]["status"], "metadata-format-only");
    assert_eq!(
        asset["image_metadata"]["dimension_source"],
        "format-only-no-dimensions"
    );
    assert!(asset["image_metadata"].get("width").is_none());
    assert!(asset["image_metadata"].get("height").is_none());
    assert_eq!(
        asset["image_metadata"]["optimization"]["optimizer_invoked"],
        false
    );
    assert!(
        asset["image_metadata"]["optimization"]
            .get("placeholder")
            .is_none()
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["variants_emitted"],
        0
    );

    let image_receipt = read_json(&root.join(".dx/receipts/build/image-metadata.json"));
    assert_eq!(image_receipt["summary"]["image_assets"], 1);
    assert_eq!(image_receipt["summary"]["metadata_assets"], 0);
    assert_eq!(image_receipt["summary"]["placeholders_emitted"], 0);
    assert_eq!(image_receipt["summary"]["formats"]["png"], 1);
    assert_eq!(
        image_receipt["summary"]["dimension_sources"]["format-only-no-dimensions"],
        1
    );
    assert_eq!(image_receipt["summary"]["optimized_variants_emitted"], 0);
    assert_eq!(
        image_receipt["assets"][0]["referenced_by_routes"][0]["specifier"],
        "/images/broken.png"
    );

    let graph_snapshot = read_json(&report.graph_snapshot_path);
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["imageAssetCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["metadataAssetCount"],
        0
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["formatCounts"]["png"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["dimensionSourceCounts"]["format-only-no-dimensions"],
        1
    );

    let build_readiness = read_json(&report.build_readiness_path);
    assert_eq!(build_readiness["graph"]["image_formats"]["png"], 1);
    assert_eq!(
        build_readiness["graph"]["image_dimension_sources"]["format-only-no-dimensions"],
        1
    );
}

#[test]
fn source_build_image_metadata_emits_svg_placeholder_data_url_without_optimizer() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("public/images")).expect("public images dir");
    fs::write(
        root.join("app/page.tsx"),
        r#"export default function Page() {
  return <main><img src="/images/logo.svg" alt="DX" /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("public/images/logo.svg"),
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="48" height="24" viewBox="0 0 48 24"><path d="M0 0h48v24H0z"/></svg>"##,
    )
    .expect("svg source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    let manifest = read_json(&report.manifest_path);
    let asset = &manifest["assets"][0];
    assert_eq!(asset["path"], "public/images/logo.svg");
    assert_eq!(asset["image_metadata"]["format"], "svg");
    assert_eq!(asset["image_metadata"]["width"], 48);
    assert_eq!(asset["image_metadata"]["height"], 24);
    assert_eq!(
        asset["image_metadata"]["optimization"]["status"],
        "metadata-plus-svg-placeholder"
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["variants_emitted"],
        0
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["optimizer_invoked"],
        false
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["resize_emitted"],
        false
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["encoding_emitted"],
        false
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["blur_placeholder_emitted"],
        false
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["boundary"],
        "metadata-plus-svg-placeholder-no-resize-or-encoding"
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["placeholder"]["kind"],
        "svg-placeholder-data-url"
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["placeholder"]["width"],
        48
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["placeholder"]["height"],
        24
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["placeholder"]["source"],
        "dx-source-owned-svg-placeholder"
    );
    let placeholder_data_url = asset["image_metadata"]["optimization"]["placeholder"]["data_url"]
        .as_str()
        .expect("placeholder data url");
    assert!(placeholder_data_url.starts_with("data:image/svg+xml,"));
    assert!(placeholder_data_url.contains("%3Csvg"));
    assert!(placeholder_data_url.contains("48"));
    assert!(placeholder_data_url.contains("24"));
    assert!(placeholder_data_url.contains("%23f3f4f6"));
    let placeholder_output = asset["image_metadata"]["optimization"]["placeholder"]["output"]
        .as_str()
        .expect("placeholder output");
    assert!(
        placeholder_output.starts_with(".dx/www/output/image-placeholders/public/images/logo-")
    );
    assert!(placeholder_output.ends_with(".placeholder.svg"));
    assert!(root.join(placeholder_output).is_file());
    let placeholder_artifact =
        fs::read_to_string(root.join(placeholder_output)).expect("placeholder artifact");
    assert!(placeholder_artifact.contains(r#"width="48""#));
    assert!(placeholder_artifact.contains(r#"height="24""#));
    assert!(placeholder_artifact.contains("#f3f4f6"));
    assert_eq!(
        asset["image_metadata"]["optimization"]["placeholder"]["artifact_bytes"],
        placeholder_artifact.as_bytes().len()
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["placeholder"]["hash"]
            .as_str()
            .expect("placeholder hash")
            .len(),
        16
    );

    let image_receipt = read_json(&root.join(".dx/receipts/build/image-metadata.json"));
    assert_eq!(image_receipt["summary"]["image_assets"], 1);
    assert_eq!(image_receipt["summary"]["metadata_assets"], 1);
    assert_eq!(image_receipt["summary"]["optimized_variants_emitted"], 0);
    assert_eq!(image_receipt["summary"]["placeholders_emitted"], 1);
    assert_eq!(
        image_receipt["boundary"]["optimization"],
        "metadata-plus-placeholder-artifacts-no-resize-or-encoding"
    );
    assert_eq!(
        image_receipt["boundary"]["placeholder_generation"],
        "svg-artifact-placeholders-when-dimensions-known"
    );
    assert_eq!(
        image_receipt["assets"][0]["image_metadata"]["optimization"]["placeholder"]["data_url"],
        placeholder_data_url
    );
    assert_eq!(
        image_receipt["assets"][0]["image_metadata"]["optimization"]["placeholder"]["output"],
        placeholder_output
    );

    let graph_snapshot = read_json(&report.graph_snapshot_path);
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["optimizedVariantCount"],
        0
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["placeholderCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["status"],
        "metadata-plus-placeholder-artifacts-boundary"
    );
}
