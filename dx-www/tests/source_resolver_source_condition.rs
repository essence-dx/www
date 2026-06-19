use std::fs;

use dx_www::build::{SourceBuildEngine, SourceBuildOptions};
use serde_json::Value;

#[test]
fn source_build_engine_prefers_package_source_conditions_without_node_modules() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("dist")).expect("dist dir");
    fs::create_dir_all(root.join("node_modules/node-only")).expect("node dir");
    fs::create_dir_all(root.join("node_modules/types-only")).expect("types dir");
    fs::create_dir_all(root.join("src")).expect("src dir");
    fs::create_dir_all(root.join("types")).expect("types dir");

    fs::write(
        root.join("package.json"),
        r###"{
  "name": "dx-source-condition-fixture",
  "imports": {
    "#feature": {
      "types": "./node_modules/types-only/feature.d.ts",
      "node": "./node_modules/node-only/feature.ts",
      "production": "./node_modules/node-only/production-feature.ts",
      "import": "./dist/feature.js",
      "source": "./src/feature.ts",
      "default": "./dist/default-feature.js"
    }
  },
  "exports": {
    ".": {
      "types": "./node_modules/types-only/public.d.ts",
      "node": "./node_modules/node-only/public.ts",
      "production": "./node_modules/node-only/production-public.ts",
      "import": "./dist/public.js",
      "source": "./src/public.ts",
      "default": "./dist/default-public.js"
    },
    "./widget": {
      "node": "./node_modules/node-only/widget.tsx",
      "production": "./node_modules/node-only/production-widget.tsx",
      "import": "./dist/widget.js",
      "source": "./src/widget.tsx",
      "default": "./dist/default-widget.js"
    }
  }
}
"###,
    )
    .expect("package json");
    fs::write(
        root.join("app/page.tsx"),
        r##"import { featureLabel } from "#feature";
import { packageLabel } from "dx-source-condition-fixture";
import { Widget } from "dx-source-condition-fixture/widget";

export default function Page() {
  return <main><Widget label={`${packageLabel()} ${featureLabel()}`} /></main>;
}
"##,
    )
    .expect("route source");
    fs::write(
        root.join("src/feature.ts"),
        r#"export function featureLabel() {
  return "source import condition";
}
"#,
    )
    .expect("source import condition");
    fs::write(
        root.join("src/public.ts"),
        r#"export function packageLabel() {
  return "source export condition";
}
"#,
    )
    .expect("source public export");
    fs::write(
        root.join("src/widget.tsx"),
        r#"export function Widget(props: { label: string }) {
  return <section data-widget="source-condition">{props.label}</section>;
}
"#,
    )
    .expect("source widget export");
    fs::write(
        root.join("dist/feature.js"),
        r#"export function featureLabel() {
  return "import condition";
}
"#,
    )
    .expect("dist feature");
    fs::write(
        root.join("dist/public.js"),
        r#"export function packageLabel() {
  return "import export condition";
}
"#,
    )
    .expect("dist public");
    fs::write(
        root.join("dist/widget.js"),
        r#"export function Widget(props) {
  return props.label;
}
"#,
    )
    .expect("dist widget");
    fs::write(
        root.join("dist/default-feature.js"),
        "export default null;\n",
    )
    .expect("default feature");
    fs::write(
        root.join("dist/default-public.js"),
        "export default null;\n",
    )
    .expect("default public");
    fs::write(
        root.join("dist/default-widget.js"),
        "export default null;\n",
    )
    .expect("default widget");
    fs::write(
        root.join("types/feature.d.ts"),
        "export declare const featureLabel: string;\n",
    )
    .expect("feature types");
    fs::write(
        root.join("types/public.d.ts"),
        "export declare const packageLabel: string;\n",
    )
    .expect("public types");
    fs::write(
        root.join("node_modules/types-only/feature.d.ts"),
        "export declare const featureLabel: string;\n",
    )
    .expect("node_modules feature types");
    fs::write(
        root.join("node_modules/types-only/public.d.ts"),
        "export declare const packageLabel: string;\n",
    )
    .expect("node_modules public types");
    fs::write(
        root.join("node_modules/node-only/feature.ts"),
        "export function featureLabel() { return 'node condition'; }\n",
    )
    .expect("node-only feature");
    fs::write(
        root.join("node_modules/node-only/public.ts"),
        "export function packageLabel() { return 'node export condition'; }\n",
    )
    .expect("node-only public");
    fs::write(
        root.join("node_modules/node-only/widget.tsx"),
        "export function Widget(props: { label: string }) { return props.label; }\n",
    )
    .expect("node-only widget");
    fs::write(
        root.join("node_modules/node-only/production-feature.ts"),
        "export function featureLabel() { return 'production condition'; }\n",
    )
    .expect("production feature");
    fs::write(
        root.join("node_modules/node-only/production-public.ts"),
        "export function packageLabel() { return 'production export condition'; }\n",
    )
    .expect("production public");
    fs::write(
        root.join("node_modules/node-only/production-widget.tsx"),
        "export function Widget(props: { label: string }) { return props.label; }\n",
    )
    .expect("production widget");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    assert!(!report.manifest.node_modules_required);
    assert!(!report.receipt.node_modules_required);

    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&report.manifest_path).expect("manifest json"))
            .expect("parse manifest");
    let source_module_chunks = manifest["route_outputs"][0]["source_module_chunks"]
        .as_array()
        .expect("source module chunks");

    assert_eq!(source_module_chunks.len(), 4);
    for source_path in ["src/feature.ts", "src/public.ts", "src/widget.tsx"] {
        assert!(
            source_module_chunks.iter().any(|chunk| {
                chunk["source_path"] == source_path && chunk["node_modules_required"] == false
            }),
            "missing source chunk {source_path}: {source_module_chunks:#?}"
        );
    }
    for generated_path in [
        "dist/feature.js",
        "dist/public.js",
        "dist/widget.js",
        "dist/default-feature.js",
        "dist/default-public.js",
        "dist/default-widget.js",
        "node_modules/node-only/feature.ts",
        "node_modules/node-only/public.ts",
        "node_modules/node-only/widget.tsx",
        "node_modules/node-only/production-feature.ts",
        "node_modules/node-only/production-public.ts",
        "node_modules/node-only/production-widget.tsx",
        "node_modules/types-only/feature.d.ts",
        "node_modules/types-only/public.d.ts",
        "types/feature.d.ts",
        "types/public.d.ts",
    ] {
        assert!(
            source_module_chunks
                .iter()
                .all(|chunk| chunk["source_path"] != generated_path),
            "generated/type target should not be linked: {generated_path}"
        );
    }

    let page_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "app/page.tsx")
        .expect("page chunk");
    let dependencies = page_chunk["dependencies"]
        .as_array()
        .expect("page dependencies");

    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "#feature"
                && dependency["resolved_path"] == "src/feature.ts"
                && dependency["kind"] == "ts"
                && dependency["resolver_source"] == "package-import"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().all(|dependency| {
            dependency["specifier"] != "#feature"
                || dependency["kind"] != "package-import-adapter-boundary"
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "dx-source-condition-fixture"
                && dependency["resolved_path"] == "src/public.ts"
                && dependency["kind"] == "ts"
                && dependency["resolver_source"] == "package-self-reference"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().all(|dependency| {
            dependency["specifier"] != "dx-source-condition-fixture"
                || dependency["kind"] != "package-export-adapter-boundary"
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "dx-source-condition-fixture/widget"
                && dependency["resolved_path"] == "src/widget.tsx"
                && dependency["kind"] == "tsx"
                && dependency["resolver_source"] == "package-self-reference"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().all(|dependency| {
            dependency["specifier"] != "dx-source-condition-fixture/widget"
                || dependency["kind"] != "package-export-adapter-boundary"
        }),
        "dependencies: {dependencies:#?}"
    );
}
