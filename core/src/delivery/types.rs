use serde::{Deserialize, Serialize};

use crate::analyzer::RuntimeVariant;

/// Compiler-selected delivery mode for a page or interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DxDeliveryMode {
    /// HTML and CSS only.
    Static,
    /// A small generated JavaScript instruction set.
    MicroJs,
    /// Cached templates plus dynamic slot values.
    TemplateSlots,
    /// Repeated slot values encoded by column.
    ColumnarSlots,
    /// Proven semantic pattern such as a range, enum, prefix, date, or price.
    SemanticCodec,
    /// Fine-grained live update operations.
    PatchStream,
    /// Compact range operation for bulk updates.
    RangeOp,
    /// Server-rendered fragment workflow.
    ServerFragment,
    /// WASM runtime core.
    WasmCore,
    /// Split WASM runtime loaded by feature.
    WasmSplit,
}

impl DxDeliveryMode {
    /// Stable manifest label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Static => "static",
            Self::MicroJs => "js",
            Self::TemplateSlots => "template-slots",
            Self::ColumnarSlots => "columnar-slots",
            Self::SemanticCodec => "semantic-codec",
            Self::PatchStream => "patch-stream",
            Self::RangeOp => "range-op",
            Self::ServerFragment => "server-fragment",
            Self::WasmCore => "wasm-core",
            Self::WasmSplit => "wasm-split",
        }
    }
}

/// Byte estimates for candidate delivery modes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DxDeliveryEstimates {
    /// Approximate static template HTML bytes.
    pub static_html_bytes: usize,
    /// Current HTIP stream bytes.
    pub htip_bytes: usize,
    /// DX template-slot packet bytes.
    pub template_slot_bytes: usize,
    /// Gzip size for the template-slot packet.
    pub template_slot_gzip_bytes: usize,
    /// Estimated columnar packet bytes.
    pub columnar_slot_bytes: usize,
    /// Estimated semantic codec bytes.
    pub semantic_codec_bytes: usize,
    /// Estimated first viewport packet bytes.
    pub viewport_packet_bytes: usize,
    /// Estimated patch-stream bytes for a small live update.
    pub patch_stream_bytes: usize,
    /// Estimated range-op bytes for a bulk update.
    pub range_op_bytes: usize,
    /// Estimated generated JavaScript bytes.
    pub micro_js_bytes: usize,
    /// Estimated WASM boot/runtime bytes.
    pub wasm_runtime_bytes: usize,
}

/// Structural facts extracted from compiler output.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DxShapeStats {
    /// Number of compiled templates.
    pub template_count: usize,
    /// Number of unique template hashes.
    pub unique_template_count: usize,
    /// Number of templates that reused a known hash.
    pub repeated_template_count: usize,
    /// Total dynamic slots.
    pub slot_count: usize,
    /// Number of bindings generated from list iteration.
    pub iteration_binding_count: usize,
    /// Number of conditional bindings.
    pub conditional_binding_count: usize,
    /// Number of state fields.
    pub state_field_count: usize,
    /// State fields with number-like types.
    pub numeric_state_field_count: usize,
    /// State fields with boolean-like types.
    pub boolean_state_field_count: usize,
    /// Whether the compiler sees data that can become columnar.
    pub has_columnar_shape: bool,
    /// Whether the compiler sees data likely to support semantic codecs.
    pub has_semantic_shape: bool,
}

/// Final delivery decision manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxDeliveryPlan {
    /// Primary selected mode.
    pub primary_mode: DxDeliveryMode,
    /// Existing Rust runtime decision retained for compatibility.
    pub runtime_variant: RuntimeVariant,
    /// Ordered fallback modes the runtime/build can use safely.
    pub fallback_modes: Vec<DxDeliveryMode>,
    /// Extracted structural facts.
    pub shape: DxShapeStats,
    /// Candidate byte estimates.
    pub estimates: DxDeliveryEstimates,
    /// Human-readable reasons for the decision.
    pub reasons: Vec<String>,
    /// Honest warnings/caveats.
    pub warnings: Vec<String>,
}

/// Small JavaScript state program for no-WASM interactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxMicroJsProgram {
    /// Initial numeric state.
    pub initial_value: i64,
    /// DOM id whose text mirrors the state.
    pub target_id: String,
    /// Element-bound actions.
    pub actions: Vec<DxMicroJsAction>,
}

/// One generated JS event action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxMicroJsAction {
    /// DOM id for the triggering element.
    pub element_id: String,
    /// Native DOM event name for the listener.
    #[serde(default = "default_micro_js_event")]
    pub event: String,
    /// DOM id whose text mirrors this action's state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// Initial value for the action state when it differs from the program default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<i64>,
    /// Operation to apply to the numeric state.
    pub op: DxMicroJsOp,
}

/// Minimal operations supported by the generated JS emitter.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DxMicroJsOp {
    /// Add a signed delta.
    Add(i64),
    /// Set an exact value.
    Set(i64),
    /// Toggle a boolean-like slot.
    Toggle,
}

fn default_micro_js_event() -> String {
    "click".to_string()
}

/// Typed slot kind for columnar/template packet generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DxSlotKind {
    /// Text node or text-like value.
    Text,
    /// Number value.
    Number,
    /// Boolean value.
    Boolean,
    /// Enum value encoded by variant id.
    Enum,
    /// HTML attribute value.
    Attribute,
    /// Class toggle.
    Class,
    /// Event binding.
    Event,
    /// Child range.
    Children,
}

/// Columnar data for repeated component slots.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxColumnBatch {
    /// Template hash this batch instantiates.
    pub template_hash: String,
    /// Number of logical rows.
    pub row_count: u32,
    /// Slot columns.
    pub columns: Vec<DxColumn>,
}

/// One encoded column.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxColumn {
    /// Slot id in the template schema.
    pub slot_id: u32,
    /// Slot kind.
    pub kind: DxSlotKind,
    /// Column values.
    pub data: DxColumnData,
}

/// Column storage variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DxColumnData {
    /// Plain text values.
    Text(Vec<String>),
    /// Integer range: start + row_index * step.
    NumberRange { start: i64, step: i64, count: u32 },
    /// Enum dictionary plus per-row variant ids.
    Enum {
        variants: Vec<String>,
        values: Vec<u16>,
    },
    /// Packed booleans.
    Boolean(Vec<bool>),
}

/// Proven semantic sequence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxSemanticSequence {
    /// Text prefix.
    pub prefix: String,
    /// Text suffix.
    pub suffix: String,
    /// First value.
    pub start: i64,
    /// Step per row.
    pub step: i64,
    /// Number of rows.
    pub count: u32,
}

/// Live DOM/data patch operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DxPatchOp {
    /// Set text by node id.
    SetText { node_id: u32, value: String },
    /// Set attribute by node id.
    SetAttr {
        node_id: u32,
        name: String,
        value: String,
    },
    /// Toggle a class by node id.
    ToggleClass {
        node_id: u32,
        class_name: String,
        enabled: bool,
    },
    /// Set an enum column value in a row.
    SetEnum { row: u32, column: u16, variant: u16 },
    /// Set a numeric column value in a row.
    SetNumber { row: u32, column: u16, value: i64 },
    /// Apply one enum variant to a contiguous row range.
    RangeSet {
        start: u32,
        end: u32,
        column: u16,
        variant: u16,
    },
    /// Insert an instance of a cached template.
    Insert {
        parent_id: u32,
        index: u32,
        template_id: u32,
    },
    /// Remove a node.
    Remove { node_id: u32 },
    /// Move a node.
    Move {
        node_id: u32,
        parent_id: u32,
        index: u32,
    },
}
