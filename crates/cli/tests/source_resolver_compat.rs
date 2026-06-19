use std::fs;

use dx_www::build::{SourceBuildEngine, SourceBuildOptions};
use serde_json::Value;

#[test]
fn source_build_engine_resolves_package_json_imports_without_node_modules() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("lib")).expect("lib dir");

    fs::write(
        root.join("package.json"),
        r###"{
  "name": "dx-source-resolver-fixture",
  "imports": {
    "#ui/*": "./components/*",
    "#lib/status": "./lib/status.ts",
    "#blocked/*": "left-pad/*"
  }
}
"###,
    )
    .expect("package json");
    fs::write(
        root.join("app/page.tsx"),
        r###"import { Hero } from "#ui/Hero";
import { statusLabel } from "#lib/status";
import blockedWidget from "#blocked/Widget";

export default function Page() {
  return <main data-blocked={typeof blockedWidget}><Hero label={statusLabel()} /></main>;
}
"###,
    )
    .expect("route source");
    fs::write(
        root.join("components/Hero.tsx"),
        r#"export function Hero(props: { label: string }) {
  return <section data-hero="package-import">{props.label}</section>;
}
"#,
    )
    .expect("component source");
    fs::write(
        root.join("lib/status.ts"),
        r#"export function statusLabel() {
  return "package imports";
}
"#,
    )
    .expect("helper source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    assert!(!report.manifest.node_modules_required);
    assert!(!report.receipt.node_modules_required);
    assert!(report.receipt.adapters.iter().any(|adapter| {
        adapter.name == "dx-source-resolver-adapter"
            && adapter.status.contains("package.json imports")
            && adapter
                .informed_by
                .iter()
                .any(|name| name == "turbopack-resolve")
    }));
    assert_eq!(
        report
            .receipt
            .summary
            .resolver_adapter_boundary_dependencies,
        1
    );
    assert!(
        report
            .receipt
            .summary
            .resolver_adapter_boundary_details
            .iter()
            .any(|detail| detail.resolver_source == "package-import-boundary"
                && detail.resolver_detail == "package-import-no-source-owned-target"
                && detail.dependencies == 1)
    );

    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&report.manifest_path).expect("manifest json"))
            .expect("parse manifest");
    let source_module_chunks = manifest["route_outputs"][0]["source_module_chunks"]
        .as_array()
        .expect("source module chunks");
    assert_eq!(source_module_chunks.len(), 3);
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "components/Hero.tsx" && chunk["node_modules_required"] == false
    }));
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "lib/status.ts" && chunk["node_modules_required"] == false
    }));

    let page_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "app/page.tsx")
        .expect("page chunk");
    let dependencies = page_chunk["dependencies"]
        .as_array()
        .expect("page dependencies");
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "#ui/Hero"
                && dependency["resolved_path"] == "components/Hero.tsx"
                && dependency["kind"] == "tsx"
                && dependency["resolver_source"] == "package-import"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "#lib/status"
                && dependency["resolved_path"] == "lib/status.ts"
                && dependency["kind"] == "ts"
                && dependency["resolver_source"] == "package-import"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "#blocked/Widget"
                && dependency["resolved_path"].is_null()
                && dependency["chunk_output"].is_null()
                && dependency["kind"] == "package-import-adapter-boundary"
                && dependency["resolver_source"] == "package-import-boundary"
                && dependency["resolver_detail"] == "package-import-no-source-owned-target"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies
            .iter()
            .all(|dependency| dependency["kind"] != "external-adapter-boundary")
    );

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let graph_edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    let graph_nodes = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes");
    assert!(
        graph_nodes.iter().any(|node| {
            node["kind"] == "adapter-boundary-import"
                && node["specifier"] == "#blocked/Widget"
                && node["dependency_kind"] == "package-import-adapter-boundary"
                && node["resolver_source"] == "package-import-boundary"
                && node["resolver_detail"] == "package-import-no-source-owned-target"
                && node["node_modules_required"] == false
        }),
        "graph nodes: {graph_nodes:#?}"
    );
    assert!(
        graph_edges.iter().any(|edge| {
            edge["kind"] == "imports-source-module"
                && edge["specifier"] == "#ui/Hero"
                && edge["resolver_source"] == "package-import"
        }),
        "graph edges: {graph_edges:#?}"
    );
    assert!(
        graph_edges.iter().any(|edge| {
            edge["kind"] == "adapter-boundary"
                && edge["specifier"] == "#blocked/Widget"
                && edge["dependency_kind"] == "package-import-adapter-boundary"
                && edge["resolver_source"] == "package-import-boundary"
                && edge["resolver_detail"] == "package-import-no-source-owned-target"
                && edge["node_modules_required"] == false
        }),
        "graph edges: {graph_edges:#?}"
    );
}

#[test]
fn source_build_engine_resolves_project_root_alias_from_src_app_to_src_modules_without_config() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("src/app")).expect("src app dir");
    fs::create_dir_all(root.join("src/components")).expect("src components dir");
    fs::create_dir_all(root.join("src/lib")).expect("src lib dir");

    fs::write(
        root.join("src/app/page.tsx"),
        r###"import { Hero } from "@/components/Hero";
import { statusLabel } from "@/lib/status";

export default function Page() {
  return <main><Hero label={statusLabel()} /></main>;
}
"###,
    )
    .expect("route source");
    fs::write(
        root.join("src/components/Hero.tsx"),
        r#"export function Hero(props: { label: string }) {
  return <section data-hero="src-root-alias">{props.label}</section>;
}
"#,
    )
    .expect("component source");
    fs::write(
        root.join("src/lib/status.ts"),
        r#"export function statusLabel() {
  return "src app alias";
}
"#,
    )
    .expect("status source");

    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(chunks.iter().any(|chunk| {
        chunk["source_path"] == "src/components/Hero.tsx" && chunk["node_modules_required"] == false
    }));
    assert!(chunks.iter().any(|chunk| {
        chunk["source_path"] == "src/lib/status.ts" && chunk["node_modules_required"] == false
    }));
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "@/components/Hero"
                && dependency["resolved_path"] == "src/components/Hero.tsx"
                && dependency["kind"] == "tsx"
                && dependency["resolver_source"] == "src-project-root-alias"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "@/lib/status"
                && dependency["resolved_path"] == "src/lib/status.ts"
                && dependency["kind"] == "ts"
                && dependency["resolver_source"] == "src-project-root-alias"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
}

#[test]
fn source_build_engine_resolves_package_json_imports_parent_segments_inside_project() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("node_modules/trap")).expect("blocked dir");

    fs::write(
        root.join("package.json"),
        r###"{
  "name": "dx-source-resolver-fixture",
  "imports": {
    "#shared/*": "./src/../components/*",
    "#blocked/*": "./src/../node_modules/trap/*"
  }
}
"###,
    )
    .expect("package json");
    fs::write(root.join("app/page.tsx"), r###"import { Hero } from "#shared/Hero";
import blockedWidget from "#blocked/Widget";

export default function Page() {
  return <main data-blocked={typeof blockedWidget}><Hero label="normalized package import" /></main>;
}
"###).expect("route source");
    fs::write(root.join("components/Hero.tsx"), "export function Hero(props: { label: string }) { return <section>{props.label}</section>; }\n").expect("component source");
    fs::write(
        root.join("node_modules/trap/Widget.ts"),
        "export default 'blocked';\n",
    )
    .expect("blocked source");

    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(chunks.iter().any(|chunk| {
        chunk["source_path"] == "components/Hero.tsx" && chunk["node_modules_required"] == false
    }));
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "node_modules/trap/Widget.ts")
    );
    assert!(dependencies.iter().any(|dependency| {
        dependency["specifier"] == "#shared/Hero"
            && dependency["resolved_path"] == "components/Hero.tsx"
            && dependency["resolver_source"] == "package-import"
            && dependency["node_modules_required"] == false
    }));
    assert!(dependencies.iter().any(|dependency| {
        dependency["specifier"] == "#blocked/Widget"
            && dependency["kind"] == "package-import-adapter-boundary"
            && dependency["resolver_source"] == "package-import-boundary"
            && dependency["resolver_detail"] == "package-import-target-node-modules-boundary"
            && dependency["node_modules_required"] == false
    }));
}

