use super::quarantine::validate_import_relative_path;
use super::slice::{DxForgeImportSliceCandidate, classify_import_slice};
use super::types::{
    DX_FORGE_IMPORT_ECOSYSTEMS, DxForgeImportDecision, DxForgeImportEcosystem,
    DxForgeImportRiskFlag, DxForgeImportSliceKind,
};
use super::{acquire_rule_for_ecosystem, acquisition_plan_for_package};
use super::{
    cargo_import_plan_surface, composer_import_plan_surface, cran_import_plan_surface,
    gem_import_plan_surface, go_import_plan_surface, hex_import_plan_surface,
    jsr_import_plan_surface, maven_import_plan_surface, npm_import_plan_surface,
    nuget_import_plan_surface, pip_import_plan_surface, pub_import_plan_surface,
    swift_import_plan_surface,
};

#[test]
fn acquire_rules_never_execute_package_code() {
    for &ecosystem in DX_FORGE_IMPORT_ECOSYSTEMS {
        let rule = acquire_rule_for_ecosystem(ecosystem);
        assert!(!rule.executes_package_code);
        assert!(!rule.forbidden_commands.is_empty());
    }
}

#[test]
fn acquisition_plans_are_non_executing_and_path_stable() {
    let npm_plan =
        acquisition_plan_for_package(DxForgeImportEcosystem::Npm, "lodash", "npm/lodash");
    assert_eq!(npm_plan.expected_source_dir, ".dx/cache/npm/lodash/package");
    assert_eq!(npm_plan.quarantine_dir, ".dx/forge/quarantine/npm/lodash");
    assert_eq!(
        npm_plan.evidence_receipt_path,
        ".dx/forge/import-receipts/npm-lodash-acquire.sr"
    );
    assert!(npm_plan.live_fetching_enabled);
    assert!(!npm_plan.package_manager_execution);
    assert!(!npm_plan.executes_package_code);
    assert!(npm_plan.source_dir_required_for_materialization);
    assert!(
        npm_plan
            .metadata_references
            .iter()
            .any(|item| item.contains("npm"))
    );
    assert!(
        npm_plan
            .artifact_references
            .iter()
            .any(|item| item.contains("dist.tarball"))
    );

    let jsr_plan =
        acquisition_plan_for_package(DxForgeImportEcosystem::Jsr, "@std/path", "jsr/@std/path");
    assert_eq!(
        jsr_plan.expected_source_dir,
        ".dx/cache/jsr/std-path/package"
    );
    assert_eq!(
        jsr_plan.evidence_receipt_path,
        ".dx/forge/import-receipts/jsr-std-path-acquire.sr"
    );
    assert!(!jsr_plan.forbidden_commands.is_empty());
    assert!(
        jsr_plan
            .metadata_references
            .iter()
            .any(|item| item.contains("JSR"))
    );
}

#[test]
fn ecosystem_import_plan_surfaces_are_non_executing_and_receipt_gated() {
    for surface in [
        npm_import_plan_surface(),
        pip_import_plan_surface(),
        cargo_import_plan_surface(),
        go_import_plan_surface(),
        jsr_import_plan_surface(),
        pub_import_plan_surface(),
        maven_import_plan_surface(),
        nuget_import_plan_surface(),
        composer_import_plan_surface(),
        gem_import_plan_surface(),
        swift_import_plan_surface(),
        hex_import_plan_surface(),
        cran_import_plan_surface(),
    ] {
        assert_eq!(
            surface.live_fetching_enabled,
            surface.ecosystem == DxForgeImportEcosystem::Npm
        );
        assert!(!surface.package_manager_execution);
        assert!(surface.accepted_import_receipt_required);
        assert!(!surface.metadata_inputs.is_empty());
        assert!(!surface.artifact_inputs.is_empty());
        assert!(!surface.manual_review_triggers.is_empty());
        assert!(!surface.forbidden_commands.is_empty());
        assert!(surface.unsupported_dx_add_form.starts_with("dx add "));
    }
}

