use std::collections::BTreeSet;

use super::encoding::{DxPacketEncoder, gzip_len};
use super::types::{DxDeliveryEstimates, DxDeliveryMode, DxDeliveryPlan, DxShapeStats};
use crate::analyzer::{ComplexityMetrics, RuntimeVariant, StateComplexity};
use crate::splitter::{Binding, BindingFlag, StateSchema, Template};

/// Build a delivery plan from the current compiler output.
pub fn plan_delivery(
    metrics: &ComplexityMetrics,
    runtime_variant: RuntimeVariant,
    templates: &[Template],
    bindings: &[Binding],
    schemas: &[StateSchema],
    htip_bytes: usize,
) -> DxDeliveryPlan {
    let shape = shape_stats(templates, bindings, schemas);
    let template_packet = DxPacketEncoder::encode_template_slots(templates);
    let estimates = estimate_delivery(metrics, &shape, &template_packet, htip_bytes);
    let (primary_mode, mut reasons) = choose_mode(metrics, &shape);
    let fallback_modes = fallback_modes(primary_mode, runtime_variant, metrics);
    let mut warnings = warnings_for(metrics, primary_mode, &estimates);

    if primary_mode == DxDeliveryMode::SemanticCodec {
        warnings.push(
            "semantic-codec requires compiler proof; otherwise fall back to columnar-slots"
                .to_string(),
        );
    }

    reasons.push(format!(
        "selected {} from {} templates, {} slots, {} bindings",
        primary_mode.as_str(),
        shape.template_count,
        shape.slot_count,
        bindings.len()
    ));

    DxDeliveryPlan {
        primary_mode,
        runtime_variant,
        fallback_modes,
        shape,
        estimates,
        reasons,
        warnings,
    }
}

fn shape_stats(
    templates: &[Template],
    bindings: &[Binding],
    schemas: &[StateSchema],
) -> DxShapeStats {
    let mut hashes = BTreeSet::new();
    let mut repeated_template_count = 0usize;
    let mut slot_count = 0usize;
    for template in templates {
        slot_count += template.slots.len();
        if !hashes.insert(template.hash.clone()) {
            repeated_template_count += 1;
        }
    }

    let state_counts = state_field_counts(schemas);
    let iteration_binding_count = count_bindings(bindings, BindingFlag::Iteration);
    let conditional_binding_count = count_bindings(bindings, BindingFlag::Conditional);
    let has_columnar_shape =
        iteration_binding_count > 0 || repeated_template_count > 0 || slot_count >= 16;
    let has_semantic_shape =
        state_counts.numeric > 0 || state_counts.boolean > 0 || has_enum_like_bindings(bindings);

    DxShapeStats {
        template_count: templates.len(),
        unique_template_count: hashes.len(),
        repeated_template_count,
        slot_count,
        iteration_binding_count,
        conditional_binding_count,
        state_field_count: state_counts.total,
        numeric_state_field_count: state_counts.numeric,
        boolean_state_field_count: state_counts.boolean,
        has_columnar_shape,
        has_semantic_shape,
    }
}

#[derive(Default)]
struct StateFieldCounts {
    total: usize,
    numeric: usize,
    boolean: usize,
}

fn state_field_counts(schemas: &[StateSchema]) -> StateFieldCounts {
    let mut counts = StateFieldCounts::default();
    for schema in schemas {
        for field in &schema.fields {
            counts.total += 1;
            let ty = field.type_name.to_ascii_lowercase();
            if is_number_type(&ty) {
                counts.numeric += 1;
            }
            if ty == "bool" || ty == "boolean" {
                counts.boolean += 1;
            }
        }
    }
    counts
}

fn count_bindings(bindings: &[Binding], flag: BindingFlag) -> usize {
    bindings
        .iter()
        .filter(|binding| binding.flag == flag)
        .count()
}

