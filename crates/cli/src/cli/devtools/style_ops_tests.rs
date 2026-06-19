use super::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn element_selection_payload_carries_parent_chain_shape_and_breakpoint() {
    let payload = DxElementSelectionPayload::new(
        DxSelectedElement {
            stable_id: "hero-title".to_string(),
            selector: "[data-dx-editable-section=\"hero\"] h1".to_string(),
            tag_name: "h1".to_string(),
            id: None,
            classes: vec!["headline".to_string()],
            attributes: BTreeMap::from([(
                "data-dx-editable-section".to_string(),
                "hero".to_string(),
            )]),
            source_target: None,
        },
        vec![DxClosestParentChainNode {
            depth: 0,
            selector: "[data-dx-editable-section=\"hero\"]".to_string(),
            tag_name: "section".to_string(),
            id: None,
            classes: vec!["hero".to_string()],
            data_dx_component: Some("launch-hero".to_string()),
            data_dx_editable_section: Some("hero".to_string()),
            source_target: None,
        }],
        computed_css("font-size", "32px"),
        valid_box_model(),
        1024,
        None,
    );

    assert_eq!(payload.schema, ELEMENT_SELECTION_SCHEMA);
    assert_eq!(payload.breakpoint_label, "lg");
    assert_eq!(payload.closest_parent_chain[0].depth, 0);
    assert_eq!(
        payload.closest_parent_chain[0]
            .data_dx_editable_section
            .as_deref(),
        Some("hero")
    );
    assert_eq!(payload.source.status, DxSourceWriteStatus::PreviewOnly);
}

#[test]
fn breakpoint_labels_match_preview_contract() {
    assert_eq!(breakpoint_label_for_width(375), "base");
    assert_eq!(breakpoint_label_for_width(640), "sm");
    assert_eq!(breakpoint_label_for_width(768), "md");
    assert_eq!(breakpoint_label_for_width(1024), "lg");
    assert_eq!(breakpoint_label_for_width(1280), "xl");
    assert_eq!(breakpoint_label_for_width(1536), "2xl");
}

#[test]
fn source_targets_reject_forbidden_paths_and_generated_css() {
    for path in [
        ".dx/receipts/style/check.json",
        "node_modules/pkg/style.css",
        "target/debug/style.css",
        "dist/app.css",
        "../styles/app.css",
        "C:/tmp/app.css",
        "/tmp/app.css",
        "styles/generated.css",
        "styles/app.generated.css",
    ] {
        let eligibility = source_write_eligibility(Some(&DxSourceTarget::known_exact(
            path,
            DxSourceTargetKind::AuthoredCss,
            DxSourceRange {
                start_byte: 0,
                end_byte: 0,
                expected_text: String::new(),
            },
        )));
        assert_eq!(
            eligibility.status,
            DxSourceWriteStatus::Rejected,
            "{path} should be rejected"
        );
    }
}

#[test]
fn unknown_or_inexact_source_targets_are_preview_only() {
    assert_eq!(
        source_write_eligibility(None).status,
        DxSourceWriteStatus::PreviewOnly
    );

    let eligibility = source_write_eligibility(Some(&DxSourceTarget::preview_only(
        "styles/app.css",
        DxSourceTargetKind::AuthoredCss,
    )));
    assert_eq!(eligibility.status, DxSourceWriteStatus::PreviewOnly);
    assert!(!eligibility.writable);

    let eligibility = source_write_eligibility(Some(&DxSourceTarget::known_exact(
        "styles/app.css",
        DxSourceTargetKind::Unknown,
        DxSourceRange {
            start_byte: 0,
            end_byte: 0,
            expected_text: String::new(),
        },
    )));
    assert_eq!(eligibility.status, DxSourceWriteStatus::PreviewOnly);
    assert!(!eligibility.writable);
}

