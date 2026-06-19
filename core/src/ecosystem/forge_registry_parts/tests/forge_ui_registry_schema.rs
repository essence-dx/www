#[test]
fn forge_ui_registry_json_preserves_current_registry_surfaces() {
    let registry = parse_forge_ui_registry_json(
        r##"{
              "$schema": "https://dx.dev/schema/forge-ui-registry.json",
              "name": "dx",
              "homepage": "https://dx.local",
              "items": [
                {
                  "name": "button",
                  "type": "registry:ui",
                  "title": "Button",
                  "author": "dx",
                  "description": "A button primitive.",
                  "dependencies": ["@radix-ui/react-slot"],
                  "devDependencies": ["tailwindcss"],
                  "registryDependencies": ["utils"],
                  "files": [
                    {
                      "path": "button.tsx",
                      "type": "registry:ui"
                    }
                  ],
                  "tailwind": {
                    "config": {
                      "content": ["app/**/*.{ts,tsx}"],
                      "plugins": ["tailwindcss-animate"]
                    }
                  },
                  "cssVars": {
                    "theme": {
                      "radius": "0.5rem"
                    },
                    "light": {
                      "background": "0 0% 100%"
                    },
                    "dark": {
                      "background": "0 0% 0%"
                    }
                  },
                  "css": {
                    ".dx-button": {
                      "display": "inline-flex"
                    }
                  },
                  "envVars": {
                    "NEXT_PUBLIC_DX": "true"
                  },
                  "meta": {
                    "figma": true
                  },
                  "docs": "Install with dx forge.",
                  "categories": ["ui"]
                },
                {
                  "name": "inter",
                  "type": "registry:font",
                  "font": {
                    "family": "Inter",
                    "provider": "google",
                    "import": "Inter",
                    "variable": "--font-inter",
                    "weight": ["400", "700"],
                    "subsets": ["latin"],
                    "selector": "html",
                    "dependency": "next/font/google"
                  }
                },
                {
                  "name": "base",
                  "type": "registry:base",
                  "config": {
                    "style": "new-york",
                    "tsx": true,
                    "tailwind": {
                      "css": "app/globals.css",
                      "baseColor": "neutral"
                    },
                    "aliases": {
                      "components": "@/components",
                      "utils": "@/lib/utils"
                    }
                  }
                }
              ]
            }"##,
    )
    .expect("parse Forge UI registry");

    assert_eq!(
        registry.schema.as_deref(),
        Some("https://dx.dev/schema/forge-ui-registry.json")
    );
    assert_eq!(registry.name.as_deref(), Some("dx"));
    assert_eq!(registry.homepage.as_deref(), Some("https://dx.local"));
    assert_eq!(registry.items.len(), 3);

    let button = registry
        .items
        .iter()
        .find(|item| item.name == "button")
        .expect("button item");
    assert_eq!(button.item_type, DxForgeUiRegistryItemType::Ui);
    assert_eq!(button.dependencies, vec!["@radix-ui/react-slot"]);
    assert_eq!(button.dev_dependencies, vec!["tailwindcss"]);
    assert_eq!(button.registry_dependencies, vec!["utils"]);
    assert_eq!(button.files[0].path, "button.tsx");
    assert_eq!(button.files[0].file_type, DxForgeUiRegistryItemType::Ui);
    assert!(button.tailwind.is_some());
    assert!(button.css_vars.is_some());
    assert!(!button.css.is_empty());
    assert_eq!(
        button.env_vars.get("NEXT_PUBLIC_DX"),
        Some(&"true".to_string())
    );
    assert_eq!(button.docs.as_deref(), Some("Install with dx forge."));
    assert_eq!(button.categories, vec!["ui"]);

    let font = registry
        .items
        .iter()
        .find(|item| item.name == "inter")
        .expect("font item");
    assert_eq!(font.item_type, DxForgeUiRegistryItemType::Font);
    assert_eq!(font.font.as_ref().expect("font metadata").family, "Inter");

    let base = registry
        .items
        .iter()
        .find(|item| item.name == "base")
        .expect("base item");
    assert_eq!(base.item_type, DxForgeUiRegistryItemType::Base);
    assert!(base.config.is_some());

    let report = validate_forge_ui_registry_catalog(&registry).expect("validate registry");
    assert_eq!(report.item_count, 3);
    assert_eq!(report.file_count, 1);
    assert_eq!(report.include_count, 0);
}

#[test]
fn forge_ui_registry_base_item_preserves_official_top_level_metadata() {
    let registry = parse_forge_ui_registry_json(
        r##"{
              "name": "dx",
              "items": [
                {
                  "name": "base",
                  "type": "registry:base",
                  "style": "new-york",
                  "iconLibrary": "lucide",
                  "baseColor": "neutral",
                  "theme": "zinc"
                }
              ]
            }"##,
    )
    .expect("parse registry");

    let base = registry.items.first().expect("base item");
    assert_eq!(base.item_type, DxForgeUiRegistryItemType::Base);
    assert_eq!(base.style.as_deref(), Some("new-york"));
    assert_eq!(base.icon_library.as_deref(), Some("lucide"));
    assert_eq!(base.base_color.as_deref(), Some("neutral"));
    assert_eq!(base.theme.as_deref(), Some("zinc"));

    let report = validate_forge_ui_registry_catalog(&registry).expect("validate registry");
    assert_eq!(report.item_count, 1);

    let docs = describe_forge_ui_registry_item(&registry, "base").expect("describe base item");
    assert!(docs.config_present);
    assert_eq!(docs.base_style.as_deref(), Some("new-york"));
    assert_eq!(docs.base_icon_library.as_deref(), Some("lucide"));
    assert_eq!(docs.base_color.as_deref(), Some("neutral"));
    assert_eq!(docs.base_theme.as_deref(), Some("zinc"));

    let roundtrip = serde_json::to_string(&registry).expect("serialize registry");
    assert!(roundtrip.contains("\"iconLibrary\":\"lucide\""));
    assert!(roundtrip.contains("\"baseColor\":\"neutral\""));
}

