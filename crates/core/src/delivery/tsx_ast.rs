use serde::{Deserialize, Serialize};

use super::jsx_lowering::{DxReactImport, DxReactImportSpecifier};

/// Source span for TSX compiler diagnostics and extracted declarations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxTsxSpan {
    /// Byte offset where the node starts.
    pub start: usize,
    /// Byte offset where the node ends.
    pub end: usize,
    /// One-based line number.
    pub line: usize,
    /// One-based column number.
    pub column: usize,
}

/// TSX module diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxTsxDiagnostic {
    /// Stable diagnostic code.
    pub code: String,
    /// Human-readable diagnostic message.
    pub message: String,
    /// Source span that triggered the diagnostic.
    pub span: DxTsxSpan,
}

/// Parser backend evidence for the TSX compiler surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxTsxParserBackend {
    /// Stable machine-readable schema for downstream DX Check, Forge, and Studio consumers.
    pub schema: String,
    /// Numeric revision kept separate from the schema name so public contracts stay readable.
    pub schema_revision: u16,
    /// Backend selected for syntax validation in this build.
    pub active_backend: String,
    /// Human-readable status.
    pub status: String,
    /// Whether the Oxc parser path was compiled into this binary.
    pub oxc_available: bool,
    /// Whether the custom scanner is still used for DX-WWW's bounded semantic extraction.
    pub custom_scanner_active: bool,
    /// Syntax validation diagnostics reported by the active backend.
    pub diagnostics: Vec<DxTsxDiagnostic>,
    /// Extra backend-specific validation metadata.
    pub validation: DxTsxParserBackendValidation,
}

/// Backend-specific TSX parser validation details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxTsxParserBackendValidation {
    /// Backend crate or scanner identity.
    pub parser: String,
    /// Source type inferred for the file.
    pub source_type: String,
    /// Whether the parser treated the source as TypeScript.
    pub typescript: bool,
    /// Whether the parser treated the source as JSX/TSX-capable.
    pub jsx: bool,
    /// Whether the parser treated the source as an ES module.
    pub module: bool,
    /// Syntax error count reported by the backend.
    pub syntax_errors: usize,
    /// Whether the backend reported a parser panic.
    pub panicked: bool,
}

/// Next-style route metadata extracted from a TSX module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxTsxRouteMetadata {
    /// Metadata title.
    pub title: Option<String>,
    /// Metadata description.
    pub description: Option<String>,
    /// Canonical URL from `alternates.canonical`.
    pub canonical: Option<String>,
    /// Source span for the metadata object.
    pub span: DxTsxSpan,
}

/// TSX module surface used by DX-WWW's React-shaped compiler.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxTsxModuleAst {
    /// Project-relative source path.
    pub source_path: String,
    /// Import declarations in source order.
    pub imports: Vec<DxReactImport>,
    /// Route metadata object, when present.
    pub metadata: Option<DxTsxRouteMetadata>,
    /// Parse diagnostics.
    pub diagnostics: Vec<DxTsxDiagnostic>,
    /// Parser backend used to validate this TSX module.
    pub parser_backend: DxTsxParserBackend,
}

/// Parse the compiler-facing TSX module surface without executing source code.
pub fn parse_tsx_module(source_path: &str, source: &str) -> DxTsxModuleAst {
    let mut parser = TsxModuleParser::new(source_path, source);
    parser.parse();
    let parser_backend_parse = parse_tsx_backend(source_path, source);
    if let Some(imports) = parser_backend_parse.imports {
        parser.imports = imports;
    }
    if let Some(metadata) = parser_backend_parse.metadata {
        parser.metadata = Some(metadata);
    }
    parser.into_ast(parser_backend_parse.backend)
}

struct DxTsxBackendParse {
    backend: DxTsxParserBackend,
    imports: Option<Vec<DxReactImport>>,
    metadata: Option<DxTsxRouteMetadata>,
}

struct TsxModuleParser<'a> {
    source_path: &'a str,
    source: &'a str,
    imports: Vec<DxReactImport>,
    metadata: Option<DxTsxRouteMetadata>,
    diagnostics: Vec<DxTsxDiagnostic>,
}

