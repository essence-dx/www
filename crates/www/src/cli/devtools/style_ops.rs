use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub(crate) const ELEMENT_SELECTION_SCHEMA: &str = "dx.visual_edit.element_selection";
pub(crate) const STYLE_OPERATION_SCHEMA: &str = "dx.visual_edit.style_operation";
pub(crate) const STYLE_PREVIEW_OPERATION: &str = "style-preview";
pub(crate) const STYLE_APPLY_OPERATION: &str = "style-apply";
pub(crate) const STYLE_UNDO_OPERATION: &str = "style-undo";

const FORBIDDEN_SOURCE_SEGMENTS: [&str; 4] = [".dx", "node_modules", "target", "dist"];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DxElementSelectionPayload {
    pub schema: String,
    pub format: u16,
    pub selected_element: DxSelectedElement,
    pub closest_parent_chain: Vec<DxClosestParentChainNode>,
    pub computed_css: DxComputedCssPayload,
    pub box_model: DxBoxModelPayload,
    pub breakpoint_label: String,
    pub source: DxSourceWriteEligibility,
}

impl DxElementSelectionPayload {
    pub(crate) fn new(
        selected_element: DxSelectedElement,
        closest_parent_chain: Vec<DxClosestParentChainNode>,
        computed_css: DxComputedCssPayload,
        box_model: DxBoxModelPayload,
        viewport_width: u32,
        source_target: Option<DxSourceTarget>,
    ) -> Self {
        Self {
            schema: ELEMENT_SELECTION_SCHEMA.to_string(),
            format: 1,
            selected_element,
            closest_parent_chain,
            computed_css,
            box_model,
            breakpoint_label: breakpoint_label_for_width(viewport_width).to_string(),
            source: source_write_eligibility(source_target.as_ref()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DxSelectedElement {
    pub stable_id: String,
    pub selector: String,
    pub tag_name: String,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: BTreeMap<String, String>,
    pub source_target: Option<DxSourceTarget>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DxClosestParentChainNode {
    pub depth: u16,
    pub selector: String,
    pub tag_name: String,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub data_dx_component: Option<String>,
    pub data_dx_editable_section: Option<String>,
    pub source_target: Option<DxSourceTarget>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DxComputedCssPayload {
    pub properties: BTreeMap<String, String>,
}

impl DxComputedCssPayload {
    pub(crate) fn property_value(&self, property: &str) -> Option<&str> {
        let normalized = normalize_css_property_name(property)?;
        self.properties
            .get(&normalized)
            .or_else(|| self.properties.get(property))
            .map(String::as_str)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DxBoxModelPayload {
    pub content: DxBoxRect,
    pub padding: DxBoxEdges,
    pub border: DxBoxEdges,
    pub margin: DxBoxEdges,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DxBoxRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DxBoxEdges {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum DxStyleOperationName {
    StylePreview,
    StyleApply,
}

impl DxStyleOperationName {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::StylePreview => STYLE_PREVIEW_OPERATION,
            Self::StyleApply => STYLE_APPLY_OPERATION,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum DxSourceTargetKind {
    AuthoredCss,
    ComponentClassName,
    InlineStyleAttribute,
    DesignToken,
    GeneratedCss,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DxSourceTarget {
    pub relative_path: String,
    pub kind: DxSourceTargetKind,
    pub range: Option<DxSourceRange>,
}

impl DxSourceTarget {
    pub(crate) fn known_exact(
        relative_path: impl Into<String>,
        kind: DxSourceTargetKind,
        range: DxSourceRange,
    ) -> Self {
        Self {
            relative_path: relative_path.into(),
            kind,
            range: Some(range),
        }
    }

    pub(crate) fn preview_only(relative_path: impl Into<String>, kind: DxSourceTargetKind) -> Self {
        Self {
            relative_path: relative_path.into(),
            kind,
            range: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DxSourceRange {
    pub start_byte: usize,
    pub end_byte: usize,
    pub expected_text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum DxSourceWriteStatus {
    Writable,
    PreviewOnly,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DxSourceWriteEligibility {
    pub status: DxSourceWriteStatus,
    pub writable: bool,
    pub preview_only: bool,
    pub reason: String,
    pub relative_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DxStyleChangeRequest {
    pub schema: String,
    pub operation: DxStyleOperationName,
    pub property: String,
    pub value: String,
    pub breakpoint_label: Option<String>,
    pub viewport_width: Option<u32>,
    pub source_target: Option<DxSourceTarget>,
    pub computed_css: DxComputedCssPayload,
    pub box_model: DxBoxModelPayload,
}

impl DxStyleChangeRequest {
    pub(crate) fn new(
        operation: DxStyleOperationName,
        property: impl Into<String>,
        value: impl Into<String>,
        source_target: Option<DxSourceTarget>,
        computed_css: DxComputedCssPayload,
        box_model: DxBoxModelPayload,
    ) -> Self {
        Self {
            schema: STYLE_OPERATION_SCHEMA.to_string(),
            operation,
            property: property.into(),
            value: value.into(),
            breakpoint_label: None,
            viewport_width: None,
            source_target,
            computed_css,
            box_model,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DxStyleOperationValidation {
    pub valid: bool,
    pub reasons: Vec<String>,
    pub property: Option<String>,
    pub value: Option<String>,
    pub visual_property: Option<String>,
    pub breakpoint_label: String,
    pub current_computed_value: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum DxStyleChangeStatus {
    PreviewOnly,
    Applied,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DxStyleChangeOutcome {
    pub operation: String,
    pub status: DxStyleChangeStatus,
    pub preview_only: bool,
    pub writable: bool,
    pub mutated: bool,
    pub reason: String,
    pub validation: DxStyleOperationValidation,
    pub source: DxSourceWriteEligibility,
    pub preview_css: Option<String>,
    pub written_path: Option<String>,
}

pub(crate) fn breakpoint_label_for_width(width_px: u32) -> &'static str {
    match width_px {
        0..=639 => "base",
        640..=767 => "sm",
        768..=1023 => "md",
        1024..=1279 => "lg",
        1280..=1535 => "xl",
        _ => "2xl",
    }
}

pub(crate) fn source_write_eligibility(
    source_target: Option<&DxSourceTarget>,
) -> DxSourceWriteEligibility {
    let Some(target) = source_target else {
        return source_preview_only("unknown-source-location", None);
    };

    match validate_relative_source_path(&target.relative_path, target.kind) {
        Ok(relative_path) => {
            if target.kind == DxSourceTargetKind::Unknown {
                source_preview_only("unknown-source-location", Some(relative_path))
            } else if target.kind != DxSourceTargetKind::AuthoredCss {
                source_preview_only("source-kind-not-writable", Some(relative_path))
            } else if !is_authored_css_path(&relative_path) {
                source_preview_only("non-css-source-target-not-writable", Some(relative_path))
            } else if target.range.is_none() {
                source_preview_only("missing-exact-source-range", Some(relative_path))
            } else if target
                .range
                .as_ref()
                .is_some_and(|range| !looks_like_css_declaration(&range.expected_text))
            {
                source_preview_only("source-range-not-css-declaration", Some(relative_path))
            } else {
                DxSourceWriteEligibility {
                    status: DxSourceWriteStatus::Writable,
                    writable: true,
                    preview_only: false,
                    reason: "known-exact-source-target".to_string(),
                    relative_path: Some(relative_path),
                }
            }
        }
        Err(reason) => DxSourceWriteEligibility {
            status: DxSourceWriteStatus::Rejected,
            writable: false,
            preview_only: false,
            reason,
            relative_path: None,
        },
    }
}

pub(crate) fn validate_style_operation(
    request: &DxStyleChangeRequest,
) -> DxStyleOperationValidation {
    let mut reasons = Vec::new();
    let property = match normalize_css_property_name(&request.property) {
        Some(property) => Some(property),
        None => {
            reasons.push("invalid-css-property".to_string());
            None
        }
    };

    let value = match sanitize_css_value(&request.value) {
        Some(value) => Some(value),
        None => {
            reasons.push("invalid-css-value".to_string());
            None
        }
    };

    let visual_property = property
        .as_deref()
        .and_then(editable_visual_property)
        .map(str::to_string);
    if property.is_some() && visual_property.is_none() {
        reasons.push("unsupported-visual-property".to_string());
    }

    let current_computed_value = property
        .as_deref()
        .and_then(|property| request.computed_css.property_value(property))
        .map(str::to_string);
    if property
        .as_deref()
        .is_some_and(|property| !property.starts_with("--"))
        && current_computed_value.is_none()
    {
        reasons.push("computed-css-property-missing".to_string());
    }

    if !box_model_is_valid(&request.box_model) {
        reasons.push("invalid-box-model".to_string());
    }

    let breakpoint_label =
        normalized_breakpoint_label(request.breakpoint_label.as_deref(), request.viewport_width);
    if breakpoint_label.is_none() {
        reasons.push("invalid-breakpoint-label".to_string());
    }

    DxStyleOperationValidation {
        valid: reasons.is_empty(),
        reasons,
        property,
        value,
        visual_property,
        breakpoint_label: breakpoint_label.unwrap_or_else(|| "base".to_string()),
        current_computed_value,
    }
}

pub(crate) fn preview_style_change(request: &DxStyleChangeRequest) -> DxStyleChangeOutcome {
    let validation = validate_style_operation(request);
    let source = source_write_eligibility(request.source_target.as_ref());

    if !validation.valid {
        return rejected_outcome(
            request.operation,
            "style-operation-validation-failed",
            validation,
            source,
        );
    }

    if source.status == DxSourceWriteStatus::Rejected {
        return rejected_outcome(
            request.operation,
            "source-target-rejected",
            validation,
            source,
        );
    }

    DxStyleChangeOutcome {
        operation: request.operation.as_str().to_string(),
        status: DxStyleChangeStatus::PreviewOnly,
        preview_only: true,
        writable: false,
        mutated: false,
        reason: "preview-does-not-mutate-source".to_string(),
        preview_css: preview_declaration(&validation),
        validation,
        source: source_preview_only(source.reason.as_str(), source.relative_path),
        written_path: None,
    }
}

pub(crate) fn preview_style_change_json(request: &DxStyleChangeRequest) -> Value {
    style_change_outcome_json(preview_style_change(request))
}

pub(crate) fn apply_style_change(
    project_root: &Path,
    request: &DxStyleChangeRequest,
) -> io::Result<DxStyleChangeOutcome> {
    if request.operation == DxStyleOperationName::StylePreview {
        return Ok(preview_style_change(request));
    }

    let validation = validate_style_operation(request);
    let source = source_write_eligibility(request.source_target.as_ref());

    if !validation.valid {
        return Ok(rejected_outcome(
            request.operation,
            "style-operation-validation-failed",
            validation,
            source,
        ));
    }

    if source.status == DxSourceWriteStatus::Rejected {
        return Ok(rejected_outcome(
            request.operation,
            "source-target-rejected",
            validation,
            source,
        ));
    }

    if source.status == DxSourceWriteStatus::PreviewOnly {
        return Ok(DxStyleChangeOutcome {
            operation: request.operation.as_str().to_string(),
            status: DxStyleChangeStatus::PreviewOnly,
            preview_only: true,
            writable: false,
            mutated: false,
            reason: source.reason.clone(),
            preview_css: preview_declaration(&validation),
            validation,
            source,
            written_path: None,
        });
    }

    let Some(target) = request.source_target.as_ref() else {
        return Ok(rejected_outcome(
            request.operation,
            "missing-source-target",
            validation,
            source,
        ));
    };
    let Some(range) = target.range.as_ref() else {
        return Ok(rejected_outcome(
            request.operation,
            "missing-exact-source-range",
            validation,
            source,
        ));
    };
    let Some(relative_path) = source.relative_path.clone() else {
        return Ok(rejected_outcome(
            request.operation,
            "missing-normalized-source-path",
            validation,
            source,
        ));
    };
    let Some(project_path) = safe_project_path(project_root, &relative_path)? else {
        return Ok(rejected_outcome(
            request.operation,
            "source-path-escapes-project-root",
            validation,
            source,
        ));
    };

    let source_text = match fs::read_to_string(&project_path) {
        Ok(source_text) => source_text,
        Err(error) => {
            return Ok(rejected_outcome(
                request.operation,
                &format!("source-file-unreadable:{error}"),
                validation,
                source,
            ));
        }
    };

    let Some(current_text) = source_text.get(range.start_byte..range.end_byte) else {
        return Ok(rejected_outcome(
            request.operation,
            "exact-source-range-invalid",
            validation,
            source,
        ));
    };

    if current_text != range.expected_text {
        return Ok(rejected_outcome(
            request.operation,
            "exact-source-range-mismatch",
            validation,
            source,
        ));
    }

    if !declaration_property_matches_validation(&range.expected_text, &validation) {
        return Ok(rejected_outcome(
            request.operation,
            "source-range-property-mismatch",
            validation,
            source,
        ));
    }

    let replacement = replacement_declaration(&validation, &range.expected_text);
    let Some(prefix) = source_text.get(..range.start_byte) else {
        return Ok(rejected_outcome(
            request.operation,
            "exact-source-range-invalid",
            validation,
            source,
        ));
    };
    let Some(suffix) = source_text.get(range.end_byte..) else {
        return Ok(rejected_outcome(
            request.operation,
            "exact-source-range-invalid",
            validation,
            source,
        ));
    };

    let mut updated = String::with_capacity(
        source_text
            .len()
            .saturating_sub(current_text.len())
            .saturating_add(replacement.len()),
    );
    updated.push_str(prefix);
    updated.push_str(&replacement);
    updated.push_str(suffix);
    fs::write(&project_path, updated)?;

    Ok(DxStyleChangeOutcome {
        operation: request.operation.as_str().to_string(),
        status: DxStyleChangeStatus::Applied,
        preview_only: false,
        writable: true,
        mutated: true,
        reason: "applied-exact-source-range".to_string(),
        preview_css: preview_declaration(&validation),
        validation,
        source,
        written_path: Some(relative_path),
    })
}

pub(crate) fn apply_style_change_json(
    project_root: &Path,
    request: &DxStyleChangeRequest,
) -> Value {
    match apply_style_change(project_root, request) {
        Ok(outcome) => style_change_outcome_json(outcome),
        Err(error) => style_change_outcome_json(DxStyleChangeOutcome {
            operation: request.operation.as_str().to_string(),
            status: DxStyleChangeStatus::Rejected,
            preview_only: false,
            writable: false,
            mutated: false,
            reason: format!("source-write-error:{error}"),
            validation: validate_style_operation(request),
            source: source_write_eligibility(request.source_target.as_ref()),
            preview_css: None,
            written_path: None,
        }),
    }
}

pub(crate) fn undo_style_change_json(project_root: &Path, receipt: &Value) -> Value {
    match undo_style_change(project_root, receipt) {
        Ok(outcome) => outcome,
        Err(error) => json!({
            "schema": "dx.visual_edit.undo_operation",
            "operation": STYLE_UNDO_OPERATION,
            "status": "rejected",
            "undone": false,
            "mutated": false,
            "reason": format!("undo-source-write-error:{error}"),
        }),
    }
}

fn undo_style_change(project_root: &Path, receipt: &Value) -> io::Result<Value> {
    let Some(patch) = receipt.get("undo_patch").and_then(Value::as_object) else {
        return Ok(style_undo_rejected("missing-undo-patch", None));
    };
    let Some(source_path) = patch.get("source_path").and_then(Value::as_str) else {
        return Ok(style_undo_rejected("missing-undo-source-path", None));
    };
    let Some(start_byte) = patch
        .get("start_byte")
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
    else {
        return Ok(style_undo_rejected(
            "missing-undo-start-byte",
            Some(source_path),
        ));
    };
    let Some(end_byte) = patch
        .get("end_byte")
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
    else {
        return Ok(style_undo_rejected(
            "missing-undo-end-byte",
            Some(source_path),
        ));
    };
    let Some(expected_text_after) = patch.get("expected_text_after").and_then(Value::as_str) else {
        return Ok(style_undo_rejected(
            "missing-undo-expected-text-after",
            Some(source_path),
        ));
    };
    let Some(restore_text_before) = patch.get("restore_text_before").and_then(Value::as_str) else {
        return Ok(style_undo_rejected(
            "missing-undo-restore-text-before",
            Some(source_path),
        ));
    };

    let relative_path =
        match validate_relative_source_path(source_path, DxSourceTargetKind::AuthoredCss) {
            Ok(relative_path) if is_authored_css_path(&relative_path) => relative_path,
            Ok(relative_path) => {
                return Ok(style_undo_rejected(
                    "undo-source-target-not-authored-css",
                    Some(&relative_path),
                ));
            }
            Err(reason) => return Ok(style_undo_rejected(&reason, Some(source_path))),
        };

    if !looks_like_css_declaration(expected_text_after)
        || !looks_like_css_declaration(restore_text_before)
    {
        return Ok(style_undo_rejected(
            "undo-patch-not-css-declaration",
            Some(&relative_path),
        ));
    }
    if let Some(property) = patch.get("property").and_then(Value::as_str) {
        let normalized = normalize_css_property_name(property);
        if normalized.as_deref() != declaration_property_name(expected_text_after).as_deref()
            || normalized.as_deref() != declaration_property_name(restore_text_before).as_deref()
        {
            return Ok(style_undo_rejected(
                "undo-patch-property-mismatch",
                Some(&relative_path),
            ));
        }
    }

    let Some(project_path) = safe_project_path(project_root, &relative_path)? else {
        return Ok(style_undo_rejected(
            "undo-source-path-escapes-project-root",
            Some(&relative_path),
        ));
    };
    let source_text = fs::read_to_string(&project_path)?;
    let Some(current_text) = source_text.get(start_byte..end_byte) else {
        return Ok(style_undo_rejected(
            "undo-source-range-invalid",
            Some(&relative_path),
        ));
    };
    if current_text != expected_text_after {
        return Ok(style_undo_rejected(
            "undo-source-range-mismatch",
            Some(&relative_path),
        ));
    }
    let Some(prefix) = source_text.get(..start_byte) else {
        return Ok(style_undo_rejected(
            "undo-source-range-invalid",
            Some(&relative_path),
        ));
    };
    let Some(suffix) = source_text.get(end_byte..) else {
        return Ok(style_undo_rejected(
            "undo-source-range-invalid",
            Some(&relative_path),
        ));
    };

    let mut updated = String::with_capacity(
        source_text
            .len()
            .saturating_sub(current_text.len())
            .saturating_add(restore_text_before.len()),
    );
    updated.push_str(prefix);
    updated.push_str(restore_text_before);
    updated.push_str(suffix);
    fs::write(&project_path, updated)?;

    Ok(json!({
        "schema": "dx.visual_edit.undo_operation",
        "operation": STYLE_UNDO_OPERATION,
        "status": "undone",
        "undone": true,
        "mutated": true,
        "reason": "undone-exact-source-range",
        "source_path": relative_path,
        "range": {
            "startByte": start_byte,
            "endByte": start_byte + restore_text_before.len(),
            "expectedTextAfter": expected_text_after,
            "restoreTextBefore": restore_text_before,
        },
    }))
}

fn style_undo_rejected(reason: &str, source_path: Option<&str>) -> Value {
    json!({
        "schema": "dx.visual_edit.undo_operation",
        "operation": STYLE_UNDO_OPERATION,
        "status": "rejected",
        "undone": false,
        "mutated": false,
        "reason": reason,
        "source_path": source_path,
    })
}

fn style_change_outcome_json(outcome: DxStyleChangeOutcome) -> Value {
    match serde_json::to_value(outcome) {
        Ok(value) => value,
        Err(error) => json!({
            "operation": "style-operation",
            "status": "rejected",
            "previewOnly": false,
            "writable": false,
            "mutated": false,
            "reason": format!("style-operation-serialization-error:{error}"),
            "validation": {
                "valid": false,
                "reasons": ["style-operation-serialization-error"],
                "property": null,
                "value": null,
                "visualProperty": null,
                "breakpointLabel": "base",
                "currentComputedValue": null
            },
            "source": {
                "status": "rejected",
                "writable": false,
                "previewOnly": false,
                "reason": "style-operation-serialization-error",
                "relativePath": null
            },
            "previewCss": null,
            "writtenPath": null
        }),
    }
}

fn source_preview_only(reason: &str, relative_path: Option<String>) -> DxSourceWriteEligibility {
    DxSourceWriteEligibility {
        status: DxSourceWriteStatus::PreviewOnly,
        writable: false,
        preview_only: true,
        reason: reason.to_string(),
        relative_path,
    }
}

fn rejected_outcome(
    operation: DxStyleOperationName,
    reason: &str,
    validation: DxStyleOperationValidation,
    source: DxSourceWriteEligibility,
) -> DxStyleChangeOutcome {
    DxStyleChangeOutcome {
        operation: operation.as_str().to_string(),
        status: DxStyleChangeStatus::Rejected,
        preview_only: false,
        writable: false,
        mutated: false,
        reason: reason.to_string(),
        preview_css: preview_declaration(&validation),
        validation,
        source,
        written_path: None,
    }
}

fn validate_relative_source_path(
    relative_path: &str,
    kind: DxSourceTargetKind,
) -> Result<String, String> {
    if kind == DxSourceTargetKind::GeneratedCss {
        return Err("generated-css-not-writable".to_string());
    }

    let candidate = relative_path.trim().replace('\\', "/");
    if candidate.is_empty() {
        return Err("empty-source-path".to_string());
    }
    if looks_absolute_or_drive_prefixed(&candidate) {
        return Err("absolute-source-path-not-allowed".to_string());
    }
    if candidate.contains(':') {
        return Err("source-path-with-drive-or-stream-prefix-not-allowed".to_string());
    }

    let mut normalized = Vec::new();
    for component in Path::new(&candidate).components() {
        match component {
            Component::Normal(segment) => {
                let segment = segment.to_string_lossy();
                let segment_lower = segment.to_ascii_lowercase();
                if FORBIDDEN_SOURCE_SEGMENTS.contains(&segment_lower.as_str()) {
                    return Err(format!("forbidden-source-segment:{segment_lower}"));
                }
                normalized.push(segment.to_string());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                return Err("parent-traversal-not-allowed".to_string());
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err("absolute-source-path-not-allowed".to_string());
            }
        }
    }

    if normalized.is_empty() {
        return Err("empty-source-path".to_string());
    }

    let normalized = normalized.join("/");
    if is_generated_css_path(&normalized) {
        return Err("generated-css-not-writable".to_string());
    }

    Ok(normalized)
}

fn looks_absolute_or_drive_prefixed(path: &str) -> bool {
    path.starts_with('/')
        || path.starts_with('\\')
        || path.starts_with("//")
        || path.as_bytes().get(1).is_some_and(|second| *second == b':')
        || Path::new(path).is_absolute()
}

fn is_generated_css_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.ends_with("/generated.css")
        || lower == "generated.css"
        || lower.ends_with(".generated.css")
        || lower.contains("/generated/")
}

fn is_authored_css_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.ends_with(".css")
        || lower.ends_with(".dx.css")
        || lower.ends_with(".dx-style")
        || lower.ends_with(".dxstyle")
}

fn looks_like_css_declaration(source: &str) -> bool {
    let trimmed = source.trim();
    if trimmed.is_empty()
        || trimmed.contains('{')
        || trimmed.contains('}')
        || trimmed.contains('<')
        || trimmed.contains('>')
        || trimmed.starts_with('@')
    {
        return false;
    }
    let Some((property, value)) = trimmed.trim_end_matches(';').split_once(':') else {
        return false;
    };
    normalize_css_property_name(property).is_some() && !value.trim().is_empty()
}

fn normalize_css_property_name(property: &str) -> Option<String> {
    let trimmed = property.trim();
    if trimmed.is_empty() || trimmed.len() > 96 {
        return None;
    }
    if trimmed.starts_with("--") {
        let valid = trimmed
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'));
        return valid.then(|| trimmed.to_string());
    }

    let normalized = trimmed.to_ascii_lowercase();
    let valid = normalized
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
        && normalized.chars().any(|ch| ch.is_ascii_lowercase());
    valid.then_some(normalized)
}

fn sanitize_css_value(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.len() > 512 {
        return None;
    }
    let lower = trimmed.to_ascii_lowercase();
    if trimmed
        .chars()
        .any(|ch| ch.is_control() && !matches!(ch, '\t' | '\n' | '\r'))
        || trimmed.contains(';')
        || trimmed.contains('{')
        || trimmed.contains('}')
        || trimmed.contains('<')
        || trimmed.contains('>')
        || lower.contains("javascript:")
        || lower.contains("expression(")
        || lower.contains("@import")
    {
        return None;
    }

    Some(trimmed.to_string())
}

fn editable_visual_property(property: &str) -> Option<&'static str> {
    if property.starts_with("--") {
        Some("token")
    } else if matches!(
        property,
        "margin" | "margin-top" | "margin-right" | "margin-bottom" | "margin-left"
    ) || matches!(
        property,
        "padding"
            | "padding-top"
            | "padding-right"
            | "padding-bottom"
            | "padding-left"
            | "gap"
            | "row-gap"
            | "column-gap"
            | "inset"
            | "top"
            | "right"
            | "bottom"
            | "left"
    ) {
        Some("spacing")
    } else if matches!(
        property,
        "width"
            | "height"
            | "min-width"
            | "min-height"
            | "max-width"
            | "max-height"
            | "aspect-ratio"
    ) {
        Some("size")
    } else if property.contains("color")
        || matches!(
            property,
            "background"
                | "background-color"
                | "background-image"
                | "background-position"
                | "background-size"
                | "background-repeat"
        )
    {
        Some("color")
    } else if property == "border-radius" {
        Some("radius")
    } else if property.starts_with("border") || property.starts_with("outline") {
        Some("border")
    } else if matches!(
        property,
        "display"
            | "position"
            | "grid-template-columns"
            | "grid-template-rows"
            | "flex"
            | "flex-direction"
            | "flex-wrap"
            | "align-items"
            | "align-content"
            | "justify-content"
            | "place-items"
            | "overflow"
            | "overflow-x"
            | "overflow-y"
            | "z-index"
            | "order"
            | "visibility"
            | "object-fit"
    ) {
        Some("layout")
    } else if property.starts_with("font")
        || matches!(
            property,
            "line-height"
                | "letter-spacing"
                | "text-align"
                | "text-decoration"
                | "text-transform"
                | "white-space"
        )
    {
        Some("typography")
    } else if matches!(
        property,
        "opacity" | "filter" | "backdrop-filter" | "-webkit-backdrop-filter" | "box-shadow"
    ) {
        Some("effect")
    } else if property == "transform"
        || property.starts_with("translate")
        || property.starts_with("rotate")
        || property.starts_with("scale")
    {
        Some("transform")
    } else if property.starts_with("transition") || property.starts_with("animation") {
        Some("motion")
    } else {
        None
    }
}

fn box_model_is_valid(box_model: &DxBoxModelPayload) -> bool {
    rect_is_valid(&box_model.content)
        && non_negative_edges_are_valid(&box_model.padding)
        && non_negative_edges_are_valid(&box_model.border)
        && finite_edges_are_valid(&box_model.margin)
}

fn rect_is_valid(rect: &DxBoxRect) -> bool {
    rect.x.is_finite()
        && rect.y.is_finite()
        && rect.width.is_finite()
        && rect.height.is_finite()
        && rect.width >= 0.0
        && rect.height >= 0.0
}

fn non_negative_edges_are_valid(edges: &DxBoxEdges) -> bool {
    finite_edges_are_valid(edges)
        && edges.top >= 0.0
        && edges.right >= 0.0
        && edges.bottom >= 0.0
        && edges.left >= 0.0
}

fn finite_edges_are_valid(edges: &DxBoxEdges) -> bool {
    edges.top.is_finite()
        && edges.right.is_finite()
        && edges.bottom.is_finite()
        && edges.left.is_finite()
}

fn normalized_breakpoint_label(label: Option<&str>, viewport_width: Option<u32>) -> Option<String> {
    let label = label.map(str::trim).filter(|label| !label.is_empty());
    if let Some(label) = label {
        let known = matches!(label, "base" | "sm" | "md" | "lg" | "xl" | "2xl");
        let custom = label.starts_with("custom:")
            && label["custom:".len()..]
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'));
        if known || custom {
            return Some(label.to_string());
        }
        return None;
    }

    Some(breakpoint_label_for_width(viewport_width.unwrap_or(0)).to_string())
}

fn preview_declaration(validation: &DxStyleOperationValidation) -> Option<String> {
    Some(format!(
        "{}: {};",
        validation.property.as_ref()?,
        validation.value.as_ref()?
    ))
}

fn replacement_declaration(validation: &DxStyleOperationValidation, expected_text: &str) -> String {
    let indent = expected_text
        .chars()
        .take_while(|ch| ch.is_whitespace())
        .collect::<String>();
    let semicolon = if expected_text.trim_end().ends_with(';') {
        ";"
    } else {
        ""
    };
    format!(
        "{}{}: {}{}",
        indent,
        validation.property.as_deref().unwrap_or_default(),
        validation.value.as_deref().unwrap_or_default(),
        semicolon
    )
}

fn declaration_property_matches_validation(
    expected_text: &str,
    validation: &DxStyleOperationValidation,
) -> bool {
    let Some(expected_property) = declaration_property_name(expected_text) else {
        return false;
    };
    validation
        .property
        .as_deref()
        .is_some_and(|property| property == expected_property)
}

fn declaration_property_name(source: &str) -> Option<String> {
    source
        .trim()
        .trim_end_matches(';')
        .split_once(':')
        .and_then(|(property, _)| normalize_css_property_name(property))
}

fn safe_project_path(project_root: &Path, relative_path: &str) -> io::Result<Option<PathBuf>> {
    let root = fs::canonicalize(project_root)?;
    let candidate = root.join(relative_path);
    if candidate.exists() {
        let canonical_candidate = fs::canonicalize(&candidate)?;
        if !canonical_candidate.starts_with(&root) {
            return Ok(None);
        }
    }
    let Some(parent) = candidate.parent() else {
        return Ok(None);
    };
    let parent = match fs::canonicalize(parent) {
        Ok(parent) => parent,
        Err(_) => return Ok(None),
    };
    if !parent.starts_with(&root) {
        return Ok(None);
    }
    Ok(Some(candidate))
}

#[cfg(test)]
#[path = "style_ops_tests.rs"]
mod style_ops_tests;