#[test]
fn source_build_engine_resolves_conditional_package_imports_without_node_modules() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("lib")).expect("lib dir");
    fs::create_dir_all(root.join("node")).expect("node dir");
    fs::create_dir_all(root.join("types")).expect("types dir");

    fs::write(
        root.join("package.json"),
        r###"{
  "name": "dx-source-resolver-fixture",
  "imports": {
    "#feature": {
      "types": "./types/feature.d.ts",
      "node": "./node/feature.ts",
      "import": "./lib/feature.ts",
      "default": "./lib/default.ts"
    },
    "#ui/*": {
      "types": "./types/*.d.ts",
      "import": "./components/*.tsx"
    },
    "#array-fallback": [
      {
        "types": "./types/array-fallback.d.ts",
        "node": "./node/array-fallback.ts"
      },
      {
        "import": "./lib/array-fallback.ts"
      }
    ]
  }
}
"###,
    )
    .expect("package json");
    fs::write(
        root.join("app/page.tsx"),
        r###"import { Hero } from "#ui/Hero";
import { featureLabel } from "#feature";
import { arrayFallbackLabel } from "#array-fallback";

export default function Page() {
  return <main><Hero label={`${featureLabel()} ${arrayFallbackLabel()}`} /></main>;
}
"###,
    )
    .expect("route source");
    fs::write(
        root.join("components/Hero.tsx"),
        r#"export function Hero(props: { label: string }) {
  return <section data-hero="conditional-package-import">{props.label}</section>;
}
"#,
    )
    .expect("component source");
    fs::write(
        root.join("lib/feature.ts"),
        r#"export function featureLabel() {
  return "conditional import";
}
"#,
    )
    .expect("runtime feature source");
    fs::write(
        root.join("lib/default.ts"),
        r#"export function featureLabel() {
  return "default fallback";
}
"#,
    )
    .expect("default feature source");
    fs::write(
        root.join("lib/array-fallback.ts"),
        r#"export function arrayFallbackLabel() {
  return "array fallback";
}
"#,
    )
    .expect("array fallback source");
    fs::write(
        root.join("node/feature.ts"),
        r#"export function featureLabel() {
  return "node condition";
}
"#,
    )
    .expect("node condition source");
    fs::write(
        root.join("node/array-fallback.ts"),
        r#"export function arrayFallbackLabel() {
  return "node array fallback";
}
"#,
    )
    .expect("node array fallback source");
    fs::write(
        root.join("types/feature.d.ts"),
        "export declare const featureLabel: string;\n",
    )
    .expect("feature types");
    fs::write(
        root.join("types/array-fallback.d.ts"),
        "export declare const arrayFallbackLabel: string;\n",
    )
    .expect("array fallback types");
    fs::write(
        root.join("types/Hero.d.ts"),
        "export declare const Hero: unknown;\n",
    )
    .expect("hero types");

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
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "components/Hero.tsx" && chunk["node_modules_required"] == false
    }));
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "lib/feature.ts" && chunk["node_modules_required"] == false
    }));
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "lib/array-fallback.ts" && chunk["node_modules_required"] == false
    }));
    assert!(
        source_module_chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "node/feature.ts")
    );
    assert!(
        source_module_chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "node/array-fallback.ts")
    );
    assert!(source_module_chunks.iter().all(|chunk| {
        chunk["source_path"] != "types/feature.d.ts"
            && chunk["source_path"] != "types/array-fallback.d.ts"
            && chunk["source_path"] != "types/Hero.d.ts"
    }));

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
                && dependency["resolved_path"] == "lib/feature.ts"
                && dependency["kind"] == "ts"
                && dependency["resolver_source"] == "package-import"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "#ui/Hero"
                && dependency["resolved_path"] == "components/Hero.tsx"
                && dependency["kind"] == "tsx"
                && dependency["resolver_source"] == "package-import"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "#array-fallback"
                && dependency["resolved_path"] == "lib/array-fallback.ts"
                && dependency["kind"] == "ts"
                && dependency["resolver_source"] == "package-import"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let graph_edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(
        graph_edges.iter().any(|edge| {
            edge["kind"] == "imports-source-module"
                && edge["specifier"] == "#feature"
                && edge["resolver_source"] == "package-import"
        }),
        "graph edges: {graph_edges:#?}"
    );
    assert!(
        graph_edges.iter().any(|edge| {
            edge["kind"] == "imports-source-module"
                && edge["specifier"] == "#array-fallback"
                && edge["resolver_source"] == "package-import"
        }),
        "graph edges: {graph_edges:#?}"
    );
}

#[test]
fn source_build_engine_keeps_base_url_node_modules_imports_at_adapter_boundary_without_node_modules()
 {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("lib")).expect("lib dir");
    fs::create_dir_all(root.join("node_modules")).expect("node_modules dir");

    fs::write(
        root.join("tsconfig.json"),
        r###"{
  "compilerOptions": {
    "baseUrl": "."
  }
}
"###,
    )
    .expect("tsconfig json");
    fs::write(
        root.join("app/page.tsx"),
        r#"import { Hero } from "components/Hero";
import { statusLabel } from "lib/status";
import trap from "node_modules/trap";

export default function Page() {
  return <main data-trap={typeof trap}><Hero label={statusLabel()} /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("components/Hero.tsx"),
        r#"export function Hero(props: { label: string }) {
  return <section data-hero="base-url-import">{props.label}</section>;
}
"#,
    )
    .expect("component source");
    fs::write(
        root.join("lib/status.ts"),
        r#"export function statusLabel() {
  return "base url imports";
}
"#,
    )
    .expect("helper source");
    fs::write(
        root.join("node_modules/trap.ts"),
        "export default 'trap';\n",
    )
    .expect("node_modules trap");

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
    assert_eq!(source_module_chunks.len(), 3);
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "components/Hero.tsx" && chunk["node_modules_required"] == false
    }));
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "lib/status.ts" && chunk["node_modules_required"] == false
    }));
    assert!(
        source_module_chunks
            .iter()
            .all(|chunk| { chunk["source_path"] != "node_modules/trap.ts" })
    );

    let page_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "app/page.tsx")
        .expect("page chunk");
    let dependencies = page_chunk["dependencies"]
        .as_array()
        .expect("page dependencies");
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "components/Hero"
                && dependency["resolved_path"] == "components/Hero.tsx"
                && dependency["kind"] == "tsx"
                && dependency["resolver_source"] == "tsconfig-base-url"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "lib/status"
                && dependency["resolved_path"] == "lib/status.ts"
                && dependency["kind"] == "ts"
                && dependency["resolver_source"] == "tsconfig-base-url"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "node_modules/trap"
                && dependency["resolved_path"].is_null()
                && dependency["chunk_output"].is_null()
                && dependency["kind"] == "base-url-node-modules-adapter-boundary"
                && dependency["resolver_source"] == "base-url-node-modules-boundary"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let graph_edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(
        graph_edges.iter().any(|edge| {
            edge["kind"] == "imports-source-module"
                && edge["specifier"] == "components/Hero"
                && edge["resolver_source"] == "tsconfig-base-url"
        }),
        "graph edges: {graph_edges:#?}"
    );
}