#[test]
fn forge_ui_registry_rejects_base_metadata_on_non_base_items() {
    let registry = parse_forge_ui_registry_json(
        r##"{
              "name": "dx",
              "items": [
                {
                  "name": "button",
                  "type": "registry:ui",
                  "style": "new-york"
                }
              ]
            }"##,
    )
    .expect("parse registry");

    let error = validate_forge_ui_registry_catalog(&registry)
        .expect_err("top-level base metadata belongs to registry:base only")
        .to_string();

    assert!(error.contains("declares base metadata"));
    assert!(error.contains("registry:base"));
}

#[test]
fn forge_ui_registry_validation_rejects_package_manager_dependency_commands() {
    let registry = parse_forge_ui_registry_json(
        r##"{
              "name": "dx",
              "items": [
                {
                  "name": "button",
                  "type": "registry:ui",
                  "dependencies": ["npm install @radix-ui/react-slot"]
                }
              ]
            }"##,
    )
    .expect("parse registry");

    let error = validate_forge_ui_registry_catalog(&registry)
        .expect_err("package-manager commands do not belong in registry dependency metadata")
        .to_string();

    assert!(error.contains("unsafe dependencies entry"));
    assert!(error.contains("button"));
    assert!(error.contains("npm install @radix-ui/react-slot"));
}

#[test]
fn forge_ui_registry_validation_rejects_unsafe_tailwind_plugin_entries() {
    let registry = parse_forge_ui_registry_json(
        r##"{
              "name": "dx",
              "items": [
                {
                  "name": "button",
                  "type": "registry:ui",
                  "tailwind": {
                    "config": {
                      "plugins": [
                        "tailwindcss-animate",
                        "npm install tailwindcss-animate"
                      ]
                    }
                  }
                }
              ]
            }"##,
    )
    .expect("parse registry");

    let error = validate_forge_ui_registry_catalog(&registry)
        .expect_err("package-manager commands do not belong in Tailwind plugin metadata")
        .to_string();

    assert!(error.contains("unsafe tailwind.config.plugins entry"));
    assert!(error.contains("button"));
    assert!(error.contains("npm install tailwindcss-animate"));
}

#[test]
fn forge_ui_registry_validation_rejects_unsafe_env_var_names() {
    let registry = parse_forge_ui_registry_json(
        r##"{
              "name": "dx",
              "items": [
                {
                  "name": "button",
                  "type": "registry:ui",
                  "envVars": {
                    "DX_BUTTON_MODE": "reviewed",
                    "npm install TOKEN": "bad"
                  }
                }
              ]
            }"##,
    )
    .expect("parse registry");

    let error = validate_forge_ui_registry_catalog(&registry)
        .expect_err("unsafe env var names must fail catalog validation");
    let error = format!("{error:#}");

    assert!(error.contains("unsafe envVars entry"), "{error}");
    assert!(error.contains("button"), "{error}");
    assert!(error.contains("npm install TOKEN"), "{error}");
}

#[test]
fn forge_ui_registry_plan_reports_materialization_and_bridge_decisions() {
    let registry = parse_forge_ui_registry_json(
        r##"{
              "name": "dx",
              "items": [
                {
                  "name": "utils",
                  "type": "registry:lib",
                  "files": [
                    {
                      "path": "utils.ts",
                      "type": "registry:lib",
                      "content": "export function cn(value: string) { return value }"
                    }
                  ]
                },
                {
                  "name": "button",
                  "type": "registry:ui",
                  "dependencies": ["@radix-ui/react-slot"],
                  "devDependencies": ["tailwindcss"],
                  "registryDependencies": ["utils"],
                  "files": [
                    {
                      "path": "button.tsx",
                      "type": "registry:ui",
                      "content": "export function Button() { return <button /> }"
                    }
                  ],
                  "cssVars": {
                    "theme": {
                      "radius": "0.5rem"
                    }
                  },
                  "css": {
                    ".dx-button": {
                      "display": "inline-flex"
                    }
                  },
                  "envVars": {
                    "DX_BUTTON_MODE": "reviewed"
                  }
                }
              ]
            }"##,
    )
    .expect("parse registry");

    let report = plan_forge_ui_registry_item(&registry, "button", "G:/workspace")
        .expect("plan registry item");

    assert!(report.passed);
    assert_eq!(report.score, 85);
    assert!(report.no_package_manager_execution);
    assert_eq!(report.file_count, 2);
    assert_eq!(report.write_file_count, 2);
    assert_eq!(report.inline_content_file_count, 2);
    assert_eq!(report.missing_inline_content_count, 0);
    assert_eq!(report.registry_dependency_order, vec!["utils", "button"]);
    assert_eq!(report.registry_dependency_edges[0].from, "button");
    assert_eq!(report.registry_dependency_edges[0].to, "utils");
    assert_eq!(report.files[0].item_name, "utils");
    assert_eq!(report.files[0].target_path, "lib/utils.ts");
    assert_eq!(report.files[1].item_name, "button");
    assert_eq!(report.files[1].target_path, "components/ui/button.tsx");
    assert_eq!(
        report.files[1].action,
        DxForgeUiRegistryPlanAction::Materialize
    );
    assert!(
        report
            .forbidden_commands
            .iter()
            .any(|command| command == "npm install")
    );
    assert!(report.decisions.iter().any(|decision| {
        decision.subject == "button:@radix-ui/react-slot"
            && decision.decision == DxForgeUiRegistryPlanDecisionKind::BridgeDependency
    }));
    assert!(report.decisions.iter().any(|decision| {
        decision.subject == "button -> utils"
            && decision.decision == DxForgeUiRegistryPlanDecisionKind::ResolveRegistryDependency
    }));
    assert!(report.decisions.iter().any(|decision| {
        decision.subject == "button:tailwindcss"
            && decision.decision == DxForgeUiRegistryPlanDecisionKind::IgnoreDevDependency
    }));
    assert!(report.decisions.iter().any(|decision| {
        decision.subject == "button:DX_BUTTON_MODE"
            && decision.decision == DxForgeUiRegistryPlanDecisionKind::RequireEnvironment
    }));
    assert!(report.decisions.iter().any(|decision| {
        decision.subject == "button"
            && decision.decision == DxForgeUiRegistryPlanDecisionKind::MergeStyle
    }));
}