impl<'a> TsxModuleParser<'a> {
    fn new(source_path: &'a str, source: &'a str) -> Self {
        Self {
            source_path,
            source,
            imports: Vec::new(),
            metadata: None,
            diagnostics: Vec::new(),
        }
    }

    fn parse(&mut self) {
        let mut cursor = 0usize;
        while let Some(index) = find_keyword_outside_code(self.source, "import", cursor) {
            cursor = self.parse_import(index);
        }
        self.metadata = self.parse_metadata();
    }

    fn into_ast(mut self, parser_backend: DxTsxParserBackend) -> DxTsxModuleAst {
        self.diagnostics
            .extend(parser_backend.diagnostics.iter().cloned());
        DxTsxModuleAst {
            source_path: self.source_path.to_string(),
            imports: self.imports,
            metadata: self.metadata,
            diagnostics: self.diagnostics,
            parser_backend,
        }
    }

    fn parse_import(&mut self, start: usize) -> usize {
        let after_import = skip_ws(self.source, start + "import".len());
        if self.source[after_import..].starts_with('(')
            || self.source[after_import..].starts_with('.')
        {
            return after_import + 1;
        }

        if let Some((source, end)) = parse_quoted_at(self.source, after_import) {
            let end = consume_optional_semicolon(self.source, end);
            self.imports.push(DxReactImport {
                source,
                default: None,
                namespace: None,
                specifiers: Vec::new(),
                side_effect_only: true,
                type_only: false,
                span: span_at(self.source, start, end),
            });
            return end;
        }

        let Some(end) = find_statement_end(self.source, after_import) else {
            self.diagnostics.push(DxTsxDiagnostic {
                code: "tsx-import-unclosed".to_string(),
                message: "Import declaration is missing a semicolon or statement end.".to_string(),
                span: span_at(self.source, start, self.source.len()),
            });
            return self.source.len();
        };

        let segment = &self.source[after_import..end];
        let Some(from_index) = find_keyword_in_segment(segment, "from") else {
            self.diagnostics.push(DxTsxDiagnostic {
                code: "tsx-import-missing-from".to_string(),
                message: "Import declaration is missing a `from` clause.".to_string(),
                span: span_at(self.source, start, end),
            });
            return consume_optional_semicolon(self.source, end);
        };

        let clause = segment[..from_index].trim();
        let source_start = skip_ws(self.source, after_import + from_index + "from".len());
        let Some((source, _source_end)) = parse_quoted_at(self.source, source_start) else {
            self.diagnostics.push(DxTsxDiagnostic {
                code: "tsx-import-missing-source".to_string(),
                message: "Import declaration is missing a quoted module specifier.".to_string(),
                span: span_at(self.source, start, end),
            });
            return consume_optional_semicolon(self.source, end);
        };

        self.imports
            .push(parse_import_clause(self.source, start, end, clause, source));
        consume_optional_semicolon(self.source, end)
    }

    fn parse_metadata(&mut self) -> Option<DxTsxRouteMetadata> {
        let start = find_sequence_outside_code(self.source, &["export", "const", "metadata"])?;
        let equals = self.source[start..]
            .find('=')
            .map(|offset| start + offset)?;
        let object_start = self.source[equals..]
            .find('{')
            .map(|offset| equals + offset)?;
        let object_end = find_balanced_block(self.source, object_start)?;
        let object = &self.source[object_start..=object_end];
        Some(DxTsxRouteMetadata {
            title: object_string_field(object, "title"),
            description: object_string_field(object, "description"),
            canonical: object_string_field(object, "canonical"),
            span: span_at(self.source, object_start, object_end + 1),
        })
    }
}

fn parse_tsx_backend(source_path: &str, source: &str) -> DxTsxBackendParse {
    #[cfg(feature = "oxc")]
    {
        parse_tsx_with_oxc(source_path, source)
    }
    #[cfg(not(feature = "oxc"))]
    {
        let _ = source;
        let source_type = fallback_source_type(source_path);
        DxTsxBackendParse {
            backend: DxTsxParserBackend {
                schema: "dx.tsx.parserBackend".to_string(),
                schema_revision: 1,
                active_backend: "custom-scanner".to_string(),
                status: "custom-scanner-active-oxc-not-compiled".to_string(),
                oxc_available: false,
                custom_scanner_active: true,
                diagnostics: Vec::new(),
                validation: DxTsxParserBackendValidation {
                    parser: "dx-www-custom-scanner".to_string(),
                    source_type: source_type.to_string(),
                    typescript: matches!(source_type, "ts" | "tsx"),
                    jsx: matches!(source_type, "jsx" | "tsx"),
                    module: true,
                    syntax_errors: 0,
                    panicked: false,
                },
            },
            imports: None,
            metadata: None,
        }
    }
}

