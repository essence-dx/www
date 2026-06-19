/// balanced-inline-server-actions scanner for Lane 11 Next custom-transform receipts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct InlineServerAction {
    pub(super) name: String,
    pub(super) async_export: bool,
    pub(super) export_kind: &'static str,
}

pub(super) fn collect_inline_server_actions(source: &str) -> Vec<InlineServerAction> {
    let mut actions = Vec::new();
    collect_function_actions(source, &mut actions);
    collect_const_arrow_actions(source, &mut actions);
    actions.sort_by(|left, right| left.name.cmp(&right.name));
    actions.dedup_by(|left, right| left.name == right.name);
    actions
}

fn collect_function_actions(source: &str, actions: &mut Vec<InlineServerAction>) {
    let mut cursor = 0usize;
    while let Some(offset) = source[cursor..].find("function") {
        let function_index = cursor + offset;
        let before = previous_word(source, function_index);
        let async_export = before == Some("async");
        let after_function = function_index + "function".len();
        if !word_boundary(source, function_index, after_function) {
            cursor = after_function;
            continue;
        }
        let Some((name, name_end)) = identifier_after(source, after_function) else {
            cursor = after_function;
            continue;
        };
        let Some(parameters_start) = next_non_ws(source, name_end).filter(|index| {
            source[*index..]
                .chars()
                .next()
                .is_some_and(|character| character == '(')
        }) else {
            cursor = name_end;
            continue;
        };
        let Some(parameters_end) = find_balanced_delimiter(source, parameters_start, '(', ')')
        else {
            cursor = parameters_start + 1;
            continue;
        };
        let Some(body_start) = next_non_ws(source, parameters_end + 1).filter(|index| {
            source[*index..]
                .chars()
                .next()
                .is_some_and(|character| character == '{')
        }) else {
            cursor = parameters_end + 1;
            continue;
        };
        let Some(body_end) = find_balanced_delimiter(source, body_start, '{', '}') else {
            cursor = body_start + 1;
            continue;
        };
        let body = &source[body_start + 1..body_end];
        if server_directive_in_body(body) {
            actions.push(InlineServerAction {
                name,
                async_export,
                export_kind: "function",
            });
        }
        cursor = body_start + 1;
    }
}

fn collect_const_arrow_actions(source: &str, actions: &mut Vec<InlineServerAction>) {
    let mut cursor = 0usize;
    while let Some(offset) = source[cursor..].find("const") {
        let const_index = cursor + offset;
        let after_const = const_index + "const".len();
        if !word_boundary(source, const_index, after_const) {
            cursor = after_const;
            continue;
        }
        let Some((name, name_end)) = identifier_after(source, after_const) else {
            cursor = after_const;
            continue;
        };
        let Some(eq_index) = source[name_end..].find('=').map(|offset| name_end + offset) else {
            cursor = name_end;
            continue;
        };
        let signature = &source[eq_index + 1..];
        let signature_trimmed = signature.trim_start();
        let async_export = signature_trimmed.starts_with("async ");
        let Some(arrow_offset) = signature.find("=>") else {
            cursor = eq_index + 1;
            continue;
        };
        let after_arrow = eq_index + 1 + arrow_offset + "=>".len();
        let Some(body_start) = next_non_ws(source, after_arrow).filter(|index| {
            source[*index..]
                .chars()
                .next()
                .is_some_and(|character| character == '{')
        }) else {
            cursor = after_arrow;
            continue;
        };
        let Some(body_end) = find_balanced_delimiter(source, body_start, '{', '}') else {
            cursor = body_start + 1;
            continue;
        };
        let body = &source[body_start + 1..body_end];
        if server_directive_in_body(body) {
            actions.push(InlineServerAction {
                name,
                async_export,
                export_kind: "const-arrow",
            });
        }
        cursor = body_end + 1;
    }
}

fn server_directive_in_body(body: &str) -> bool {
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        let statement = trimmed.trim_end_matches(';').trim();
        return matches!(
            statement,
            "\"use server\"" | "'use server'" | "`use server`"
        );
    }
    false
}

fn identifier_after(source: &str, start: usize) -> Option<(String, usize)> {
    let mut cursor = next_non_ws(source, start)?;
    let first = source[cursor..].chars().next()?;
    if !is_identifier_start(first) {
        return None;
    }
    let mut name = String::new();
    name.push(first);
    cursor += first.len_utf8();
    while cursor < source.len() {
        let Some(character) = source[cursor..].chars().next() else {
            break;
        };
        if !is_identifier_continue(character) {
            break;
        }
        name.push(character);
        cursor += character.len_utf8();
    }
    Some((name, cursor))
}

fn previous_word(source: &str, index: usize) -> Option<&str> {
    let before = source[..index].trim_end();
    let start = before
        .rfind(|character: char| !is_identifier_continue(character))
        .map(|position| position + 1)
        .unwrap_or(0);
    let word = &before[start..];
    (!word.is_empty()).then_some(word)
}

fn next_non_ws(source: &str, mut cursor: usize) -> Option<usize> {
    while cursor < source.len() {
        let character = source[cursor..].chars().next()?;
        if !character.is_whitespace() {
            return Some(cursor);
        }
        cursor += character.len_utf8();
    }
    None
}

fn word_boundary(source: &str, start: usize, end: usize) -> bool {
    let before = source[..start]
        .chars()
        .last()
        .is_none_or(|character| !is_identifier_continue(character));
    let after = source[end..]
        .chars()
        .next()
        .is_none_or(|character| !is_identifier_continue(character));
    before && after
}

fn is_identifier_start(character: char) -> bool {
    character == '_' || character == '$' || character.is_ascii_alphabetic()
}

fn is_identifier_continue(character: char) -> bool {
    is_identifier_start(character) || character.is_ascii_digit()
}

fn find_balanced_delimiter(
    source: &str,
    mut cursor: usize,
    open: char,
    close: char,
) -> Option<usize> {
    let mut quote = None;
    let mut depth = 0usize;
    while cursor < source.len() {
        let character = source[cursor..].chars().next()?;
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            } else if character == '\\' {
                cursor += character.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..].chars().next()?.len_utf8();
                    continue;
                }
            }
            cursor += character.len_utf8();
            continue;
        }
        match character {
            '"' | '\'' | '`' => quote = Some(character),
            _ if character == open => depth += 1,
            _ if character == close => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(cursor);
                }
            }
            _ => {}
        }
        cursor += character.len_utf8();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_inline_server_action_with_object_literal_return() {
        let actions = collect_inline_server_actions(
            r#"async function saveDashboard() {
  "use server";
  return { ok: true, nested: { id: 1 } };
}"#,
        );

        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].name, "saveDashboard");
        assert!(actions[0].async_export);
        assert_eq!(actions[0].export_kind, "function");
    }

    #[test]
    fn detects_inline_arrow_action_with_nested_blocks() {
        let actions = collect_inline_server_actions(
            r#"const save = async (formData) => {
  "use server";
  if (formData) {
    return { ok: true };
  }
  return { ok: false };
};"#,
        );

        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].name, "save");
        assert!(actions[0].async_export);
        assert_eq!(actions[0].export_kind, "const-arrow");
    }

    #[test]
    fn ignores_misplaced_directive_after_real_statement() {
        let actions = collect_inline_server_actions(
            r#"function notAction() {
  const ok = true;
  "use server";
  return { ok };
}"#,
        );

        assert!(
            actions.is_empty(),
            "misplaced directive must not become a server action"
        );
    }
}