#[test]
fn source_build_engine_links_forge_materialized_re_exports_without_node_modules() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join(".dx/forge/cache/reactive-store")).expect("forge cache dir");

    fs::write(
        root.join("package.json"),
        r###"{
  "name": "dx-forge-reexport-fixture",
  "imports": {
    "#forge/reactive-store": "./.dx/forge/cache/reactive-store/index.ts"
  }
}
"###,
    )
    .expect("package json");
    fs::write(
        root.join("app/page.tsx"),
        r###"import { createDxStore, reactiveStoreMetadata } from "#forge/reactive-store";

export default function Page() {
  return <main data-store={createDxStore().name}>{reactiveStoreMetadata.packageId}</main>;
}
"###,
    )
    .expect("route source");
    fs::write(
        root.join(".dx/forge/cache/reactive-store/index.ts"),
        r#"export * from "./store";
export { reactiveStoreMetadata } from "./metadata";
export * from "left-pad";
export * from "@scope/widget";
"#,
    )
    .expect("forge index");
    fs::write(
        root.join(".dx/forge/cache/reactive-store/store.ts"),
        r#"export function createDxStore() {
  return { name: "reactive-store" };
}
"#,
    )
    .expect("forge store");
    fs::write(
        root.join(".dx/forge/cache/reactive-store/metadata.ts"),
        r#"export const reactiveStoreMetadata = {
  packageId: "state/reactive-store",
};
"#,
    )
    .expect("forge metadata");

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
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == ".dx/forge/cache/reactive-store/index.ts"
            && chunk["node_modules_required"] == false
    }));
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == ".dx/forge/cache/reactive-store/store.ts"
            && chunk["node_modules_required"] == false
    }));
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == ".dx/forge/cache/reactive-store/metadata.ts"
            && chunk["node_modules_required"] == false
    }));

    let index_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == ".dx/forge/cache/reactive-store/index.ts")
        .expect("forge index chunk");
    let dependencies = index_chunk["dependencies"]
        .as_array()
        .expect("forge index dependencies");
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "./store"
                && dependency["resolved_path"] == ".dx/forge/cache/reactive-store/store.ts"
                && dependency["kind"] == "ts"
                && dependency["resolver_source"] == "relative"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "./metadata"
                && dependency["resolved_path"] == ".dx/forge/cache/reactive-store/metadata.ts"
                && dependency["kind"] == "ts"
                && dependency["resolver_source"] == "relative"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "left-pad"
                && dependency["resolved_path"].is_null()
                && dependency["chunk_output"].is_null()
                && dependency["kind"] == "external-package-adapter-boundary"
                && dependency["resolver_source"] == "external-package-boundary"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "@scope/widget"
                && dependency["resolved_path"].is_null()
                && dependency["chunk_output"].is_null()
                && dependency["kind"] == "external-package-adapter-boundary"
                && dependency["resolver_source"] == "external-package-boundary"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let graph_edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(
        graph_edges.iter().any(|edge| {
            edge["kind"] == "imports-source-module"
                && edge["specifier"] == "./store"
                && edge["resolver_source"] == "relative"
        }),
        "graph edges: {graph_edges:#?}"
    );
}

#[test]
fn source_build_engine_resolves_package_self_reference_root_without_node_modules() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::write(
        root.join("package.json"),
        r###"{
  "name": "dx-source-resolver-fixture"
}
"###,
    )
    .expect("package json");
    fs::write(
        root.join("app/page.tsx"),
        r#"import { packageLabel } from "dx-source-resolver-fixture";

export default function Page() {
  return <main>{packageLabel()}</main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("index.ts"),
        r#"export function packageLabel() {
  return "package self reference root";
}
"#,
    )
    .expect("package root source");

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
    assert_eq!(source_module_chunks.len(), 2);
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "index.ts" && chunk["node_modules_required"] == false
    }));

    let page_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "app/page.tsx")
        .expect("page chunk");
    let dependencies = page_chunk["dependencies"]
        .as_array()
        .expect("page dependencies");
    assert! {
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "dx-source-resolver-fixture"
                && dependency["resolved_path"] == "index.ts"
                && dependency["kind"] == "ts"
                && dependency["resolver_source"] == "package-self-reference"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    };

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let graph_edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(
        graph_edges.iter().any(|edge| {
            edge["kind"] == "imports-source-module"
                && edge["specifier"] == "dx-source-resolver-fixture"
                && edge["resolver_source"] == "package-self-reference"
        }),
        "graph edges: {graph_edges:#?}"
    );
}

#[test]
fn source_build_engine_resolves_package_self_references_without_exports() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("lib")).expect("lib dir");

    fs::write(
        root.join("package.json"),
        r###"{
  "name": "dx-source-resolver-fixture"
}
"###,
    )
    .expect("package json");
    fs::write(
        root.join("app/page.tsx"),
        r#"import { Hero } from "dx-source-resolver-fixture/components/Hero";
import { statusLabel } from "dx-source-resolver-fixture/lib/status";

export default function Page() {
  return <main><Hero label={statusLabel()} /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("components/Hero.tsx"),
        r#"export function Hero(props: { label: string }) {
  return <section data-hero="self-reference">{props.label}</section>;
}
"#,
    )
    .expect("component source");
    fs::write(
        root.join("lib/status.ts"),
        r#"export function statusLabel() {
  return "package self reference";
}
"#,
    )
    .expect("helper source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    assert!(!report.manifest.node_modules_required);
    assert!(!report.receipt.node_modules_required);
    assert!(report.receipt.adapters.iter().any(|adapter| {
        adapter.name == "dx-source-resolver-adapter"
            && adapter.status.contains("package self-references")
            && adapter
                .informed_by
                .iter()
                .any(|name| name == "turbopack-resolve")
    }));

    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&report.manifest_path).expect("manifest json"))
            .expect("parse manifest");
    let source_module_chunks = manifest["route_outputs"][0]["source_module_chunks"]
        .as_array()
        .expect("source module chunks");
    assert_eq!(source_module_chunks.len(), 3);
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "components/Hero.tsx" && chunk["node_modules_required"] == false
    }));
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "lib/status.ts" && chunk["node_modules_required"] == false
    }));

    let page_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "app/page.tsx")
        .expect("page chunk");
    let dependencies = page_chunk["dependencies"]
        .as_array()
        .expect("page dependencies");
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "dx-source-resolver-fixture/components/Hero"
                && dependency["resolved_path"] == "components/Hero.tsx"
                && dependency["kind"] == "tsx"
                && dependency["resolver_source"] == "package-self-reference"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "dx-source-resolver-fixture/lib/status"
                && dependency["resolved_path"] == "lib/status.ts"
                && dependency["kind"] == "ts"
                && dependency["resolver_source"] == "package-self-reference"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies
            .iter()
            .all(|dependency| dependency["kind"] != "external-adapter-boundary")
    );

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let graph_edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(
        graph_edges.iter().any(|edge| {
            edge["kind"] == "imports-source-module"
                && edge["specifier"] == "dx-source-resolver-fixture/components/Hero"
                && edge["resolver_source"] == "package-self-reference"
        }),
        "graph edges: {graph_edges:#?}"
    );
}

#[test]
fn source_build_engine_records_adapter_boundary_resolver_source_without_node_modules() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"import { motion } from "motion/react";
import leftPad from "left-pad";
import scopedWidget from "@scope/widget";