#[cfg(feature = "oxc")]
fn parse_tsx_with_oxc(source_path: &str, source: &str) -> DxTsxBackendParse {
    use oxc_allocator::Allocator;
    use oxc_parser::Parser;
    use oxc_span::SourceType;

    let allocator = Allocator::default();
    let source_type = SourceType::from_path(source_path).unwrap_or_else(|_| SourceType::tsx());
    let parser_return = Parser::new(&allocator, source, source_type).parse();
    let imports = extract_oxc_imports(&parser_return.program, source);
    let metadata = extract_oxc_metadata(&parser_return.program, source);
    let panicked = parser_return.panicked;
    let diagnostics = parser_return
        .errors
        .into_iter()
        .map(|error| DxTsxDiagnostic {
            code: "tsx-oxc-parse-error".to_string(),
            message: format!("{error:?}"),
            span: DxTsxSpan::default(),
        })
        .collect::<Vec<_>>();
    let syntax_errors = diagnostics.len();
    let status = if panicked {
        "oxc-parser-panicked"
    } else if syntax_errors == 0 {
        "oxc-validated"
    } else {
        "oxc-validated-with-errors"
    };

    DxTsxBackendParse {
        backend: DxTsxParserBackend {
            schema: "dx.tsx.parserBackend".to_string(),
            schema_revision: 1,
            active_backend: "oxc-parser".to_string(),
            status: status.to_string(),
            oxc_available: true,
            custom_scanner_active: true,
            diagnostics,
            validation: DxTsxParserBackendValidation {
                parser: "oxc-parser".to_string(),
                source_type: oxc_source_type_label(source_type).to_string(),
                typescript: source_type.is_typescript(),
                jsx: source_type.is_jsx(),
                module: source_type.is_module(),
                syntax_errors,
                panicked,
            },
        },
        imports: Some(imports),
        metadata,
    }
}

#[cfg(feature = "oxc")]
fn extract_oxc_imports(
    program: &oxc_ast::ast::Program<'_>,
    source_text: &str,
) -> Vec<DxReactImport> {
    oxc_import_declarations(program, source_text)
}

#[cfg(feature = "oxc")]
fn oxc_import_declarations(
    program: &oxc_ast::ast::Program<'_>,
    source_text: &str,
) -> Vec<DxReactImport> {
    program
        .body
        .iter()
        .filter_map(|statement| {
            let oxc_ast::ast::Statement::ImportDeclaration(import) = statement else {
                return None;
            };

            let type_only = import.import_kind.is_type();
            let span_start = import.span.start as usize;
            let span_end = import.span.end as usize;
            let mut default = None;
            let mut namespace = None;
            let mut specifiers = Vec::new();

            if let Some(import_specifiers) = &import.specifiers {
                for specifier in import_specifiers {
                    match specifier {
                        oxc_ast::ast::ImportDeclarationSpecifier::ImportSpecifier(specifier) => {
                            let specifier_type_only = type_only || specifier.import_kind.is_type();
                            specifiers.push(DxReactImportSpecifier {
                                imported: specifier.imported.name().to_string(),
                                local: specifier.local.name.to_string(),
                                type_only: specifier_type_only,
                                span: oxc_span_at(
                                    source_text,
                                    specifier.span.start as usize,
                                    specifier.span.end as usize,
                                ),
                            });
                        }
                        oxc_ast::ast::ImportDeclarationSpecifier::ImportDefaultSpecifier(
                            specifier,
                        ) => {
                            default = Some(specifier.local.name.to_string());
                        }
                        oxc_ast::ast::ImportDeclarationSpecifier::ImportNamespaceSpecifier(
                            specifier,
                        ) => {
                            namespace = Some(specifier.local.name.to_string());
                        }
                    }
                }
            }

            Some(DxReactImport {
                source: import.source.value.to_string(),
                default,
                namespace,
                specifiers,
                side_effect_only: import
                    .specifiers
                    .as_ref()
                    .is_none_or(|specifiers| specifiers.is_empty()),
                type_only,
                span: oxc_span_at(source_text, span_start, span_end),
            })
        })
        .collect()
}