#[test]
fn hex_and_cran_are_non_executing_source_snapshot_surfaces() {
    for (ecosystem, segment, blocked_command, surface) in [
        (
            DxForgeImportEcosystem::Hex,
            "hex",
            "mix deps.get",
            hex_import_plan_surface(),
        ),
        (
            DxForgeImportEcosystem::Cran,
            "cran",
            "R CMD INSTALL",
            cran_import_plan_surface(),
        ),
    ] {
        assert!(DX_FORGE_IMPORT_ECOSYSTEMS.contains(&ecosystem));
        assert_eq!(ecosystem.as_segment(), segment);
        assert_eq!(
            DxForgeImportEcosystem::from_segment(segment.to_ascii_uppercase().as_str()),
            Some(ecosystem)
        );
        assert!(
            DxForgeImportEcosystem::supported_segments_help().contains(segment),
            "help should include {segment}"
        );
        assert!(ecosystem.blocked_commands().contains(&blocked_command));
        assert_eq!(surface.ecosystem, ecosystem);
        assert!(!surface.live_fetching_enabled);
        assert!(!surface.package_manager_execution);
        assert!(surface.accepted_import_receipt_required);
        assert!(!surface.metadata_inputs.is_empty());
        assert!(!surface.artifact_inputs.is_empty());
        assert!(!surface.manual_review_triggers.is_empty());
    }
}

#[test]
fn jsr_is_a_first_class_non_executing_javascript_registry_surface() {
    assert!(DX_FORGE_IMPORT_ECOSYSTEMS.contains(&DxForgeImportEcosystem::Jsr));
    assert_eq!(DxForgeImportEcosystem::Jsr.as_segment(), "jsr");
    assert_eq!(
        DxForgeImportEcosystem::from_segment("JSR"),
        Some(DxForgeImportEcosystem::Jsr)
    );
    assert!(DxForgeImportEcosystem::supported_segments_help().contains("jsr"));
    assert!(
        DxForgeImportEcosystem::Jsr
            .blocked_commands()
            .contains(&"deno add")
    );

    let surface = jsr_import_plan_surface();
    assert_eq!(surface.ecosystem, DxForgeImportEcosystem::Jsr);
    assert!(!surface.live_fetching_enabled);
    assert!(!surface.package_manager_execution);
    assert!(surface.accepted_import_receipt_required);
    assert!(
        surface
            .metadata_inputs
            .iter()
            .any(|input| input.contains("jsr.io"))
    );
    assert!(
        surface
            .artifact_inputs
            .iter()
            .any(|input| input.contains("source"))
    );
    assert!(
        surface
            .manual_review_triggers
            .iter()
            .any(|trigger| trigger.contains("Deno permission"))
    );
    assert_eq!(
        surface.unsupported_dx_add_form,
        "dx add jsr/<scope>/<package>"
    );
}

#[test]
fn import_paths_are_slash_normalized_and_project_relative() {
    let safe = validate_import_relative_path("src/package/index.ts").unwrap();
    assert_eq!(safe.as_str(), "src/package/index.ts");

    assert!(validate_import_relative_path("../escape.ts").is_err());
    assert!(validate_import_relative_path("node_modules/pkg/index.js").is_err());
    assert!(validate_import_relative_path(".secret/token.txt").is_err());
    assert!(validate_import_relative_path("pkg\\index.js").is_err());
}

#[test]
fn blocking_risks_fail_closed_for_importable_source() {
    let decision = classify_import_slice(DxForgeImportSliceCandidate {
        requested_kind: DxForgeImportSliceKind::SourceCopy,
        risk_flags: vec![DxForgeImportRiskFlag::LifecycleScript],
        writes_importable_source: true,
    });

    assert_eq!(decision.slice_kind, DxForgeImportSliceKind::Blocked);
    assert_eq!(decision.decision, DxForgeImportDecision::Block);
}