export default function Page() {
  return <main data-motion={typeof motion} data-pad={typeof leftPad} data-scoped={typeof scopedWidget}>Adapter boundary</main>;
}
"#,
    )
    .expect("route source");

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
    assert_eq!(source_module_chunks.len(), 1);
    let page_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "app/page.tsx")
        .expect("page chunk");
    let dependencies = page_chunk["dependencies"]
        .as_array()
        .expect("page dependencies");
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "motion/react"
                && dependency["resolved_path"].is_null()
                && dependency["chunk_output"].is_null()
                && dependency["kind"] == "source-owned-adapter-boundary"
                && dependency["resolver_source"] == "adapter-boundary"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "left-pad"
                && dependency["resolved_path"].is_null()
                && dependency["chunk_output"].is_null()
                && dependency["kind"] == "external-package-adapter-boundary"
                && dependency["resolver_source"] == "external-package-boundary"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let graph_nodes = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes");
    assert!(
        graph_nodes.iter().any(|node| {
            node["kind"] == "adapter-boundary-import"
                && node["specifier"] == "motion/react"
                && node["dependency_kind"] == "source-owned-adapter-boundary"
                && node["resolver_source"] == "adapter-boundary"
                && node["node_modules_required"] == false
        }),
        "graph nodes: {graph_nodes:#?}"
    );

    let graph_edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(
        graph_edges.iter().any(|edge| {
            edge["kind"] == "adapter-boundary"
                && edge["specifier"] == "motion/react"
                && edge["dependency_kind"] == "source-owned-adapter-boundary"
                && edge["resolver_source"] == "adapter-boundary"
                && edge["node_modules_required"] == false
        }),
        "graph edges: {graph_edges:#?}"
    );
}

#[test]
fn source_build_engine_resolves_static_dynamic_imports_without_node_modules() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"const panelModule = import("@/components/LazyPanel");

export default function Page() {
  return <main data-panel={typeof panelModule}>Dynamic import</main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("components/LazyPanel.tsx"),
        r#"export const lazyPanelLabel = "linked dynamic import";
"#,
    )
    .expect("dynamic import source");

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
    assert_eq!(source_module_chunks.len(), 2);
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "components/LazyPanel.tsx"
            && chunk["node_modules_required"] == false
    }));

    let page_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "app/page.tsx")
        .expect("page chunk");
    let dependencies = page_chunk["dependencies"]
        .as_array()
        .expect("page dependencies");
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "@/components/LazyPanel"
                && dependency["resolved_path"] == "components/LazyPanel.tsx"
                && dependency["kind"] == "tsx"
                && dependency["resolver_source"] == "project-root-alias"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        page_chunk["ecmascript_analysis"]["dynamic_imports"]
            .as_array()
            .expect("dynamic imports")
            .iter()
            .any(|dynamic_import| dynamic_import["specifier"] == "@/components/LazyPanel"),
        "page chunk: {page_chunk:#?}"
    );

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let graph_edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(
        graph_edges.iter().any(|edge| {
            edge["kind"] == "imports-source-module"
                && edge["specifier"] == "@/components/LazyPanel"
                && edge["resolver_source"] == "project-root-alias"
        }),
        "graph edges: {graph_edges:#?}"
    );
}

#[test]
fn source_build_engine_resolves_three_project_root_alias_to_source_owned_barrel_without_node_modules()
 {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("three")).expect("three alias dir");
    fs::create_dir_all(root.join("lib/scene")).expect("scene dir");
    fs::create_dir_all(root.join("node_modules/three")).expect("node_modules trap dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"import { sceneLabel } from "@/three";

export default function Page() {
  return <main data-scene={sceneLabel()}>Source-owned three alias</main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("three/index.ts"),
        r#"export * from "../lib/scene/index";
"#,
    )
    .expect("three source alias");
    fs::write(
        root.join("lib/scene/index.ts"),
        r#"export function sceneLabel() {
  return "source-owned scene";
}
"#,
    )
    .expect("scene source");
    fs::write(
        root.join("node_modules/three/index.ts"),
        "export const sceneLabel = 'node_modules trap';\n",
    )
    .expect("node_modules trap");

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
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "three/index.ts" && chunk["node_modules_required"] == false
    }));
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "lib/scene/index.ts" && chunk["node_modules_required"] == false
    }));
    assert!(
        source_module_chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "node_modules/three/index.ts")
    );

    let page_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "app/page.tsx")
        .expect("page chunk");
    let page_dependencies = page_chunk["dependencies"]
        .as_array()
        .expect("page dependencies");
    assert!(
        page_dependencies.iter().any(|dependency| {
            dependency["specifier"] == "@/three"
                && dependency["resolved_path"] == "three/index.ts"
                && dependency["kind"] == "ts"
                && dependency["resolver_source"] == "project-root-alias"
                && dependency["node_modules_required"] == false
        }),
        "page dependencies: {page_dependencies:#?}"
    );

    let alias_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "three/index.ts")
        .expect("three alias chunk");
    let alias_dependencies = alias_chunk["dependencies"]
        .as_array()
        .expect("alias dependencies");
    assert!(
        alias_dependencies.iter().any(|dependency| {
            dependency["specifier"] == "../lib/scene/index"
                && dependency["resolved_path"] == "lib/scene/index.ts"
                && dependency["kind"] == "ts"
                && dependency["resolver_source"] == "relative"
                && dependency["node_modules_required"] == false
        }),
        "alias dependencies: {alias_dependencies:#?}"
    );
}

#[test]
fn source_build_engine_keeps_bare_three_at_external_package_boundary_without_node_modules() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("node_modules/three")).expect("node_modules trap dir");
    fs::write(
        root.join("app/page.tsx"),
        r#"import { Vector3 } from "three";

export default function Page() {
  return <main data-vector={typeof Vector3}>Bare three boundary</main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("node_modules/three/index.ts"),
        "export const Vector3 = 'node_modules trap';\n",
    )
    .expect("node_modules trap");

    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert_eq!(chunks.len(), 1);
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "node_modules/three/index.ts")
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "three"
                && dependency["resolved_path"].is_null()
                && dependency["chunk_output"].is_null()
                && dependency["kind"] == "external-package-adapter-boundary"
                && dependency["resolver_source"] == "external-package-boundary"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
}

#[test]
fn source_build_engine_prefers_tsconfig_project_root_alias_over_builtin_project_root_alias() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("src/components")).expect("src components dir");

    fs::write(
        root.join("tsconfig.json"),
        r###"{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  }
}
"###,
    )
    .expect("tsconfig json");
    fs::write(
        root.join("app/page.tsx"),
        r#"import { Hero } from "@/components/Hero";

export default function Page() {
  return <main><Hero label="src alias" /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("src/components/Hero.tsx"),
        r#"export function Hero(props: { label: string }) {
  return <section data-hero="src-alias">{props.label}</section>;
}
"#,
    )
    .expect("hero source");

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
    assert_eq!(source_module_chunks.len(), 2);
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "src/components/Hero.tsx" && chunk["node_modules_required"] == false
    }));

    let page_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "app/page.tsx")
        .expect("page chunk");
    let dependencies = page_chunk["dependencies"]
        .as_array()
        .expect("page dependencies");
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "@/components/Hero"
                && dependency["resolved_path"] == "src/components/Hero.tsx"
                && dependency["kind"] == "tsx"
                && dependency["resolver_source"] == "tsconfig-path"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().all(|dependency| {
            dependency["specifier"] != "@/components/Hero"
                || dependency["resolver_source"] != "project-root-alias"
        }),
        "dependencies: {dependencies:#?}"
    );

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let graph_edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(
        graph_edges.iter().any(|edge| {
            edge["kind"] == "imports-source-module"
                && edge["specifier"] == "@/components/Hero"
                && edge["resolver_source"] == "tsconfig-path"
        }),
        "graph edges: {graph_edges:#?}"
    );
}

