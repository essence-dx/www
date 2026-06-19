use std::fs;

use dx_www::build::{SourceBuildEngine, SourceBuildOptions};
use serde_json::Value;

#[test]
fn source_build_engine_resolves_jsconfig_extends_paths_without_node_modules() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("config")).expect("config dir");
    fs::create_dir_all(root.join("shared/base")).expect("base shared dir");
    fs::create_dir_all(root.join("shared/overrides")).expect("override shared dir");
    fs::create_dir_all(root.join("shared/commented")).expect("commented shared dir");
    fs::create_dir_all(root.join("shared")).expect("shared dir");
    fs::create_dir_all(root.join("src")).expect("src dir");
    fs::create_dir_all(root.join("node_modules/shared")).expect("node modules trap dir");

    fs::write(
        root.join("jsconfig.json"),
        r###"{
  // JSONC comments are valid in source-owned TS configs.
  "extends": "./config/jsconfig.base.json",
  "compilerOptions": {
    "paths": {
      "@commented/*": ["shared/commented/*"],
      "@missing-override/*": ["missing/overrides/*"],
      "@override/*": ["shared/overrides/*"],
    }
  },
}
"###,
    )
    .expect("jsconfig json");
    fs::write(
        root.join("config/jsconfig.base.json"),
        r###"{
  "compilerOptions": {
    "baseUrl": "..",
    "paths": {
      "@missing-override/*": ["shared/base/*"],
      "@shared/*": ["shared/*"],
      "@override/*": ["shared/base/*"],
      "~/*": ["src/*"]
    }
  }
}
"###,
    )
    .expect("base jsconfig json");
    fs::write(
        root.join("app/page.tsx"),
        r#"import { Hero } from "@shared/Hero";
import { MissingPanel } from "@missing-override/Panel";
import { Panel } from "@override/Panel";
import { CommentedPanel } from "@commented/Panel";
import { statusLabel } from "~/status";
import trap from "node_modules/shared/trap";

export default function Page() {
  return <main data-missing={typeof MissingPanel} data-trap={typeof trap}><Hero label={statusLabel()} /><Panel /><CommentedPanel /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("shared/Hero.tsx"),
        r#"export function Hero(props: { label: string }) {
  return <section data-hero="jsconfig-extends">{props.label}</section>;
}
"#,
    )
    .expect("hero source");
    fs::write(
        root.join("shared/base/Panel.tsx"),
        r#"export function Panel() {
  return <aside data-panel="base">Base</aside>;
}
"#,
    )
    .expect("base panel source");
    fs::write(
        root.join("shared/overrides/Panel.tsx"),
        r#"export function Panel() {
  return <aside data-panel="override">Override</aside>;
}
"#,
    )
    .expect("override panel source");
    fs::write(
        root.join("shared/commented/Panel.tsx"),
        r#"export function CommentedPanel() {
  return <aside data-panel="commented">Commented</aside>;
}
"#,
    )
    .expect("commented panel source");
    fs::write(
        root.join("src/status.ts"),
        r#"export function statusLabel() {
  return "jsconfig extends";
}
"#,
    )
    .expect("status source");
    fs::write(
        root.join("node_modules/shared/trap.ts"),
        "export default 'trap';\n",
    )
    .expect("node modules trap");

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
    assert_eq!(source_module_chunks.len(), 5);
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "shared/Hero.tsx" && chunk["node_modules_required"] == false
    }));
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "shared/overrides/Panel.tsx"
            && chunk["node_modules_required"] == false
    }));
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "shared/commented/Panel.tsx"
            && chunk["node_modules_required"] == false
    }));
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "src/status.ts" && chunk["node_modules_required"] == false
    }));
    assert!(source_module_chunks.iter().all(|chunk| {
        chunk["source_path"] != "shared/base/Panel.tsx"
            && chunk["source_path"] != "node_modules/shared/trap.ts"
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
                && dependency["resolved_path"] == "shared/Hero.tsx"
                && dependency["kind"] == "tsx"
                && dependency["resolver_source"] == "jsconfig-path"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "@missing-override/Panel"
                && dependency["resolved_path"].is_null()
                && dependency["kind"] == "unresolved-source-alias"
                && dependency["resolver_source"] == "source-alias-unresolved"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "@override/Panel"
                && dependency["resolved_path"] == "shared/overrides/Panel.tsx"
                && dependency["kind"] == "tsx"
                && dependency["resolver_source"] == "jsconfig-path"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "@commented/Panel"
                && dependency["resolved_path"] == "shared/commented/Panel.tsx"
                && dependency["kind"] == "tsx"
                && dependency["resolver_source"] == "jsconfig-path"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "~/status"
                && dependency["resolved_path"] == "src/status.ts"
                && dependency["kind"] == "ts"
                && dependency["resolver_source"] == "jsconfig-path"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "node_modules/shared/trap"
                && dependency["resolved_path"].is_null()
                && dependency["kind"] == "base-url-node-modules-adapter-boundary"
                && dependency["resolver_source"] == "base-url-node-modules-boundary"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
}

#[test]
fn source_build_engine_resolves_jsconfig_array_extends_with_last_local_precedence() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("config")).expect("config dir");
    fs::create_dir_all(root.join("shared/array-a")).expect("array a dir");
    fs::create_dir_all(root.join("shared/array-b")).expect("array b dir");
    fs::create_dir_all(root.join("src")).expect("src dir");

    fs::write(
        root.join("jsconfig.json"),
        r###"{
  "extends": [
    "./config/jsconfig.base-a.json",
    "./config/jsconfig.base-b.json",
    "next/tsconfig"
  ],
  "compilerOptions": {
    "paths": {
      "@root/*": ["src/*"]
    }
  }
}
"###,
    )
    .expect("jsconfig json");
    fs::write(
        root.join("config/jsconfig.base-a.json"),
        r###"{
  "compilerOptions": {
    "baseUrl": "..",
    "paths": {
      "@array-shared/*": ["shared/array-a/*"],
      "@from-a/*": ["shared/array-a/*"]
    }
  }
}
"###,
    )
    .expect("base a jsconfig json");
    fs::write(
        root.join("config/jsconfig.base-b.json"),
        r###"{
  "compilerOptions": {
    "baseUrl": "..",
    "paths": {
      "@array-shared/*": ["shared/array-b/*"],
      "@from-b/*": ["shared/array-b/*"]
    }
  }
}
"###,
    )
    .expect("base b jsconfig json");
    fs::write(
        root.join("app/page.tsx"),
        r#"import { Panel } from "@array-shared/Panel";
import { panelA } from "@from-a/Panel";
import { panelB } from "@from-b/Panel";
import { statusLabel } from "@root/status";

export default function Page() {
  return <main data-a={panelA()} data-b={panelB()}><Panel label={statusLabel()} /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("shared/array-a/Panel.tsx"),
        r#"export function panelA() {
  return "array-a";
}
export function Panel(props: { label: string }) {
  return <section data-panel="array-a">{props.label}</section>;
}
"#,
    )
    .expect("array a panel source");
    fs::write(
        root.join("shared/array-b/Panel.tsx"),
        r#"export function panelB() {
  return "array-b";
}
export function Panel(props: { label: string }) {
  return <section data-panel="array-b">{props.label}</section>;
}
"#,
    )
    .expect("array b panel source");
    fs::write(
        root.join("src/status.ts"),
        r#"export function statusLabel() {
  return "jsconfig array extends";
}
"#,
    )
    .expect("status source");

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
        chunk["source_path"] == "shared/array-b/Panel.tsx"
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
            dependency["specifier"] == "@array-shared/Panel"
                && dependency["resolved_path"] == "shared/array-b/Panel.tsx"
                && dependency["kind"] == "tsx"
                && dependency["resolver_source"] == "jsconfig-path"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(dependencies.iter().all(|dependency| {
        dependency["specifier"] != "@array-shared/Panel"
            || dependency["resolved_path"] != "shared/array-a/Panel.tsx"
    }));
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "@from-a/Panel"
                && dependency["resolved_path"] == "shared/array-a/Panel.tsx"
                && dependency["kind"] == "tsx"
                && dependency["resolver_source"] == "jsconfig-path"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "@from-b/Panel"
                && dependency["resolved_path"] == "shared/array-b/Panel.tsx"
                && dependency["kind"] == "tsx"
                && dependency["resolver_source"] == "jsconfig-path"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "@root/status"
                && dependency["resolved_path"] == "src/status.ts"
                && dependency["kind"] == "ts"
                && dependency["resolver_source"] == "jsconfig-path"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
}