fn estimate_delivery(
    metrics: &ComplexityMetrics,
    shape: &DxShapeStats,
    template_packet: &[u8],
    htip_bytes: usize,
) -> DxDeliveryEstimates {
    let static_html_bytes = metrics
        .total_jsx_nodes
        .saturating_mul(42)
        .max(template_packet.len() / 2);
    let micro_js_bytes = micro_js_bytes(metrics);
    let columnar_slot_bytes = if shape.has_columnar_shape {
        (template_packet.len() / 2).max(32)
    } else {
        template_packet.len()
    };
    let semantic_codec_bytes = if shape.has_semantic_shape {
        24 + shape.state_field_count * 8 + shape.conditional_binding_count * 4
    } else {
        columnar_slot_bytes
    };

    DxDeliveryEstimates {
        static_html_bytes,
        htip_bytes,
        template_slot_bytes: template_packet.len(),
        template_slot_gzip_bytes: gzip_len(template_packet),
        columnar_slot_bytes,
        semantic_codec_bytes,
        viewport_packet_bytes: viewport_packet_bytes(shape, static_html_bytes),
        patch_stream_bytes: patch_stream_bytes(metrics, shape),
        range_op_bytes: range_op_bytes(shape),
        micro_js_bytes,
        wasm_runtime_bytes: wasm_runtime_bytes(metrics),
    }
}

fn micro_js_bytes(metrics: &ComplexityMetrics) -> usize {
    if metrics.event_handler_count == 0 {
        0
    } else {
        96 + metrics.event_handler_count * 48 + metrics.total_state_vars * 18
    }
}

fn viewport_packet_bytes(shape: &DxShapeStats, static_html_bytes: usize) -> usize {
    if shape.has_columnar_shape {
        96 + shape.slot_count.min(40) * 12
    } else {
        static_html_bytes
    }
}

fn patch_stream_bytes(metrics: &ComplexityMetrics, shape: &DxShapeStats) -> usize {
    if metrics.event_handler_count > 0 || shape.iteration_binding_count > 0 {
        16 + metrics.event_handler_count * 10 + shape.slot_count.min(24) * 4
    } else {
        0
    }
}

fn range_op_bytes(shape: &DxShapeStats) -> usize {
    if shape.has_columnar_shape || shape.has_semantic_shape {
        10
    } else {
        0
    }
}

fn wasm_runtime_bytes(metrics: &ComplexityMetrics) -> usize {
    match metrics.state_complexity {
        StateComplexity::Low if metrics.event_handler_count <= 2 => 0,
        StateComplexity::Low | StateComplexity::Medium => 1_500,
        StateComplexity::High => 7_500,
    }
}

fn choose_mode(metrics: &ComplexityMetrics, shape: &DxShapeStats) -> (DxDeliveryMode, Vec<String>) {
    let mut reasons = Vec::new();

    if metrics.event_handler_count == 0
        && metrics.total_state_vars == 0
        && shape.iteration_binding_count == 0
    {
        reasons.push("no events, state, or list bindings; no runtime needed".to_string());
        return (DxDeliveryMode::Static, reasons);
    }

    if metrics.event_handler_count <= 3
        && metrics.state_complexity != StateComplexity::High
        && metrics.total_jsx_nodes <= 24
    {
        reasons.push("tiny interaction can avoid the shared runtime with generated JS".to_string());
        return (DxDeliveryMode::MicroJs, reasons);
    }

    if shape.has_semantic_shape && shape.has_columnar_shape {
        reasons.push(
            "repeated data has numeric/enum-like shape; semantic codec can dominate".to_string(),
        );
        return (DxDeliveryMode::SemanticCodec, reasons);
    }

    if shape.has_columnar_shape {
        reasons.push(
            "repeated slots/list data detected; columnar packets reduce duplicate structure"
                .to_string(),
        );
        return (DxDeliveryMode::ColumnarSlots, reasons);
    }

    if metrics.has_async_logic && metrics.total_hooks <= 2 {
        reasons.push(
            "server-owned async flow may be cheaper as fragments than full client state"
                .to_string(),
        );
        return (DxDeliveryMode::ServerFragment, reasons);
    }

    if metrics.state_complexity == StateComplexity::High || metrics.event_handler_count >= 10 {
        reasons.push("complex client state requires the WASM core runtime".to_string());
        return (DxDeliveryMode::WasmCore, reasons);
    }

    reasons.push("dynamic page without strong columnar shape uses template slots".to_string());
    (DxDeliveryMode::TemplateSlots, reasons)
}