#[cfg(feature = "oxc")]
fn oxc_span_at(source: &str, start: usize, end: usize) -> DxTsxSpan {
    span_at(source, start.min(source.len()), end.min(source.len()))
}

#[cfg(feature = "oxc")]
fn extract_oxc_metadata(
    program: &oxc_ast::ast::Program<'_>,
    source_text: &str,
) -> Option<DxTsxRouteMetadata> {
    oxc_route_metadata(program, source_text)
}

#[cfg(feature = "oxc")]
fn oxc_route_metadata(
    program: &oxc_ast::ast::Program<'_>,
    source_text: &str,
) -> Option<DxTsxRouteMetadata> {
    program.body.iter().find_map(|statement| {
        let oxc_ast::ast::Statement::ExportNamedDeclaration(export) = statement else {
            return None;
        };
        let Some(oxc_ast::ast::Declaration::VariableDeclaration(variable)) = &export.declaration
        else {
            return None;
        };

        variable.declarations.iter().find_map(|declaration| {
            if oxc_binding_identifier_name(&declaration.id).as_deref() != Some("metadata") {
                return None;
            }
            let Some(oxc_ast::ast::Expression::ObjectExpression(object)) = &declaration.init else {
                return None;
            };
            Some(DxTsxRouteMetadata {
                title: oxc_object_string_field(object, "title"),
                description: oxc_object_string_field(object, "description"),
                canonical: oxc_object_string_field(object, "canonical").or_else(|| {
                    oxc_object_field(object, "alternates")
                        .and_then(|alternates| oxc_object_string_field(alternates, "canonical"))
                }),
                span: oxc_span_at(
                    source_text,
                    object.span.start as usize,
                    object.span.end as usize,
                ),
            })
        })
    })
}

#[cfg(feature = "oxc")]
fn oxc_binding_identifier_name(pattern: &oxc_ast::ast::BindingPattern<'_>) -> Option<String> {
    let oxc_ast::ast::BindingPatternKind::BindingIdentifier(identifier) = &pattern.kind else {
        return None;
    };
    Some(identifier.name.to_string())
}

#[cfg(feature = "oxc")]
fn oxc_object_field<'a>(
    object: &'a oxc_ast::ast::ObjectExpression<'a>,
    field: &str,
) -> Option<&'a oxc_ast::ast::ObjectExpression<'a>> {
    object.properties.iter().find_map(|property| {
        let oxc_ast::ast::ObjectPropertyKind::ObjectProperty(property) = property else {
            return None;
        };
        if property.computed || oxc_property_name(&property.key).as_deref() != Some(field) {
            return None;
        }
        let oxc_ast::ast::Expression::ObjectExpression(value) = &property.value else {
            return None;
        };
        Some(value.as_ref())
    })
}

#[cfg(feature = "oxc")]
fn oxc_object_string_field(
    object: &oxc_ast::ast::ObjectExpression<'_>,
    field: &str,
) -> Option<String> {
    object.properties.iter().find_map(|property| {
        let oxc_ast::ast::ObjectPropertyKind::ObjectProperty(property) = property else {
            return None;
        };
        if property.computed || oxc_property_name(&property.key).as_deref() != Some(field) {
            return None;
        }
        oxc_string_literal_expression(&property.value)
    })
}

#[cfg(feature = "oxc")]
fn oxc_property_name(key: &oxc_ast::ast::PropertyKey<'_>) -> Option<String> {
    match key {
        oxc_ast::ast::PropertyKey::StaticIdentifier(identifier) => {
            Some(identifier.name.to_string())
        }
        oxc_ast::ast::PropertyKey::StringLiteral(literal) => Some(literal.value.to_string()),
        _ => None,
    }
}

#[cfg(feature = "oxc")]
fn oxc_string_literal_expression(expression: &oxc_ast::ast::Expression<'_>) -> Option<String> {
    match expression {
        oxc_ast::ast::Expression::StringLiteral(literal) => Some(literal.value.to_string()),
        _ => None,
    }
}

