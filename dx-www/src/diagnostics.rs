//! DX-WWW diagnostics modeled after production code-frame error UX.

#[path = "diagnostics/code_frame.rs"]
mod code_frame;
#[path = "diagnostics/contract.rs"]
mod contract;

use code_frame::{DxCodeFrameLocation, DxCodeFrameOptions, render_dx_code_frame};
pub use contract::{
    DX_DIAGNOSTIC_CODE_FRAME_CONTRACT, DX_DIAGNOSTIC_CODE_FRAME_RECEIPT_VIEW,
    DxDiagnosticCodeFrameContract, DxDiagnosticCodeFrameReceiptView,
    dx_diagnostic_code_frame_contract, dx_diagnostic_code_frame_receipt_view,
};

/// Severity for DX-WWW diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DxDiagnosticSeverity {
    /// Build or runtime warning.
    Warning,
    /// Build or runtime error.
    Error,
}

/// A one-based source range for a DX-WWW diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DxDiagnosticSourceRange {
    /// Starting line number.
    pub start_line: usize,
    /// Starting column number.
    pub start_column: usize,
    /// Ending line number.
    pub end_line: usize,
    /// Ending column number, treated as half-open on the final line.
    pub end_column: usize,
}

/// Caller-provided code-frame rendering options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DxDiagnosticCodeFrameOptions {
    /// Number of context lines before the marked range.
    pub lines_above: usize,
    /// Number of context lines after the marked range.
    pub lines_below: usize,
    /// Maximum rendered frame width in terminal columns.
    pub max_width: usize,
}

impl Default for DxDiagnosticCodeFrameOptions {
    fn default() -> Self {
        Self {
            lines_above: 2,
            lines_below: 2,
            max_width: 100,
        }
    }
}

/// A DX-WWW diagnostic that can be rendered in the CLI or browser overlay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxDiagnostic {
    /// Diagnostic severity.
    pub severity: DxDiagnosticSeverity,
    /// Short title.
    pub title: String,
    /// Human-readable message.
    pub message: String,
    /// Stable machine-readable diagnostic code.
    pub code: Option<String>,
    /// Source file path, if known.
    pub file: Option<String>,
    /// One-based line number, if known.
    pub line: Option<usize>,
    /// One-based column number, if known.
    pub column: Option<usize>,
    /// Source range, if the diagnostic points at a token or span.
    pub range: Option<DxDiagnosticSourceRange>,
    /// Source text, if available.
    pub source: Option<String>,
    /// Fix-oriented hint.
    pub hint: Option<String>,
}

