//! # Error Overlay
//!
//! Displays compilation and runtime errors in the browser during development.

#![allow(dead_code)]

use crate::diagnostics::{DxDiagnostic, DxDiagnosticSeverity};
use crate::error::{DxError, ErrorOverlayData};

// =============================================================================
// Error Overlay
// =============================================================================

/// Error overlay for displaying errors in the browser.
#[derive(Debug, Default)]
pub struct ErrorOverlay {
    /// Currently displayed error
    current_error: Option<ErrorInfo>,
    /// Error history
    history: Vec<ErrorInfo>,
}

impl ErrorOverlay {
    /// Create a new error overlay.
    pub fn new() -> Self {
        Self {
            current_error: None,
            history: Vec::new(),
        }
    }

    /// Show an error.
    pub fn show(&mut self, error: &DxError) {
        let info = ErrorInfo::from_error(error);
        self.history.push(info.clone());
        self.current_error = Some(info);
    }

    /// Show a structured DX diagnostic.
    pub fn show_diagnostic(&mut self, diagnostic: &DxDiagnostic) {
        let info = ErrorInfo::from_diagnostic(diagnostic);
        self.history.push(info.clone());
        self.current_error = Some(info);
    }

    /// Show a structured overlay payload.
    pub fn show_payload(&mut self, payload: &ErrorOverlayData) {
        let info = ErrorInfo::from_payload(payload);
        self.history.push(info.clone());
        self.current_error = Some(info);
    }

    /// Clear the current error.
    pub fn clear(&mut self) {
        self.current_error = None;
    }

    /// Get the current error.
    pub fn current(&self) -> Option<&ErrorInfo> {
        self.current_error.as_ref()
    }

    /// Get error history.
    pub fn history(&self) -> &[ErrorInfo] {
        &self.history
    }

    /// Clear history.
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Generate HTML for the error overlay.
    pub fn to_html(&self) -> String {
        match &self.current_error {
            Some(error) => generate_error_html(error),
            None => String::new(),
        }
    }

    /// Generate JavaScript for the error overlay.
    pub fn to_script(&self) -> String {
        OVERLAY_SCRIPT.to_string()
    }
}

// =============================================================================
// Error Info
// =============================================================================

/// Information about an error for display.
#[derive(Debug, Clone)]
pub struct ErrorInfo {
    /// Error title
    pub title: String,
    /// Error message
    pub message: String,
    /// Source file path
    pub file: Option<String>,
    /// Line number
    pub line: Option<usize>,
    /// Column number
    pub column: Option<usize>,
    /// Code snippet with context
    pub code_snippet: Option<String>,
    /// Stack trace
    pub stack_trace: Option<String>,
    /// Error type
    pub error_type: ErrorType,
    /// Machine-readable diagnostic severity.
    pub severity: DxDiagnosticSeverity,
    /// Stable machine-readable diagnostic code.
    pub diagnostic_code: Option<String>,
    /// Rendered DX code frame.
    pub code_frame: Option<String>,
    /// Fix-oriented next action.
    pub next_action: Option<String>,
    /// Suggested fixes.
    pub suggestions: Vec<String>,
}

impl ErrorInfo {
    /// Create error info from a DxError.
    pub fn from_error(error: &DxError) -> Self {
        let payload = ErrorOverlayData::from_error(error);
        let title = title_for_payload(&payload);
        match error {
            DxError::ParseError {
                message,
                file,
                line,
                column,
                src,
                ..
            } => Self {
                title,
                message: message.clone(),
                file: Some(file.to_string_lossy().to_string()),
                line: line.map(|value| value as usize),
                column: column.map(|value| value as usize),
                code_snippet: src.clone(),
                stack_trace: None,
                error_type: ErrorType::Parse,
                severity: payload.severity,
                diagnostic_code: payload.diagnostic_code,
                code_frame: payload.code_frame,
                next_action: payload.next_action,
                suggestions: payload.suggestions,
            },
            DxError::CompilationError {
                message, file, src, ..
            } => Self {
                title,
                message: message.clone(),
                file: Some(file.to_string_lossy().to_string()),
                line: None,
                column: None,
                code_snippet: src.clone(),
                stack_trace: None,
                error_type: ErrorType::Compilation,
                severity: payload.severity,
                diagnostic_code: payload.diagnostic_code,
                code_frame: payload.code_frame,
                next_action: payload.next_action,
                suggestions: payload.suggestions,
            },
            _ => Self {
                title,
                message: error.to_string(),
                file: None,
                line: None,
                column: None,
                code_snippet: None,
                stack_trace: None,
                error_type: ErrorType::Unknown,
                severity: payload.severity,
                diagnostic_code: payload.diagnostic_code,
                code_frame: payload.code_frame,
                next_action: payload.next_action,
                suggestions: payload.suggestions,
            },
        }
    }

    /// Create error info from a structured DX diagnostic.
    pub fn from_diagnostic(diagnostic: &DxDiagnostic) -> Self {
        let payload = ErrorOverlayData::from_diagnostic(diagnostic);
        Self {
            title: diagnostic.title.clone(),
            message: diagnostic.message.clone(),
            file: diagnostic.file.clone(),
            line: diagnostic.line,
            column: diagnostic.column,
            code_snippet: diagnostic.source.clone(),
            stack_trace: None,
            error_type: ErrorType::Diagnostic,
            severity: payload.severity,
            diagnostic_code: payload.diagnostic_code,
            code_frame: payload.code_frame,
            next_action: payload.next_action,
            suggestions: payload.suggestions,
        }
    }

    /// Create error info from a structured overlay payload.
    pub fn from_payload(payload: &ErrorOverlayData) -> Self {
        Self {
            title: title_for_payload(payload),
            message: payload.message.clone(),
            file: payload
                .file_path
                .as_ref()
                .map(|path| path.display().to_string()),
            line: payload.line,
            column: payload.column,
            code_snippet: payload.code_context.clone(),
            stack_trace: stack_trace_text(payload),
            error_type: error_type_for_payload(payload.error_type),
            severity: payload.severity,
            diagnostic_code: payload.diagnostic_code.clone(),
            code_frame: payload.code_frame.clone(),
            next_action: payload.next_action.clone(),
            suggestions: payload.suggestions.clone(),
        }
    }
}

/// Type of error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorType {
    /// General DX diagnostic
    Diagnostic,
    /// Parse error
    Parse,
    /// Compilation error
    Compilation,
    /// Template error
    Template,
    /// Configuration error
    Config,
    /// Data loading error
    DataLoad,
    /// API error
    Api,
    /// Runtime error
    Runtime,
    /// Network error
    Network,
    /// Unknown error
    Unknown,
}

// =============================================================================
// HTML Generation
// =============================================================================

fn title_for_payload(payload: &ErrorOverlayData) -> String {
    let title = payload.title.trim();
    if !title.is_empty() {
        return title.to_string();
    }

    if let Some(title) = payload
        .diagnostic_code
        .as_deref()
        .and_then(title_from_diagnostic_code)
    {
        return title;
    }

    match payload.severity {
        DxDiagnosticSeverity::Warning => "DX warning".to_string(),
        DxDiagnosticSeverity::Error => "DX error".to_string(),
    }
}