#[test]
fn forge_ui_registry_plan_resolves_transitive_dependencies_in_write_order() {
    let registry = parse_forge_ui_registry_json(
        r#"{
              "items": [
                {
                  "name": "tokens",
                  "type": "registry:theme",
                  "files": [
                    {
                      "path": "tokens.css",
                      "type": "registry:theme",
                      "content": ":root { --radius: 8px; }"
                    }
                  ]
                },
                {
                  "name": "utils",
                  "type": "registry:lib",
                  "registryDependencies": ["tokens"],
                  "files": [
                    {
                      "path": "utils.ts",
                      "type": "registry:lib",
                      "content": "export const cn = String"
                    }
                  ]
                },
                {
                  "name": "button",
                  "type": "registry:ui",
                  "registryDependencies": ["utils"],
                  "files": [
                    {
                      "path": "button.tsx",
                      "type": "registry:ui",
                      "content": "export const Button = 'button'"
                    }
                  ]
                }
              ]
            }"#,
    )
    .expect("parse registry");

    let report = plan_forge_ui_registry_item(&registry, "button", ".").expect("plan graph");

    assert!(report.passed);
    assert_eq!(report.score, 100);
    assert_eq!(
        report.registry_dependency_order,
        vec!["tokens", "utils", "button"]
    );
    assert_eq!(report.registry_dependency_count, 2);
    assert_eq!(
        report
            .files
            .iter()
            .map(|file| file.item_name.as_str())
            .collect::<Vec<_>>(),
        vec!["tokens", "utils", "button"]
    );
    assert!(report.decisions.iter().any(|decision| {
        decision.subject == "button -> utils"
            && decision.decision == DxForgeUiRegistryPlanDecisionKind::ResolveRegistryDependency
    }));
    assert!(report.decisions.iter().any(|decision| {
        decision.subject == "utils -> tokens"
            && decision.decision == DxForgeUiRegistryPlanDecisionKind::ResolveRegistryDependency
    }));
}

#[test]
fn forge_ui_registry_plan_resolves_extends_before_child_item() {
    let registry = parse_forge_ui_registry_json(
        r#"{
              "items": [
                {
                  "name": "base-button",
                  "type": "registry:ui",
                  "files": [
                    {
                      "path": "base-button.tsx",
                      "type": "registry:ui",
                      "content": "export const BaseButton = 'base'"
                    }
                  ]
                },
                {
                  "name": "fancy-button",
                  "type": "registry:ui",
                  "extends": "base-button",
                  "files": [
                    {
                      "path": "fancy-button.tsx",
                      "type": "registry:ui",
                      "content": "export const FancyButton = 'fancy'"
                    }
                  ]
                }
              ]
            }"#,
    )
    .expect("parse registry");

    let validation =
        validate_forge_ui_registry_catalog(&registry).expect("validate extends relationship");
    assert_eq!(validation.registry_dependency_count, 1);

    let report =
        plan_forge_ui_registry_item(&registry, "fancy-button", ".").expect("plan extends graph");

    assert!(report.passed);
    assert_eq!(
        report.registry_dependency_order,
        vec!["base-button", "fancy-button"]
    );
    assert_eq!(report.registry_dependency_count, 1);
    assert_eq!(report.registry_dependency_edges[0].from, "fancy-button");
    assert_eq!(report.registry_dependency_edges[0].to, "base-button");
    assert_eq!(
        report
            .files
            .iter()
            .map(|file| file.item_name.as_str())
            .collect::<Vec<_>>(),
        vec!["base-button", "fancy-button"]
    );
    assert!(report.decisions.iter().any(|decision| {
        decision.subject == "fancy-button -> base-button"
            && decision.decision == DxForgeUiRegistryPlanDecisionKind::ResolveRegistryDependency
    }));
}

#[test]
fn forge_ui_registry_extends_is_validated_as_an_authored_graph_relationship() {
    for (json, expected) in [
        (
            r#"{
              "items": [
                {
                  "name": "button",
                  "type": "registry:ui",
                  "extends": ""
                }
              ]
            }"#,
            "empty extends target",
        ),
        (
            r#"{
              "items": [
                {
                  "name": "button",
                  "type": "registry:ui",
                  "extends": "button"
                }
              ]
            }"#,
            "cannot extend itself",
        ),
        (
            r#"{
              "items": [
                {
                  "name": "button",
                  "type": "registry:ui",
                  "extends": "utils",
                  "registryDependencies": ["utils"]
                },
                {
                  "name": "utils",
                  "type": "registry:lib"
                }
              ]
            }"#,
            "duplicate registry relationship `utils`",
        ),
    ] {
        let registry = parse_forge_ui_registry_json(json).expect("parse registry");
        let error = format!(
            "{:?}",
            validate_forge_ui_registry_catalog(&registry).expect_err("extends must fail")
        );

        assert!(error.contains(expected), "{error}");
    }
}

#[test]
fn forge_ui_registry_plan_bridges_external_extends_without_network() {
    let registry = parse_forge_ui_registry_json(
        r#"{
              "items": [
                {
                  "name": "hero",
                  "type": "registry:block",
                  "extends": "https://example.com/r/base-block.json",
                  "files": [
                    {
                      "path": "hero.tsx",
                      "type": "registry:block",
                      "content": "export const Hero = 'hero'"
                    }
                  ]
                }
              ]
            }"#,
    )
    .expect("parse registry");

    let report =
        plan_forge_ui_registry_item(&registry, "hero", ".").expect("plan external extends bridge");

    assert!(!report.passed);
    assert_eq!(report.score, 75);
    assert_eq!(report.registry_dependency_order, vec!["hero"]);
    assert_eq!(report.registry_dependency_count, 1);
    assert!(report.decisions.iter().any(|decision| {
        decision.subject == "hero -> https://example.com/r/base-block.json"
            && decision.decision == DxForgeUiRegistryPlanDecisionKind::BridgeDependency
    }));
    assert!(
        report
            .warnings
            .iter()
            .any(|warning| warning.contains("https://example.com/r/base-block.json"))
    );
    assert!(report.no_package_manager_execution);
}