#[test]
fn source_targets_require_authored_css_declaration_ranges() {
    for (path, kind, expected_text, reason) in [
        (
            "app/page.tsx",
            DxSourceTargetKind::InlineStyleAttribute,
            "padding: 8px;",
            "source-kind-not-writable",
        ),
        (
            "Cargo.toml",
            DxSourceTargetKind::AuthoredCss,
            "padding: 8px;",
            "non-css-source-target-not-writable",
        ),
        (
            "styles/app.css",
            DxSourceTargetKind::AuthoredCss,
            "not a declaration",
            "source-range-not-css-declaration",
        ),
    ] {
        let eligibility = source_write_eligibility(Some(&DxSourceTarget::known_exact(
            path,
            kind,
            DxSourceRange {
                start_byte: 0,
                end_byte: expected_text.len(),
                expected_text: expected_text.to_string(),
            },
        )));
        assert_eq!(eligibility.status, DxSourceWriteStatus::PreviewOnly);
        assert!(!eligibility.writable);
        assert_eq!(eligibility.reason, reason);
    }
}

#[test]
fn style_operation_requires_computed_css_and_valid_box_model() {
    let request = DxStyleChangeRequest::new(
        DxStyleOperationName::StylePreview,
        "padding",
        "12px",
        None,
        DxComputedCssPayload {
            properties: BTreeMap::new(),
        },
        valid_box_model(),
    );
    let validation = validate_style_operation(&request);
    assert!(!validation.valid);
    assert!(
        validation
            .reasons
            .contains(&"computed-css-property-missing".to_string())
    );

    let request = DxStyleChangeRequest::new(
        DxStyleOperationName::StylePreview,
        "padding",
        "12px",
        None,
        computed_css("padding", "8px"),
        DxBoxModelPayload {
            content: DxBoxRect {
                x: 0.0,
                y: 0.0,
                width: f64::NAN,
                height: 10.0,
            },
            padding: DxBoxEdges {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            },
            border: DxBoxEdges {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            },
            margin: DxBoxEdges {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            },
        },
    );
    let validation = validate_style_operation(&request);
    assert!(!validation.valid);
    assert!(
        validation
            .reasons
            .contains(&"invalid-box-model".to_string())
    );
}

#[test]
fn style_preview_never_mutates_known_writable_source() {
    let project = temp_project("style-preview");
    let path = project.join("styles");
    fs::create_dir_all(&path).expect("styles dir");
    let file = path.join("app.css");
    let source = ".card {\n  padding: 8px;\n}\n";
    fs::write(&file, source).expect("write source");
    let range_start = source.find("padding: 8px;").expect("declaration");
    let request = request_for_source(range_start, "padding: 8px;", "12px");

    let outcome = preview_style_change(&request);
    assert_eq!(outcome.status, DxStyleChangeStatus::PreviewOnly);
    assert!(!outcome.mutated);
    assert_eq!(fs::read_to_string(&file).expect("read source"), source);

    let outcome = apply_style_change(&project, &{
        let mut preview_request = request;
        preview_request.operation = DxStyleOperationName::StylePreview;
        preview_request
    })
    .expect("style preview through apply helper");
    assert_eq!(outcome.status, DxStyleChangeStatus::PreviewOnly);
    assert!(!outcome.mutated);
    assert_eq!(fs::read_to_string(&file).expect("read source"), source);
}

#[test]
fn style_apply_writes_only_matching_exact_source_range() {
    let project = temp_project("style-apply");
    let path = project.join("styles");
    fs::create_dir_all(&path).expect("styles dir");
    let file = path.join("app.css");
    let source = ".card {\n  padding: 8px;\n}\n";
    fs::write(&file, source).expect("write source");
    let range_start = source.find("padding: 8px;").expect("declaration");
    let request = request_for_source(range_start, "padding: 8px;", "12px");

    let outcome = apply_style_change(&project, &request).expect("style apply");
    assert_eq!(outcome.status, DxStyleChangeStatus::Applied);
    assert!(outcome.mutated);
    assert_eq!(
        fs::read_to_string(&file).expect("read source"),
        ".card {\n  padding: 12px;\n}\n"
    );
}

