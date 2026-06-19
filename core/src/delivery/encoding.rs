use flate2::{Compression, write::GzEncoder};
use std::io::Write;

use super::types::{DxColumnBatch, DxColumnData, DxPatchOp, DxSemanticSequence, DxSlotKind};
use crate::splitter::Template;

/// Concrete compact encoders used by the delivery planner and benchmarks.
pub struct DxPacketEncoder;

impl DxPacketEncoder {
    /// Encode cached templates and slot metadata.
    pub fn encode_template_slots(templates: &[Template]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"DXT1");
        put_varint(&mut out, templates.len() as u64);
        for template in templates {
            put_varint(&mut out, template.id as u64);
            put_bytes(&mut out, template.hash.as_bytes());
            put_bytes(&mut out, template.html.as_bytes());
            put_varint(&mut out, template.slots.len() as u64);
            for slot in &template.slots {
                put_varint(&mut out, slot.slot_id as u64);
                out.push(slot_kind_code(map_packet_slot_kind(&slot.slot_type)));
                put_varint(&mut out, slot.path.len() as u64);
                for item in &slot.path {
                    put_varint(&mut out, *item as u64);
                }
            }
        }
        out
    }

    /// Encode repeated slots by column.
    pub fn encode_column_batch(batch: &DxColumnBatch) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"DXC1");
        put_bytes(&mut out, batch.template_hash.as_bytes());
        put_varint(&mut out, batch.row_count as u64);
        put_varint(&mut out, batch.columns.len() as u64);
        for column in &batch.columns {
            put_varint(&mut out, column.slot_id as u64);
            out.push(slot_kind_code(column.kind));
            encode_column_data(&mut out, &column.data);
        }
        out
    }

    /// Encode a proven semantic sequence.
    pub fn encode_semantic_sequence(sequence: &DxSemanticSequence) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"DXS1");
        put_bytes(&mut out, sequence.prefix.as_bytes());
        put_bytes(&mut out, sequence.suffix.as_bytes());
        put_i64(&mut out, sequence.start);
        put_i64(&mut out, sequence.step);
        put_varint(&mut out, sequence.count as u64);
        out
    }

    /// Encode live patch operations.
    pub fn encode_patch_stream(ops: &[DxPatchOp]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"DXP1");
        put_varint(&mut out, ops.len() as u64);
        for op in ops {
            encode_patch_op(&mut out, op);
        }
        out
    }
}

fn encode_column_data(out: &mut Vec<u8>, data: &DxColumnData) {
    match data {
        DxColumnData::Text(values) => {
            out.push(0x01);
            put_varint(out, values.len() as u64);
            for value in values {
                put_bytes(out, value.as_bytes());
            }
        }
        DxColumnData::NumberRange { start, step, count } => {
            out.push(0x02);
            put_i64(out, *start);
            put_i64(out, *step);
            put_varint(out, *count as u64);
        }
        DxColumnData::Enum { variants, values } => {
            out.push(0x03);
            put_varint(out, variants.len() as u64);
            for variant in variants {
                put_bytes(out, variant.as_bytes());
            }
            put_varint(out, values.len() as u64);
            for value in values {
                put_varint(out, *value as u64);
            }
        }
        DxColumnData::Boolean(values) => {
            out.push(0x04);
            put_varint(out, values.len() as u64);
            encode_bool_bits(out, values);
        }
    }
}

fn encode_bool_bits(out: &mut Vec<u8>, values: &[bool]) {
    let mut byte = 0u8;
    for (index, value) in values.iter().enumerate() {
        if *value {
            byte |= 1 << (index % 8);
        }
        if index % 8 == 7 {
            out.push(byte);
            byte = 0;
        }
    }
    if !values.len().is_multiple_of(8) {
        out.push(byte);
    }
}

fn encode_patch_op(out: &mut Vec<u8>, op: &DxPatchOp) {
    match op {
        DxPatchOp::SetText { node_id, value } => {
            out.push(0x01);
            put_varint(out, *node_id as u64);
            put_bytes(out, value.as_bytes());
        }
        DxPatchOp::SetAttr {
            node_id,
            name,
            value,
        } => {
            out.push(0x02);
            put_varint(out, *node_id as u64);
            put_bytes(out, name.as_bytes());
            put_bytes(out, value.as_bytes());
        }
        DxPatchOp::ToggleClass {
            node_id,
            class_name,
            enabled,
        } => {
            out.push(0x03);
            put_varint(out, *node_id as u64);
            put_bytes(out, class_name.as_bytes());
            out.push(u8::from(*enabled));
        }
        DxPatchOp::SetEnum {
            row,
            column,
            variant,
        } => {
            out.push(0x04);
            put_varint(out, *row as u64);
            put_varint(out, *column as u64);
            put_varint(out, *variant as u64);
        }
        DxPatchOp::SetNumber { row, column, value } => {
            out.push(0x05);
            put_varint(out, *row as u64);
            put_varint(out, *column as u64);
            put_i64(out, *value);
        }
        DxPatchOp::RangeSet {
            start,
            end,
            column,
            variant,
        } => {
            out.push(0x06);
            put_varint(out, *start as u64);
            put_varint(out, *end as u64);
            put_varint(out, *column as u64);
            put_varint(out, *variant as u64);
        }
        DxPatchOp::Insert {
            parent_id,
            index,
            template_id,
        } => {
            out.push(0x07);
            put_varint(out, *parent_id as u64);
            put_varint(out, *index as u64);
            put_varint(out, *template_id as u64);
        }
        DxPatchOp::Remove { node_id } => {
            out.push(0x08);
            put_varint(out, *node_id as u64);
        }
        DxPatchOp::Move {
            node_id,
            parent_id,
            index,
        } => {
            out.push(0x09);
            put_varint(out, *node_id as u64);
            put_varint(out, *parent_id as u64);
            put_varint(out, *index as u64);
        }
    }
}

fn map_packet_slot_kind(slot_type: &dx_www_packet::SlotType) -> DxSlotKind {
    match slot_type {
        dx_www_packet::SlotType::Text => DxSlotKind::Text,
        dx_www_packet::SlotType::Attribute | dx_www_packet::SlotType::Property => {
            DxSlotKind::Attribute
        }
        dx_www_packet::SlotType::Event => DxSlotKind::Event,
    }
}

fn slot_kind_code(kind: DxSlotKind) -> u8 {
    match kind {
        DxSlotKind::Text => 1,
        DxSlotKind::Number => 2,
        DxSlotKind::Boolean => 3,
        DxSlotKind::Enum => 4,
        DxSlotKind::Attribute => 5,
        DxSlotKind::Class => 6,
        DxSlotKind::Event => 7,
        DxSlotKind::Children => 8,
    }
}

fn put_varint(out: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        out.push((value as u8) | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
}

fn put_i64(out: &mut Vec<u8>, value: i64) {
    let zigzag = ((value << 1) ^ (value >> 63)) as u64;
    put_varint(out, zigzag);
}

fn put_bytes(out: &mut Vec<u8>, bytes: &[u8]) {
    put_varint(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

pub(super) fn gzip_len(bytes: &[u8]) -> usize {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    if encoder.write_all(bytes).is_err() {
        return bytes.len();
    }
    encoder
        .finish()
        .map_or(bytes.len(), |compressed| compressed.len())
}