fn fallback_modes(
    primary: DxDeliveryMode,
    runtime_variant: RuntimeVariant,
    metrics: &ComplexityMetrics,
) -> Vec<DxDeliveryMode> {
    let mut modes = base_fallback_modes(primary);
    if runtime_variant == RuntimeVariant::Macro || metrics.total_hooks > 3 {
        modes.push(DxDeliveryMode::WasmSplit);
    }
    dedupe_modes(modes)
}

fn base_fallback_modes(primary: DxDeliveryMode) -> Vec<DxDeliveryMode> {
    match primary {
        DxDeliveryMode::Static => vec![DxDeliveryMode::Static],
        DxDeliveryMode::MicroJs => vec![DxDeliveryMode::MicroJs, DxDeliveryMode::TemplateSlots],
        DxDeliveryMode::SemanticCodec => vec![
            DxDeliveryMode::SemanticCodec,
            DxDeliveryMode::ColumnarSlots,
            DxDeliveryMode::TemplateSlots,
        ],
        DxDeliveryMode::ColumnarSlots => {
            vec![DxDeliveryMode::ColumnarSlots, DxDeliveryMode::TemplateSlots]
        }
        DxDeliveryMode::PatchStream | DxDeliveryMode::RangeOp => {
            vec![primary, DxDeliveryMode::TemplateSlots]
        }
        DxDeliveryMode::ServerFragment => {
            vec![
                DxDeliveryMode::ServerFragment,
                DxDeliveryMode::TemplateSlots,
            ]
        }
        DxDeliveryMode::TemplateSlots => vec![DxDeliveryMode::TemplateSlots],
        DxDeliveryMode::WasmCore | DxDeliveryMode::WasmSplit => {
            vec![primary, DxDeliveryMode::TemplateSlots]
        }
    }
}

fn warnings_for(
    metrics: &ComplexityMetrics,
    primary_mode: DxDeliveryMode,
    estimates: &DxDeliveryEstimates,
) -> Vec<String> {
    let mut warnings = Vec::new();
    if primary_mode != DxDeliveryMode::Static && estimates.wasm_runtime_bytes > 0 {
        warnings.push("runtime bytes must be counted in production benchmarks".to_string());
    }
    if metrics.total_jsx_nodes <= 10 && primary_mode != DxDeliveryMode::Static {
        warnings.push(
            "tiny pages can still lose if the compiler ships unnecessary runtime".to_string(),
        );
    }
    warnings
}

fn dedupe_modes(modes: Vec<DxDeliveryMode>) -> Vec<DxDeliveryMode> {
    let mut seen = BTreeSet::new();
    let mut deduped = Vec::new();
    for mode in modes {
        if seen.insert(mode.as_str()) {
            deduped.push(mode);
        }
    }
    deduped
}

fn has_enum_like_bindings(bindings: &[Binding]) -> bool {
    let enum_markers = ["status", "kind", "type", "variant", "plan", "role", "state"];
    bindings.iter().any(|binding| {
        let expression = binding.expression.to_ascii_lowercase();
        enum_markers
            .iter()
            .any(|marker| expression.contains(marker))
    })
}

fn is_number_type(ty: &str) -> bool {
    matches!(
        ty,
        "number"
            | "int"
            | "integer"
            | "usize"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "f32"
            | "f64"
    )
}