#[cfg(feature = "oxc")]
fn oxc_source_type_label(source_type: oxc_span::SourceType) -> &'static str {
    if source_type.is_typescript() && source_type.is_jsx() {
        "tsx"
    } else if source_type.is_typescript() {
        "ts"
    } else if source_type.is_jsx() {
        "jsx"
    } else {
        "js"
    }
}

#[cfg(not(feature = "oxc"))]
fn fallback_source_type(source_path: &str) -> &'static str {
    if source_path.ends_with(".tsx") {
        "tsx"
    } else if source_path.ends_with(".ts") {
        "ts"
    } else if source_path.ends_with(".jsx") {
        "jsx"
    } else {
        "js"
    }
}

fn parse_import_clause(
    source_text: &str,
    start: usize,
    end: usize,
    clause: &str,
    source: String,
) -> DxReactImport {
    let mut clause = clause.trim();
    let type_only = clause
        .strip_prefix("type ")
        .map(|rest| {
            clause = rest.trim();
            true
        })
        .unwrap_or(false);
    let mut default = None;
    let mut namespace = None;
    let mut specifiers = Vec::new();

    if let Some(rest) = clause.strip_prefix("* as ") {
        namespace = Some(rest.trim().to_string());
    } else if let (Some(open), Some(close)) = (clause.find('{'), clause.rfind('}')) {
        let default_part = clause[..open].trim().trim_end_matches(',').trim();
        if !default_part.is_empty() {
            default = Some(default_part.to_string());
        }
        specifiers.extend(clause[open + 1..close].split(',').filter_map(|specifier| {
            parse_import_specifier(source_text, start, specifier, type_only)
        }));
    } else if !clause.is_empty() {
        default = Some(clause.to_string());
    }

    DxReactImport {
        source,
        default,
        namespace,
        specifiers,
        side_effect_only: false,
        type_only,
        span: span_at(source_text, start, end),
    }
}

fn parse_import_specifier(
    source_text: &str,
    import_start: usize,
    value: &str,
    parent_type_only: bool,
) -> Option<DxReactImportSpecifier> {
    let mut value = value.trim();
    if value.is_empty() {
        return None;
    }
    let type_only = parent_type_only
        || value
            .strip_prefix("type ")
            .map(|rest| {
                value = rest.trim();
                true
            })
            .unwrap_or(false);
    let (imported, local) = value
        .split_once(" as ")
        .map(|(imported, local)| (imported.trim(), local.trim()))
        .unwrap_or((value, value));
    let specifier_start = source_text[import_start..]
        .find(value)
        .map(|offset| import_start + offset)
        .unwrap_or(import_start);
    Some(DxReactImportSpecifier {
        imported: imported.to_string(),
        local: local.to_string(),
        type_only,
        span: span_at(
            source_text,
            specifier_start,
            specifier_start.saturating_add(value.len()),
        ),
    })
}

fn find_keyword_outside_code(source: &str, keyword: &str, from: usize) -> Option<usize> {
    let mut scanner = CodeScanner::new(source, from);
    while let Some(index) = scanner.next_code_index() {
        if source[index..].starts_with(keyword) && is_word_boundary(source, index, keyword.len()) {
            return Some(index);
        }
    }
    None
}

fn find_sequence_outside_code(source: &str, words: &[&str]) -> Option<usize> {
    let mut cursor = 0usize;
    while let Some(start) = find_keyword_outside_code(source, words[0], cursor) {
        let mut index = start + words[0].len();
        let mut matched = true;
        for word in &words[1..] {
            index = skip_ws(source, index);
            if !source[index..].starts_with(word) || !is_word_boundary(source, index, word.len()) {
                matched = false;
                break;
            }
            index += word.len();
        }
        if matched {
            return Some(start);
        }
        cursor = start + words[0].len();
    }
    None
}

fn find_keyword_in_segment(segment: &str, keyword: &str) -> Option<usize> {
    let mut cursor = 0usize;
    while cursor < segment.len() {
        if segment[cursor..].starts_with(keyword)
            && is_word_boundary(segment, cursor, keyword.len())
        {
            return Some(cursor);
        }
        cursor += segment[cursor..].chars().next()?.len_utf8();
    }
    None
}