#[test]
fn forge_ui_registry_plan_bridges_remote_registry_dependency_urls_without_network() {
    let registry = parse_forge_ui_registry_json(
        r#"{
              "items": [
                {
                  "name": "button",
                  "type": "registry:ui",
                  "files": [
                    {
                      "path": "button.tsx",
                      "type": "registry:ui",
                      "content": "export const Button = 'button'"
                    }
                  ]
                },
                {
                  "name": "hero",
                  "type": "registry:block",
                  "registryDependencies": [
                    "button",
                    "https://example.com/r/chart.json"
                  ],
                  "files": [
                    {
                      "path": "hero.tsx",
                      "type": "registry:block",
                      "content": "export const Hero = 'hero'"
                    }
                  ]
                }
              ]
            }"#,
    )
    .expect("parse registry");

    let report = plan_forge_ui_registry_item(&registry, "hero", ".")
        .expect("plan remote registry dependency bridge");

    assert!(!report.passed);
    assert_eq!(report.score, 75);
    assert_eq!(report.registry_dependency_order, vec!["button", "hero"]);
    assert_eq!(report.registry_dependency_count, 2);
    assert!(report.no_package_manager_execution);
    assert!(report.decisions.iter().any(|decision| {
        decision.subject == "hero -> button"
            && decision.decision == DxForgeUiRegistryPlanDecisionKind::ResolveRegistryDependency
    }));
    assert!(report.decisions.iter().any(|decision| {
        decision.subject == "hero -> https://example.com/r/chart.json"
            && decision.decision == DxForgeUiRegistryPlanDecisionKind::BridgeDependency
    }));
    assert!(report.warnings.iter().any(|warning| {
        warning.contains("https://example.com/r/chart.json")
            && warning.contains("external registry dependency")
    }));
    assert!(
        report
            .next_actions
            .iter()
            .any(|action| action.contains("Resolve external registry dependencies"))
    );
}

#[test]
fn forge_ui_registry_plan_bridges_external_registry_item_addresses_without_network() {
    let registry = parse_forge_ui_registry_json(
        r#"{
              "items": [
                {
                  "name": "hero",
                  "type": "registry:block",
                  "registryDependencies": [
                    "@acme/input-form",
                    "acme/ui/button#v1.2.0",
                    "./editor.json"
                  ],
                  "files": [
                    {
                      "path": "hero.tsx",
                      "type": "registry:block",
                      "content": "export const Hero = 'hero'"
                    }
                  ]
                }
              ]
            }"#,
    )
    .expect("parse registry");

    let report = plan_forge_ui_registry_item(&registry, "hero", ".")
        .expect("plan external registry dependency bridge");

    assert!(!report.passed);
    assert_eq!(report.score, 75);
    assert_eq!(report.registry_dependency_order, vec!["hero"]);
    assert_eq!(report.registry_dependency_count, 3);
    for dependency in ["@acme/input-form", "acme/ui/button#v1.2.0", "./editor.json"] {
        assert!(report.decisions.iter().any(|decision| {
            decision.subject == format!("hero -> {dependency}")
                && decision.decision == DxForgeUiRegistryPlanDecisionKind::BridgeDependency
        }));
    }
    assert!(
        report
            .warnings
            .iter()
            .any(|warning| warning.contains("@acme/input-form"))
    );
    assert!(report.no_package_manager_execution);
}

#[test]
fn forge_ui_registry_docs_report_exposes_reviewed_item_metadata_without_writes() {
    let registry = parse_forge_ui_registry_json(
        r##"{
              "name": "dx",
              "items": [
                {
                  "name": "utils",
                  "type": "registry:lib",
                  "files": [
                    {
                      "path": "utils.ts",
                      "type": "registry:lib",
                      "content": "export function cn(value: string) { return value }"
                    }
                  ]
                },
                {
                  "name": "button",
                  "type": "registry:ui",
                  "title": "Button",
                  "description": "A reviewed button primitive.",
                  "dependencies": ["@radix-ui/react-slot"],
                  "devDependencies": ["tailwindcss"],
                  "registryDependencies": ["utils"],
                  "files": [
                    {
                      "path": "button.tsx",
                      "type": "registry:ui",
                      "content": "export function Button() { return <button /> }"
                    }
                  ],
                  "cssVars": {
                    "theme": {
                      "radius": "0.5rem"
                    }
                  },
                  "envVars": {
                    "DX_BUTTON_MODE": "reviewed"
                  },
                  "docs": "Use this button for reviewed Forge UI actions."
                }
              ]
            }"##,
    )
    .expect("parse registry");

    let report = describe_forge_ui_registry_item(&registry, "button")
        .expect("describe reviewed registry item");

    assert_eq!(report.item_name, "button");
    assert_eq!(report.title.as_deref(), Some("Button"));
    assert_eq!(
        report.description.as_deref(),
        Some("A reviewed button primitive.")
    );
    assert_eq!(
        report.docs.as_deref(),
        Some("Use this button for reviewed Forge UI actions.")
    );
    assert!(report.has_docs);
    assert_eq!(report.registry_dependency_order, vec!["utils", "button"]);
    assert_eq!(report.files[0].target_path, "components/ui/button.tsx");
    assert_eq!(report.dependencies, vec!["@radix-ui/react-slot"]);
    assert_eq!(report.dev_dependencies, vec!["tailwindcss"]);
    assert_eq!(report.env_vars, vec!["DX_BUTTON_MODE"]);
    assert_eq!(report.css_var_count, 1);
    assert!(report.no_package_manager_execution);
    assert!(
        report
            .next_actions
            .iter()
            .any(|action| action.contains("dx forge registry plan --item button"))
    );
}

