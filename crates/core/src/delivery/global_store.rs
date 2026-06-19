use regex::Regex;

use super::contract::{
    DxDerivedStateSlot, DxGlobalStore, DxGlobalStoreAction, DxStateEffectSlot, DxStateScope,
    DxStateSlot,
};

pub(super) fn global_store_bindings(source_path: &str, source: &str) -> Vec<DxGlobalStore> {
    let Ok(store_assignment) = Regex::new(
        r#"(?:export\s+)?const\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*=\s*store\s*(?:<[^>\n]+>\s*)?\("#,
    ) else {
        return Vec::new();
    };

    store_assignment
        .captures_iter(source)
        .filter_map(|capture| {
            let store_name = capture.get(1)?.as_str();
            let call_open = capture.get(0)?.end().saturating_sub(1);
            let call_inner = balanced_inner(source, call_open, '(', ')')?;
            let object_start = call_inner.find('{')?;
            let object_inner = balanced_inner(call_inner, object_start, '{', '}')?;
            global_store_from_object(source_path, store_name, object_inner)
        })
        .collect()
}

fn global_store_from_object(
    source_path: &str,
    store_name: &str,
    object_inner: &str,
) -> Option<DxGlobalStore> {
    let store_id = format!("store-{}-{store_name}", stable_fragment(source_path));
    let mut slots = Vec::new();
    let mut derived_slots = Vec::new();
    let mut actions = Vec::new();
    let mut effects = Vec::new();

    for entry in split_top_level(object_inner, ',') {
        let Some((property, value)) = split_property(entry) else {
            continue;
        };
        let Some(property) = clean_property_name(property) else {
            continue;
        };
        let store_slot_name = format!("{store_name}.{property}");

        if let Some(initial_source) = call_inner(value, "state") {
            slots.push(DxStateSlot {
                id: format!("{store_id}-state-{property}"),
                name: store_slot_name,
                setter: None,
                scope: DxStateScope::Global,
                source_path: source_path.to_string(),
                initial_source: initial_source.trim().to_string(),
                value_kind: value_kind(initial_source),
            });
            continue;
        }

        if let Some(expression) = call_inner(value, "derived") {
            let dependencies = state_dependencies(expression, store_name, &slots);
            derived_slots.push(DxDerivedStateSlot {
                id: format!("{store_id}-derived-{property}"),
                name: store_slot_name,
                expression: expression.trim().to_string(),
                dependencies,
                source_path: source_path.to_string(),
            });
            continue;
        }

        if let Some(handler) = call_inner(value, "action") {
            actions.push(DxGlobalStoreAction {
                id: format!("{store_id}-action-{property}"),
                name: format!("{store_name}.{property}"),
                source_path: source_path.to_string(),
                handler: handler.trim().to_string(),
                state_dependencies: state_dependencies(handler, store_name, &slots),
            });
            continue;
        }

        if let Some(handler) = call_inner(value, "effect") {
            effects.push(DxStateEffectSlot {
                id: format!("{store_id}-effect-{property}"),
                kind: "store-effect".to_string(),
                source_path: source_path.to_string(),
                dependencies: state_dependencies(handler, store_name, &slots),
            });
        }
    }

    (!slots.is_empty() || !derived_slots.is_empty() || !actions.is_empty() || !effects.is_empty())
        .then(|| DxGlobalStore {
            id: store_id,
            name: store_name.to_string(),
            source_path: source_path.to_string(),
            slots,
            derived_slots,
            actions,
            effects,
        })
}

fn call_inner<'a>(source: &'a str, callee: &str) -> Option<&'a str> {
    let mut rest = source.trim_start();
    rest = rest.strip_prefix(callee)?;
    rest = rest.trim_start();
    if rest.starts_with('<') {
        let generic_end = matching_angle_end(rest)?;
        rest = rest[generic_end + 1..].trim_start();
    }
    if !rest.starts_with('(') {
        return None;
    }
    balanced_inner(rest, 0, '(', ')')
}

fn split_property(entry: &str) -> Option<(&str, &str)> {
    let colon = top_level_delimiter(entry, ':')?;
    Some((&entry[..colon], &entry[colon + 1..]))
}

fn split_top_level(source: &str, delimiter: char) -> Vec<&str> {
    let mut entries = Vec::new();
    let mut start = 0usize;
    let mut index = 0usize;
    while let Some(next) = top_level_delimiter(&source[index..], delimiter) {
        let absolute = index + next;
        let entry = source[start..absolute].trim();
        if !entry.is_empty() {
            entries.push(entry);
        }
        index = absolute + delimiter.len_utf8();
        start = index;
    }
    let tail = source[start..].trim();
    if !tail.is_empty() {
        entries.push(tail);
    }
    entries
}