fn find_statement_end(source: &str, mut cursor: usize) -> Option<usize> {
    let mut quote = None;
    let mut brace_depth = 0usize;
    while cursor < source.len() {
        let ch = source[cursor..].chars().next()?;
        if let Some(active_quote) = quote {
            if ch == active_quote {
                quote = None;
            } else if ch == '\\' {
                cursor += ch.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..].chars().next()?.len_utf8();
                    continue;
                }
            }
            cursor += ch.len_utf8();
            continue;
        }
        match ch {
            '"' | '\'' | '`' => quote = Some(ch),
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            ';' if brace_depth == 0 => return Some(cursor),
            '\n' if brace_depth == 0 => return Some(cursor),
            _ => {}
        }
        cursor += ch.len_utf8();
    }
    Some(source.len())
}

fn find_balanced_block(source: &str, mut cursor: usize) -> Option<usize> {
    let mut quote = None;
    let mut depth = 0usize;
    while cursor < source.len() {
        let ch = source[cursor..].chars().next()?;
        if let Some(active_quote) = quote {
            if ch == active_quote {
                quote = None;
            } else if ch == '\\' {
                cursor += ch.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..].chars().next()?.len_utf8();
                    continue;
                }
            }
            cursor += ch.len_utf8();
            continue;
        }
        match ch {
            '"' | '\'' | '`' => quote = Some(ch),
            '{' => depth += 1,
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(cursor);
                }
            }
            _ => {}
        }
        cursor += ch.len_utf8();
    }
    None
}

fn object_string_field(object: &str, field: &str) -> Option<String> {
    let field_index = object.find(field)?;
    let after_field = skip_ws(object, field_index + field.len());
    if !object[after_field..].starts_with(':') {
        return None;
    }
    let value_start = skip_ws(object, after_field + 1);
    parse_quoted_at(object, value_start).map(|(value, _)| value)
}

fn parse_quoted_at(source: &str, start: usize) -> Option<(String, usize)> {
    let quote = source[start..].chars().next()?;
    if !matches!(quote, '"' | '\'') {
        return None;
    }
    let mut cursor = start + quote.len_utf8();
    let value_start = cursor;
    while cursor < source.len() {
        let ch = source[cursor..].chars().next()?;
        if ch == quote {
            return Some((
                source[value_start..cursor].to_string(),
                cursor + ch.len_utf8(),
            ));
        }
        if ch == '\\' {
            cursor += ch.len_utf8();
            if cursor < source.len() {
                cursor += source[cursor..].chars().next()?.len_utf8();
                continue;
            }
        }
        cursor += ch.len_utf8();
    }
    None
}

fn consume_optional_semicolon(source: &str, cursor: usize) -> usize {
    let cursor = skip_ws(source, cursor);
    if source[cursor..].starts_with(';') {
        cursor + 1
    } else {
        cursor
    }
}

fn skip_ws(source: &str, mut cursor: usize) -> usize {
    while cursor < source.len()
        && source[cursor..]
            .chars()
            .next()
            .is_some_and(char::is_whitespace)
    {
        cursor += source[cursor..]
            .chars()
            .next()
            .unwrap_or_default()
            .len_utf8();
    }
    cursor
}

fn is_word_boundary(source: &str, start: usize, len: usize) -> bool {
    let before = source[..start]
        .chars()
        .next_back()
        .is_none_or(|ch| !is_identifier_char(ch));
    let after = source[start + len..]
        .chars()
        .next()
        .is_none_or(|ch| !is_identifier_char(ch));
    before && after
}

fn is_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '_' | '$')
}

fn span_at(source: &str, start: usize, end: usize) -> DxTsxSpan {
    let mut line = 1usize;
    let mut column = 1usize;
    for ch in source[..start].chars() {
        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    DxTsxSpan {
        start,
        end,
        line,
        column,
    }
}

struct CodeScanner<'a> {
    source: &'a str,
    cursor: usize,
    quote: Option<char>,
    line_comment: bool,
    block_comment: bool,
    last_significant_code_char: Option<char>,
    last_significant_code_word: Option<&'a str>,
}

impl<'a> CodeScanner<'a> {
    fn new(source: &'a str, cursor: usize) -> Self {
        Self {
            source,
            cursor,
            quote: None,
            line_comment: false,
            block_comment: false,
            last_significant_code_char: None,
            last_significant_code_word: None,
        }
    }