#[test]
fn source_build_engine_falls_back_to_src_project_root_alias_after_missing_safe_config_aliases() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("src/app")).expect("src app dir");
    fs::create_dir_all(root.join("src/components")).expect("src components dir");

    fs::write(
        root.join("tsconfig.json"),
        r###"{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@/*": ["generated/*", "missing/*"]
    }
  }
}
"###,
    )
    .expect("tsconfig json");
    fs::write(
        root.join("src/app/page.tsx"),
        r#"import { Hero } from "@/components/Hero";

export default function Page() {
  return <main><Hero label="src fallback" /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("src/components/Hero.tsx"),
        r#"export function Hero(props: { label: string }) {
  return <section data-hero="src-fallback">{props.label}</section>;
}
"#,
    )
    .expect("hero source");

    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(chunks.iter().any(|chunk| {
        chunk["source_path"] == "src/components/Hero.tsx" && chunk["node_modules_required"] == false
    }));
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "@/components/Hero"
                && dependency["resolved_path"] == "src/components/Hero.tsx"
                && dependency["kind"] == "tsx"
                && dependency["resolver_source"] == "src-project-root-alias"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().all(|dependency| {
            dependency["specifier"] != "@/components/Hero"
                || dependency["resolver_source"] != "source-alias-unresolved"
        }),
        "dependencies: {dependencies:#?}"
    );
}

#[test]
fn source_build_engine_resolves_path_alias_parent_segments_that_stay_inside_project() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("src")).expect("src dir");
    fs::create_dir_all(root.join("components")).expect("components dir");

    fs::write(
        root.join("tsconfig.json"),
        r###"{
  "compilerOptions": {
    "baseUrl": "src",
    "paths": {
      "@shared/*": ["../components/*"]
    }
  }
}
"###,
    )
    .expect("tsconfig json");
    fs::write(
        root.join("app/page.tsx"),
        r#"import { Hero } from "@shared/Hero";

export default function Page() {
  return <main><Hero label="normalized alias" /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("components/Hero.tsx"),
        r#"export function Hero(props: { label: string }) {
  return <section data-hero="normalized-alias">{props.label}</section>;
}
"#,
    )
    .expect("hero source");

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
    assert_eq!(source_module_chunks.len(), 2);
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "components/Hero.tsx" && chunk["node_modules_required"] == false
    }));

    let page_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "app/page.tsx")
        .expect("page chunk");
    let dependencies = page_chunk["dependencies"]
        .as_array()
        .expect("page dependencies");
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "@shared/Hero"
                && dependency["resolved_path"] == "components/Hero.tsx"
                && dependency["kind"] == "tsx"
                && dependency["resolver_source"] == "tsconfig-path"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().all(|dependency| {
            dependency["specifier"] != "@shared/Hero"
                || dependency["resolver_source"] != "source-alias-boundary"
        }),
        "dependencies: {dependencies:#?}"
    );
}

#[test]
fn source_build_engine_keeps_mixed_missing_and_external_package_imports_at_adapter_boundary_without_node_modules()
 {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("node_modules/left-pad")).expect("external dir");
    fs::write(
        root.join("package.json"),
        r###"{
  "name": "dx-source-resolver-fixture",
  "imports": {
    "#safe/*": "./components/*",
    "#mixed/*": ["./missing/*", "left-pad/*"]
  }
}
"###,
    )
    .expect("package json");
    fs::write(root.join("app/page.tsx"), r###"import { Hero } from "#safe/Hero";
import mixedWidget from "#mixed/Widget";
export default function Page() { return <main data-mixed={typeof mixedWidget}><Hero label="mixed package import" /></main>; }
"###).expect("route source");
    fs::write(root.join("components/Hero.tsx"), "export function Hero(props: { label: string }) { return <section>{props.label}</section>; }\n").expect("component source");
    fs::write(
        root.join("node_modules/left-pad/Widget.ts"),
        "export default 'external fallback';\n",
    )
    .expect("external fallback source");
    let (_chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(dependencies.iter().any(|dependency| {
        dependency["specifier"] == "#mixed/Widget"
            && dependency["kind"] == "package-import-adapter-boundary"
            && dependency["resolver_source"] == "package-import-boundary"
            && dependency["node_modules_required"] == false
    }));
}

#[test]
fn source_build_engine_keeps_mixed_safe_and_external_package_imports_at_adapter_boundary_without_node_modules()
 {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("node_modules/left-pad")).expect("external dir");
    fs::write(
        root.join("package.json"),
        r###"{
  "name": "dx-source-resolver-fixture",
  "imports": {
    "#mixed/*": ["./components/*", "left-pad/*"]
  }
}
"###,
    )
    .expect("package json");
    fs::write(root.join("app/page.tsx"), r###"import mixedWidget from "#mixed/Widget";
export default function Page() { return <main data-mixed={typeof mixedWidget}>Mixed package import boundary</main>; }
"###).expect("route source");
    fs::write(
        root.join("components/Widget.ts"),
        "export default 'safe source must not be linked while an external fallback is present';\n",
    )
    .expect("safe source");
    fs::write(
        root.join("node_modules/left-pad/Widget.ts"),
        "export default 'external fallback';\n",
    )
    .expect("external fallback source");
    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "components/Widget.ts"
                && chunk["source_path"] != "node_modules/left-pad/Widget.ts")
    );
    assert!(dependencies.iter().any(|dependency| {
        dependency["specifier"] == "#mixed/Widget"
            && dependency["kind"] == "package-import-adapter-boundary"
            && dependency["resolver_source"] == "package-import-boundary"
            && dependency["node_modules_required"] == false
    }));
}

#[test]
fn source_build_engine_resolves_safe_parent_segments_in_tsconfig_path_aliases_without_boundary() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("node_modules/trap")).expect("trap dir");
    fs::create_dir_all(root.join("src")).expect("src dir");
    fs::write(
        root.join("tsconfig.json"),
        r###"{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@shared/*": ["src/../components/*"],
      "@blocked/*": ["src/../node_modules/trap/*"]
    }
  }
}
"###,
    )
    .expect("tsconfig json");
    fs::write(
        root.join("app/page.tsx"),
        r#"import { Hero } from "@shared/Hero";
import blocked from "@blocked/Widget";
export default function Page() {
  return <main data-blocked={typeof blocked}><Hero label="safe parent alias" /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("components/Hero.tsx"),
        "export function Hero(props: { label: string }) { return <section>{props.label}</section>; }\n",
    )
    .expect("component source");
    fs::write(
        root.join("node_modules/trap/Widget.ts"),
        "export default 'trap';\n",
    )
    .expect("trap source");

    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(chunks.iter().any(|chunk| {
        chunk["source_path"] == "components/Hero.tsx" && chunk["node_modules_required"] == false
    }));
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "node_modules/trap/Widget.ts")
    );
    assert!(dependencies.iter().any(|dependency| {
        dependency["specifier"] == "@shared/Hero"
            && dependency["resolved_path"] == "components/Hero.tsx"
            && dependency["kind"] == "tsx"
            && dependency["resolver_source"] == "tsconfig-path"
            && dependency["node_modules_required"] == false
    }));
    assert!(dependencies.iter().any(|dependency| {
        dependency["specifier"] == "@blocked/Widget"
            && dependency["kind"] == "source-alias-adapter-boundary"
            && dependency["resolver_source"] == "source-alias-boundary"
            && dependency["resolver_detail"] == "source-alias-target-node-modules-boundary"
            && dependency["node_modules_required"] == false
    }));
}

#[test]
fn source_build_engine_keeps_tsconfig_node_modules_path_aliases_at_adapter_boundary_without_node_modules()
 {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("node_modules/vendor")).expect("vendor dir");
    fs::write(
        root.join("tsconfig.json"),
        r###"{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@safe/*": ["components/*"],
      "@vendor/*": ["node_modules/vendor/*"]
    }
  }
}
"###,
    )
    .expect("tsconfig json");
    fs::write(root.join("app/page.tsx"), r#"import { Hero } from "@safe/Hero";
import { vendorWidget } from "@vendor/Button";
export default function Page() { return <main data-vendor={typeof vendorWidget}><Hero label="path alias" /></main>; }
"#).expect("route source");
    fs::write(root.join("components/Hero.tsx"), "export function Hero(props: { label: string }) { return <section>{props.label}</section>; }\n").expect("component source");
    fs::write(
        root.join("node_modules/vendor/Button.ts"),
        "export const vendorWidget = 'vendor';\n",
    )
    .expect("vendor source");
    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "node_modules/vendor/Button.ts")
    );
    assert!(
        dependencies
            .iter()
            .any(|dependency| dependency["specifier"] == "@vendor/Button"
                && dependency["kind"] == "source-alias-adapter-boundary"
                && dependency["resolver_source"] == "source-alias-boundary"
                && dependency["resolver_detail"] == "source-alias-target-node-modules-boundary"
                && dependency["node_modules_required"] == false)
    );
}