impl DxDiagnostic {
    /// Create an error diagnostic.
    #[must_use]
    pub fn error(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: DxDiagnosticSeverity::Error,
            title: title.into(),
            message: message.into(),
            code: None,
            file: None,
            line: None,
            column: None,
            range: None,
            source: None,
            hint: None,
        }
    }

    /// Create a warning diagnostic.
    #[must_use]
    pub fn warning(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: DxDiagnosticSeverity::Warning,
            title: title.into(),
            message: message.into(),
            code: None,
            file: None,
            line: None,
            column: None,
            range: None,
            source: None,
            hint: None,
        }
    }

    /// Attach source location and source text.
    #[must_use]
    pub fn with_source(
        mut self,
        file: impl Into<String>,
        line: usize,
        column: usize,
        source: impl Into<String>,
    ) -> Self {
        self.file = Some(file.into());
        self.line = Some(line);
        self.column = Some(column);
        self.range = None;
        self.source = Some(source.into());
        self
    }

    /// Attach source text and a source range.
    #[must_use]
    pub fn with_source_range(
        mut self,
        file: impl Into<String>,
        start_line: usize,
        start_column: usize,
        end_line: usize,
        end_column: usize,
        source: impl Into<String>,
    ) -> Self {
        self.file = Some(file.into());
        self.line = Some(start_line);
        self.column = Some(start_column);
        self.range = Some(DxDiagnosticSourceRange {
            start_line,
            start_column,
            end_line,
            end_column,
        });
        self.source = Some(source.into());
        self
    }

    /// Attach a stable machine-readable diagnostic code.
    #[must_use]
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        let code = code.into();
        let code = code.trim();
        if !code.is_empty() {
            self.code = Some(code.to_string());
        }
        self
    }

    /// Attach a fix-oriented hint.
    #[must_use]
    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// Return the fix-oriented next action for CLI and overlay consumers.
    #[must_use]
    pub fn next_action(&self) -> Option<&str> {
        self.hint
            .as_deref()
            .map(str::trim)
            .filter(|hint| !hint.is_empty())
    }

    /// Render a compact code frame.
    #[must_use]
    pub fn code_frame(&self) -> Option<String> {
        self.code_frame_with_options(DxDiagnosticCodeFrameOptions::default())
    }

    /// Render a compact code frame with caller-provided display options.
    #[must_use]
    pub fn code_frame_with_options(&self, options: DxDiagnosticCodeFrameOptions) -> Option<String> {
        let source = self.source.as_ref()?;
        let line_number = self.line?;
        let column = self.column.unwrap_or(1).max(1);
        let range = self.range.unwrap_or(DxDiagnosticSourceRange {
            start_line: line_number,
            start_column: column,
            end_line: line_number,
            end_column: column + 1,
        });
        render_dx_code_frame(
            source,
            DxCodeFrameLocation {
                start_line: range.start_line,
                start_column: range.start_column,
                end_line: range.end_line,
                end_column: range.end_column,
            },
            DxCodeFrameOptions {
                lines_above: options.lines_above,
                lines_below: options.lines_below,
                max_width: options.max_width,
            },
        )
    }

    /// Render for terminal output.
    #[must_use]
    pub fn render_terminal(&self) -> String {
        self.render_terminal_with_options(DxDiagnosticCodeFrameOptions::default())
    }

    /// Render for terminal output with caller-provided code-frame display options.
    #[must_use]
    pub fn render_terminal_with_options(&self, options: DxDiagnosticCodeFrameOptions) -> String {
        let severity = match self.severity {
            DxDiagnosticSeverity::Warning => "warning",
            DxDiagnosticSeverity::Error => "error",
        };
        let mut output = format!("DX-WWW {severity}: {}\n{}\n", self.title, self.message);
        if let Some(code) = &self.code {
            output.push_str(&format!("code: {code}\n"));
        }
        if let Some(file) = &self.file {
            output.push_str(&format!("--> {file}"));
            if let Some(location) = self.rendered_location() {
                output.push_str(&location);
            }
            output.push('\n');
        }
        if let Some(frame) = self.code_frame_with_options(options) {
            output.push_str(&frame);
        }
        if let Some(next_action) = self.next_action() {
            output.push_str(&format!("next action: {next_action}\n"));
        }
        if let Some(hint) = &self.hint {
            output.push_str(&format!("hint: {hint}\n"));
        }
        output
    }

    fn rendered_location(&self) -> Option<String> {
        if let Some(range) = self.range {
            if range.start_line == range.end_line && range.start_column + 1 == range.end_column {
                return Some(format!(":{}:{}", range.start_line, range.start_column));
            }

            if range.start_line == range.end_line {
                return Some(format!(
                    ":{}:{}-{}",
                    range.start_line, range.start_column, range.end_column
                ));
            }

            return Some(format!(
                ":{}:{}-{}:{}",
                range.start_line, range.start_column, range.end_line, range.end_column
            ));
        }

        let line = self.line?;
        let mut location = format!(":{line}");
        if let Some(column) = self.column {
            location.push_str(&format!(":{column}"));
        }
        Some(location)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_dx_www_code_frame() {
        let diagnostic = DxDiagnostic::error("Compile failed", "Unexpected token")
            .with_source(
                "app/page.tsx",
                2,
                8,
                "export default function Page() {\n  return <main>\n}\n",
            )
            .with_hint("Close the JSX tag or remove the incomplete element.");
        let rendered = diagnostic.render_terminal();
        assert!(rendered.contains("DX-WWW error: Compile failed"));
        assert!(rendered.contains("--> app/page.tsx:2:8"));
        assert!(rendered.contains("> 2 |   return <main>"));
        assert!(rendered.contains("hint: Close the JSX tag"));
    }

    #[test]
    fn render_terminal_includes_dx_next_action() {
        let diagnostic = DxDiagnostic::error("Compile failed", "Unexpected token")
            .with_hint("Fix the marked source before rebuilding.");

        let rendered = diagnostic.render_terminal();

        assert!(
            rendered.contains("next action: Fix the marked source before rebuilding."),
            "{rendered}"
        );
        assert!(
            rendered.contains("hint: Fix the marked source before rebuilding."),
            "{rendered}"
        );
        assert_eq!(
            diagnostic.next_action(),
            Some("Fix the marked source before rebuilding.")
        );
    }

    #[test]
    fn render_terminal_includes_dx_diagnostic_code() {
        let diagnostic = DxDiagnostic::error("Compile failed", "Unexpected token")
            .with_code("dx.source.parse_error")
            .with_hint("Fix the marked source before rebuilding.");

        let rendered = diagnostic.render_terminal();

        assert!(
            rendered.contains("code: dx.source.parse_error"),
            "{rendered}"
        );
    }

    #[test]
    fn serializes_dx_diagnostic_severity_as_snake_case() {
        assert_eq!(
            serde_json::to_value(DxDiagnosticSeverity::Warning)
                .expect("warning severity should serialize"),
            serde_json::json!("warning")
        );
        assert_eq!(
            serde_json::to_value(DxDiagnosticSeverity::Error)
                .expect("error severity should serialize"),
            serde_json::json!("error")
        );
    }

    #[test]
    fn truncates_long_dx_www_code_frame_around_column() {
        let long_line = format!(
            "const before = \"{}\"; const target = issue_here; const after = \"{}\";",
            "a".repeat(96),
            "b".repeat(96)
        );
        let column = long_line
            .find("issue_here")
            .expect("fixture should include target token")
            + 1;
        let source = format!("export default function Page() {{\n{long_line}\n}}\n");
        let diagnostic = DxDiagnostic::error("Compile failed", "Unexpected token").with_source(
            "app/page.tsx",
            2,
            column,
            source,
        );

        let frame = diagnostic
            .code_frame()
            .expect("diagnostic should render a focused frame");
        let marked_line = frame
            .lines()
            .find(|line| line.contains("> 2 |"))
            .expect("frame should include the marked source line");
        let caret_line = frame
            .lines()
            .find(|line| line.contains('^'))
            .expect("frame should include a caret line");

        assert!(marked_line.contains("..."), "{marked_line}");
        assert!(marked_line.contains("issue_here"), "{marked_line}");
        assert!(marked_line.chars().count() <= 110, "{marked_line}");
        assert!(!marked_line.contains(&"a".repeat(64)), "{marked_line}");
        assert!(caret_line.chars().count() <= 110, "{caret_line}");
    }

    #[test]
    fn renders_dx_www_token_range_marker() {
        let source = "export default function Page() {\n  return issue_here + 1\n}\n";
        let start_column = source
            .lines()
            .nth(1)
            .expect("fixture should include source line")
            .find("issue_here")
            .expect("fixture should include target token")
            + 1;
        let diagnostic = DxDiagnostic::error("Compile failed", "Unexpected token")
            .with_source_range(
                "app/page.tsx",
                2,
                start_column,
                2,
                start_column + "issue_here".len(),
                source,
            );

        let frame = diagnostic
            .code_frame()
            .expect("diagnostic should render a token frame");
        let caret_line = frame
            .lines()
            .find(|line| line.contains('^'))
            .expect("frame should include a caret line");

        assert!(
            caret_line.contains("^^^^^^^^^^"),
            "range should underline the full token: {caret_line}"
        );
    }

    #[test]
    fn renders_dx_www_code_frame_with_caller_width() {
        let long_line = format!(
            "const before = \"{}\"; const target = issue_here; const after = \"{}\";",
            "a".repeat(48),
            "b".repeat(48)
        );
        let column = long_line
            .find("issue_here")
            .expect("fixture should include target token")
            + 1;
        let source = format!("export default function Page() {{\n{long_line}\n}}\n");
        let diagnostic = DxDiagnostic::error("Compile failed", "Unexpected token").with_source(
            "app/page.tsx",
            2,
            column,
            source,
        );

        let frame = diagnostic
            .code_frame_with_options(DxDiagnosticCodeFrameOptions {
                max_width: 64,
                ..DxDiagnosticCodeFrameOptions::default()
            })
            .expect("diagnostic should render with caller width");
        let marked_line = frame
            .lines()
            .find(|line| line.contains("> 2 |"))
            .expect("frame should include the marked source line");

        assert!(marked_line.contains("issue_here"), "{marked_line}");
        assert!(marked_line.contains("..."), "{marked_line}");
        assert!(marked_line.chars().count() <= 64, "{marked_line}");
    }

    #[test]
    fn render_terminal_includes_dx_www_source_range_header() {
        let source = "export default function Page() {\n  return (\n    <main>\n  )\n}\n";
        let diagnostic = DxDiagnostic::error("Compile failed", "Unclosed JSX element")
            .with_source_range("app/page.tsx", 2, 10, 4, 4, source);

        let rendered = diagnostic.render_terminal();

        assert!(rendered.contains("--> app/page.tsx:2:10-4:4"), "{rendered}");
    }

    #[test]
    fn render_terminal_with_options_respects_caller_width() {
        let long_line = format!(
            "const before = \"{}\"; const target = issue_here; const after = \"{}\";",
            "a".repeat(48),
            "b".repeat(48)
        );
        let column = long_line
            .find("issue_here")
            .expect("fixture should include target token")
            + 1;
        let source = format!("export default function Page() {{\n{long_line}\n}}\n");
        let diagnostic = DxDiagnostic::error("Compile failed", "Unexpected token").with_source(
            "app/page.tsx",
            2,
            column,
            source,
        );

        let rendered = diagnostic.render_terminal_with_options(DxDiagnosticCodeFrameOptions {
            max_width: 64,
            ..DxDiagnosticCodeFrameOptions::default()
        });
        let marked_line = rendered
            .lines()
            .find(|line| line.contains("> 2 |"))
            .expect("terminal rendering should include the marked source line");

        assert!(marked_line.contains("issue_here"), "{marked_line}");
        assert!(marked_line.contains("..."), "{marked_line}");
        assert!(marked_line.chars().count() <= 64, "{marked_line}");
    }

    #[test]
    fn multiline_range_truncates_middle_lines_around_content() {
        let middle_line = format!("{}middle_anchor = call();", " ".repeat(96));
        let source = format!(
            "export default function Page() {{\n  const range_start = true;\n{middle_line}\n  const range_end = true;\n}}\n"
        );
        let diagnostic = DxDiagnostic::error("Compile failed", "Invalid component body")
            .with_source_range("app/page.tsx", 2, 3, 4, 25, source);

        let frame = diagnostic
            .code_frame_with_options(DxDiagnosticCodeFrameOptions {
                lines_above: 0,
                lines_below: 0,
                max_width: 72,
            })
            .expect("diagnostic should render the multi-line range");
        let rendered_middle = frame
            .lines()
            .find(|line| line.contains("> 3 |"))
            .expect("frame should include the middle marked line");

        assert!(rendered_middle.contains("middle_anchor"), "{frame}");
        assert!(rendered_middle.contains("..."), "{rendered_middle}");
        assert!(rendered_middle.chars().count() <= 72, "{rendered_middle}");
    }

    #[test]
    fn diagnostic_code_frame_contract_keeps_next_code_frame_adapter_boundary() {
        let contract = dx_diagnostic_code_frame_contract();

        assert_eq!(contract.renderer, "dx-www.diagnostics.code-frame");
        assert_eq!(
            contract.upstream_reference,
            "vendor/next-rust/crates/next-code-frame"
        );
        assert_eq!(
            contract.boundary,
            "adapter-boundary: diagnostics formatting only"
        );
        assert_eq!(contract.dx_brand, "DX-WWW");
        assert!(contract.source_of_truth.contains("dx-check"));
        assert!(!contract.requires_react);
        assert!(!contract.requires_rsc);
        assert!(!contract.requires_node);
        assert!(!contract.requires_napi);
        assert!(!contract.requires_npm);
        assert!(!contract.requires_node_modules);
        assert!(!contract.requires_turborepo);
        assert!(!contract.public_turbopack_dependency);
        assert!(!contract.runtime_takeover);
        assert!(!contract.next_code_frame_parity_claimed);
        assert!(contract.supports_source_ranges);
        assert!(contract.supports_caller_width);
        assert!(contract.supports_multiline_ranges);

        let receipt_view = dx_diagnostic_code_frame_receipt_view();
        assert_eq!(receipt_view.schema, "dx.diagnostics.code_frame.contract");
        assert_eq!(receipt_view.renderer, contract.renderer);
        assert_eq!(receipt_view.boundary, contract.boundary);
        assert_eq!(receipt_view.upstream_reference, contract.upstream_reference);
        assert_eq!(receipt_view.dx_brand, contract.dx_brand);
        assert_eq!(receipt_view.source_of_truth, contract.source_of_truth);
        assert!(!receipt_view.full_next_parity_claimed);
        assert!(!receipt_view.runtime_takeover);
        assert!(receipt_view.forbidden_foundations.contains(&"react"));
        assert!(
            receipt_view
                .forbidden_foundations
                .contains(&"react-server-components")
        );
        assert!(receipt_view.forbidden_foundations.contains(&"node_modules"));
        assert!(
            receipt_view
                .forbidden_foundations
                .contains(&"public-turbopack-dependency")
        );
        assert!(receipt_view.supported_features.contains(&"source-ranges"));
        assert!(receipt_view.supported_features.contains(&"caller-width"));
        assert!(
            receipt_view
                .supported_features
                .contains(&"multi-line-ranges")
        );
    }
}