    fn next_code_index(&mut self) -> Option<usize> {
        while self.cursor < self.source.len() {
            let index = self.cursor;
            let rest = &self.source[index..];
            let ch = rest.chars().next()?;
            self.cursor += ch.len_utf8();

            if self.line_comment {
                if ch == '\n' {
                    self.line_comment = false;
                }
                continue;
            }
            if self.block_comment {
                if rest.starts_with("*/") {
                    self.cursor = index + 2;
                    self.block_comment = false;
                }
                continue;
            }
            if let Some(active_quote) = self.quote {
                if ch == active_quote {
                    self.quote = None;
                } else if ch == '\\' && self.cursor < self.source.len() {
                    self.cursor += self.source[self.cursor..]
                        .chars()
                        .next()
                        .unwrap_or_default()
                        .len_utf8();
                }
                continue;
            }

            if rest.starts_with("//") {
                self.cursor = index + 2;
                self.line_comment = true;
                continue;
            }
            if rest.starts_with("/*") {
                self.cursor = index + 2;
                self.block_comment = true;
                continue;
            }
            if matches!(ch, '"' | '\'' | '`') {
                self.quote = Some(ch);
                continue;
            }
            if is_identifier_start(ch) {
                let end = identifier_end(self.source, index);
                let word = &self.source[index..end];
                self.cursor = end;
                self.last_significant_code_char = word.chars().next_back();
                self.last_significant_code_word = Some(word);
                return Some(index);
            }
            if ch == '/'
                && can_start_regex_literal(
                    self.last_significant_code_char,
                    self.last_significant_code_word,
                )
                && let Some(end) = skip_regex_literal(self.source, index)
            {
                self.cursor = end;
                continue;
            }
            if !ch.is_whitespace() {
                self.last_significant_code_char = Some(ch);
                self.last_significant_code_word = None;
            }
            return Some(index);
        }
        None
    }
}

fn can_start_regex_literal(previous_char: Option<char>, previous_word: Option<&str>) -> bool {
    if previous_word.is_some_and(regex_expression_keyword) {
        return true;
    }

    previous_char.is_none_or(|ch| {
        matches!(
            ch,
            '=' | '>' | '(' | '[' | '{' | ',' | ':' | ';' | '!' | '?' | '|' | '&'
        )
    })
}

fn regex_expression_keyword(word: &str) -> bool {
    matches!(
        word,
        "return" | "throw" | "case" | "yield" | "await" | "typeof" | "void" | "delete" | "new"
    )
}

fn is_identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || matches!(ch, '_' | '$')
}

fn identifier_end(source: &str, start: usize) -> usize {
    let mut cursor = start;
    while cursor < source.len()
        && source[cursor..]
            .chars()
            .next()
            .is_some_and(is_identifier_char)
    {
        cursor += source[cursor..]
            .chars()
            .next()
            .unwrap_or_default()
            .len_utf8();
    }
    cursor
}

fn skip_regex_literal(source: &str, start: usize) -> Option<usize> {
    debug_assert!(source[start..].starts_with('/'));

    let mut cursor = start + 1;
    let mut escaped = false;
    let mut in_character_class = false;
    let mut saw_body = false;

    while cursor < source.len() {
        let ch = source[cursor..].chars().next()?;
        if escaped {
            escaped = false;
            saw_body = true;
            cursor += ch.len_utf8();
            continue;
        }

        match ch {
            '\\' => {
                escaped = true;
                cursor += ch.len_utf8();
                continue;
            }
            '[' => {
                in_character_class = true;
                saw_body = true;
            }
            ']' if in_character_class => {
                in_character_class = false;
                saw_body = true;
            }
            '/' if !in_character_class => {
                cursor += ch.len_utf8();
                while cursor < source.len() {
                    let flag = source[cursor..].chars().next()?;
                    if flag.is_ascii_alphabetic() {
                        cursor += flag.len_utf8();
                    } else {
                        break;
                    }
                }
                return saw_body.then_some(cursor);
            }
            '\n' | '\r' => return None,
            _ => saw_body = true,
        }
        cursor += ch.len_utf8();
    }

    None
}