#[test]
fn source_build_engine_keeps_mixed_missing_and_node_modules_path_aliases_at_adapter_boundary_without_node_modules()
 {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("node_modules/vendor")).expect("vendor dir");
    fs::write(
        root.join("tsconfig.json"),
        r###"{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@safe/*": ["components/*"],
      "@mixed/*": ["missing/*", "node_modules/vendor/*"]
    }
  }
}
"###,
    )
    .expect("tsconfig json");
    fs::write(root.join("app/page.tsx"), r#"import { Hero } from "@safe/Hero";
import { vendorWidget } from "@mixed/Button";
export default function Page() { return <main data-vendor={typeof vendorWidget}><Hero label="mixed path alias" /></main>; }
"#).expect("route source");
    fs::write(root.join("components/Hero.tsx"), "export function Hero(props: { label: string }) { return <section>{props.label}</section>; }\n").expect("component source");
    fs::write(
        root.join("node_modules/vendor/Button.ts"),
        "export const vendorWidget = 'vendor';\n",
    )
    .expect("vendor source");
    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "missing/Button.ts"
                && chunk["source_path"] != "node_modules/vendor/Button.ts")
    );
    assert!(
        dependencies
            .iter()
            .any(|dependency| dependency["specifier"] == "@mixed/Button"
                && dependency["kind"] == "source-alias-adapter-boundary"
                && dependency["resolver_source"] == "source-alias-boundary"
                && dependency["node_modules_required"] == false)
    );
}

#[test]
fn source_build_engine_keeps_mixed_safe_and_node_modules_path_aliases_at_adapter_boundary_without_node_modules()
 {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("node_modules/vendor")).expect("vendor dir");
    fs::write(
        root.join("tsconfig.json"),
        r###"{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@mixed/*": ["components/*", "node_modules/vendor/*"]
    }
  }
}
"###,
    )
    .expect("tsconfig json");
    fs::write(root.join("app/page.tsx"), r#"import { vendorWidget } from "@mixed/Button";
export default function Page() { return <main data-vendor={typeof vendorWidget}>Mixed path alias boundary</main>; }
"#).expect("route source");
    fs::write(
        root.join("components/Button.ts"),
        "export const vendorWidget = 'safe source must not be linked while a node_modules fallback is present';\n",
    )
    .expect("safe source");
    fs::write(
        root.join("node_modules/vendor/Button.ts"),
        "export const vendorWidget = 'vendor';\n",
    )
    .expect("vendor source");
    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "components/Button.ts"
                && chunk["source_path"] != "node_modules/vendor/Button.ts")
    );
    assert!(
        dependencies
            .iter()
            .any(|dependency| dependency["specifier"] == "@mixed/Button"
                && dependency["kind"] == "source-alias-adapter-boundary"
                && dependency["resolver_source"] == "source-alias-boundary"
                && dependency["node_modules_required"] == false)
    );
}

#[test]
fn source_build_engine_keeps_outside_project_path_aliases_at_adapter_boundary_without_node_modules()
{
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::write(
        root.join("tsconfig.json"),
        r###"{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@safe/*": ["components/*"],
      "@outside/*": ["../outside/*"]
    }
  }
}
"###,
    )
    .expect("tsconfig json");
    fs::write(root.join("app/page.tsx"), r#"import { Hero } from "@safe/Hero";
import { secretLabel } from "@outside/Secret";
export default function Page() { return <main data-secret={typeof secretLabel}><Hero label="outside path alias" /></main>; }
"#).expect("route source");
    fs::write(root.join("components/Hero.tsx"), "export function Hero(props: { label: string }) { return <section>{props.label}</section>; }\n").expect("component source");
    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "../outside/Secret.ts")
    );
    assert!(
        dependencies
            .iter()
            .any(|dependency| dependency["specifier"] == "@outside/Secret"
                && dependency["kind"] == "source-alias-adapter-boundary"
                && dependency["resolver_source"] == "source-alias-boundary"
                && dependency["node_modules_required"] == false)
    );
}

#[test]
fn source_build_engine_keeps_path_aliases_with_outside_base_url_at_adapter_boundary_without_node_modules()
 {
    let project = tempfile::tempdir().expect("temp project");
    let workspace = project.path();
    let root = workspace.join("www");
    let outside = workspace.join("outside");
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(&outside).expect("outside dir");
    fs::write(
        outside.join("Secret.ts"),
        "export function secretLabel() { return 'must not be linked from outside baseUrl'; }\n",
    )
    .expect("outside source");
    let outside_base_url = outside.to_string_lossy().replace('\\', "\\\\");
    fs::write(
        root.join("tsconfig.json"),
        format!(
            r###"{{
  "compilerOptions": {{
    "baseUrl": "{outside_base_url}",
    "paths": {{
      "@outside/*": ["*"]
    }}
  }}
}}
"###
        ),
    )
    .expect("tsconfig json");
    fs::write(root.join("app/page.tsx"), r#"import { secretLabel } from "@outside/Secret";
export default function Page() { return <main data-secret={typeof secretLabel}>Outside baseUrl path alias</main>; }
"#).expect("route source");
    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(&root);
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "../outside/Secret.ts")
    );
    assert!(
        dependencies
            .iter()
            .any(|dependency| dependency["specifier"] == "@outside/Secret"
                && dependency["kind"] == "source-alias-adapter-boundary"
                && dependency["resolver_source"] == "source-alias-boundary"
                && dependency["node_modules_required"] == false)
    );
}

#[test]
fn source_build_engine_keeps_project_root_node_modules_aliases_at_adapter_boundary_without_node_modules()
 {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("node_modules")).expect("node_modules dir");
    fs::write(root.join("app/page.tsx"), r#"import { Hero } from "@/components/Hero";
import trap from "@/node_modules/trap";
export default function Page() { return <main data-trap={typeof trap}><Hero label="project root alias" /></main>; }
"#).expect("route source");
    fs::write(root.join("components/Hero.tsx"), "export function Hero(props: { label: string }) { return <section>{props.label}</section>; }\n").expect("component source");
    fs::write(
        root.join("node_modules/trap.ts"),
        "export default 'trap';\n",
    )
    .expect("node_modules trap");
    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "node_modules/trap.ts")
    );
    assert!(dependencies.iter().any(
        |dependency| dependency["specifier"] == "@/node_modules/trap"
            && dependency["kind"] == "project-root-alias-adapter-boundary"
            && dependency["resolver_source"] == "project-root-alias-boundary"
            && dependency["node_modules_required"] == false
    ));
}

#[test]
fn source_build_engine_keeps_project_root_alias_parent_segments_at_adapter_boundary_without_node_modules()
 {
    let project = tempfile::tempdir().expect("temp project");
    let workspace = project.path();
    let root = workspace.join("www");
    let outside = workspace.join("outside");
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(&outside).expect("outside dir");
    fs::write(
        outside.join("Secret.ts"),
        "export default 'must not be linked through project-root alias';\n",
    )
    .expect("outside source");
    fs::write(
        root.join("app/page.tsx"),
        r#"import secret from "@/../outside/Secret";
export default function Page() { return <main data-secret={typeof secret}>Project-root alias boundary</main>; }
"#,
    )
    .expect("route source");

    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(&root);
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "../outside/Secret.ts")
    );
    assert!(dependencies.iter().any(
        |dependency| dependency["specifier"] == "@/../outside/Secret"
            && dependency["kind"] == "project-root-alias-adapter-boundary"
            && dependency["resolver_source"] == "project-root-alias-boundary"
            && dependency["resolver_detail"] == "project-root-alias-outside-project-boundary"
            && dependency["node_modules_required"] == false
    ));
}

