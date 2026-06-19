pub(super) fn read_export_value(source: &str, start: usize) -> (String, usize) {
    let Some(value_start) = next_non_ws(source, start) else {
        return (String::new(), start);
    };
    let mut cursor = value_start;
    let mut value_end = value_start;
    let mut quote = None;
    let mut depth = 0usize;
    let mut saw_value = false;

    while cursor < source.len() {
        let Some(character) = source[cursor..].chars().next() else {
            break;
        };
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            } else if character == '\\' {
                cursor += character.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..]
                        .chars()
                        .next()
                        .map(char::len_utf8)
                        .unwrap_or_default();
                    value_end = cursor;
                    continue;
                }
            }
            cursor += character.len_utf8();
            value_end = cursor;
            continue;
        }

        match character {
            '"' | '\'' | '`' => {
                quote = Some(character);
                saw_value = true;
            }
            '(' | '[' | '{' => {
                depth += 1;
                saw_value = true;
            }
            ')' | ']' | '}' => {
                depth = depth.saturating_sub(1);
                saw_value = true;
            }
            ';' if depth == 0 => break,
            '\r' | '\n' if depth == 0 && saw_value => break,
            character if !character.is_whitespace() => saw_value = true,
            _ => {}
        }

        cursor += character.len_utf8();
        value_end = cursor;
    }

    (source[value_start..value_end].trim().to_string(), cursor)
}

pub(super) fn read_reexport_source(after_brace: &str) -> Option<String> {
    let trimmed = after_brace.trim_start();
    let after_from = trimmed.strip_prefix("from")?.trim_start();
    read_string_literal(after_from)
}

pub(super) fn next_keyword(source: &str, start: usize, word: &str) -> Option<usize> {
    let index = next_non_ws(source, start)?;
    source[index..]
        .starts_with(word)
        .then_some(index)
        .filter(|index| word_boundary(source, *index, *index + word.len()))
}

pub(super) fn find_word(source: &str, start: usize, word: &str) -> Option<usize> {
    let mut cursor = start;
    while let Some(offset) = source[cursor..].find(word) {
        let index = cursor + offset;
        let end = index + word.len();
        if word_boundary(source, index, end) {
            return Some(index);
        }
        cursor = end;
    }
    None
}

pub(super) fn find_balanced_delimiter(
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

pub(super) fn identifier_after(source: &str, start: usize) -> Option<(String, usize)> {
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

pub(super) fn next_non_ws(source: &str, mut cursor: usize) -> Option<usize> {
    while cursor < source.len() {
        let character = source[cursor..].chars().next()?;
        if !character.is_whitespace() {
            return Some(cursor);
        }
        cursor += character.len_utf8();
    }
    None
}

pub(super) fn word_boundary(source: &str, start: usize, end: usize) -> bool {
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

fn read_string_literal(source: &str) -> Option<String> {
    let quote = source.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let mut value = String::new();
    let mut escaped = false;
    for character in source[quote.len_utf8()..].chars() {
        if escaped {
            value.push(character);
            escaped = false;
            continue;
        }
        if character == '\\' {
            escaped = true;
            continue;
        }
        if character == quote {
            return Some(value);
        }
        value.push(character);
    }
    None
}

fn is_identifier_start(character: char) -> bool {
    character == '_' || character == '$' || character.is_ascii_alphabetic()
}

fn is_identifier_continue(character: char) -> bool {
    is_identifier_start(character) || character.is_ascii_digit()
}