#[test]
fn source_build_engine_ignores_external_jsconfig_extends_without_reading_outside_project() {
    let project = tempfile::tempdir().expect("temp project");
    let workspace = project.path();
    let root = workspace.join("www");
    let outside = workspace.join("shared-config");

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("local")).expect("local dir");
    fs::create_dir_all(outside.join("external")).expect("outside dir");

    fs::write(
        outside.join("jsconfig.base.json"),
        r###"{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@external/*": ["external/*"]
    }
  }
}
"###,
    )
    .expect("outside config");
    fs::write(
        outside.join("external/Secret.ts"),
        "export const secretLabel = 'must not be linked from external jsconfig extends';\n",
    )
    .expect("outside source");
    fs::write(
        root.join("jsconfig.json"),
        r###"{
  "extends": "../shared-config/jsconfig.base.json",
  "compilerOptions": {
    "paths": {
      "@local/*": ["local/*"]
    }
  }
}
"###,
    )
    .expect("jsconfig json");
    fs::write(
        root.join("app/page.tsx"),
        r#"import { localLabel } from "@local/status";
import { secretLabel } from "@external/Secret";

export default function Page() {
  return <main data-secret={typeof secretLabel}>{localLabel()}</main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("local/status.ts"),
        "export function localLabel() { return 'local config still loads'; }\n",
    )
    .expect("local source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(&root)
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
        chunk["source_path"] == "local/status.ts" && chunk["node_modules_required"] == false
    }));
    assert!(
        source_module_chunks
            .iter()
            .all(|chunk| chunk["source_path"] != "../shared-config/external/Secret.ts")
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
            dependency["specifier"] == "@local/status"
                && dependency["resolved_path"] == "local/status.ts"
                && dependency["kind"] == "ts"
                && dependency["resolver_source"] == "jsconfig-path"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "@external/Secret"
                && dependency["resolved_path"].is_null()
                && dependency["kind"] == "external-package-adapter-boundary"
                && dependency["resolver_source"] == "external-package-boundary"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
}