#[test]
fn source_build_engine_keeps_relative_node_modules_imports_at_adapter_boundary_without_node_modules()
 {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("node_modules")).expect("node_modules dir");
    fs::write(
        root.join("app/page.tsx"),
        r#"import trap from "../node_modules/trap";
export default function Page() { return <main data-trap={typeof trap}>Relative boundary</main>; }
"#,
    )
    .expect("route source");
    fs::write(
        root.join("node_modules/trap.ts"),
        "export default 'trap';\n",
    )
    .expect("node_modules trap");
    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "node_modules/trap.ts")
    );
    assert!(dependencies.iter().any(|dependency| {
        dependency["specifier"] == "../node_modules/trap"
            && dependency["kind"] == "local-node-modules-adapter-boundary"
            && dependency["resolver_source"] == "local-node-modules-boundary"
            && dependency["node_modules_required"] == false
    }));
}

#[test]
fn source_build_engine_keeps_outside_project_base_url_at_adapter_boundary_without_node_modules() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::write(
        root.join("tsconfig.json"),
        r###"{
  "compilerOptions": {
    "baseUrl": ".."
  }
}
"###,
    )
    .expect("tsconfig json");
    fs::write(
        root.join("app/page.tsx"),
        r#"import { secretLabel } from "outside/Secret";
export default function Page() { return <main data-secret={typeof secretLabel}>Outside baseUrl</main>; }
"#,
    )
    .expect("route source");
    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "../outside/Secret.ts")
    );
    assert!(
        dependencies
            .iter()
            .any(|dependency| dependency["specifier"] == "outside/Secret"
                && dependency["kind"] == "base-url-adapter-boundary"
                && dependency["resolver_source"] == "base-url-boundary"
                && dependency["node_modules_required"] == false)
    );
}

#[test]
fn source_build_engine_resolves_package_self_reference_exports_without_node_modules() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("src/features")).expect("feature dir");
    fs::create_dir_all(root.join("node_modules/not-source")).expect("blocked dir");
    fs::write(
        root.join("package.json"),
        r###"{
  "name": "dx-source-resolver-fixture",
  "exports": {
    ".": "./src/public.ts",
    "./feature/*": "./src/features/*.tsx",
    "./blocked/*": "./node_modules/not-source/*"
  }
}
"###,
    )
    .expect("package json");
    fs::write(root.join("app/page.tsx"), r#"import { packageLabel } from "dx-source-resolver-fixture";
import { Hero } from "dx-source-resolver-fixture/feature/Hero";
import blockedWidget from "dx-source-resolver-fixture/blocked/Widget";
export default function Page() { return <main data-blocked={typeof blockedWidget}><Hero label={packageLabel()} /></main>; }
"#).expect("route source");
    fs::write(
        root.join("src/public.ts"),
        "export function packageLabel() { return 'package exports root'; }\n",
    )
    .expect("package root source");
    fs::write(root.join("src/features/Hero.tsx"), "export function Hero(props: { label: string }) { return <section>{props.label}</section>; }\n").expect("feature source");
    fs::write(
        root.join("node_modules/not-source/Widget.ts"),
        "export default 'blocked';\n",
    )
    .expect("blocked source");
    let (_chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(
        dependencies
            .iter()
            .any(|dependency| dependency["resolved_path"] == "src/public.ts"
                && dependency["resolver_source"] == "package-self-reference")
    );
    assert!(
        dependencies
            .iter()
            .any(
                |dependency| dependency["resolved_path"] == "src/features/Hero.tsx"
                    && dependency["resolver_source"] == "package-self-reference"
            )
    );
    assert!(dependencies.iter().any(|dependency| dependency["kind"]
        == "package-export-adapter-boundary"
        && dependency["resolver_source"] == "package-export-boundary"));
}

#[test]
fn source_build_engine_resolves_package_self_reference_exports_parent_segments_inside_project() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("src")).expect("src dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::write(
        root.join("package.json"),
        r###"{
  "name": "dx-source-resolver-fixture",
  "exports": {
    "./shared/*": "./src/../components/*.tsx"
  }
}
"###,
    )
    .expect("package json");
    fs::write(root.join("app/page.tsx"), r#"import { Hero } from "dx-source-resolver-fixture/shared/Hero";
export default function Page() { return <main><Hero label="package export normalized parent segment" /></main>; }
"#).expect("route source");
    fs::write(root.join("components/Hero.tsx"), "export function Hero(props: { label: string }) { return <section>{props.label}</section>; }\n").expect("component source");
    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(chunks.iter().any(|chunk| {
        chunk["source_path"] == "components/Hero.tsx" && chunk["node_modules_required"] == false
    }));
    assert!(dependencies.iter().any(|dependency| dependency["specifier"]
        == "dx-source-resolver-fixture/shared/Hero"
        && dependency["resolved_path"] == "components/Hero.tsx"
        && dependency["kind"] == "tsx"
        && dependency["resolver_source"] == "package-self-reference"
        && dependency["node_modules_required"] == false));
    assert!(dependencies.iter().all(|dependency| dependency["specifier"]
        != "dx-source-resolver-fixture/shared/Hero"
        || dependency["kind"] != "package-export-adapter-boundary"));
}

#[test]
fn source_build_engine_keeps_mixed_package_self_reference_exports_at_adapter_boundary_without_node_modules()
 {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("src/features")).expect("feature dir");
    fs::create_dir_all(root.join("node_modules/not-source")).expect("blocked dir");
    fs::write(
        root.join("package.json"),
        r###"{
  "name": "dx-source-resolver-fixture",
  "exports": {
    "./mixed/*": ["./src/features/*.tsx", "./node_modules/not-source/*"]
  }
}
"###,
    )
    .expect("package json");
    fs::write(root.join("app/page.tsx"), r#"import { Hero } from "dx-source-resolver-fixture/mixed/Hero";
export default function Page() { return <main data-hero={typeof Hero}>Mixed package export boundary</main>; }
"#).expect("route source");
    fs::write(root.join("src/features/Hero.tsx"), "export function Hero() { return <section>safe source must not link while export fallback is external</section>; }\n").expect("feature source");
    fs::write(
        root.join("node_modules/not-source/Hero.ts"),
        "export const Hero = 'blocked';\n",
    )
    .expect("blocked source");
    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "src/features/Hero.tsx"
                && chunk["source_path"] != "node_modules/not-source/Hero.ts")
    );
    assert!(dependencies.iter().any(|dependency| dependency["specifier"]
        == "dx-source-resolver-fixture/mixed/Hero"
        && dependency["kind"] == "package-export-adapter-boundary"
        && dependency["resolver_source"] == "package-export-boundary"
        && dependency["resolver_detail"] == "package-export-target-node-modules-boundary"
        && dependency["node_modules_required"] == false));
}

