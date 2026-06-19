use std::collections::BTreeSet;

pub(super) fn collect_top_level_directives(source: &str) -> BTreeSet<String> {
    let mut directives = BTreeSet::new();
    let mut cursor = 0usize;
    loop {
        cursor = skip_whitespace_and_comments(source, cursor);
        if cursor >= source.len() {
            return directives;
        }

        let Some((directive, statement_end)) = read_directive_statement(source, cursor) else {
            return directives;
        };
        if !directive.starts_with("use ") {
            return directives;
        }
        directives.insert(directive);
        cursor = statement_end;
    }
}

pub(super) fn has_use_client_directive(source: &str) -> bool {
    collect_top_level_directives(source).contains("use client")
}

fn read_directive_statement(source: &str, cursor: usize) -> Option<(String, usize)> {
    let quote = source[cursor..].chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }

    let (directive, quoted_end) = parse_quoted_value(&source[cursor..], quote)?;
    let statement_end = directive_statement_end(source, cursor + quoted_end)?;
    Some((directive, statement_end))
}

fn skip_whitespace_and_comments(source: &str, mut cursor: usize) -> usize {
    loop {
        let next = skip_whitespace(source, cursor);
        if next != cursor {
            cursor = next;
            continue;
        }
        if source[cursor..].starts_with("//") {
            cursor = skip_line_comment(source, cursor);
            continue;
        }
        if source[cursor..].starts_with("/*") {
            cursor = skip_block_comment(source, cursor);
            continue;
        }
        return cursor;
    }
}

fn directive_statement_end(source: &str, cursor: usize) -> Option<usize> {
    let cursor = skip_inline_whitespace(source, cursor);
    if cursor >= source.len() {
        return Some(cursor);
    }
    if source[cursor..].starts_with(';') {
        return Some(cursor + ';'.len_utf8());
    }
    if source[cursor..].starts_with("//") {
        return Some(skip_line_comment(source, cursor));
    }
    if source[cursor..].starts_with("\r\n") {
        return Some(cursor + "\r\n".len());
    }
    if source[cursor..].starts_with('\n') || source[cursor..].starts_with('\r') {
        return Some(cursor + '\n'.len_utf8());
    }
    None
}

fn skip_whitespace(source: &str, mut cursor: usize) -> usize {
    while cursor < source.len() {
        let Some(character) = source[cursor..].chars().next() else {
            break;
        };
        if !character.is_whitespace() {
            break;
        }
        cursor += character.len_utf8();
    }
    cursor
}

fn skip_inline_whitespace(source: &str, mut cursor: usize) -> usize {
    while cursor < source.len() {
        let Some(character) = source[cursor..].chars().next() else {
            break;
        };
        if !matches!(character, ' ' | '\t') {
            break;
        }
        cursor += character.len_utf8();
    }
    cursor
}

fn skip_line_comment(source: &str, cursor: usize) -> usize {
    source[cursor..]
        .find('\n')
        .map(|offset| cursor + offset + '\n'.len_utf8())
        .unwrap_or(source.len())
}

fn skip_block_comment(source: &str, cursor: usize) -> usize {
    source[cursor..]
        .find("*/")
        .map(|offset| cursor + offset + "*/".len())
        .unwrap_or(source.len())
}

fn parse_quoted_value(source: &str, quote: char) -> Option<(String, usize)> {
    let mut value = String::new();
    let mut cursor = quote.len_utf8();
    let mut escaped = false;
    while cursor < source.len() {
        let character = source[cursor..].chars().next()?;
        cursor += character.len_utf8();
        if escaped {
            value.push(match character {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                escaped => escaped,
            });
            escaped = false;
            continue;
        }
        if character == '\\' {
            escaped = true;
            continue;
        }
        if character == quote {
            return Some((value, cursor));
        }
        value.push(character);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_directives_after_comments_and_stops_at_imports() {
        let source = r#"
            /* license */
            "use strict";
            'use client';
            "use cache: private";
            import value from "./value";
            "use server";
        "#;

        let directives = collect_top_level_directives(source);

        assert!(directives.contains("use strict"));
        assert!(directives.contains("use client"));
        assert!(directives.contains("use cache: private"));
        assert!(!directives.contains("use server"));
        assert!(has_use_client_directive(source));
    }
}