#[test]
fn style_apply_refuses_mismatched_exact_source_range_without_fake_write() {
    let project = temp_project("style-apply-mismatch");
    let path = project.join("styles");
    fs::create_dir_all(&path).expect("styles dir");
    let file = path.join("app.css");
    let source = ".card {\n  padding: 8px;\n}\n";
    fs::write(&file, source).expect("write source");
    let range_start = source.find("padding: 8px;").expect("declaration");
    let request = request_for_source(range_start, "padding: 10px;", "12px");

    let outcome = apply_style_change(&project, &request).expect("style apply");
    assert_eq!(outcome.status, DxStyleChangeStatus::Rejected);
    assert!(!outcome.mutated);
    assert_eq!(fs::read_to_string(&file).expect("read source"), source);
}

#[test]
fn style_apply_refuses_non_css_or_property_mismatched_exact_ranges() {
    let project = temp_project("style-apply-unsafe-target");
    let app = project.join("app");
    fs::create_dir_all(&app).expect("app dir");
    let page = app.join("page.tsx");
    let page_source = "export default function Page(){ return <main /> }\n";
    fs::write(&page, page_source).expect("write page");
    let request = DxStyleChangeRequest::new(
        DxStyleOperationName::StyleApply,
        "padding",
        "12px",
        Some(DxSourceTarget::known_exact(
            "app/page.tsx",
            DxSourceTargetKind::InlineStyleAttribute,
            DxSourceRange {
                start_byte: 0,
                end_byte: "export".len(),
                expected_text: "export".to_string(),
            },
        )),
        computed_css("padding", "8px"),
        valid_box_model(),
    );
    let outcome = apply_style_change(&project, &request).expect("style apply");
    assert_eq!(outcome.status, DxStyleChangeStatus::PreviewOnly);
    assert!(!outcome.mutated);
    assert_eq!(fs::read_to_string(&page).expect("read page"), page_source);

    let styles = project.join("styles");
    fs::create_dir_all(&styles).expect("styles dir");
    let file = styles.join("app.css");
    let source = ".card {\n  margin: 8px;\n}\n";
    fs::write(&file, source).expect("write source");
    let range_start = source.find("margin: 8px;").expect("declaration");
    let request = request_for_source(range_start, "margin: 8px;", "12px");
    let outcome = apply_style_change(&project, &request).expect("style apply");
    assert_eq!(outcome.status, DxStyleChangeStatus::Rejected);
    assert_eq!(outcome.reason, "source-range-property-mismatch");
    assert!(!outcome.mutated);
    assert_eq!(fs::read_to_string(&file).expect("read source"), source);
}

fn request_for_source(
    start_byte: usize,
    expected_text: &str,
    next_value: &str,
) -> DxStyleChangeRequest {
    DxStyleChangeRequest {
        breakpoint_label: Some("md".to_string()),
        viewport_width: None,
        ..DxStyleChangeRequest::new(
            DxStyleOperationName::StyleApply,
            "padding",
            next_value,
            Some(DxSourceTarget::known_exact(
                "styles/app.css",
                DxSourceTargetKind::AuthoredCss,
                DxSourceRange {
                    start_byte,
                    end_byte: start_byte + expected_text.len(),
                    expected_text: expected_text.to_string(),
                },
            )),
            computed_css("padding", "8px"),
            valid_box_model(),
        )
    }
}

fn computed_css(property: &str, value: &str) -> DxComputedCssPayload {
    DxComputedCssPayload {
        properties: BTreeMap::from([(property.to_string(), value.to_string())]),
    }
}

fn valid_box_model() -> DxBoxModelPayload {
    DxBoxModelPayload {
        content: DxBoxRect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 50.0,
        },
        padding: DxBoxEdges {
            top: 8.0,
            right: 8.0,
            bottom: 8.0,
            left: 8.0,
        },
        border: DxBoxEdges {
            top: 1.0,
            right: 1.0,
            bottom: 1.0,
            left: 1.0,
        },
        margin: DxBoxEdges {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        },
    }
}

fn temp_project(prefix: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    std::env::temp_dir().join(format!("dx-www-{prefix}-{nonce}"))
}