#[test]
fn source_build_engine_marks_escaping_package_self_reference_exports_at_outside_boundary() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::write(
        root.join("package.json"),
        r###"{
  "name": "dx-source-resolver-fixture",
  "exports": {
    "./private/*": "./../outside/*.tsx"
  }
}
"###,
    )
    .expect("package json");
    fs::write(root.join("app/page.tsx"), r#"import { Secret } from "dx-source-resolver-fixture/private/Secret";
export default function Page() { return <main data-secret={typeof Secret}>Escaping export boundary</main>; }
"#).expect("route source");
    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");
    assert!(!report.manifest.node_modules_required);
    assert!(!report.receipt.node_modules_required);
    assert!(
        report
            .receipt
            .summary
            .resolver_adapter_boundary_sources
            .iter()
            .any(|source| source.resolver_source == "package-export-boundary"
                && source.dependencies == 1)
    );
    assert!(
        report
            .receipt
            .summary
            .resolver_adapter_boundary_details
            .iter()
            .any(|detail| detail.resolver_source == "package-export-boundary"
                && detail.resolver_detail == "package-export-target-outside-package-boundary"
                && detail.dependencies == 1)
    );
    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&report.manifest_path).expect("manifest json"))
            .expect("parse manifest");
    let chunks = manifest["route_outputs"][0]["source_module_chunks"]
        .as_array()
        .expect("source module chunks");
    let dependencies = chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "app/page.tsx")
        .expect("page chunk")["dependencies"]
        .as_array()
        .expect("page dependencies");
    assert!(dependencies.iter().any(|dependency| dependency["specifier"]
        == "dx-source-resolver-fixture/private/Secret"
        && dependency["kind"] == "package-export-adapter-boundary"
        && dependency["resolver_source"] == "package-export-boundary"
        && dependency["resolver_detail"] == "package-export-target-outside-package-boundary"
        && dependency["node_modules_required"] == false));
    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let graph_nodes = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes");
    let graph_edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(
        graph_nodes
            .iter()
            .any(|node| node["kind"] == "adapter-boundary-import"
                && node["specifier"] == "dx-source-resolver-fixture/private/Secret"
                && node["resolver_source"] == "package-export-boundary"
                && node["resolver_detail"] == "package-export-target-outside-package-boundary"
                && node["node_modules_required"] == false)
    );
    assert!(
        graph_edges
            .iter()
            .any(|edge| edge["kind"] == "adapter-boundary"
                && edge["specifier"] == "dx-source-resolver-fixture/private/Secret"
                && edge["resolver_source"] == "package-export-boundary"
                && edge["resolver_detail"] == "package-export-target-outside-package-boundary"
                && edge["node_modules_required"] == false)
    );
}

#[test]
fn source_build_engine_keeps_unexported_package_self_reference_subpaths_at_exports_boundary_without_node_modules()
 {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("src/features")).expect("feature dir");
    fs::create_dir_all(root.join("internal")).expect("internal dir");
    fs::write(
        root.join("package.json"),
        r###"{
  "name": "dx-source-resolver-fixture",
  "exports": {
    ".": "./src/public.ts",
    "./feature/*": "./src/features/*.tsx"
  }
}
"###,
    )
    .expect("package json");
    fs::write(root.join("app/page.tsx"), r#"import { packageLabel } from "dx-source-resolver-fixture";
import { Hero } from "dx-source-resolver-fixture/feature/Hero";
import { secretLabel } from "dx-source-resolver-fixture/internal/Secret";
export default function Page() { return <main data-secret={typeof secretLabel}><Hero label={packageLabel()} /></main>; }
"#).expect("route source");
    fs::write(
        root.join("src/public.ts"),
        "export function packageLabel() { return 'package exports root'; }\n",
    )
    .expect("package root source");
    fs::write(root.join("src/features/Hero.tsx"), "export function Hero(props: { label: string }) { return <section>{props.label}</section>; }\n").expect("feature source");
    fs::write(root.join("internal/Secret.ts"), "export function secretLabel() { return 'must not be linked through package self-reference exports'; }\n").expect("internal source");
    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "internal/Secret.ts")
    );
    assert!(dependencies.iter().any(|dependency| dependency["specifier"]
        == "dx-source-resolver-fixture/internal/Secret"
        && dependency["kind"] == "package-export-adapter-boundary"
        && dependency["resolver_source"] == "package-export-boundary"));
}

#[test]
fn source_build_engine_keeps_invalid_package_names_out_of_self_reference_namespace() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("src")).expect("src dir");
    fs::create_dir_all(root.join("internal")).expect("internal dir");
    fs::write(
        root.join("package.json"),
        r###"{
  "name": "plain/pkg",
  "exports": {
    ".": "./src/public.ts"
  }
}
"###,
    )
    .expect("package json");
    fs::write(
        root.join("app/page.tsx"),
        r#"import { packageLabel } from "plain/pkg";
import { secretLabel } from "plain/pkg/internal/Secret";

export default function Page() {
  return <main data-package={typeof packageLabel} data-secret={typeof secretLabel}>Invalid package name boundary</main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("src/public.ts"),
        "export function packageLabel() { return 'must not be linked from invalid package name'; }\n",
    )
    .expect("package root source");
    fs::write(
        root.join("internal/Secret.ts"),
        "export function secretLabel() { return 'must not be linked through invalid package name'; }\n",
    )
    .expect("internal source");

    let (chunks, dependencies) = source_resolver_page_chunks_and_dependencies(root);
    assert!(chunks.iter().all(|chunk| {
        chunk["source_path"] != "src/public.ts" && chunk["source_path"] != "internal/Secret.ts"
    }));
    for specifier in ["plain/pkg", "plain/pkg/internal/Secret"] {
        assert!(
            dependencies
                .iter()
                .any(|dependency| dependency["specifier"] == specifier
                    && dependency["resolved_path"].is_null()
                    && dependency["kind"] == "external-package-adapter-boundary"
                    && dependency["resolver_source"] == "external-package-boundary"
                    && dependency["node_modules_required"] == false),
            "dependencies: {dependencies:#?}"
        );
    }
    assert!(dependencies.iter().all(|dependency| {
        dependency["specifier"] != "plain/pkg"
            || dependency["resolver_source"] != "package-self-reference"
    }));
}

#[test]
fn source_build_engine_keeps_sibling_prefix_base_url_at_adapter_boundary_without_node_modules() {
    let project = tempfile::tempdir().expect("temp project");
    let workspace = project.path();
    let root = workspace.join("www");
    let sibling = workspace.join("www-sibling");
    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(sibling.join("outside")).expect("sibling outside dir");
    fs::write(
        sibling.join("outside/Secret.ts"),
        "export const secretLabel = 'must not be linked from sibling baseUrl';\n",
    )
    .expect("sibling source");
    let sibling_base_url = sibling.to_string_lossy().replace('\\', "\\\\");
    fs::write(
        root.join("tsconfig.json"),
        format!(
            r###"{{
  "compilerOptions": {{
    "baseUrl": "{sibling_base_url}"
  }}
}}
"###
        ),
    )
    .expect("tsconfig json");
    fs::write(
        root.join("app/page.tsx"),
        r#"import { secretLabel } from "outside/Secret";
export default function Page() { return <main data-secret={typeof secretLabel}>Sibling baseUrl</main>; }
"#,
    )
    .expect("route source");
    let (_chunks, dependencies) = source_resolver_page_chunks_and_dependencies(&root);
    assert!(
        dependencies
            .iter()
            .any(|dependency| dependency["specifier"] == "outside/Secret"
                && dependency["kind"] == "base-url-adapter-boundary"
                && dependency["resolver_source"] == "base-url-boundary"
                && dependency["node_modules_required"] == false)
    );
}

fn source_resolver_page_chunks_and_dependencies(
    root: &std::path::Path,
) -> (Vec<Value>, Vec<Value>) {
    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");
    assert!(!report.manifest.node_modules_required);
    assert!(!report.receipt.node_modules_required);
    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&report.manifest_path).expect("manifest json"))
            .expect("parse manifest");
    let chunks = manifest["route_outputs"][0]["source_module_chunks"]
        .as_array()
        .expect("source module chunks")
        .clone();
    let dependencies = chunks
        .iter()
        .find(|chunk| {
            chunk["source_path"] == "app/page.tsx" || chunk["source_path"] == "src/app/page.tsx"
        })
        .expect("page chunk")["dependencies"]
        .as_array()
        .expect("page dependencies")
        .clone();
    (chunks, dependencies)
}
