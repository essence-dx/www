const DEFAULT_LINES_ABOVE: usize = 2;
const DEFAULT_LINES_BELOW: usize = 2;
const DEFAULT_MAX_WIDTH: usize = 100;
const MIN_SOURCE_WIDTH: usize = 20;
const ELLIPSIS: &str = "...";
const TAB_WIDTH: usize = 2;

pub(super) struct DxCodeFrameLocation {
    pub(super) start_line: usize,
    pub(super) start_column: usize,
    pub(super) end_line: usize,
    pub(super) end_column: usize,
}

pub(super) struct DxCodeFrameOptions {
    pub(super) lines_above: usize,
    pub(super) lines_below: usize,
    pub(super) max_width: usize,
}

impl Default for DxCodeFrameOptions {
    fn default() -> Self {
        Self {
            lines_above: DEFAULT_LINES_ABOVE,
            lines_below: DEFAULT_LINES_BELOW,
            max_width: DEFAULT_MAX_WIDTH,
        }
    }
}

pub(super) fn render_dx_code_frame(
    source: &str,
    location: DxCodeFrameLocation,
    options: DxCodeFrameOptions,
) -> Option<String> {
    if location.start_line == 0
        || location.start_column == 0
        || location.end_line == 0
        || location.end_column == 0
        || source.is_empty()
    {
        return None;
    }

    let lines: Vec<&str> = source.lines().collect();
    if location.start_line > lines.len() {
        return None;
    }

    let (end_line, end_column) = normalize_end_location(&location, lines.len());
    let first_visible_line = location.start_line.saturating_sub(options.lines_above + 1) + 1;
    let last_visible_line = (end_line + options.lines_below).min(lines.len());
    let gutter_width = last_visible_line.to_string().len();
    let source_prefix_width = 2 + gutter_width + 3;
    let marker_prefix_width = 2 + gutter_width + 3;
    let source_width = options.max_width.saturating_sub(source_prefix_width);
    if source_width < MIN_SOURCE_WIDTH {
        return None;
    }

    let mut frame = String::new();
    for current in first_visible_line..=last_visible_line {
        let is_marked_line = current >= location.start_line && current <= end_line;
        let marker = if is_marked_line { ">" } else { " " };
        let source_line = lines[current - 1];
        let rendered_line = expand_tabs_for_code_frame(source_line);
        let focus_column =
            focus_column_for_line(&location, end_line, end_column, current, source_line);
        let focus_column = expanded_source_column(source_line, focus_column);
        let truncated = truncate_line_around_column(&rendered_line, focus_column, source_width);

        frame.push_str(&format!(
            "{marker} {current:>gutter_width$} | {}\n",
            truncated.visible
        ));

        if is_marked_line {
            let (range_start, range_end) =
                marker_range_for_line(&location, end_line, end_column, source_line, current);
            let range_start = expanded_source_column(source_line, range_start);
            let range_end = expanded_source_column(source_line, range_end);
            let marker_range = truncated.marker_range(range_start, range_end);
            let caret_padding = marker_prefix_width + marker_range.start.saturating_sub(1);
            frame.push_str(&format!(
                "{}{}\n",
                " ".repeat(caret_padding),
                "^".repeat(marker_range.len)
            ));
        }
    }

    Some(frame)
}

fn normalize_end_location(location: &DxCodeFrameLocation, line_count: usize) -> (usize, usize) {
    let end_line = location.end_line.min(line_count);
    let end_before_start = end_line < location.start_line
        || (end_line == location.start_line && location.end_column <= location.start_column);

    if end_before_start {
        (location.start_line, location.start_column + 1)
    } else {
        (end_line, location.end_column)
    }
}

fn focus_column_for_line(
    location: &DxCodeFrameLocation,
    end_line: usize,
    end_column: usize,
    current_line: usize,
    line: &str,
) -> usize {
    if current_line == location.start_line {
        location.start_column
    } else if current_line == end_line {
        end_column.saturating_sub(1).max(1)
    } else {
        first_non_whitespace_column(line)
    }
}

fn first_non_whitespace_column(line: &str) -> usize {
    line.chars()
        .position(|ch| !ch.is_whitespace())
        .map_or(1, |index| index + 1)
}

fn marker_range_for_line(
    location: &DxCodeFrameLocation,
    end_line: usize,
    end_column: usize,
    line: &str,
    current_line: usize,
) -> (usize, usize) {
    if location.start_line == end_line {
        return (location.start_column, end_column);
    }

    let line_end = line.chars().count() + 1;
    if current_line == location.start_line {
        (location.start_column, line_end)
    } else if current_line == end_line {
        (1, end_column)
    } else {
        (1, line_end)
    }
}

fn expand_tabs_for_code_frame(line: &str) -> String {
    if !line.contains('\t') {
        return line.to_string();
    }

    let mut expanded = String::with_capacity(line.len());
    for ch in line.chars() {
        if ch == '\t' {
            expanded.extend(std::iter::repeat_n(' ', TAB_WIDTH));
        } else {
            expanded.push(ch);
        }
    }
    expanded
}

fn expanded_source_column(line: &str, source_column: usize) -> usize {
    if source_column <= 1 {
        return 1;
    }

    let mut original_column = 1usize;
    let mut expanded_column = 1usize;

    for ch in line.chars() {
        if original_column == source_column {
            return expanded_column;
        }

        expanded_column += if ch == '\t' { TAB_WIDTH } else { 1 };
        original_column += 1;
    }

    expanded_column + source_column.saturating_sub(original_column)
}

struct TruncatedLine {
    visible: String,
    source_start: usize,
    source_end: usize,
    prefix_width: usize,
    display_widths: Vec<usize>,
}