#[test]
fn forge_ui_registry_plan_resolves_alias_target_placeholders_from_base_config() {
    let registry = parse_forge_ui_registry_json(
        r##"{
              "name": "dx",
              "items": [
                {
                  "name": "base",
                  "type": "registry:base",
                  "config": {
                    "aliases": {
                      "components": "src/components",
                      "ui": "src/components/ui",
                      "lib": "src/lib",
                      "hooks": "src/hooks"
                    }
                  }
                },
                {
                  "name": "button",
                  "type": "registry:ui",
                  "registryDependencies": ["base"],
                  "files": [
                    {
                      "path": "registry/new-york/button.tsx",
                      "type": "registry:ui",
                      "target": "@ui/button.tsx",
                      "content": "export function Button() { return <button /> }"
                    },
                    {
                      "path": "registry/new-york/card.tsx",
                      "type": "registry:component",
                      "target": "@components/card.tsx",
                      "content": "export function Card() { return <section /> }"
                    },
                    {
                      "path": "registry/new-york/format-date.ts",
                      "type": "registry:lib",
                      "target": "@lib/format-date.ts",
                      "content": "export const formatDate = String"
                    },
                    {
                      "path": "registry/new-york/use-demo.ts",
                      "type": "registry:hook",
                      "target": "@hooks/use-demo.ts",
                      "content": "export function useDemo() { return null }"
                    }
                  ]
                }
              ]
            }"##,
    )
    .expect("parse registry");

    let report = plan_forge_ui_registry_item(&registry, "button", ".")
        .expect("plan alias target placeholders");
    let targets = report
        .files
        .iter()
        .map(|file| file.target_path.as_str())
        .collect::<Vec<_>>();

    assert!(report.passed);
    assert!(targets.contains(&"src/components/ui/button.tsx"));
    assert!(targets.contains(&"src/components/card.tsx"));
    assert!(targets.contains(&"src/lib/format-date.ts"));
    assert!(targets.contains(&"src/hooks/use-demo.ts"));
}

#[test]
fn forge_ui_registry_validation_rejects_unsafe_config_alias_targets() {
    let registry = parse_forge_ui_registry_json(
        r##"{
              "name": "dx",
              "items": [
                {
                  "name": "base",
                  "type": "registry:base",
                  "config": {
                    "aliases": {
                      "ui": "@/../private/ui"
                    }
                  }
                }
              ]
            }"##,
    )
    .expect("parse registry");

    let error = validate_forge_ui_registry_catalog(&registry)
        .expect_err("unsafe alias target must fail catalog validation");
    let error = format!("{error:#}");

    assert!(error.contains("alias `ui`"), "{error}");
    assert!(error.contains("base"), "{error}");
    assert!(
        error.contains("path cannot escape the project root"),
        "{error}"
    );
}

#[test]
fn forge_ui_registry_validation_rejects_unsafe_config_registry_targets() {
    let registry = parse_forge_ui_registry_json(
        r##"{
              "name": "dx",
              "items": [
                {
                  "name": "base",
                  "type": "registry:base",
                  "config": {
                    "registries": {
                      "local": "packages/ui/registry.json",
                      "remote": "https://example.com/r/{name}.json",
                      "unsafe": "npm install @acme/ui"
                    }
                  }
                }
              ]
            }"##,
    )
    .expect("parse registry");

    let error = validate_forge_ui_registry_catalog(&registry)
        .expect_err("unsafe registry target must fail catalog validation");
    let error = format!("{error:#}");

    assert!(error.contains("registry `unsafe`"), "{error}");
    assert!(error.contains("base"), "{error}");
    assert!(
        error.contains("package-manager commands are not registry targets"),
        "{error}"
    );
}

#[test]
fn forge_ui_registry_validation_rejects_unsafe_advanced_registry_headers() {
    let registry = parse_forge_ui_registry_json(
        r##"{
              "name": "dx",
              "items": [
                {
                  "name": "base",
                  "type": "registry:base",
                  "config": {
                    "registries": {
                      "remote": {
                        "url": "https://example.com/r/{name}.json",
                        "params": {
                          "style": "new-york"
                        },
                        "headers": {
                          "Authorization": "Bearer ${DX_FORGE_REGISTRY_TOKEN}",
                          "X-DX\nInjected": "bad"
                        }
                      }
                    }
                  }
                }
              ]
            }"##,
    )
    .expect("parse registry");

    let error = validate_forge_ui_registry_catalog(&registry)
        .expect_err("unsafe advanced registry header must fail catalog validation");
    let error = format!("{error:#}");

    assert!(error.contains("registry `remote`"), "{error}");
    assert!(error.contains("header"), "{error}");
    assert!(error.contains("base"), "{error}");
}

#[test]
fn forge_ui_registry_plan_treats_unknown_target_placeholder_as_literal_path() {
    let registry = parse_forge_ui_registry_json(
        r##"{
              "name": "dx",
              "items": [
                {
                  "name": "base",
                  "type": "registry:base",
                  "config": {
                    "aliases": {
                      "ui": "components/ui"
                    }
                  }
                },
                {
                  "name": "custom",
                  "type": "registry:file",
                  "registryDependencies": ["base"],
                  "files": [
                    {
                      "path": "registry/new-york/custom.ts",
                      "type": "registry:file",
                      "target": "@custom/custom.ts",
                      "content": "export const custom = true"
                    }
                  ]
                }
              ]
            }"##,
    )
    .expect("parse registry");

    let report = plan_forge_ui_registry_item(&registry, "custom", ".")
        .expect("plan literal unknown target placeholder");

    assert_eq!(report.files[0].target_path, "custom/custom.ts");
}

#[test]
fn forge_ui_registry_plan_rejects_missing_self_and_cyclic_registry_dependencies() {
    for (json, item, expected) in [
        (
            r#"{
              "items": [
                {
                  "name": "button",
                  "type": "registry:ui",
                  "registryDependencies": ["utils"]
                }
              ]
            }"#,
            "button",
            "depends on missing registry item `utils`",
        ),
        (
            r#"{
              "items": [
                {
                  "name": "button",
                  "type": "registry:ui",
                  "registryDependencies": ["button"]
                }
              ]
            }"#,
            "button",
            "cannot depend on itself",
        ),
        (
            r#"{
              "items": [
                {
                  "name": "a",
                  "type": "registry:ui",
                  "registryDependencies": ["b"]
                },
                {
                  "name": "b",
                  "type": "registry:ui",
                  "registryDependencies": ["a"]
                }
              ]
            }"#,
            "a",
            "dependency cycle detected: a -> b -> a",
        ),
    ] {
        let registry = parse_forge_ui_registry_json(json).expect("parse registry");
        let error = format!(
            "{:?}",
            plan_forge_ui_registry_item(&registry, item, ".").expect_err("graph must fail")
        );

        assert!(error.contains(expected), "{error}");
    }
}

#[test]
fn forge_ui_registry_dependency_graph_validation_rejects_unresolved_authored_graphs() {
    let registry = parse_forge_ui_registry_json(
        r#"{
              "items": [
                {
                  "name": "dialog",
                  "type": "registry:ui",
                  "registryDependencies": ["button"]
                }
              ]
            }"#,
    )
    .expect("parse registry");

    let error = format!(
        "{:?}",
        validate_forge_ui_registry_dependency_graphs(&registry).expect_err("graph must fail")
    );

    assert!(
        error.contains("depends on missing registry item `button`"),
        "{error}"
    );
}

