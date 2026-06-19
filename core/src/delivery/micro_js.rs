use std::collections::BTreeMap;

use super::types::{DxMicroJsOp, DxMicroJsProgram};

/// Emits small inline JavaScript for interactions that do not justify WASM.
pub struct DxMicroJsEmitter;

impl DxMicroJsEmitter {
    /// Emit a compact counter/state script.
    pub fn emit(program: &DxMicroJsProgram) -> String {
        if program
            .actions
            .iter()
            .any(|action| action.target_id.is_some())
        {
            return Self::emit_multi_state(program);
        }

        let mut js = format!(
            "(()=>{{const $=document.getElementById.bind(document);let c={};const e=$({});",
            program.initial_value,
            js_string(&program.target_id)
        );

        for action in &program.actions {
            let op = match action.op {
                DxMicroJsOp::Add(delta) if delta >= 0 => format!("c+={delta}"),
                DxMicroJsOp::Add(delta) => format!("c-={}", delta.abs()),
                DxMicroJsOp::Set(value) => format!("c={value}"),
                DxMicroJsOp::Toggle => "c=+!c".to_string(),
            };
            js.push_str(&format!(
                "{{const n=$({});if(n)n.addEventListener({},()=>{{{};e.textContent=c}})}};",
                js_string(&action.element_id),
                js_string(&action.event),
                op
            ));
        }

        js.push_str("})()");
        js
    }

    fn emit_multi_state(program: &DxMicroJsProgram) -> String {
        let mut states = BTreeMap::from([(program.target_id.clone(), program.initial_value)]);
        for action in &program.actions {
            if let Some(target_id) = action.target_id.as_ref() {
                states
                    .entry(target_id.clone())
                    .or_insert(action.initial_value.unwrap_or_default());
            }
        }
        let mut js = String::from(
            "(()=>{const $=document.getElementById.bind(document);const s=Object.create(null);",
        );
        for (target_id, value) in states {
            js.push_str(&format!("s[{}]={value};", js_string(&target_id)));
        }
        js.push_str("const u=(id)=>{const e=$(id);if(e)e.textContent=s[id]};");
        for action in &program.actions {
            let target_id = action
                .target_id
                .as_deref()
                .unwrap_or(program.target_id.as_str());
            let target = js_string(target_id);
            let op = match action.op {
                DxMicroJsOp::Add(delta) if delta >= 0 => format!("s[{target}]+={delta}"),
                DxMicroJsOp::Add(delta) => format!("s[{target}]-={}", delta.abs()),
                DxMicroJsOp::Set(value) => format!("s[{target}]={value}"),
                DxMicroJsOp::Toggle => format!("s[{target}]=+!s[{target}]"),
            };
            js.push_str(&format!(
                "{{const n=$({});if(n)n.addEventListener({},()=>{{{};u({})}})}};",
                js_string(&action.element_id),
                js_string(&action.event),
                op,
                target
            ));
        }
        js.push_str("})()");
        js
    }

    /// Emit a complete inline script tag.
    pub fn emit_script_tag(program: &DxMicroJsProgram) -> String {
        format!("<script>{}</script>", Self::emit(program))
    }
}

fn js_string(value: &str) -> String {
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
}