fn title_from_diagnostic_code(code: &str) -> Option<String> {
    let code = code.trim();
    if code.is_empty() {
        return None;
    }

    let code = code
        .strip_prefix("dx.")
        .or_else(|| code.strip_prefix("DX."))
        .or_else(|| code.strip_prefix("dx_"))
        .or_else(|| code.strip_prefix("DX_"))
        .unwrap_or(code);

    let words = code
        .split(['.', '_', '-'])
        .filter(|part| !part.is_empty())
        .map(|part| part.to_ascii_lowercase())
        .collect::<Vec<_>>();
    if words.is_empty() {
        return None;
    }

    Some(format!("DX {}", words.join(" ")))
}

fn error_type_for_payload(error_type: crate::error::ErrorType) -> ErrorType {
    match error_type {
        crate::error::ErrorType::Diagnostic => ErrorType::Diagnostic,
        crate::error::ErrorType::Compilation => ErrorType::Compilation,
        crate::error::ErrorType::Runtime => ErrorType::Runtime,
        crate::error::ErrorType::DataLoad => ErrorType::DataLoad,
        crate::error::ErrorType::Api => ErrorType::Api,
        crate::error::ErrorType::Config => ErrorType::Config,
    }
}

fn stack_trace_text(payload: &ErrorOverlayData) -> Option<String> {
    let frames = payload.stack_trace.as_ref()?;
    if frames.is_empty() {
        return None;
    }

    Some(
        frames
            .iter()
            .map(|frame| {
                let location = frame.file.as_ref().map_or_else(String::new, |file| {
                    let mut location = format!(" ({file}", file = file.display());
                    if let Some(line) = frame.line {
                        location.push_str(&format!(":{line}"));
                        if let Some(column) = frame.column {
                            location.push_str(&format!(":{column}"));
                        }
                    }
                    location.push(')');
                    location
                });
                format!("{}{}", frame.function, location)
            })
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

/// Generate HTML for an error.
fn generate_error_html(error: &ErrorInfo) -> String {
    let file_info = match (&error.file, error.line, error.column) {
        (Some(file), Some(line), Some(col)) => format!("{} ({}:{})", file, line, col),
        (Some(file), Some(line), None) => format!("{} (line {})", file, line),
        (Some(file), None, None) => file.clone(),
        _ => String::new(),
    };

    let display_code = error.code_frame.as_ref().or(error.code_snippet.as_ref());
    let code_block = display_code
        .map(|code| format!(r#"<pre class="dx-error-code">{}</pre>"#, html_escape(code)))
        .unwrap_or_default();
    let diagnostic_code = error.diagnostic_code.as_deref().unwrap_or_default();
    let diagnostic_code_block = if diagnostic_code.is_empty() {
        String::new()
    } else {
        format!(
            r#"<p class="dx-error-diagnostic-code">{}</p>"#,
            html_escape(diagnostic_code)
        )
    };

    let stack_block = error
        .stack_trace
        .as_ref()
        .map(|stack| {
            format!(
                r#"<pre class="dx-error-stack">{}</pre>"#,
                html_escape(stack)
            )
        })
        .unwrap_or_default();
    let suggestions_block = suggestion_items_html(&error.suggestions);
    let next_action = error
        .next_action
        .as_deref()
        .or_else(|| error.suggestions.first().map(String::as_str))
        .unwrap_or("Fix the diagnostic and save to reload.");

    format!(
        r#"
<div id="dx-error-overlay" class="dx-error-overlay" data-dx-error-severity="{}" data-dx-error-code="{}">
    <div class="dx-error-container">
        <div class="dx-error-header">
            <span class="dx-error-icon">DX</span>
            <h1 class="dx-error-title">{}</h1>
            <button class="dx-error-close" onclick="window.__DX_HIDE_ERROR__()">x</button>
        </div>
        <div class="dx-error-body">
            <p class="dx-error-message">{}</p>
            {}
            {}
            {}
            {}
            {}
        </div>
        <div class="dx-error-footer">
            <span class="dx-error-hint">{}</span>
        </div>
    </div>
</div>
"#,
        severity_label(error.severity),
        html_escape(diagnostic_code),
        html_escape(&error.title),
        html_escape(&error.message),
        diagnostic_code_block,
        if !file_info.is_empty() {
            format!(
                r#"<p class="dx-error-file">{}</p>"#,
                html_escape(&file_info)
            )
        } else {
            String::new()
        },
        code_block,
        stack_block,
        suggestions_block,
        html_escape(next_action)
    )
}

fn suggestion_items_html(suggestions: &[String]) -> String {
    let items = suggestions
        .iter()
        .map(|suggestion| suggestion.trim())
        .filter(|suggestion| !suggestion.is_empty())
        .map(|suggestion| format!("<li>{}</li>", html_escape(suggestion)))
        .collect::<String>();

    if items.is_empty() {
        String::new()
    } else {
        format!(r#"<ul class="dx-error-suggestions">{items}</ul>"#)
    }
}

fn severity_label(severity: DxDiagnosticSeverity) -> &'static str {
    match severity {
        DxDiagnosticSeverity::Warning => "warning",
        DxDiagnosticSeverity::Error => "error",
    }
}

/// Escape HTML special characters.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

// =============================================================================
// Overlay Script
// =============================================================================

/// JavaScript for the error overlay.
const OVERLAY_SCRIPT: &str = r#"
(function() {
    const DX_FEEDBACK_ERRORS_ENDPOINT = '/_dx/feedback/errors';

    function optionalPayloadText(value) {
        return typeof value === 'string' ? value : '';
    }

    function setOptionalText(node, value) {
        const text = optionalPayloadText(value);
        node.textContent = text;
        node.hidden = text.length === 0;
    }

    function formatPayloadLocation(normalized) {
        const file = normalized.file_path || normalized.file || '';
        if (!file) {
            return '';
        }
        const line = Number.isInteger(normalized.line) ? normalized.line : null;
        const column = Number.isInteger(normalized.column) ? normalized.column : null;
        if (line === null) {
            return String(file);
        }
        if (column === null) {
            return String(file) + ' (line ' + line + ')';
        }
        return String(file) + ' (' + line + ':' + column + ')';
    }

    function titleFromDiagnosticCode(code) {
        const text = optionalPayloadText(code).trim();
        if (!text) {
            return '';
        }
        const codeWithoutPrefix = text.replace(/^(dx[._])/i, '');
        const words = codeWithoutPrefix
            .split(/[._-]+/)
            .map((part) => part.trim().toLowerCase())
            .filter(Boolean);
        return words.length > 0 ? 'DX ' + words.join(' ') : '';
    }

    function normalizeOverlayPayload(payload) {
        const normalized = typeof payload === 'object' && payload !== null
            ? Object.assign({}, payload)
            : { message: String(payload || '') };
        const severity = overlaySeverityName(normalized);
        const diagnosticCode = optionalPayloadText(normalized.diagnostic_code)
            || optionalPayloadText(normalized.code)
            || optionalPayloadText(normalized.rule);
        const suggestions = collectPayloadSuggestions(normalized, [
            ['suggestions'],
            ['suggestion'],
            ['hints'],
            ['hint'],
            ['actions'],
            ['action'],
            ['fixes'],
            ['fix'],
            ['next_actions'],
            ['nextActions'],
            ['remediation'],
            ['diagnostic', 'suggestions'],
            ['diagnostic', 'hints'],
            ['diagnostic', 'remediation']
        ]);
        const explicitTitle = optionalPayloadText(normalized.title);
        const inferredTitle = titleFromDiagnosticCode(diagnosticCode);
        const directFilePath = firstPayloadTextAtPath(normalized, [
            ['file_path'],
            ['file'],
            ['source_path'],
            ['sourcePath'],
            ['path'],
            ['source_location', 'path'],
            ['source_location', 'file'],
            ['sourceLocation', 'path'],
            ['sourceLocation', 'file'],
            ['source', 'path'],
            ['source', 'file'],
            ['location', 'file'],
            ['span', 'source', 'path']
        ]);
        const directLine = firstPayloadIntegerAtPath(normalized, [
            ['line'],
            ['lineNumber'],
            ['start_line'],
            ['startLine'],
            ['source_location', 'line'],
            ['sourceLocation', 'line'],
            ['location', 'line'],
            ['span', 'start', 'line']
        ]);
        const directColumn = firstPayloadIntegerAtPath(normalized, [
            ['column'],
            ['columnNumber'],
            ['start_column'],
            ['startColumn'],
            ['source_location', 'column'],
            ['sourceLocation', 'column'],
            ['location', 'column'],
            ['span', 'start', 'column']
        ]);
        const directCodeFrame = firstPayloadTextAtPath(normalized, [
            ['code_frame'],
            ['codeFrame'],
            ['frame'],
            ['renderedCodeFrame'],
            ['diagnostic', 'code_frame'],
            ['diagnostic', 'codeFrame'],
            ['codeFrame', 'rendered'],
            ['code_frame', 'rendered']
        ]);
        const directCodeContext = firstPayloadTextAtPath(normalized, [
            ['code_context'],
            ['codeContext'],
            ['source'],
            ['source', 'content'],
            ['source', 'text'],
            ['source', 'snippet']
        ]);
        const directNextAction = firstPayloadTextAtPath(normalized, [
            ['next_action'],
            ['nextAction'],
            ['hint'],
            ['hint', 'message'],
            ['hint', 'title'],
            ['action'],
            ['fix'],
            ['remediation'],
            ['diagnostic', 'remediation']
        ]) || feedbackNextActionText(normalized.hint);

        normalized.severity = severity;
        normalized.diagnostic_code = diagnosticCode;
        normalized.suggestions = suggestions;
        if (directFilePath) {
            normalized.file_path = directFilePath;
        }
        if (Number.isInteger(directLine)) {
            normalized.line = directLine;
        }
        if (Number.isInteger(directColumn)) {
            normalized.column = directColumn;
        }
        normalized.code_frame = directCodeFrame || optionalPayloadText(normalized.code_frame);
        normalized.code_context = directCodeContext || optionalPayloadText(normalized.code_context);
        normalized.next_action = directNextAction || optionalPayloadText(normalized.next_action);
        normalized.title = explicitTitle
            || inferredTitle
            || optionalPayloadText(normalized.error_type)
            || (severity === 'warning' ? 'DX warning' : 'DX error');
        return normalized;
    }

    function firstPayloadText(payload, keys) {
        for (const key of keys) {
            const text = optionalPayloadText(payload[key]).trim();
            if (text) {
                return text;
            }
        }
        return '';
    }

    function firstPayloadInteger(payload, keys) {
        for (const key of keys) {
            const value = payload[key];
            if (Number.isInteger(value)) {
                return value;
            }
            if (typeof value === 'string' && value.trim()) {
                const parsed = Number(value);
                if (Number.isInteger(parsed)) {
                    return parsed;
                }
            }
        }
        return undefined;
    }

    function payloadValueAtPath(payload, path) {
        let current = payload;
        for (const key of path) {
            if (typeof current !== 'object' || current === null) {
                return undefined;
            }
            current = current[key];
        }
        return current;
    }

    function firstPayloadTextAtPath(payload, paths) {
        for (const path of paths) {
            const text = optionalPayloadText(payloadValueAtPath(payload, path)).trim();
            if (text) {
                return text;
            }
        }
        return '';
    }

    function firstPayloadIntegerAtPath(payload, paths) {
        for (const path of paths) {
            const value = payloadValueAtPath(payload, path);
            if (Number.isInteger(value)) {
                return value;
            }
            if (typeof value === 'string' && value.trim()) {
                const parsed = Number(value);
                if (Number.isInteger(parsed)) {
                    return parsed;
                }
            }
        }
        return undefined;
    }

    function severityText(value) {
        if (typeof value === 'string') {
            return value;
        }
        if (typeof value === 'object' && value !== null) {
            return firstPayloadTextAtPath(value, [
                ['severity'],
                ['diagnostic', 'severity'],
                ['diagnostic', 'level'],
                ['level'],
                ['error', 'severity'],
                ['issue', 'severity']
            ]);
        }
        return '';
    }

    function normalizeSeverityName(value) {
        const severity = severityText(value).trim().toLowerCase();
        if (severity === 'error' || severity === 'fatal' || severity === 'failure' || severity === 'fail') {
            return 'error';
        }
        if (severity === 'warning' || severity === 'warn') {
            return 'warning';
        }
        if (severity === 'info' || severity === 'notice' || severity === 'hint') {
            return 'info';
        }
        return 'unknown';
    }

    function overlaySeverityName(value) {
        const severity = normalizeSeverityName(value);
        return severity === 'info' || severity === 'warning' ? 'warning' : 'error';
    }

    function suggestionText(value) {
        if (typeof value === 'string') {
            return value.trim();
        }
        if (typeof value === 'object' && value !== null) {
            return firstPayloadTextAtPath(value, [
                ['message'],
                ['title'],
                ['text'],
                ['label'],
                ['description'],
                ['hint'],
                ['hint', 'message'],
                ['hint', 'title'],
                ['action'],
                ['action', 'message'],
                ['action', 'title'],
                ['fix'],
                ['fix', 'message'],
                ['fix', 'title'],
                ['next_action'],
                ['nextAction']
            ]);
        }
        return '';
    }

    function suggestionTextsFromValue(value) {
        const suggestions = [];
        const values = Array.isArray(value) ? value : [value];
        for (const candidate of values) {
            const text = suggestionText(candidate);
            if (text && !suggestions.includes(text)) {
                suggestions.push(text);
            }
        }
        return suggestions;
    }

    function collectPayloadSuggestions(payload, paths) {
        const suggestions = [];
        for (const path of paths) {
            for (const suggestion of suggestionTextsFromValue(payloadValueAtPath(payload, path))) {
                if (!suggestions.includes(suggestion)) {
                    suggestions.push(suggestion);
                }
            }
        }
        return suggestions;
    }

    function feedbackNextActionText(value) {
        if (typeof value === 'string') {
            return value.trim();
        }
        if (typeof value === 'object' && value !== null) {
            return firstPayloadText(value, ['message', 'title', 'type']);
        }
        return '';
    }

    function issueClassName(issue) {
        return firstPayloadText(issue, ['class_name', 'className', 'class']);
    }

    function issueLooksLikeUnsupportedStyleClass(issue, diagnosticCode) {
        if (!issueClassName(issue)) {
            return false;
        }
        const text = [
            diagnosticCode,
            firstPayloadText(issue, ['title', 'kind', 'rule']),
            firstPayloadText(issue, ['message', 'detail', 'reason'])
        ].join(' ').toLowerCase();
        return text.includes('dx-style')
            || text.includes('dx.style')
            || text.includes('unsupported')
            || text.includes('not supported');
    }

    function issueMessageText(issue) {
        const diagnosticCode = firstPayloadText(issue, ['diagnostic_code', 'code', 'rule']);
        const message = firstPayloadText(issue, ['message', 'detail', 'reason']);
        const className = issueClassName(issue);
        if (className && issueLooksLikeUnsupportedStyleClass(issue, diagnosticCode)) {
            if (message.startsWith('dx-style unsupported class `')) {
                return message;
            }
            return 'dx-style unsupported class `' + className + '`: '
                + (message || 'not supported by the DX-owned style engine');
        }
        return message;
    }

    function issueSeverityName(issue) {
        return normalizeSeverityName(issue);
    }

    function issueSeverityRank(issue) {
        const severity = issueSeverityName(issue);
        if (severity === 'error') {
            return 3;
        }
        if (severity === 'warning') {
            return 2;
        }
        if (severity === 'info') {
            return 1;
        }
        return 0;
    }

    function issueShouldOpenOverlay(issue) {
        return issueSeverityName(issue) !== 'info';
    }

    function highestSeverityIssue(issues) {
        if (!Array.isArray(issues)) {
            return null;
        }

        let selected = null;
        let selectedRank = -1;
        for (const issue of issues) {
            if (typeof issue !== 'object' || issue === null) {
                continue;
            }
            if (!issueShouldOpenOverlay(issue)) {
                continue;
            }
            const rank = issueSeverityRank(issue);
            if (selected === null || rank > selectedRank) {
                selected = issue;
                selectedRank = rank;
            }
        }
        return selected;
    }

    function issueToOverlayPayload(issue, nextAction) {
        const diagnosticCode = firstPayloadText(issue, ['diagnostic_code', 'code', 'rule']);
        const severityName = issueSeverityName(issue);
        const suggestions = collectPayloadSuggestions(issue, [
            ['suggestions'],
            ['suggestion'],
            ['hints'],
            ['hint'],
            ['actions'],
            ['action'],
            ['fixes'],
            ['fix'],
            ['next_actions'],
            ['nextActions'],
            ['remediation']
        ]);
        const filePath = firstPayloadTextAtPath(issue, [
            ['file_path'],
            ['file'],
            ['source_path'],
            ['sourcePath'],
            ['path'],
            ['source_location', 'path'],
            ['source_location', 'file'],
            ['sourceLocation', 'path'],
            ['sourceLocation', 'file'],
            ['source', 'path'],
            ['source', 'file'],
            ['location', 'file'],
            ['span', 'source', 'path']
        ]);
        const line = firstPayloadIntegerAtPath(issue, [
            ['line'],
            ['lineNumber'],
            ['start_line'],
            ['startLine'],
            ['source_location', 'line'],
            ['sourceLocation', 'line'],
            ['location', 'line'],
            ['span', 'start', 'line']
        ]);
        const column = firstPayloadIntegerAtPath(issue, [
            ['column'],
            ['columnNumber'],
            ['start_column'],
            ['startColumn'],
            ['source_location', 'column'],
            ['sourceLocation', 'column'],
            ['location', 'column'],
            ['span', 'start', 'column']
        ]);
        const codeFrame = firstPayloadTextAtPath(issue, [
            ['code_frame'],
            ['codeFrame'],
            ['frame'],
            ['renderedCodeFrame'],
            ['diagnostic', 'code_frame'],
            ['diagnostic', 'codeFrame'],
            ['codeFrame', 'rendered'],
            ['code_frame', 'rendered']
        ]);
        const codeContext = firstPayloadTextAtPath(issue, [
            ['code_context'],
            ['source'],
            ['source', 'content'],
            ['source', 'text'],
            ['source', 'snippet']
        ]);
        const issueNextAction = firstPayloadTextAtPath(issue, [
            ['next_action'],
            ['nextAction'],
            ['hint'],
            ['hint', 'message'],
            ['hint', 'title'],
            ['action'],
            ['action', 'message'],
            ['action', 'title'],
            ['fix'],
            ['fix', 'message'],
            ['fix', 'title'],
            ['remediation'],
            ['diagnostic', 'next_action'],
            ['diagnostic', 'nextAction'],
            ['diagnostic', 'hint'],
            ['diagnostic', 'hint', 'message'],
            ['diagnostic', 'remediation']
        ]);

        return {
            title: firstPayloadText(issue, ['title', 'kind']) || titleFromDiagnosticCode(diagnosticCode),
            severity: severityName === 'warning' ? 'warning' : 'error',
            diagnostic_code: diagnosticCode,
            message: issueMessageText(issue),
            file_path: filePath,
            line,
            column,
            code_frame: codeFrame,
            code_context: codeContext,
            next_action: issueNextAction || feedbackNextActionText(nextAction),
            suggestions
        };
    }

    function overlayPayloadFromFeedbackErrors(snapshot) {
        const issues = snapshot && Array.isArray(snapshot.issues)
            ? snapshot.issues
            : [];
        const issue = highestSeverityIssue(issues);
        if (!issue) {
            return null;
        }
        return issueToOverlayPayload(issue, snapshot && snapshot.next_action);
    }

    function feedbackSnapshotDiagnosticsArtifactStatus(snapshot) {
        if (!snapshot || typeof snapshot !== 'object') {
            return '';
        }
        const directArtifact = snapshot.diagnostics_artifact || snapshot.diagnosticsArtifact;
        if (directArtifact && typeof directArtifact === 'object') {
            return optionalPayloadText(directArtifact.status);
        }
        if (snapshot.recovery && typeof snapshot.recovery === 'object') {
            return optionalPayloadText(snapshot.recovery.diagnostics_artifact_status);
        }
        return '';
    }

    function feedbackSnapshotHasAuthoritativeEmptyIssueList(snapshot) {
        if (!snapshot || !Array.isArray(snapshot.issues)) {
            return false;
        }
        if (snapshot.issues.length !== 0) {
            return false;
        }
        if (snapshot.recovery && snapshot.recovery.clears_overlay === false) {
            return false;
        }
        const status = feedbackSnapshotDiagnosticsArtifactStatus(snapshot);
        return !status || status === 'current';
    }

    function feedbackSnapshotClearsOverlay(snapshot) {
        if (!snapshot || typeof snapshot !== 'object') {
            return false;
        }
        if (snapshot.recovery && snapshot.recovery.clears_overlay === true) {
            return true;
        }
        const nextAction = snapshot.next_action || snapshot.nextAction;
        if (nextAction && typeof nextAction === 'object' && nextAction.type === 'clear-overlay') {
            return true;
        }
        return false;
    }

    function parseFeedbackEventPayload(value) {
        if (typeof value === 'string') {
            try {
                return JSON.parse(value);
            } catch (_) {
                return null;
            }
        }
        if (typeof value === 'object' && value !== null && typeof value.data === 'string') {
            return parseFeedbackEventPayload(value.data);
        }
        return value;
    }

    function feedbackErrorsSnapshotFromEventPayload(payload) {
        const value = parseFeedbackEventPayload(payload);
        if (typeof value !== 'object' || value === null) {
            return null;
        }
        if (value.errors && typeof value.errors === 'object') {
            if (Array.isArray(value.errors.issues)) {
                return value.errors;
            }
            if (value.errors.issue_count === 0 || value.errors.issueCount === 0) {
                return {
                    issues: [],
                    recovery: value.errors.recovery || null,
                    next_action: value.errors.next_action || value.errors.nextAction || null,
                    issue_count: value.errors.issue_count ?? value.errors.issueCount ?? 0,
                    diagnostics_artifact: value.errors.diagnostics_artifact || value.errors.diagnosticsArtifact || null
                };
            }
        }
        if (value.issue_receipt && Array.isArray(value.issue_receipt.issues)) {
            return {
                issues: value.issue_receipt.issues,
                next_action: value.next_action || value.nextAction || (value.errors && (value.errors.next_action || value.errors.nextAction))
            };
        }
        if (Array.isArray(value.issues)) {
            return value;
        }
        return null;
    }

    window.__DX_SHOW_ERROR__ = function(payload) {
        const normalized = normalizeOverlayPayload(payload);
        window.__DX_LAST_ERROR_PAYLOAD__ = normalized;
        let overlay = document.getElementById('dx-error-overlay');
        if (!overlay) {
            overlay = document.createElement('div');
            overlay.id = 'dx-error-overlay';
            overlay.className = 'dx-error-overlay';
            document.body.appendChild(overlay);
        }
        overlay.setAttribute('data-dx-error-severity', normalized.severity || 'error');
        overlay.setAttribute('data-dx-error-code', normalized.diagnostic_code || '');
        overlay.setAttribute('data-dx-error-title', normalized.title || '');
        overlay.setAttribute('data-dx-error-file', formatPayloadLocation(normalized) || '');
        overlay.innerHTML = `
            <div class="dx-error-container">
                <div class="dx-error-header">
                    <span class="dx-error-icon">DX</span>
                    <h1 class="dx-error-title"></h1>
                    <button class="dx-error-close" onclick="window.__DX_HIDE_ERROR__()">x</button>
                </div>
                <div class="dx-error-body">
                    <p class="dx-error-message"></p>
                    <p class="dx-error-diagnostic-code"></p>
                    <p class="dx-error-file"></p>
                    <pre class="dx-error-code"></pre>
                    <ul class="dx-error-suggestions"></ul>
                </div>
                <div class="dx-error-footer"><span class="dx-error-hint"></span></div>
            </div>
        `;
        const titleNode = overlay.querySelector('.dx-error-title');
        const messageNode = overlay.querySelector('.dx-error-message');
        const diagnosticCodeNode = overlay.querySelector('.dx-error-diagnostic-code');
        const fileNode = overlay.querySelector('.dx-error-file');
        const codeNode = overlay.querySelector('.dx-error-code');
        const suggestionsNode = overlay.querySelector('.dx-error-suggestions');
        const hintNode = overlay.querySelector('.dx-error-hint');
        const suggestions = normalized.suggestions;
        titleNode.textContent = normalized.title;
        messageNode.textContent = optionalPayloadText(normalized.message) || 'No diagnostic message was provided.';
        setOptionalText(diagnosticCodeNode, normalized.diagnostic_code);
        setOptionalText(fileNode, formatPayloadLocation(normalized));
        setOptionalText(codeNode, normalized.code_frame || normalized.code_context || normalized.source);
        suggestionsNode.replaceChildren();
        for (const suggestion of suggestions) {
            const item = document.createElement('li');
            item.textContent = suggestion;
            suggestionsNode.appendChild(item);
        }
        suggestionsNode.hidden = suggestions.length === 0;
        setOptionalText(hintNode, normalized.next_action || normalized.hint || suggestions[0]);
        hintNode.parentElement.hidden = hintNode.hidden;
        overlay.style.display = 'flex';
    };
    
    window.__DX_HIDE_ERROR__ = function() {
        window.__DX_LAST_ERROR_PAYLOAD__ = null;
        const overlay = document.getElementById('dx-error-overlay');
        if (overlay) {
            overlay.style.display = 'none';
        }
    };

    window.__DX_APPLY_FEEDBACK_ERRORS__ = function(snapshot) {
        const payload = overlayPayloadFromFeedbackErrors(snapshot);
        if (!payload) {
            if (feedbackSnapshotClearsOverlay(snapshot)) {
                window.__DX_HIDE_ERROR__();
                return null;
            }
            if (feedbackSnapshotHasAuthoritativeEmptyIssueList(snapshot)) {
                window.__DX_HIDE_ERROR__();
            }
            return null;
        }
        window.__DX_SHOW_ERROR__(payload);
        return payload;
    };

    window.__DX_APPLY_DEV_FEEDBACK__ = function(payload) {
        const snapshot = feedbackErrorsSnapshotFromEventPayload(payload);
        if (!snapshot) {
            return null;
        }
        return window.__DX_APPLY_FEEDBACK_ERRORS__(snapshot);
    };

    window.__DX_REFRESH_ERROR_OVERLAY__ = async function() {
        if (typeof fetch !== 'function') {
            return null;
        }
        try {
            const response = await fetch(DX_FEEDBACK_ERRORS_ENDPOINT, { cache: 'no-store', headers: { accept: 'application/json' } });
            if (!response.ok) {
                return null;
            }
            const snapshot = await response.json();
            return window.__DX_APPLY_FEEDBACK_ERRORS__(snapshot);
        } catch (_) {
            return null;
        }
    };

    if (typeof window.addEventListener === 'function') {
        window.addEventListener('dx:feedback-errors', function(event) {
            window.__DX_APPLY_FEEDBACK_ERRORS__(event.detail);
        });
        window.addEventListener('dx-dev-feedback', function(event) {
            window.__DX_APPLY_DEV_FEEDBACK__(event.detail || event);
        });
        window.addEventListener('DOMContentLoaded', function() {
            window.__DX_REFRESH_ERROR_OVERLAY__();
        }, { once: true });
    }
})();
"#;

/// CSS for the error overlay.
pub const OVERLAY_STYLES: &str = r#"
.dx-error-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.85);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 999999;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.dx-error-container {
    background: #1a1a1a;
    border-radius: 8px;
    max-width: 800px;
    width: 90%;
    max-height: 90vh;
    overflow: auto;
    box-shadow: 0 4px 24px rgba(0, 0, 0, 0.5);
}

.dx-error-header {
    display: flex;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid #333;
    background: #ff5555;
    border-radius: 8px 8px 0 0;
}

.dx-error-icon {
    font-size: 24px;
    margin-right: 12px;
}

.dx-error-title {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
    color: white;
    flex: 1;
}

.dx-error-close {
    background: none;
    border: none;
    color: white;
    font-size: 24px;
    cursor: pointer;
    padding: 0;
    line-height: 1;
    opacity: 0.8;
}

.dx-error-close:hover {
    opacity: 1;
}

.dx-error-body {
    padding: 20px;
}

.dx-error-message {
    margin: 0 0 16px 0;
    color: #ff8888;
    font-size: 16px;
    line-height: 1.5;
}

.dx-error-file {
    margin: 0 0 16px 0;
    color: #888;
    font-size: 14px;
}

.dx-error-code {
    background: #0d0d0d;
    padding: 16px;
    border-radius: 4px;
    overflow-x: auto;
    font-family: 'Fira Code', 'Monaco', 'Consolas', monospace;
    font-size: 14px;
    line-height: 1.5;
    color: #e0e0e0;
    margin: 0 0 16px 0;
}

.dx-error-stack {
    background: #0d0d0d;
    padding: 16px;
    border-radius: 4px;
    overflow-x: auto;
    font-family: 'Fira Code', 'Monaco', 'Consolas', monospace;
    font-size: 12px;
    line-height: 1.5;
    color: #888;
    margin: 0;
}

.dx-error-footer {
    padding: 12px 20px;
    border-top: 1px solid #333;
    text-align: center;
}

.dx-error-hint {
    color: #666;
    font-size: 14px;
}
"#;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_overlay_new() {
        let overlay = ErrorOverlay::new();
        assert!(overlay.current().is_none());
        assert!(overlay.history().is_empty());
    }

    #[test]
    fn test_error_overlay_show_clear() {
        let mut overlay = ErrorOverlay::new();

        let error = DxError::ConfigValidationError {
            message: "test error".to_string(),
            field: None,
        };
        overlay.show(&error);

        assert!(overlay.current().is_some());
        assert_eq!(overlay.history().len(), 1);

        overlay.clear();
        assert!(overlay.current().is_none());
        assert_eq!(overlay.history().len(), 1); // History preserved
    }

    #[test]
    fn show_diagnostic_preserves_warning_overlay_payload() {
        let mut overlay = ErrorOverlay::new();
        let diagnostic = DxDiagnostic::warning("Style warning", "Slow style compile")
            .with_code("dx.style.slow_compile")
            .with_source_range(
                "app/page.tsx",
                2,
                10,
                2,
                14,
                "export default function Page() {\n  return <main>slow</main>;\n}\n",
            )
            .with_hint("Move expensive style work out of the request path.");

        overlay.show_diagnostic(&diagnostic);
        let current = overlay
            .current()
            .expect("diagnostic should be visible in the overlay");

        assert_eq!(current.title, "Style warning");
        assert_eq!(current.message, "Slow style compile");
        assert_eq!(current.severity, DxDiagnosticSeverity::Warning);
        assert_eq!(
            current.diagnostic_code.as_deref(),
            Some("dx.style.slow_compile")
        );
        assert_eq!(
            current.next_action.as_deref(),
            Some("Move expensive style work out of the request path.")
        );
        assert!(
            current
                .code_frame
                .as_deref()
                .is_some_and(|frame| frame.contains("> 2 |   return <main>slow</main>;"))
        );

        let html = overlay.to_html();
        assert!(
            html.contains(r#"data-dx-error-severity="warning""#),
            "{html}"
        );
        assert!(
            html.contains(r#"data-dx-error-code="dx.style.slow_compile""#),
            "{html}"
        );
        assert!(
            html.contains("Move expensive style work out of the request path."),
            "{html}"
        );
    }

    #[test]
    fn show_error_preserves_diagnostic_overlay_title() {
        let mut overlay = ErrorOverlay::new();
        let error = DxError::ConfigParseError {
            file: Some(std::path::PathBuf::from("dx")),
            message: "expected project name".to_string(),
            src: None,
            span: None,
        };

        overlay.show(&error);

        let current = overlay
            .current()
            .expect("diagnostic title should be visible in the overlay");
        assert_eq!(current.title, "Config parse failed");
        assert!(overlay.to_html().contains("Config parse failed"));
    }

    #[test]
    fn show_payload_preserves_structured_error_overlay_data() {
        let mut overlay = ErrorOverlay::new();
        let payload = ErrorOverlayData {
            error_type: crate::error::ErrorType::Api,
            severity: DxDiagnosticSeverity::Warning,
            title: "API method warning".to_string(),
            diagnostic_code: Some("dx.api.invalid_method".to_string()),
            message: "Method TRACE is not supported.".to_string(),
            file_path: Some(std::path::PathBuf::from("app/api/health/route.ts")),
            line: Some(7),
            column: Some(3),
            code_context: Some("export function TRACE() {}\n".to_string()),
            code_frame: Some("> 7 | export function TRACE() {}\n    |   ^^^^^".to_string()),
            next_action: Some("Export a supported HTTP method.".to_string()),
            stack_trace: Some(vec![crate::error::StackFrame {
                function: "TRACE".to_string(),
                file: Some(std::path::PathBuf::from("app/api/health/route.ts")),
                line: Some(7),
                column: Some(3),
            }]),
            suggestions: vec!["Use GET or POST for this route.".to_string()],
        };

        overlay.show_payload(&payload);
        let current = overlay
            .current()
            .expect("payload should be visible in the overlay");

        assert_eq!(current.title, "API method warning");
        assert_eq!(current.message, payload.message);
        assert!(matches!(current.error_type, ErrorType::Api));
        assert_eq!(current.severity, DxDiagnosticSeverity::Warning);
        assert_eq!(
            current.diagnostic_code.as_deref(),
            Some("dx.api.invalid_method")
        );
        assert_eq!(current.file.as_deref(), Some("app/api/health/route.ts"));
        assert_eq!(current.line, Some(7));
        assert_eq!(current.column, Some(3));
        assert_eq!(
            current.next_action.as_deref(),
            Some("Export a supported HTTP method.")
        );
        assert!(
            current
                .stack_trace
                .as_deref()
                .is_some_and(|stack| stack.contains("TRACE (app/api/health/route.ts:7:3)"))
        );

        let html = overlay.to_html();
        assert!(
            html.contains(r#"data-dx-error-severity="warning""#),
            "{html}"
        );
        assert!(
            html.contains(r#"data-dx-error-code="dx.api.invalid_method""#),
            "{html}"
        );
        assert!(html.contains("Use GET or POST for this route."), "{html}");
        assert!(
            html.contains("TRACE (app/api/health/route.ts:7:3)"),
            "{html}"
        );
    }

    #[test]
    fn show_payload_preserves_structured_error_overlay_title() {
        let mut overlay = ErrorOverlay::new();
        let payload = ErrorOverlayData {
            error_type: crate::error::ErrorType::Config,
            severity: DxDiagnosticSeverity::Error,
            title: "Config parse failed".to_string(),
            diagnostic_code: Some("dx.config.parse_error".to_string()),
            message: "Expected a project name.".to_string(),
            file_path: Some(std::path::PathBuf::from("dx")),
            line: Some(2),
            column: Some(1),
            code_context: None,
            code_frame: None,
            next_action: Some("Fix the configuration syntax.".to_string()),
            stack_trace: None,
            suggestions: Vec::new(),
        };

        overlay.show_payload(&payload);

        let current = overlay
            .current()
            .expect("payload title should be visible in the overlay");
        assert_eq!(current.title, "Config parse failed");
        assert!(overlay.to_html().contains("Config parse failed"));
    }

    #[test]
    fn show_payload_infers_title_from_diagnostic_code_when_missing() {
        let mut overlay = ErrorOverlay::new();
        let payload = ErrorOverlayData {
            error_type: crate::error::ErrorType::Compilation,
            severity: DxDiagnosticSeverity::Error,
            title: String::new(),
            diagnostic_code: Some("dx.source.parse_error".to_string()),
            message: "Unexpected token".to_string(),
            file_path: Some(std::path::PathBuf::from("app/page.tsx")),
            line: Some(4),
            column: Some(12),
            code_context: None,
            code_frame: Some("> 4 |   return <main>".to_string()),
            next_action: Some("Fix the source syntax.".to_string()),
            stack_trace: None,
            suggestions: Vec::new(),
        };

        overlay.show_payload(&payload);

        let current = overlay
            .current()
            .expect("payload title should be inferred from the diagnostic code");
        assert_eq!(current.title, "DX source parse error");
        assert!(overlay.to_html().contains("DX source parse error"));
    }

    #[test]
    fn overlay_html_and_script_render_suggestions_without_interpolation() {
        let info = ErrorInfo {
            title: "DX diagnostic".to_string(),
            message: "Source issue".to_string(),
            file: None,
            line: None,
            column: None,
            code_snippet: None,
            stack_trace: None,
            error_type: ErrorType::Diagnostic,
            severity: DxDiagnosticSeverity::Warning,
            diagnostic_code: Some("dx.source.warning".to_string()),
            code_frame: None,
            next_action: None,
            suggestions: vec![
                "Fix <script> tag".to_string(),
                "Save the file to reload".to_string(),
            ],
        };

        let html = generate_error_html(&info);
        let script = ErrorOverlay::new().to_script();

        assert!(
            html.contains(r#"<ul class="dx-error-suggestions">"#),
            "{html}"
        );
        assert!(html.contains("Fix &lt;script&gt; tag"), "{html}");
        assert!(!html.contains("Fix <script> tag"), "{html}");
        assert!(
            html.contains(r#"<span class="dx-error-hint">Fix &lt;script&gt; tag</span>"#),
            "{html}"
        );
        assert!(script.contains("function collectPayloadSuggestions(payload, paths)"));
        assert!(
            script
                .contains("const suggestionsNode = overlay.querySelector('.dx-error-suggestions')")
        );
        assert!(script.contains("item.textContent = suggestion"));
        assert!(!script.contains(concat!("$", "{suggestion}")));
    }

    #[test]
    fn overlay_script_accepts_structured_payload_without_raw_message_interpolation() {
        let script = ErrorOverlay::new().to_script();

        assert!(script.contains("window.__DX_SHOW_ERROR__ = function(payload)"));
        assert!(script.contains("normalized.next_action"));
        assert!(script.contains("setOptionalText(diagnosticCodeNode, normalized.diagnostic_code)"));
        let legacy_code_fallback = concat!(
            "codeNode.textContent = normalized.code_frame || normalized.",
            "code || ''"
        );
        assert!(!script.contains(legacy_code_fallback));
        assert!(script.contains(
            "messageNode.textContent = optionalPayloadText(normalized.message) || 'No diagnostic message was provided.'"
        ));
        assert!(!script.contains(concat!("$", "{message}")));
    }

    #[test]
    fn overlay_script_normalizes_issue_payload_title_and_code() {
        let script = ErrorOverlay::new().to_script();

        assert!(script.contains("function normalizeOverlayPayload(payload)"));
        assert!(script.contains("function titleFromDiagnosticCode(code)"));
        assert!(
            script
                .contains("const diagnosticCode = optionalPayloadText(normalized.diagnostic_code)")
        );
        assert!(script.contains("|| optionalPayloadText(normalized.code)"));
        assert!(script.contains("const inferredTitle = titleFromDiagnosticCode(diagnosticCode)"));
        assert!(script.contains("normalized.diagnostic_code = diagnosticCode"));
        assert!(script.contains("normalized.title = explicitTitle"));
        assert!(script.contains("|| inferredTitle"));
        assert!(script.contains(
            "overlay.setAttribute('data-dx-error-code', normalized.diagnostic_code || '')"
        ));
    }

    #[test]
    fn overlay_script_normalizes_direct_payload_nested_source_shapes() {
        let script = ErrorOverlay::new().to_script();

        assert!(script.contains("const directFilePath = firstPayloadTextAtPath(normalized, ["));
        assert!(script.contains("['source_location', 'path']"));
        assert!(script.contains("['sourceLocation', 'file']"));
        assert!(script.contains("const directLine = firstPayloadIntegerAtPath(normalized, ["));
        assert!(script.contains("['source_location', 'line']"));
        assert!(script.contains("const directColumn = firstPayloadIntegerAtPath(normalized, ["));
        assert!(script.contains("['source_location', 'column']"));
        assert!(script.contains("const directCodeFrame = firstPayloadTextAtPath(normalized, ["));
        assert!(script.contains("['diagnostic', 'code_frame']"));
        assert!(script.contains("const directCodeContext = firstPayloadTextAtPath(normalized, ["));
        assert!(script.contains("['source', 'snippet']"));
        assert!(script.contains("const directNextAction = firstPayloadTextAtPath(normalized, ["));
        assert!(script.contains("['hint', 'message']"));
        assert!(script.contains("normalized.file_path = directFilePath"));
        assert!(script.contains("normalized.line = directLine"));
        assert!(script.contains("normalized.column = directColumn"));
        assert!(script.contains(
            "normalized.code_frame = directCodeFrame || optionalPayloadText(normalized.code_frame)"
        ));
        assert!(
            script.contains(
                "normalized.code_context = directCodeContext || optionalPayloadText(normalized.code_context)"
            )
        );
        assert!(
            script.contains(
                "normalized.next_action = directNextAction || optionalPayloadText(normalized.next_action)"
            )
        );
    }

    #[test]
    fn overlay_script_bridges_feedback_errors_to_basic_overlay() {
        let script = ErrorOverlay::new().to_script();

        assert!(script.contains("const DX_FEEDBACK_ERRORS_ENDPOINT = '/_dx/feedback/errors';"));
        assert!(script.contains("function issueSeverityRank(issue)"));
        assert!(script.contains("function highestSeverityIssue(issues)"));
        assert!(script.contains("function issueToOverlayPayload(issue, nextAction)"));
        assert!(script.contains("function overlayPayloadFromFeedbackErrors(snapshot)"));
        assert!(script.contains("window.__DX_REFRESH_ERROR_OVERLAY__ = async function()"));
        assert!(script.contains("fetch(DX_FEEDBACK_ERRORS_ENDPOINT, { cache: 'no-store'"));
        assert!(script.contains("window.__DX_SHOW_ERROR__(payload)"));
    }

    #[test]
    fn overlay_script_applies_feedback_error_snapshots_without_fetch() {
        let script = ErrorOverlay::new().to_script();

        assert!(script.contains("window.__DX_APPLY_FEEDBACK_ERRORS__ = function(snapshot)"));
        assert!(script.contains("const payload = overlayPayloadFromFeedbackErrors(snapshot)"));
        assert!(script.contains("window.__DX_SHOW_ERROR__(payload)"));
        assert!(script.contains("return window.__DX_APPLY_FEEDBACK_ERRORS__(snapshot)"));
        assert!(script.contains("window.addEventListener('dx:feedback-errors', function(event)"));
        assert!(script.contains("window.__DX_APPLY_FEEDBACK_ERRORS__(event.detail)"));
    }

    #[test]
    fn overlay_script_accepts_nested_dev_feedback_event_payloads() {
        let script = ErrorOverlay::new().to_script();

        assert!(script.contains("function parseFeedbackEventPayload(value)"));
        assert!(script.contains("return JSON.parse(value)"));
        assert!(script.contains("typeof value.data === 'string'"));
        assert!(script.contains("function feedbackErrorsSnapshotFromEventPayload(payload)"));
        assert!(script.contains("if (Array.isArray(value.errors.issues))"));
        assert!(
            script.contains("if (value.errors.issue_count === 0 || value.errors.issueCount === 0)")
        );
        assert!(
            script
                .contains("if (value.issue_receipt && Array.isArray(value.issue_receipt.issues))")
        );
        assert!(script.contains("window.__DX_APPLY_DEV_FEEDBACK__ = function(payload)"));
        assert!(script.contains("return window.__DX_APPLY_FEEDBACK_ERRORS__(snapshot)"));
        assert!(script.contains("window.addEventListener('dx-dev-feedback', function(event)"));
        assert!(script.contains("window.__DX_APPLY_DEV_FEEDBACK__(event.detail || event)"));
    }

    #[test]
    fn overlay_script_clears_stale_feedback_errors_after_recovery() {
        let script = ErrorOverlay::new().to_script();

        assert!(script.contains("function feedbackSnapshotDiagnosticsArtifactStatus(snapshot)"));
        assert!(
            script.contains("function feedbackSnapshotHasAuthoritativeEmptyIssueList(snapshot)")
        );
        assert!(script.contains("if (snapshot.issues.length !== 0)"));
        assert!(script.contains("snapshot.recovery.clears_overlay === false"));
        assert!(script.contains("return !status || status === 'current'"));
        assert!(script.contains("if (feedbackSnapshotClearsOverlay(snapshot))"));
        assert!(script.contains("if (feedbackSnapshotHasAuthoritativeEmptyIssueList(snapshot))"));
        assert!(script.contains("window.__DX_HIDE_ERROR__()"));
    }

    #[test]
    fn overlay_script_uses_explicit_recovery_contract_to_clear() {
        let script = ErrorOverlay::new().to_script();

        assert!(script.contains("function feedbackSnapshotClearsOverlay(snapshot)"));
        assert!(script.contains("snapshot.recovery.clears_overlay === true"));
        assert!(script.contains("value.errors.recovery || null"));
        assert!(script.contains("diagnostics_artifact: value.errors.diagnostics_artifact"));
        assert!(script.contains("return false"));
        assert!(script.contains("if (feedbackSnapshotClearsOverlay(snapshot))"));
        assert!(script.contains("window.__DX_HIDE_ERROR__()"));
    }

    #[test]
    fn overlay_script_exposes_browser_visible_payload_state() {
        let script = ErrorOverlay::new().to_script();

        assert!(script.contains("window.__DX_LAST_ERROR_PAYLOAD__ = normalized"));
        assert!(script.contains("window.__DX_LAST_ERROR_PAYLOAD__ = null"));
        assert!(
            script.contains("overlay.setAttribute('data-dx-error-title', normalized.title || '')")
        );
        assert!(script.contains(
            "overlay.setAttribute('data-dx-error-file', formatPayloadLocation(normalized) || '')"
        ));
    }

    #[test]
    fn overlay_script_does_not_escalate_info_feedback_to_error() {
        let script = ErrorOverlay::new().to_script();

        assert!(script.contains("function issueSeverityName(issue)"));
        assert!(script.contains("return 'info'"));
        assert!(script.contains("function issueShouldOpenOverlay(issue)"));
        assert!(script.contains("return issueSeverityName(issue) !== 'info'"));
        assert!(script.contains("if (!issueShouldOpenOverlay(issue))"));
        assert!(script.contains("const severityName = issueSeverityName(issue)"));
        assert!(script.contains("severity: severityName === 'warning' ? 'warning' : 'error'"));
    }

    #[test]
    fn overlay_script_normalizes_warning_aliases_without_error_escalation() {
        let script = ErrorOverlay::new().to_script();

        assert!(script.contains("function severityText(value)"));
        assert!(script.contains("function normalizeSeverityName(value)"));
        assert!(script.contains("function overlaySeverityName(value)"));
        assert!(script.contains("const severity = overlaySeverityName(normalized)"));
        assert!(script.contains("['diagnostic', 'severity']"));
        assert!(script.contains("['level']"));
        assert!(script.contains("severity === 'warning' || severity === 'warn'"));
        assert!(script.contains(
            "return severity === 'info' || severity === 'warning' ? 'warning' : 'error'"
        ));
    }

    #[test]
    fn overlay_script_formats_file_line_column_payloads() {
        let script = ErrorOverlay::new().to_script();

        assert!(script.contains("function formatPayloadLocation(normalized)"));
        assert!(script.contains("Number.isInteger(normalized.line)"));
        assert!(script.contains("Number.isInteger(normalized.column)"));
        assert!(script.contains("setOptionalText(fileNode, formatPayloadLocation(normalized))"));
        assert!(
            !script
                .contains("fileNode.textContent = normalized.file_path || normalized.file || ''")
        );
    }

    #[test]
    fn overlay_script_promotes_nested_issue_next_actions() {
        let script = ErrorOverlay::new().to_script();

        assert!(script.contains("const issueNextAction = firstPayloadTextAtPath(issue, ["));
        assert!(script.contains("['nextAction']"));
        assert!(script.contains("['hint', 'message']"));
        assert!(script.contains("['diagnostic', 'remediation']"));
        assert!(
            script.contains("next_action: issueNextAction || feedbackNextActionText(nextAction)")
        );
    }

    #[test]
    fn overlay_script_hides_empty_optional_payload_fields() {
        let script = ErrorOverlay::new().to_script();

        assert!(script.contains("function optionalPayloadText(value)"));
        assert!(script.contains("function setOptionalText(node, value)"));
        assert!(script.contains("node.hidden = text.length === 0"));
        assert!(
            script.contains("setOptionalText(codeNode, normalized.code_frame || normalized.code_context || normalized.source)")
        );
        assert!(script.contains("hintNode.parentElement.hidden = hintNode.hidden"));
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("a & b"), "a &amp; b");
        assert_eq!(html_escape("\"test\""), "&quot;test&quot;");
    }

    #[test]
    fn test_error_info_from_config_error() {
        let error = DxError::ConfigValidationError {
            message: "invalid config".to_string(),
            field: None,
        };
        let info = ErrorInfo::from_error(&error);
        assert_eq!(info.title, "Config validation failed");
        assert_eq!(
            info.diagnostic_code.as_deref(),
            Some("dx.config.validation_error")
        );
        assert!(info.message.contains("invalid config"));
    }
}