#[test]
fn forge_ui_registry_plan_marks_missing_inline_content_as_not_write_ready() {
    let registry = parse_forge_ui_registry_json(
        r#"{
              "items": [
                {
                  "name": "card",
                  "type": "registry:ui",
                  "files": [
                    {
                      "path": "card.tsx",
                      "type": "registry:ui"
                    }
                  ]
                }
              ]
            }"#,
    )
    .expect("parse registry");

    let report = plan_forge_ui_registry_item(&registry, "card", ".").expect("plan registry item");

    assert!(!report.passed);
    assert_eq!(report.score, 80);
    assert_eq!(report.missing_inline_content_count, 1);
    assert_eq!(report.write_file_count, 0);
    assert_eq!(report.files[0].target_path, "components/ui/card.tsx");
    assert_eq!(
        report.files[0].action,
        DxForgeUiRegistryPlanAction::NeedsReviewedContent
    );
    assert!(
        report
            .warnings
            .iter()
            .any(|warning| warning.contains("requires reviewed content"))
    );
}

#[test]
fn forge_ui_registry_flattening_resolves_included_file_paths_before_local_items() {
    let root = parse_forge_ui_registry_json(
        r#"{
              "name": "dx",
              "homepage": "https://dx.local",
              "include": ["components/ui/registry.json", "hooks/registry.json"],
              "items": [
                {
                  "name": "landing",
                  "type": "registry:page",
                  "files": [
                    {
                      "path": "landing/page.tsx",
                      "type": "registry:page",
                      "target": "app/page.tsx"
                    }
                  ]
                }
              ]
            }"#,
    )
    .expect("parse root");
    let ui = parse_forge_ui_registry_json(
        r#"{
              "items": [
                {
                  "name": "button",
                  "type": "registry:ui",
                  "files": [
                    {
                      "path": "button.tsx",
                      "type": "registry:ui"
                    }
                  ]
                }
              ]
            }"#,
    )
    .expect("parse ui");
    let hooks = parse_forge_ui_registry_json(
        r#"{
              "items": [
                {
                  "name": "use-media-query",
                  "type": "registry:hook",
                  "files": [
                    {
                      "path": "./use-media-query.ts",
                      "type": "registry:hook"
                    }
                  ]
                }
              ]
            }"#,
    )
    .expect("parse hooks");
    let includes = BTreeMap::from([
        ("components/ui/registry.json".to_string(), ui),
        ("hooks/registry.json".to_string(), hooks),
    ]);

    let flattened = flatten_forge_ui_registry_catalogs(&root, &includes).expect("flatten registry");

    assert_eq!(flattened.include, Vec::<String>::new());
    assert_eq!(
        flattened
            .items
            .iter()
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>(),
        vec!["button", "use-media-query", "landing"]
    );
    assert_eq!(flattened.items[0].files[0].path, "components/ui/button.tsx");
    assert_eq!(flattened.items[1].files[0].path, "hooks/use-media-query.ts");
    assert_eq!(flattened.items[2].files[0].path, "landing/page.tsx");
    assert_eq!(
        flattened.items[2].files[0].target.as_deref(),
        Some("app/page.tsx")
    );

    let report = validate_forge_ui_registry_catalog(&flattened).expect("validate flat registry");
    assert_eq!(report.item_count, 3);
    assert_eq!(report.file_count, 3);
}

#[test]
fn forge_ui_registry_validation_rejects_unsafe_include_paths() {
    for (include, expected) in [
        ("https://example.com/registry.json", "remote include"),
        ("../registry.json", "cannot escape"),
        ("/registry/ui/registry.json", "must be project-relative"),
        (
            "registry/ui.json",
            "must explicitly reference registry.json",
        ),
    ] {
        let registry = parse_forge_ui_registry_json(&format!(
            r#"{{
              "items": [],
              "include": ["{include}"]
            }}"#
        ))
        .expect("parse registry");

        let error = format!(
            "{:?}",
            validate_forge_ui_registry_catalog(&registry).expect_err("unsafe include must fail")
        );

        assert!(error.contains(expected), "{error}");
    }
}

#[test]
fn forge_ui_registry_validation_requires_targets_for_file_and_page_files() {
    let registry_file = parse_forge_ui_registry_json(
        r#"{
              "items": [
                {
                  "name": "docs",
                  "type": "registry:file",
                  "files": [
                    {
                      "path": "docs.md",
                      "type": "registry:file"
                    }
                  ]
                }
              ]
            }"#,
    )
    .expect("parse registry");

    let error = validate_forge_ui_registry_catalog(&registry_file)
        .expect_err("registry:file needs target")
        .to_string();

    assert!(error.contains("requires target"));

    let registry_page = parse_forge_ui_registry_json(
        r#"{
              "items": [
                {
                  "name": "page",
                  "type": "registry:page",
                  "files": [
                    {
                      "path": "page.tsx",
                      "type": "registry:page"
                    }
                  ]
                }
              ]
            }"#,
    )
    .expect("parse registry");

    let error = validate_forge_ui_registry_catalog(&registry_page)
        .expect_err("registry:page needs target")
        .to_string();

    assert!(error.contains("requires target"));
}

#[test]
fn forge_ui_registry_flattening_rejects_duplicate_items() {
    let root = parse_forge_ui_registry_json(
        r#"{
              "name": "dx",
              "homepage": "https://dx.local",
              "include": ["components/ui/registry.json"],
              "items": [
                {
                  "name": "button",
                  "type": "registry:ui"
                }
              ]
            }"#,
    )
    .expect("parse root");
    let ui = parse_forge_ui_registry_json(
        r#"{
              "items": [
                {
                  "name": "button",
                  "type": "registry:ui"
                }
              ]
            }"#,
    )
    .expect("parse ui");
    let includes = BTreeMap::from([("components/ui/registry.json".to_string(), ui)]);

    let error = flatten_forge_ui_registry_catalogs(&root, &includes)
        .expect_err("duplicate item must fail")
        .to_string();

    assert!(error.contains("duplicate Forge UI registry item `button`"));
}

