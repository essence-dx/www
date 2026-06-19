pub fn strip_typescript_runtime(source: &str) -> String {
    strip_return_annotations(&strip_parameter_annotations(&strip_type_only_declarations(
        source,
    )))
}

fn strip_type_only_declarations(source: &str) -> String {
    let mut output = String::new();
    let mut skipping_block = false;

    for line in source.lines() {
        let trimmed = line.trim_start();
        if skipping_block {
            if trimmed.ends_with('}') || trimmed.ends_with("};") {
                skipping_block = false;
            }
            continue;
        }

        if is_type_only_declaration(trimmed) {
            if trimmed.starts_with("interface ")
                || trimmed.starts_with("export interface ")
                || trimmed.ends_with('{')
            {
                skipping_block = !trimmed.ends_with('}') && !trimmed.ends_with("};");
            }
            continue;
        }

        output.push_str(line);
        output.push('\n');
    }

    output
}

fn is_type_only_declaration(line: &str) -> bool {
    line.starts_with("type ")
        || line.starts_with("export type ")
        || line.starts_with("interface ")
        || line.starts_with("export interface ")
}

fn strip_parameter_annotations(source: &str) -> String {
    let mut output = String::new();
    let mut cursor = 0usize;

    while let Some(relative_start) = source[cursor..].find("export function ") {
        let start = cursor + relative_start;
        let Some(open) = source[start..].find('(').map(|index| start + index) else {
            break;
        };
        let Some(close) = matching_paren(source, open) else {
            break;
        };

        output.push_str(&source[cursor..=open]);
        output.push_str(&strip_parameter_list(&source[open + 1..close]));
        cursor = close;
    }

    output.push_str(&source[cursor..]);
    output
}

fn matching_paren(source: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (index, character) in source[open..].char_indices() {
        match character {
            '(' => depth += 1,
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(open + index);
                }
            }
            _ => {}
        }
    }
    None
}

fn strip_parameter_list(parameters: &str) -> String {
    let mut output = String::new();
    let mut chars = parameters.chars().peekable();
    while let Some(character) = chars.next() {
        if character == ':' {
            skip_parameter_type(&mut chars);
        } else {
            output.push(character);
        }
    }
    output
}

fn skip_parameter_type<I>(chars: &mut std::iter::Peekable<I>)
where
    I: Iterator<Item = char>,
{
    let mut brace_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut angle_depth = 0usize;

    while let Some(next) = chars.peek().copied() {
        if brace_depth == 0
            && bracket_depth == 0
            && angle_depth == 0
            && matches!(next, ',' | ')' | '=')
        {
            break;
        }

        match next {
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '<' => angle_depth += 1,
            '>' => angle_depth = angle_depth.saturating_sub(1),
            _ => {}
        }
        chars.next();
    }
}

fn strip_return_annotations(source: &str) -> String {
    let mut output = String::new();
    let mut cursor = 0usize;

    while let Some(relative_start) = source[cursor..].find("export function ") {
        let start = cursor + relative_start;
        let Some(open) = source[start..].find('(').map(|index| start + index) else {
            break;
        };
        let Some(close) = matching_paren(source, open) else {
            break;
        };
        let after_close = close + 1;
        let whitespace_end = inline_whitespace_end(source, after_close);

        output.push_str(&source[cursor..whitespace_end]);
        if source[whitespace_end..].starts_with(':') {
            cursor = skip_return_annotation_source(source, whitespace_end + 1);
        } else {
            cursor = whitespace_end;
        }
    }

    output.push_str(&source[cursor..]);
    output
}

fn inline_whitespace_end(source: &str, start: usize) -> usize {
    for (relative_index, character) in source[start..].char_indices() {
        if character == '\n' || !character.is_whitespace() {
            return start + relative_index;
        }
    }
    source.len()
}

fn skip_return_annotation_source(source: &str, start: usize) -> usize {
    let mut brace_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut angle_depth = 0usize;

    for (relative_index, next) in source[start..].char_indices() {
        if brace_depth == 0 && bracket_depth == 0 && angle_depth == 0 && matches!(next, '{' | '=') {
            return start + relative_index;
        }

        match next {
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '<' => angle_depth += 1,
            '>' => angle_depth = angle_depth.saturating_sub(1),
            _ => {}
        }
    }

    source.len()
}