fn top_level_delimiter(source: &str, delimiter: char) -> Option<usize> {
    let mut stack = Vec::new();
    let mut string_quote = None;
    let mut escape = false;
    for (index, ch) in source.char_indices() {
        if let Some(quote) = string_quote {
            if escape {
                escape = false;
                continue;
            }
            if ch == '\\' {
                escape = true;
                continue;
            }
            if ch == quote {
                string_quote = None;
            }
            continue;
        }
        match ch {
            '"' | '\'' | '`' => string_quote = Some(ch),
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.last() == Some(&'(') {
                    stack.pop();
                }
            }
            ']' => {
                if stack.last() == Some(&'[') {
                    stack.pop();
                }
            }
            '}' => {
                if stack.last() == Some(&'{') {
                    stack.pop();
                }
            }
            _ if ch == delimiter && stack.is_empty() => return Some(index),
            _ => {}
        }
    }
    None
}

fn balanced_inner(source: &str, open_index: usize, open: char, close: char) -> Option<&str> {
    let mut depth = 0usize;
    let mut string_quote = None;
    let mut escape = false;
    let mut inner_start = None;

    for (offset, ch) in source[open_index..].char_indices() {
        let index = open_index + offset;
        if let Some(quote) = string_quote {
            if escape {
                escape = false;
                continue;
            }
            if ch == '\\' {
                escape = true;
                continue;
            }
            if ch == quote {
                string_quote = None;
            }
            continue;
        }
        match ch {
            '"' | '\'' | '`' => string_quote = Some(ch),
            _ if ch == open => {
                if depth == 0 {
                    inner_start = Some(index + ch.len_utf8());
                }
                depth += 1;
            }
            _ if ch == close => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return inner_start.map(|start| &source[start..index]);
                }
            }
            _ => {}
        }
    }
    None
}

fn matching_angle_end(source: &str) -> Option<usize> {
    let mut depth = 0usize;
    for (index, ch) in source.char_indices() {
        match ch {
            '<' => depth += 1,
            '>' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(index);
                }
            }
            '\n' => return None,
            _ => {}
        }
    }
    None
}

fn clean_property_name(raw: &str) -> Option<String> {
    let name = raw.trim();
    if name.is_empty() || name.starts_with("...") {
        return None;
    }
    let unquoted = name
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .or_else(|| {
            name.strip_prefix('\'')
                .and_then(|value| value.strip_suffix('\''))
        })
        .unwrap_or(name);
    is_identifier_like(unquoted).then(|| unquoted.to_string())
}

fn is_identifier_like(value: &str) -> bool {
    let mut chars = value.chars();
    chars
        .next()
        .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '_' || ch == '$')
        && chars.all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '$' | '-'))
}

fn value_kind(source: &str) -> String {
    let value = source.trim();
    if value.starts_with('"') || value.starts_with('\'') || value.starts_with('`') {
        "string"
    } else if matches!(value, "true" | "false") {
        "boolean"
    } else if value.starts_with('[') {
        "array"
    } else if value.starts_with('{') {
        "object"
    } else if value.parse::<f64>().is_ok() {
        "number"
    } else {
        "unknown"
    }
    .to_string()
}

fn state_dependencies(source: &str, store_name: &str, slots: &[DxStateSlot]) -> Vec<String> {
    slots
        .iter()
        .filter(|slot| {
            let local_name = slot
                .name
                .strip_prefix(&format!("{store_name}."))
                .unwrap_or(&slot.name);
            contains_identifier_like(source, local_name) || source.contains(&slot.name)
        })
        .map(|slot| slot.name.clone())
        .collect()
}

fn contains_identifier_like(source: &str, name: &str) -> bool {
    let mut start = 0usize;
    while let Some(offset) = source[start..].find(name) {
        let index = start + offset;
        let before = source[..index].chars().next_back();
        let after = source[index + name.len()..].chars().next();
        let before_ok = before.is_none_or(|ch| !is_ident_char(ch));
        let after_ok = after.is_none_or(|ch| !is_ident_char(ch));
        if before_ok && after_ok {
            return true;
        }
        start = index + name.len();
    }
    false
}

fn is_ident_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '_' | '$')
}

fn stable_fragment(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_global_store_slots_actions_and_effects() {
        let stores = global_store_bindings(
            "lib/stores/counter.ts",
            r#"
export const counterStore = store({
  count: state(1),
  label: state("Counter"),
  doubled: derived((store) => store.count * 2),
  increment: action((store) => { store.count += 1; }),
  persist: effect((store) => store.label),
});
"#,
        );

        assert_eq!(stores.len(), 1);
        let store = &stores[0];
        assert_eq!(store.name, "counterStore");
        assert_eq!(store.slots.len(), 2);
        assert_eq!(store.slots[0].scope, DxStateScope::Global);
        assert_eq!(store.slots[0].name, "counterStore.count");
        assert_eq!(store.derived_slots[0].dependencies, ["counterStore.count"]);
        assert_eq!(store.actions[0].state_dependencies, ["counterStore.count"]);
        assert_eq!(store.effects[0].dependencies, ["counterStore.label"]);
    }
}