#[test]
fn forge_ui_registry_flattening_supports_nested_include_graphs() {
    let root = parse_forge_ui_registry_json(
        r#"{
          "name": "dx",
          "homepage": "https://dx.local",
          "include": ["registry/registry.json"]
        }"#,
    )
    .expect("parse root");
    let blocks = parse_forge_ui_registry_json(
        r#"{
          "include": ["ui/registry.json"],
          "items": [
            {
              "name": "hero",
              "type": "registry:block",
              "files": [
                {
                  "path": "hero.tsx",
                  "type": "registry:block"
                }
              ]
            }
          ]
        }"#,
    )
    .expect("parse blocks");
    let ui = parse_forge_ui_registry_json(
        r#"{
          "items": [
            {
              "name": "button",
              "type": "registry:ui",
              "files": [
                {
                  "path": "button.tsx",
                  "type": "registry:ui"
                }
              ]
            }
          ]
        }"#,
    )
    .expect("parse ui");
    let includes = BTreeMap::from([
        ("registry/registry.json".to_string(), blocks),
        ("registry/ui/registry.json".to_string(), ui),
    ]);

    let flattened = flatten_forge_ui_registry_catalogs(&root, &includes).expect("flatten nested");

    assert_eq!(
        flattened
            .items
            .iter()
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>(),
        vec!["button", "hero"]
    );
    assert_eq!(flattened.items[0].files[0].path, "registry/ui/button.tsx");
    assert_eq!(flattened.items[1].files[0].path, "registry/hero.tsx");
}

#[test]
fn forge_ui_registry_flattening_rejects_include_cycles() {
    let root = parse_forge_ui_registry_json(
        r#"{
          "name": "dx",
          "homepage": "https://dx.local",
          "include": ["registry/registry.json"]
        }"#,
    )
    .expect("parse root");
    let nested = parse_forge_ui_registry_json(
        r#"{
          "include": ["./registry.json"]
        }"#,
    )
    .expect("parse nested");
    let includes = BTreeMap::from([("registry/registry.json".to_string(), nested)]);

    let error = flatten_forge_ui_registry_catalogs(&root, &includes)
        .expect_err("cycle must fail")
        .to_string();

    assert!(error.contains("cycle"));
}

#[test]
fn forge_ui_registry_loader_reads_authored_registry_folder() {
    let dir = tempdir().expect("tempdir");
    fs::create_dir_all(dir.path().join("ui")).expect("ui dir");
    fs::write(
        dir.path().join("registry.json"),
        r#"{
          "name": "dx",
          "homepage": "https://dx.local",
          "include": ["ui/registry.json"],
          "items": [
            {
              "name": "landing",
              "type": "registry:page",
              "files": [
                {
                  "path": "page.tsx",
                  "type": "registry:page",
                  "target": "app/page.tsx"
                }
              ]
            }
          ]
        }"#,
    )
    .expect("root registry");
    fs::write(
        dir.path().join("ui/registry.json"),
        r#"{
          "items": [
            {
              "name": "button",
              "type": "registry:ui",
              "files": [
                {
                  "path": "button.tsx",
                  "type": "registry:ui"
                }
              ]
            }
          ]
        }"#,
    )
    .expect("ui registry");

    let catalog =
        load_forge_ui_registry_catalog_from_path(dir.path().join("registry.json")).expect("load");

    assert_eq!(catalog.include, Vec::<String>::new());
    assert_eq!(
        catalog
            .items
            .iter()
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>(),
        vec!["button", "landing"]
    );
    assert_eq!(catalog.items[0].files[0].path, "ui/button.tsx");
    assert_eq!(catalog.items[1].files[0].path, "page.tsx");
}

#[test]
fn forge_ui_registry_resolver_plans_configured_local_registry_alias() {
    let dir = tempdir().expect("tempdir");
    fs::create_dir_all(dir.path().join("packages/ui")).expect("registry dir");
    fs::write(
        dir.path().join("registry.json"),
        r#"{
          "items": [
            {
              "name": "base",
              "type": "registry:base",
              "config": {
                "registries": {
                  "@local": "packages/ui/registry.json"
                }
              }
            }
          ]
        }"#,
    )
    .expect("root registry");
    fs::write(
        dir.path().join("packages/ui/registry.json"),
        r#"{
          "items": [
            {
              "name": "utils",
              "type": "registry:lib",
              "files": [
                {
                  "path": "lib/utils.ts",
                  "type": "registry:lib",
                  "content": "export function cn(value: string) { return value }"
                }
              ]
            },
            {
              "name": "button",
              "type": "registry:ui",
              "registryDependencies": ["utils"],
              "files": [
                {
                  "path": "button.tsx",
                  "type": "registry:ui",
                  "content": "export function Button() { return <button /> }"
                }
              ]
            }
          ]
        }"#,
    )
    .expect("local registry");

    let resolved =
        resolve_forge_ui_registry_reference(dir.path().join("registry.json"), "local/button")
            .expect("resolve local registry alias");
    let plan = plan_forge_ui_registry_item(&resolved.catalog, &resolved.item_name, dir.path())
        .expect("plan local alias");

    assert_eq!(resolved.item_name, "button");
    assert_eq!(
        resolved.registry_file,
        dir.path().join("packages/ui/registry.json")
    );
    assert_eq!(resolved.source_kind, "configured-local-registry");
    assert_eq!(plan.registry_dependency_order, vec!["utils", "button"]);
    assert_eq!(plan.files[0].target_path, "lib/utils.ts");
    assert_eq!(plan.files[1].target_path, "components/ui/button.tsx");
    assert!(plan.no_package_manager_execution);
    assert!(plan.passed);
}

