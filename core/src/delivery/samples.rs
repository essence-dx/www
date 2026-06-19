use std::collections::BTreeMap;

use super::types::{
    DxColumn, DxColumnBatch, DxColumnData, DxMicroJsAction, DxMicroJsOp, DxMicroJsProgram,
    DxPatchOp, DxSlotKind,
};
use crate::splitter::Template;

/// Build a sample column batch from enum and numeric rows.
pub fn sample_dashboard_column_batch(rows: u32) -> DxColumnBatch {
    DxColumnBatch {
        template_hash: "dashboard-row".to_string(),
        row_count: rows,
        columns: vec![
            customer_name_column(rows),
            plan_column(rows),
            status_column(rows),
            revenue_column(rows),
        ],
    }
}

fn customer_name_column(rows: u32) -> DxColumn {
    DxColumn {
        slot_id: 0,
        kind: DxSlotKind::Text,
        data: DxColumnData::Text(
            (0..rows)
                .map(|index| format!("Customer {}", index + 1))
                .collect(),
        ),
    }
}

fn plan_column(rows: u32) -> DxColumn {
    DxColumn {
        slot_id: 1,
        kind: DxSlotKind::Enum,
        data: DxColumnData::Enum {
            variants: vec!["Enterprise".to_string(), "Team".to_string()],
            values: (0..rows)
                .map(|index| if index % 3 == 0 { 0 } else { 1 })
                .collect(),
        },
    }
}

fn status_column(rows: u32) -> DxColumn {
    DxColumn {
        slot_id: 2,
        kind: DxSlotKind::Enum,
        data: DxColumnData::Enum {
            variants: vec!["Review".to_string(), "Active".to_string()],
            values: (0..rows)
                .map(|index| if index % 5 == 0 { 0 } else { 1 })
                .collect(),
        },
    }
}

fn revenue_column(rows: u32) -> DxColumn {
    DxColumn {
        slot_id: 3,
        kind: DxSlotKind::Number,
        data: DxColumnData::NumberRange {
            start: 12_000,
            step: 100,
            count: rows,
        },
    }
}

/// Build sample patch operations for dashboard row updates.
pub fn sample_dashboard_patch_ops(rows: u32, change_count: u32) -> Vec<DxPatchOp> {
    let mut ops = Vec::new();
    for offset in 0..change_count {
        let row = (offset * 97 + 13) % rows.max(1);
        ops.push(DxPatchOp::SetEnum {
            row,
            column: 2,
            variant: 2,
        });
        ops.push(DxPatchOp::SetNumber {
            row,
            column: 3,
            value: 90_000 + i64::from(row) * 100,
        });
    }
    ops
}

/// Build the tiny counter program used to prove the no-WASM path.
pub fn sample_counter_micro_program() -> DxMicroJsProgram {
    DxMicroJsProgram {
        initial_value: 0,
        target_id: "counter".to_string(),
        actions: vec![
            DxMicroJsAction {
                element_id: "inc".to_string(),
                event: "click".to_string(),
                target_id: None,
                initial_value: None,
                op: DxMicroJsOp::Add(1),
            },
            DxMicroJsAction {
                element_id: "dec".to_string(),
                event: "click".to_string(),
                target_id: None,
                initial_value: None,
                op: DxMicroJsOp::Add(-1),
            },
            DxMicroJsAction {
                element_id: "reset".to_string(),
                event: "click".to_string(),
                target_id: None,
                initial_value: None,
                op: DxMicroJsOp::Set(0),
            },
        ],
    }
}

/// Count string frequencies for future dictionary training.
pub fn template_dictionary_terms(templates: &[Template]) -> BTreeMap<String, usize> {
    let mut terms = BTreeMap::new();
    for template in templates {
        for token in template
            .html
            .split(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '_'))
            .filter(|token| token.len() >= 3)
        {
            *terms.entry(token.to_string()).or_insert(0) += 1;
        }
    }
    terms
}