struct VisibleMarkerRange {
    start: usize,
    len: usize,
}

impl TruncatedLine {
    fn marker_range(
        &self,
        source_start_column: usize,
        source_end_column: usize,
    ) -> VisibleMarkerRange {
        let visible_start_column = self.source_start + 1;
        let visible_end_column = self.source_end + 1;
        let clamped_start = source_start_column.clamp(visible_start_column, visible_end_column);
        let clamped_end = source_end_column
            .max(clamped_start + 1)
            .min(visible_end_column.max(clamped_start + 1));
        let start = self.prefix_width
            + self.display_width_between_columns(visible_start_column, clamped_start)
            + 1;
        let len = self
            .display_width_between_columns(clamped_start, clamped_end)
            .max(1);

        VisibleMarkerRange { start, len }
    }

    fn display_width_between_columns(&self, start_column: usize, end_column: usize) -> usize {
        if end_column <= start_column {
            return 0;
        }

        let start_index = start_column.saturating_sub(1);
        let end_index = end_column.saturating_sub(1).min(self.display_widths.len());

        self.display_widths
            .get(start_index..end_index)
            .unwrap_or_default()
            .iter()
            .sum()
    }
}

fn truncate_line_around_column(line: &str, column: usize, max_width: usize) -> TruncatedLine {
    let chars: Vec<char> = line.chars().collect();
    let display_widths: Vec<usize> = chars.iter().map(|ch| display_width_for_char(*ch)).collect();
    let char_count = chars.len();
    if display_widths.iter().sum::<usize>() <= max_width {
        return TruncatedLine {
            visible: render_source_chars(&chars),
            source_start: 0,
            source_end: char_count,
            prefix_width: 0,
            display_widths,
        };
    }

    let focus_index = column.saturating_sub(1).min(char_count);
    let window_focus = focus_index.min(char_count.saturating_sub(1));
    let mut start = 0usize;
    let mut end = char_count;
    let mut leading_ellipsis = false;
    let mut trailing_ellipsis = false;

    for _ in 0..4 {
        let ellipsis_width = usize::from(leading_ellipsis) * ELLIPSIS.len()
            + usize::from(trailing_ellipsis) * ELLIPSIS.len();
        let visible_capacity = max_width
            .saturating_sub(ellipsis_width)
            .max(1)
            .min(char_count);
        let candidate_start = window_focus
            .saturating_sub(visible_capacity / 2)
            .min(char_count.saturating_sub(visible_capacity));
        let candidate_end = (candidate_start + visible_capacity).min(char_count);
        let next_leading = candidate_start > 0;
        let next_trailing = candidate_end < char_count;

        start = candidate_start;
        end = candidate_end;

        if next_leading == leading_ellipsis && next_trailing == trailing_ellipsis {
            break;
        }

        leading_ellipsis = next_leading;
        trailing_ellipsis = next_trailing;
    }

    let mut visible = String::new();
    let prefix_width = if leading_ellipsis { ELLIPSIS.len() } else { 0 };
    if leading_ellipsis {
        visible.push_str(ELLIPSIS);
    }
    for ch in &chars[start..end] {
        render_source_char(&mut visible, *ch);
    }
    if trailing_ellipsis {
        visible.push_str(ELLIPSIS);
    }

    TruncatedLine {
        visible,
        source_start: start,
        source_end: end,
        prefix_width,
        display_widths,
    }
}

fn render_source_chars(chars: &[char]) -> String {
    let mut rendered = String::new();
    for ch in chars {
        render_source_char(&mut rendered, *ch);
    }
    rendered
}

fn render_source_char(rendered: &mut String, ch: char) {
    if ch == '\t' {
        rendered.extend(std::iter::repeat_n(' ', TAB_WIDTH));
    } else {
        rendered.push(ch);
    }
}

fn display_width_for_char(ch: char) -> usize {
    if ch == '\t' { TAB_WIDTH } else { 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_frame_marks_end_of_line_parse_spans_without_panicking() {
        let frame = render_dx_code_frame(
            "[project\nname = \"demo\"\n",
            DxCodeFrameLocation {
                start_line: 1,
                start_column: 9,
                end_line: 1,
                end_column: 10,
            },
            DxCodeFrameOptions::default(),
        )
        .expect("end-of-line span should render");

        assert!(frame.contains("> 1 | [project"), "{frame}");
        assert!(frame.contains('^'), "{frame}");
    }

    #[test]
    fn code_frame_expands_tabs_for_terminal_alignment() {
        let source = "export default function Page() {\n\tconst answer = issue_here;\n}\n";
        let marked_source_line = source
            .lines()
            .nth(1)
            .expect("fixture should include a tabbed source line");
        let start_column = marked_source_line
            .find("issue_here")
            .expect("fixture should include target token")
            + 1;
        let frame = render_dx_code_frame(
            source,
            DxCodeFrameLocation {
                start_line: 2,
                start_column,
                end_line: 2,
                end_column: start_column + "issue_here".len(),
            },
            DxCodeFrameOptions::default(),
        )
        .expect("tabbed source span should render");
        let rendered_source_line = frame
            .lines()
            .find(|line| line.contains("> 2 |"))
            .expect("frame should include the marked source line");
        let caret_line = frame
            .lines()
            .find(|line| line.contains('^'))
            .expect("frame should include a caret line");

        assert!(!rendered_source_line.contains('\t'), "{frame}");
        assert!(
            rendered_source_line.contains("  const answer = issue_here;"),
            "{frame}"
        );
        assert_eq!(
            caret_line.find('^'),
            rendered_source_line.find("issue_here"),
            "{frame}"
        );
    }
}