#[test]
fn forge_ui_registry_resolver_expands_configured_registry_name_templates() {
    let dir = tempdir().expect("tempdir");
    fs::create_dir_all(dir.path().join("packages/button")).expect("registry dir");
    fs::write(
        dir.path().join("registry.json"),
        r#"{
          "items": [
            {
              "name": "base",
              "type": "registry:base",
              "config": {
                "registries": {
                  "local": "packages/{name}/registry.json"
                }
              }
            }
          ]
        }"#,
    )
    .expect("root registry");
    fs::write(
        dir.path().join("packages/button/registry.json"),
        r#"{
          "items": [
            {
              "name": "button",
              "type": "registry:ui",
              "files": [
                {
                  "path": "button.tsx",
                  "type": "registry:ui",
                  "content": "export function Button() { return <button /> }"
                }
              ]
            }
          ]
        }"#,
    )
    .expect("templated local registry");

    let resolved =
        resolve_forge_ui_registry_reference(dir.path().join("registry.json"), "local/button")
            .expect("resolve templated registry alias");

    assert_eq!(resolved.item_name, "button");
    assert_eq!(
        resolved.registry_file,
        dir.path().join("packages/button/registry.json")
    );
    assert_eq!(resolved.source_kind, "configured-local-registry");
}

#[test]
fn forge_ui_registry_resolver_plans_explicit_local_registry_file_reference() {
    let dir = tempdir().expect("tempdir");
    fs::create_dir_all(dir.path().join("packages/ui")).expect("registry dir");
    fs::write(
        dir.path().join("registry.json"),
        r#"{
          "items": [
            {
              "name": "base",
              "type": "registry:base",
              "config": {
                "registries": {}
              }
            }
          ]
        }"#,
    )
    .expect("root registry");
    fs::write(
        dir.path().join("packages/ui/registry.json"),
        r#"{
          "items": [
            {
              "name": "card",
              "type": "registry:ui",
              "files": [
                {
                  "path": "card.tsx",
                  "type": "registry:ui",
                  "content": "export function Card() { return <section /> }"
                }
              ]
            }
          ]
        }"#,
    )
    .expect("local registry");

    let resolved = resolve_forge_ui_registry_reference(
        dir.path().join("registry.json"),
        "packages/ui/registry.json#card",
    )
    .expect("resolve explicit registry file");

    assert_eq!(resolved.item_name, "card");
    assert_eq!(
        resolved.registry_file,
        dir.path().join("packages/ui/registry.json")
    );
    assert_eq!(resolved.source_kind, "local-registry-file");
}

#[test]
fn forge_ui_registry_resolver_rejects_remote_registry_alias_without_network() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("registry.json"),
        r#"{
          "items": [
            {
              "name": "base",
              "type": "registry:base",
              "config": {
                "registries": {
                  "remote": "https://example.com/r/{name}.json"
                }
              }
            }
          ]
        }"#,
    )
    .expect("root registry");

    let error =
        resolve_forge_ui_registry_reference(dir.path().join("registry.json"), "remote/card")
            .expect_err("remote aliases are bridge-gated")
            .to_string();

    assert!(error.contains("remote registry resolution is bridge-gated"));
    assert!(error.contains("https://example.com/r/card.json"));
    assert!(error.contains("accepted Forge registry bridge or pull receipt"));
    assert!(error.contains("no network request was made"));
}

#[test]
fn forge_ui_registry_build_embedding_reads_missing_file_contents_from_source_root() {
    let dir = tempdir().expect("tempdir");
    fs::create_dir_all(dir.path().join("components/ui")).expect("ui dir");
    fs::create_dir_all(dir.path().join("lib")).expect("lib dir");
    fs::write(
        dir.path().join("components/ui/button.tsx"),
        "export function Button() { return <button /> }",
    )
    .expect("button source");
    fs::write(
        dir.path().join("components/ui/button-empty.tsx"),
        "export function EmptyButton() { return <button /> }",
    )
    .expect("empty button source");
    fs::write(
        dir.path().join("lib/utils.ts"),
        "export function cn(value: string) { return value }",
    )
    .expect("utils source");
    let registry = parse_forge_ui_registry_json(
        r#"{
              "items": [
                {
                  "name": "utils",
                  "type": "registry:lib",
                  "files": [
                    {
                      "path": "lib/utils.ts",
                      "type": "registry:lib"
                    }
                  ]
                },
                {
                  "name": "button",
                  "type": "registry:ui",
                  "registryDependencies": ["utils"],
                  "files": [
                    {
                      "path": "components/ui/button.tsx",
                      "type": "registry:ui"
                    },
                    {
                      "path": "components/ui/button-empty.tsx",
                      "type": "registry:ui",
                      "content": ""
                    },
                    {
                      "path": "button-inline.tsx",
                      "type": "registry:ui",
                      "content": "export const InlineButton = true"
                    }
                  ]
                }
              ]
            }"#,
    )
    .expect("parse registry");

    let (embedded, report) =
        embed_forge_ui_registry_catalog_file_contents(&registry, dir.path()).expect("embed");

    assert_eq!(report.file_count, 4);
    assert_eq!(report.embedded_file_count, 3);
    assert_eq!(report.preserved_inline_content_file_count, 1);
    assert_eq!(report.files.len(), 4);
    let button_evidence = report
        .files
        .iter()
        .find(|file| file.registry_path == "components/ui/button.tsx")
        .expect("button evidence");
    assert_eq!(button_evidence.item_name, "button");
    assert_eq!(
        button_evidence.status,
        DxForgeUiRegistryContentEmbeddingStatus::EmbeddedFromSource
    );
    assert_eq!(
        button_evidence.source_path.as_deref(),
        Some("components/ui/button.tsx")
    );
    assert_eq!(button_evidence.hash_algorithm, "BLAKE3");
    assert_eq!(
        button_evidence.byte_count,
        "export function Button() { return <button /> }".len()
    );
    assert!(!button_evidence.content_hash.is_empty());
    let inline_evidence = report
        .files
        .iter()
        .find(|file| file.registry_path == "button-inline.tsx")
        .expect("inline evidence");
    assert_eq!(
        inline_evidence.status,
        DxForgeUiRegistryContentEmbeddingStatus::PreservedInlineContent
    );
    assert!(inline_evidence.source_path.is_none());
    assert_eq!(
        embedded.items[0].files[0].content.as_deref(),
        Some("export function cn(value: string) { return value }")
    );
    assert_eq!(
        embedded.items[1].files[0].content.as_deref(),
        Some("export function Button() { return <button /> }")
    );
    assert_eq!(
        embedded.items[1].files[1].content.as_deref(),
        Some("export function EmptyButton() { return <button /> }")
    );
    assert_eq!(
        embedded.items[1].files[2].content.as_deref(),
        Some("export const InlineButton = true")
    );
}
