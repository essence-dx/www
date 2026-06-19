use serde::{Deserialize, Serialize};

use super::tsx_ast::{DxTsxDiagnostic, DxTsxParserBackend, DxTsxSpan, parse_tsx_module};
#[cfg(feature = "oxc")]
use oxc_span::GetSpan;

/// Structured JSX/TSX surface extracted from React-shaped source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactJsxDocument {
    /// Project-relative source path.
    pub source_path: String,
    /// ES imports visible to the compiler.
    pub imports: Vec<DxReactImport>,
    /// JSX elements in source order.
    pub elements: Vec<DxReactJsxElement>,
    /// JSX text nodes in source order.
    pub text_nodes: Vec<String>,
    /// JSX expression bodies in source order.
    pub expressions: Vec<String>,
    /// Event attributes in source order.
    pub event_attributes: Vec<DxReactEventAttribute>,
    /// Conditional JSX branches in source order.
    pub conditional_branches: Vec<DxReactJsxConditionalBranch>,
    /// JSX list iterations in source order.
    pub list_iterations: Vec<DxReactJsxListIteration>,
    /// Key expressions used for stable keyed updates.
    pub keyed_update_hints: Vec<DxReactJsxKeyHint>,
    /// Whether the source uses a JSX fragment.
    pub has_fragment: bool,
    /// Module-level parse diagnostics.
    pub diagnostics: Vec<DxTsxDiagnostic>,
    /// TSX parser backend used to validate this source.
    pub parser_backend: DxTsxParserBackend,
    /// JSX lowering backend used for elements, attributes, text, and expression surfaces.
    pub jsx_backend: DxReactJsxExtractionBackend,
}

/// Parser-backed evidence for JSX lowering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactJsxExtractionBackend {
    /// Stable schema for DX Check, Studio, and source-render consumers.
    pub schema: String,
    /// Numeric revision kept separate from the schema name so public contracts stay readable.
    pub schema_revision: u16,
    /// Backend selected for JSX extraction.
    pub active_backend: String,
    /// Human-readable status.
    pub status: String,
    /// Whether Oxc produced the JSX surface.
    pub oxc_available: bool,
    /// Whether the custom scanner was kept as fallback or comparison surface.
    pub custom_scanner_fallback: bool,
    /// JSX element count reported by the active backend.
    pub element_count: usize,
    /// JSX expression count reported by the active backend.
    pub expression_count: usize,
}

/// One ES import statement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactImport {
    /// Module specifier, such as `../ui`.
    pub source: String,
    /// Default import local name.
    pub default: Option<String>,
    /// Namespace import local name.
    pub namespace: Option<String>,
    /// Named imports.
    pub specifiers: Vec<DxReactImportSpecifier>,
    /// Whether this is `import "module";`.
    pub side_effect_only: bool,
    /// Whether this is an `import type`.
    pub type_only: bool,
    /// Source span for the import declaration.
    pub span: DxTsxSpan,
}

/// One named import specifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactImportSpecifier {
    /// Imported symbol name.
    pub imported: String,
    /// Local binding name after aliasing.
    pub local: String,
    /// Whether this specifier is type-only.
    pub type_only: bool,
    /// Source span for this import specifier.
    pub span: DxTsxSpan,
}

/// One JSX element.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactJsxElement {
    /// Tag or component name.
    pub name: String,
    /// JSX attributes.
    pub attributes: Vec<DxReactJsxAttribute>,
    /// Whether the element is self-closing.
    pub self_closing: bool,
    /// Parent JSX element index, when this element is nested inside another element.
    pub parent_index: Option<usize>,
    /// Direct JSX children in source order.
    pub child_nodes: Vec<DxReactJsxChildNode>,
    /// Text directly owned by this element.
    pub child_text: Vec<String>,
    /// Expression bodies directly owned by this element.
    pub child_expressions: Vec<String>,
}

impl DxReactJsxElement {
    /// Return a string literal attribute value.
    pub fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|attribute| attribute.name == name)
            .and_then(|attribute| attribute.value.as_deref())
    }

    /// Return direct child text collapsed for fallback extraction.
    pub fn text_content(&self) -> String {
        self.child_text.join(" ").trim().to_string()
    }
}

/// One direct JSX child owned by an element, preserving source order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum DxReactJsxChildNode {
    /// Literal JSX text.
    Text { value: String },
    /// JSX expression container body.
    Expression { expression: String },
    /// Child JSX element by index in the document element array.
    Element { index: usize },
}

/// One JSX attribute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactJsxAttribute {
    /// Attribute name.
    pub name: String,
    /// Literal value for `"..."` or bare attribute text.
    pub value: Option<String>,
    /// Expression body for `{...}` values.
    pub expression: Option<String>,
}

/// One event handler attribute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactEventAttribute {
    /// Element that owns the event.
    pub element: String,
    /// Event attribute name, such as `onClick`.
    pub name: String,
    /// Handler expression body.
    pub expression: String,
}

/// One conditional JSX expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactJsxConditionalBranch {
    /// Condition expression that selects the branch.
    pub condition: String,
    /// JSX/expression emitted for the truthy branch.
    pub when_true: String,
    /// JSX/expression emitted for the falsy branch, when present.
    pub when_false: Option<String>,
}

/// One `.map()` JSX list expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactJsxListIteration {
    /// Source collection expression.
    pub source: String,
    /// Item binding name.
    pub item_binding: String,
    /// Key expression when present.
    pub key_expression: Option<String>,
}

/// One keyed update hint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactJsxKeyHint {
    /// Element that owns the key.
    pub element: String,
    /// Literal key value when present.
    pub value: Option<String>,
    /// Expression key value when present.
    pub expression: Option<String>,
}

/// Lower a React-shaped TSX/JSX source file into a structured compiler surface.
pub fn lower_react_jsx_source(source_path: &str, source: &str) -> DxReactJsxDocument {
    let source = source.strip_prefix('\u{feff}').unwrap_or(source);
    let module = parse_tsx_module(source_path, source);
    let mut lowerer = JsxLowerer::new(
        source_path,
        source,
        module.imports,
        module.diagnostics,
        module.parser_backend,
    );
    lowerer.lower();
    lower_with_oxc_jsx_surface(source_path, source, &mut lowerer.document);
    lowerer.document
}

#[cfg(not(feature = "oxc"))]
fn lower_with_oxc_jsx_surface(
    _source_path: &str,
    _source: &str,
    document: &mut DxReactJsxDocument,
) {
    document.jsx_backend.element_count = document.elements.len();
    document.jsx_backend.expression_count = document.expressions.len();

    if document.parser_backend.oxc_available {
        document.jsx_backend.oxc_available = true;
        document.jsx_backend.custom_scanner_fallback = true;
        document.jsx_backend.status = "oxc-validated-scanner-jsx-surface".to_string();
    }
}

struct JsxLowerer<'a> {
    source: &'a str,
    document: DxReactJsxDocument,
    open_elements: Vec<usize>,
}

impl<'a> JsxLowerer<'a> {
    fn new(
        source_path: &str,
        source: &'a str,
        imports: Vec<DxReactImport>,
        diagnostics: Vec<DxTsxDiagnostic>,
        parser_backend: DxTsxParserBackend,
    ) -> Self {
        Self {
            source,
            document: DxReactJsxDocument {
                source_path: source_path.to_string(),
                imports,
                elements: Vec::new(),
                text_nodes: Vec::new(),
                expressions: Vec::new(),
                event_attributes: Vec::new(),
                conditional_branches: Vec::new(),
                list_iterations: Vec::new(),
                keyed_update_hints: Vec::new(),
                has_fragment: false,
                diagnostics,
                parser_backend,
                jsx_backend: DxReactJsxExtractionBackend {
                    schema: "dx.tsx.scannerJsxSurface".to_string(),
                    schema_revision: 1,
                    active_backend: "custom-scanner".to_string(),
                    status: "scanner-jsx-surface".to_string(),
                    oxc_available: false,
                    custom_scanner_fallback: true,
                    element_count: 0,
                    expression_count: 0,
                },
            },
            open_elements: Vec::new(),
        }
    }

    fn lower(&mut self) {
        let mut index = 0usize;
        while index < self.source.len() {
            let rest = &self.source[index..];
            if rest.starts_with("<>") && self.looks_like_jsx_start(index) {
                self.document.has_fragment = true;
                index += 2;
                continue;
            }
            if rest.starts_with("</>") {
                index += 3;
                continue;
            }
            if rest.starts_with("</") {
                index = self.consume_closing_tag(index);
                continue;
            }
            if rest.starts_with('<') && self.looks_like_jsx_start(index) {
                if let Some(next) = self.consume_opening_tag(index) {
                    index = next;
                    continue;
                }
            }

            let next = if self.open_elements.is_empty() {
                let search_start = next_source_index(self.source, index);
                self.source[search_start..]
                    .find('<')
                    .map(|offset| search_start + offset)
                    .unwrap_or(self.source.len())
            } else {
                find_next_jsx_tag_start(self.source, index).unwrap_or(self.source.len())
            };
            if !self.open_elements.is_empty() {
                self.consume_text_and_expressions(&self.source[index..next]);
            }
            index = next;
        }
    }

    fn looks_like_jsx_start(&self, index: usize) -> bool {
        if !self.open_elements.is_empty() {
            return true;
        }
        let prefix = &self.source[..index];
        let trimmed = prefix.trim_end();
        if trimmed.ends_with("return") {
            return true;
        }
        trimmed
            .chars()
            .next_back()
            .is_some_and(|value| matches!(value, '(' | '[' | '{' | '>' | ':' | '=' | ','))
    }

    fn consume_opening_tag(&mut self, index: usize) -> Option<usize> {
        let mut cursor = index + 1;
        let name_start = cursor;
        while cursor < self.source.len() {
            let ch = self.source[cursor..].chars().next()?;
            if ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.' | ':') {
                cursor += ch.len_utf8();
            } else {
                break;
            }
        }
        if cursor == name_start {
            return None;
        }
        let name = self.source[name_start..cursor].to_string();
        let end = find_tag_end(self.source, cursor)?;
        let raw_attrs = &self.source[cursor..end];
        let self_closing = raw_attrs.trim_end().ends_with('/');
        let attrs = raw_attrs.trim().trim_end_matches('/').trim();
        let attributes = parse_attributes(attrs);
        let element_index = self.document.elements.len();
        let parent_index = self.open_elements.last().copied();
        for attribute in &attributes {
            if attribute.name.starts_with("on") && attribute.expression.is_some() {
                self.document.event_attributes.push(DxReactEventAttribute {
                    element: name.clone(),
                    name: attribute.name.clone(),
                    expression: attribute.expression.clone().unwrap_or_default(),
                });
            }
            if attribute.name == "key" {
                self.document.keyed_update_hints.push(DxReactJsxKeyHint {
                    element: name.clone(),
                    value: attribute.value.clone(),
                    expression: attribute.expression.clone(),
                });
            }
        }
        self.document.elements.push(DxReactJsxElement {
            name,
            attributes,
            self_closing,
            parent_index,
            child_nodes: Vec::new(),
            child_text: Vec::new(),
            child_expressions: Vec::new(),
        });
        if let Some(parent_index) = parent_index {
            if let Some(parent) = self.document.elements.get_mut(parent_index) {
                parent.child_nodes.push(DxReactJsxChildNode::Element {
                    index: element_index,
                });
            }
        }
        if !self_closing {
            self.open_elements.push(element_index);
        }
        Some(end + 1)
    }

    fn consume_closing_tag(&mut self, index: usize) -> usize {
        let name_start = index + 2;
        let Some(end) = self.source[name_start..]
            .find('>')
            .map(|offset| name_start + offset)
        else {
            return self.source.len();
        };
        let name = self.source[name_start..end].trim();
        if let Some(position) = self
            .open_elements
            .iter()
            .rposition(|element| self.document.elements[*element].name == name)
        {
            self.open_elements.truncate(position);
        }
        end + 1
    }

    fn consume_text_and_expressions(&mut self, segment: &str) {
        let mut cursor = 0usize;
        while cursor < segment.len() {
            let rest = &segment[cursor..];
            if rest.starts_with('{') {
                if let Some(end) = find_balanced_expression(rest) {
                    self.push_expression(&rest[1..end]);
                    cursor += end + 1;
                    continue;
                }
                cursor += 1;
                continue;
            }
            let next_expression = rest.find('{').unwrap_or(rest.len());
            self.push_text(&rest[..next_expression]);
            cursor += next_expression;
        }
    }

    fn push_text(&mut self, value: &str) {
        let text = normalize_jsx_text(value);
        if text.is_empty() {
            return;
        }
        self.document.text_nodes.push(text.clone());
        if let Some(element) = self.current_element_mut() {
            element.child_nodes.push(DxReactJsxChildNode::Text {
                value: text.clone(),
            });
            element.child_text.push(text);
        }
    }

    fn push_expression(&mut self, value: &str) {
        let expression = value.trim();
        if expression.is_empty() {
            return;
        }
        self.document.expressions.push(expression.to_string());
        if let Some(branch) = parse_conditional_branch(expression) {
            self.document.conditional_branches.push(branch);
        }
        if let Some(iteration) = parse_list_iteration(expression) {
            self.document.list_iterations.push(iteration);
        }
        if let Some(key_hint) = parse_key_hint(expression) {
            self.document.keyed_update_hints.push(key_hint);
        }
        if let Some(element) = self.current_element_mut() {
            element.child_nodes.push(DxReactJsxChildNode::Expression {
                expression: expression.to_string(),
            });
            element.child_expressions.push(expression.to_string());
        }
    }

    fn current_element_mut(&mut self) -> Option<&mut DxReactJsxElement> {
        let index = *self.open_elements.last()?;
        self.document.elements.get_mut(index)
    }
}

#[cfg(feature = "oxc")]
fn lower_with_oxc_jsx_surface(source_path: &str, source: &str, document: &mut DxReactJsxDocument) {
    if let Some(surface) = parse_oxc_jsx_surface(source_path, source) {
        document.elements = surface.elements;
        document.text_nodes = surface.text_nodes;
        document.expressions = surface.expressions;
        document.event_attributes = surface.event_attributes;
        document.conditional_branches = surface.conditional_branches;
        document.list_iterations = surface.list_iterations;
        document.keyed_update_hints = surface.keyed_update_hints;
        document.has_fragment = surface.has_fragment;
        document.jsx_backend = surface.backend;
    }
}

#[cfg(feature = "oxc")]
fn parse_oxc_jsx_surface<'a>(source_path: &str, source: &'a str) -> Option<OxcJsxSurface<'a>> {
    use oxc_allocator::Allocator;
    use oxc_parser::Parser;
    use oxc_span::SourceType;

    let allocator = Allocator::default();
    let source_type = SourceType::from_path(source_path).unwrap_or_else(|_| SourceType::tsx());
    let parser_return = Parser::new(&allocator, source, source_type).parse();
    let panicked = parser_return.panicked;
    let syntax_errors = parser_return.errors.len();
    let mut surface = OxcJsxSurface::new(source, panicked, syntax_errors);
    for statement in &parser_return.program.body {
        surface.collect_oxc_statement(statement);
    }
    surface.finish();
    Some(surface)
}

#[cfg(feature = "oxc")]
struct OxcJsxSurface<'a> {
    source: &'a str,
    backend: DxReactJsxExtractionBackend,
    elements: Vec<DxReactJsxElement>,
    text_nodes: Vec<String>,
    expressions: Vec<String>,
    event_attributes: Vec<DxReactEventAttribute>,
    conditional_branches: Vec<DxReactJsxConditionalBranch>,
    list_iterations: Vec<DxReactJsxListIteration>,
    keyed_update_hints: Vec<DxReactJsxKeyHint>,
    has_fragment: bool,
}

#[cfg(feature = "oxc")]
impl<'a> OxcJsxSurface<'a> {
    fn new(source: &'a str, panicked: bool, syntax_errors: usize) -> Self {
        let status = if panicked {
            "oxc-jsx-parser-panicked"
        } else if syntax_errors == 0 {
            "oxc-jsx-surface"
        } else {
            "oxc-jsx-surface-with-errors"
        };
        Self {
            source,
            backend: DxReactJsxExtractionBackend {
                schema: "dx.tsx.oxcJsxSurface".to_string(),
                schema_revision: 1,
                active_backend: "oxc-jsx-ast".to_string(),
                status: status.to_string(),
                oxc_available: true,
                custom_scanner_fallback: true,
                element_count: 0,
                expression_count: 0,
            },
            elements: Vec::new(),
            text_nodes: Vec::new(),
            expressions: Vec::new(),
            event_attributes: Vec::new(),
            conditional_branches: Vec::new(),
            list_iterations: Vec::new(),
            keyed_update_hints: Vec::new(),
            has_fragment: false,
        }
    }

    fn finish(&mut self) {
        self.backend.element_count = self.elements.len();
        self.backend.expression_count = self.expressions.len();
    }

    fn collect_oxc_statement(&mut self, statement: &oxc_ast::ast::Statement<'_>) {
        match statement {
            oxc_ast::ast::Statement::BlockStatement(statement) => {
                for statement in &statement.body {
                    self.collect_oxc_statement(statement);
                }
            }
            oxc_ast::ast::Statement::ReturnStatement(statement) => {
                if let Some(argument) = &statement.argument {
                    self.collect_oxc_expression(argument);
                }
            }
            oxc_ast::ast::Statement::ExpressionStatement(statement) => {
                self.collect_oxc_expression(&statement.expression);
            }
            oxc_ast::ast::Statement::VariableDeclaration(declaration) => {
                for declarator in &declaration.declarations {
                    if let Some(init) = &declarator.init {
                        self.collect_oxc_expression(init);
                    }
                }
            }
            oxc_ast::ast::Statement::FunctionDeclaration(function) => {
                self.collect_oxc_function_body(function.body.as_deref());
            }
            oxc_ast::ast::Statement::IfStatement(statement) => {
                self.collect_oxc_expression(&statement.test);
                self.collect_oxc_statement_branch(&statement.consequent);
                if let Some(alternate) = &statement.alternate {
                    self.collect_oxc_statement_branch(alternate);
                }
            }
            oxc_ast::ast::Statement::ForStatement(statement) => {
                if let Some(init) = &statement.init {
                    if let Some(expression) = init.as_expression() {
                        self.collect_oxc_expression(expression);
                    }
                }
                if let Some(test) = &statement.test {
                    self.collect_oxc_expression(test);
                }
                if let Some(update) = &statement.update {
                    self.collect_oxc_expression(update);
                }
                self.collect_oxc_statement_branch(&statement.body);
            }
            oxc_ast::ast::Statement::ForOfStatement(statement) => {
                self.collect_oxc_expression(&statement.right);
                self.collect_oxc_statement_branch(&statement.body);
            }
            oxc_ast::ast::Statement::ForInStatement(statement) => {
                self.collect_oxc_expression(&statement.right);
                self.collect_oxc_statement_branch(&statement.body);
            }
            oxc_ast::ast::Statement::WhileStatement(statement) => {
                self.collect_oxc_expression(&statement.test);
                self.collect_oxc_statement_branch(&statement.body);
            }
            oxc_ast::ast::Statement::DoWhileStatement(statement) => {
                self.collect_oxc_statement_branch(&statement.body);
                self.collect_oxc_expression(&statement.test);
            }
            oxc_ast::ast::Statement::ThrowStatement(statement) => {
                self.collect_oxc_expression(&statement.argument);
            }
            oxc_ast::ast::Statement::ExportNamedDeclaration(export) => {
                if let Some(declaration) = &export.declaration {
                    self.collect_oxc_declaration(declaration);
                }
            }
            oxc_ast::ast::Statement::ExportDefaultDeclaration(export) => {
                self.collect_oxc_export_default(&export.declaration);
            }
            _ => {}
        }
    }

    fn collect_oxc_statement_branch(&mut self, statement: &oxc_ast::ast::Statement<'_>) {
        self.collect_oxc_statement(statement);
    }

    fn collect_oxc_declaration(&mut self, declaration: &oxc_ast::ast::Declaration<'_>) {
        match declaration {
            oxc_ast::ast::Declaration::FunctionDeclaration(function) => {
                self.collect_oxc_function_body(function.body.as_deref());
            }
            oxc_ast::ast::Declaration::VariableDeclaration(declaration) => {
                for declarator in &declaration.declarations {
                    if let Some(init) = &declarator.init {
                        self.collect_oxc_expression(init);
                    }
                }
            }
            _ => {}
        }
    }

    fn collect_oxc_export_default(
        &mut self,
        declaration: &oxc_ast::ast::ExportDefaultDeclarationKind<'_>,
    ) {
        match declaration {
            oxc_ast::ast::ExportDefaultDeclarationKind::FunctionDeclaration(function) => {
                self.collect_oxc_function_body(function.body.as_deref());
            }
            oxc_ast::ast::ExportDefaultDeclarationKind::JSXElement(element) => {
                self.collect_oxc_jsx_element(element, None);
            }
            oxc_ast::ast::ExportDefaultDeclarationKind::JSXFragment(fragment) => {
                self.collect_oxc_jsx_fragment(fragment, None);
            }
            oxc_ast::ast::ExportDefaultDeclarationKind::ParenthesizedExpression(expression) => {
                self.collect_oxc_expression(&expression.expression);
            }
            _ => {}
        }
    }

    fn collect_oxc_function_body(&mut self, body: Option<&oxc_ast::ast::FunctionBody<'_>>) {
        let Some(body) = body else {
            return;
        };
        for statement in &body.statements {
            self.collect_oxc_statement(statement);
        }
    }

    fn collect_oxc_expression(&mut self, expression: &oxc_ast::ast::Expression<'_>) {
        match expression {
            oxc_ast::ast::Expression::JSXElement(element) => {
                self.collect_oxc_jsx_element(element, None);
            }
            oxc_ast::ast::Expression::JSXFragment(fragment) => {
                self.collect_oxc_jsx_fragment(fragment, None);
            }
            oxc_ast::ast::Expression::ParenthesizedExpression(expression) => {
                self.collect_oxc_expression(&expression.expression);
            }
            oxc_ast::ast::Expression::ArrayExpression(expression) => {
                self.collect_oxc_array_expression(expression);
            }
            oxc_ast::ast::Expression::ObjectExpression(expression) => {
                self.collect_oxc_object_expression(expression);
            }
            oxc_ast::ast::Expression::CallExpression(expression) => {
                self.collect_oxc_call_expression(expression);
            }
            oxc_ast::ast::Expression::NewExpression(expression) => {
                self.collect_oxc_expression(&expression.callee);
                for argument in &expression.arguments {
                    self.collect_oxc_argument(argument);
                }
            }
            oxc_ast::ast::Expression::ConditionalExpression(expression) => {
                self.collect_oxc_expression(&expression.test);
                self.collect_oxc_expression(&expression.consequent);
                self.collect_oxc_expression(&expression.alternate);
            }
            oxc_ast::ast::Expression::LogicalExpression(expression) => {
                self.collect_oxc_expression(&expression.left);
                self.collect_oxc_expression(&expression.right);
            }
            oxc_ast::ast::Expression::BinaryExpression(expression) => {
                self.collect_oxc_expression(&expression.left);
                self.collect_oxc_expression(&expression.right);
            }
            oxc_ast::ast::Expression::AssignmentExpression(expression) => {
                self.collect_oxc_expression(&expression.right);
            }
            oxc_ast::ast::Expression::AwaitExpression(expression) => {
                self.collect_oxc_expression(&expression.argument);
            }
            oxc_ast::ast::Expression::SequenceExpression(expression) => {
                for expression in &expression.expressions {
                    self.collect_oxc_expression(expression);
                }
            }
            oxc_ast::ast::Expression::UnaryExpression(expression) => {
                self.collect_oxc_expression(&expression.argument);
            }
            oxc_ast::ast::Expression::ArrowFunctionExpression(function) => {
                self.collect_oxc_function_body(Some(&function.body));
            }
            oxc_ast::ast::Expression::FunctionExpression(function) => {
                self.collect_oxc_function_body(function.body.as_deref());
            }
            oxc_ast::ast::Expression::TSAsExpression(expression) => {
                self.collect_oxc_expression(&expression.expression);
            }
            oxc_ast::ast::Expression::TSSatisfiesExpression(expression) => {
                self.collect_oxc_expression(&expression.expression);
            }
            oxc_ast::ast::Expression::TSTypeAssertion(expression) => {
                self.collect_oxc_expression(&expression.expression);
            }
            oxc_ast::ast::Expression::TSNonNullExpression(expression) => {
                self.collect_oxc_expression(&expression.expression);
            }
            oxc_ast::ast::Expression::TSInstantiationExpression(expression) => {
                self.collect_oxc_expression(&expression.expression);
            }
            _ => {}
        }
    }

    fn collect_oxc_call_expression(&mut self, expression: &oxc_ast::ast::CallExpression<'_>) {
        self.collect_oxc_expression(&expression.callee);
        for argument in &expression.arguments {
            self.collect_oxc_argument(argument);
        }
    }

    fn collect_oxc_argument(&mut self, argument: &oxc_ast::ast::Argument<'_>) {
        if let Some(expression) = argument.as_expression() {
            self.collect_oxc_expression(expression);
            return;
        }
        if let oxc_ast::ast::Argument::SpreadElement(spread) = argument {
            self.collect_oxc_expression(&spread.argument);
        }
    }

    fn collect_oxc_array_expression(&mut self, expression: &oxc_ast::ast::ArrayExpression<'_>) {
        for element in &expression.elements {
            if let Some(expression) = element.as_expression() {
                self.collect_oxc_expression(expression);
                continue;
            }
            if let oxc_ast::ast::ArrayExpressionElement::SpreadElement(spread) = element {
                self.collect_oxc_expression(&spread.argument);
            }
        }
    }

    fn collect_oxc_object_expression(&mut self, expression: &oxc_ast::ast::ObjectExpression<'_>) {
        for property in &expression.properties {
            match property {
                oxc_ast::ast::ObjectPropertyKind::ObjectProperty(property) => {
                    self.collect_oxc_expression(&property.value);
                    if property.computed {
                        if let Some(expression) = property.key.as_expression() {
                            self.collect_oxc_expression(expression);
                        }
                    }
                }
                oxc_ast::ast::ObjectPropertyKind::SpreadProperty(spread) => {
                    self.collect_oxc_expression(&spread.argument);
                }
            }
        }
    }

    fn collect_oxc_jsx_fragment(
        &mut self,
        fragment: &oxc_ast::ast::JSXFragment<'_>,
        owning_element: Option<usize>,
    ) {
        self.has_fragment = true;
        self.collect_oxc_jsx_children(owning_element, &fragment.children);
    }

    fn collect_oxc_jsx_element(
        &mut self,
        element: &oxc_ast::ast::JSXElement<'_>,
        parent_index: Option<usize>,
    ) -> usize {
        let name = oxc_jsx_element_name(&element.opening_element.name);
        let attributes = self.collect_oxc_jsx_attributes(&name, &element.opening_element);
        let element_index = self.elements.len();
        self.elements.push(DxReactJsxElement {
            name,
            attributes,
            self_closing: element.opening_element.self_closing,
            parent_index,
            child_nodes: Vec::new(),
            child_text: Vec::new(),
            child_expressions: Vec::new(),
        });
        self.collect_oxc_jsx_children(Some(element_index), &element.children);
        element_index
    }

    fn collect_oxc_jsx_children(
        &mut self,
        owning_element: Option<usize>,
        children: &oxc_allocator::Vec<'_, oxc_ast::ast::JSXChild<'_>>,
    ) {
        for child in children {
            match child {
                oxc_ast::ast::JSXChild::Text(text) => {
                    let text = normalize_jsx_text(&text.value);
                    if text.is_empty() {
                        continue;
                    }
                    self.text_nodes.push(text.clone());
                    if let Some(index) = owning_element {
                        if let Some(element) = self.elements.get_mut(index) {
                            element.child_nodes.push(DxReactJsxChildNode::Text {
                                value: text.clone(),
                            });
                            element.child_text.push(text);
                        }
                    }
                }
                oxc_ast::ast::JSXChild::ExpressionContainer(container) => {
                    self.push_oxc_expression_container(owning_element, container);
                }
                oxc_ast::ast::JSXChild::Element(element) => {
                    let child_index = self.collect_oxc_jsx_element(element, owning_element);
                    if let Some(index) = owning_element {
                        if let Some(parent) = self.elements.get_mut(index) {
                            parent
                                .child_nodes
                                .push(DxReactJsxChildNode::Element { index: child_index });
                        }
                    }
                }
                oxc_ast::ast::JSXChild::Fragment(fragment) => {
                    self.collect_oxc_jsx_fragment(fragment, owning_element);
                }
                oxc_ast::ast::JSXChild::Spread(spread) => {
                    let expression = source_span_text(
                        self.source,
                        spread.expression.span().start as usize,
                        spread.expression.span().end as usize,
                    );
                    self.push_oxc_child_expression(owning_element, expression);
                }
            }
        }
    }

    fn collect_oxc_jsx_attributes(
        &mut self,
        element_name: &str,
        opening: &oxc_ast::ast::JSXOpeningElement<'_>,
    ) -> Vec<DxReactJsxAttribute> {
        opening
            .attributes
            .iter()
            .map(|attribute| match attribute {
                oxc_ast::ast::JSXAttributeItem::Attribute(attribute) => {
                    let name = oxc_jsx_attribute_name(&attribute.name);
                    let (value, expression) =
                        oxc_jsx_attribute_value(self.source, &attribute.value);
                    if name.starts_with("on") && expression.is_some() {
                        self.event_attributes.push(DxReactEventAttribute {
                            element: element_name.to_string(),
                            name: name.clone(),
                            expression: expression.clone().unwrap_or_default(),
                        });
                    }
                    if name == "key" {
                        self.keyed_update_hints.push(DxReactJsxKeyHint {
                            element: element_name.to_string(),
                            value: value.clone(),
                            expression: expression.clone(),
                        });
                    }
                    DxReactJsxAttribute {
                        name,
                        value,
                        expression,
                    }
                }
                oxc_ast::ast::JSXAttributeItem::SpreadAttribute(attribute) => DxReactJsxAttribute {
                    name: "...".to_string(),
                    value: None,
                    expression: Some(source_span_text(
                        self.source,
                        attribute.argument.span().start as usize,
                        attribute.argument.span().end as usize,
                    )),
                },
            })
            .collect()
    }

    fn push_oxc_expression_container(
        &mut self,
        owning_element: Option<usize>,
        container: &oxc_ast::ast::JSXExpressionContainer<'_>,
    ) {
        if matches!(
            container.expression,
            oxc_ast::ast::JSXExpression::EmptyExpression(_)
        ) {
            return;
        }
        let expression = braced_inner_source(
            self.source,
            container.span.start as usize,
            container.span.end as usize,
        );
        self.collect_oxc_jsx_expression(&container.expression);
        self.push_oxc_child_expression(owning_element, expression);
    }

    fn collect_oxc_jsx_expression(&mut self, expression: &oxc_ast::ast::JSXExpression<'_>) {
        if let Some(expression) = expression.as_expression() {
            self.collect_oxc_expression(expression);
        }
    }

    fn push_oxc_child_expression(&mut self, owning_element: Option<usize>, expression: String) {
        let expression = expression.trim().to_string();
        if expression.is_empty() {
            return;
        }
        self.expressions.push(expression.clone());
        if let Some(branch) = parse_conditional_branch(&expression) {
            self.conditional_branches.push(branch);
        }
        if let Some(iteration) = parse_list_iteration(&expression) {
            self.list_iterations.push(iteration);
        }
        if let Some(key_hint) = parse_key_hint(&expression) {
            self.keyed_update_hints.push(key_hint);
        }
        if let Some(index) = owning_element {
            if let Some(element) = self.elements.get_mut(index) {
                element.child_nodes.push(DxReactJsxChildNode::Expression {
                    expression: expression.clone(),
                });
                element.child_expressions.push(expression);
            }
        }
    }
}

#[cfg(feature = "oxc")]
fn oxc_jsx_attribute_value(
    source: &str,
    value: &Option<oxc_ast::ast::JSXAttributeValue<'_>>,
) -> (Option<String>, Option<String>) {
    let Some(value) = value else {
        return (None, None);
    };
    match value {
        oxc_ast::ast::JSXAttributeValue::StringLiteral(literal) => {
            (Some(literal.value.to_string()), None)
        }
        oxc_ast::ast::JSXAttributeValue::ExpressionContainer(container) => (
            None,
            Some(braced_inner_source(
                source,
                container.span.start as usize,
                container.span.end as usize,
            )),
        ),
        oxc_ast::ast::JSXAttributeValue::Element(element) => (
            None,
            Some(source_span_text(
                source,
                element.span.start as usize,
                element.span.end as usize,
            )),
        ),
        oxc_ast::ast::JSXAttributeValue::Fragment(fragment) => (
            None,
            Some(source_span_text(
                source,
                fragment.span.start as usize,
                fragment.span.end as usize,
            )),
        ),
    }
}

#[cfg(feature = "oxc")]
fn oxc_jsx_attribute_name(name: &oxc_ast::ast::JSXAttributeName<'_>) -> String {
    match name {
        oxc_ast::ast::JSXAttributeName::Identifier(identifier) => identifier.name.to_string(),
        oxc_ast::ast::JSXAttributeName::NamespacedName(name) => {
            format!("{}:{}", name.namespace.name, name.property.name)
        }
    }
}

#[cfg(feature = "oxc")]
fn oxc_jsx_element_name(name: &oxc_ast::ast::JSXElementName<'_>) -> String {
    match name {
        oxc_ast::ast::JSXElementName::Identifier(identifier) => identifier.name.to_string(),
        oxc_ast::ast::JSXElementName::IdentifierReference(identifier) => {
            identifier.name.to_string()
        }
        oxc_ast::ast::JSXElementName::NamespacedName(name) => {
            format!("{}:{}", name.namespace.name, name.property.name)
        }
        oxc_ast::ast::JSXElementName::MemberExpression(expression) => {
            oxc_jsx_member_expression_name(expression)
        }
        oxc_ast::ast::JSXElementName::ThisExpression(_) => "this".to_string(),
    }
}

#[cfg(feature = "oxc")]
fn oxc_jsx_member_expression_name(expression: &oxc_ast::ast::JSXMemberExpression<'_>) -> String {
    format!(
        "{}.{}",
        oxc_jsx_member_expression_object_name(&expression.object),
        expression.property.name
    )
}

#[cfg(feature = "oxc")]
fn oxc_jsx_member_expression_object_name(
    object: &oxc_ast::ast::JSXMemberExpressionObject<'_>,
) -> String {
    match object {
        oxc_ast::ast::JSXMemberExpressionObject::IdentifierReference(identifier) => {
            identifier.name.to_string()
        }
        oxc_ast::ast::JSXMemberExpressionObject::MemberExpression(expression) => {
            oxc_jsx_member_expression_name(expression)
        }
        oxc_ast::ast::JSXMemberExpressionObject::ThisExpression(_) => "this".to_string(),
    }
}

#[cfg(feature = "oxc")]
fn braced_inner_source(source: &str, start: usize, end: usize) -> String {
    let start = start.saturating_add(1);
    let end = end.saturating_sub(1);
    source_span_text(source, start, end)
}

#[cfg(feature = "oxc")]
fn source_span_text(source: &str, start: usize, end: usize) -> String {
    let start = start.min(source.len());
    let end = end.min(source.len()).max(start);
    source
        .get(start..end)
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn next_source_index(source: &str, index: usize) -> usize {
    source[index..]
        .chars()
        .next()
        .map(|ch| index + ch.len_utf8())
        .unwrap_or(source.len())
}

fn parse_attributes(source: &str) -> Vec<DxReactJsxAttribute> {
    let mut attributes = Vec::new();
    let mut cursor = 0usize;
    while cursor < source.len() {
        cursor = skip_whitespace(source, cursor);
        if cursor >= source.len() {
            break;
        }
        let name_start = cursor;
        while cursor < source.len() {
            let ch = source[cursor..].chars().next().unwrap_or_default();
            if ch.is_whitespace() || ch == '=' {
                break;
            }
            cursor += ch.len_utf8();
        }
        let name = source[name_start..cursor].trim();
        if name.is_empty() {
            break;
        }
        cursor = skip_whitespace(source, cursor);
        let mut value = None;
        let mut expression = None;
        if source[cursor..].starts_with('=') {
            cursor += 1;
            cursor = skip_whitespace(source, cursor);
            if source[cursor..].starts_with('"') || source[cursor..].starts_with('\'') {
                let quote = source[cursor..].chars().next().unwrap_or('"');
                cursor += quote.len_utf8();
                let start = cursor;
                while cursor < source.len() && !source[cursor..].starts_with(quote) {
                    cursor += source[cursor..]
                        .chars()
                        .next()
                        .unwrap_or_default()
                        .len_utf8();
                }
                value = Some(source[start..cursor].to_string());
                if cursor < source.len() {
                    cursor += quote.len_utf8();
                }
            } else if source[cursor..].starts_with('{') {
                if let Some(end) = find_balanced_expression(&source[cursor..]) {
                    expression = Some(source[cursor + 1..cursor + end].trim().to_string());
                    cursor += end + 1;
                }
            } else {
                let start = cursor;
                while cursor < source.len()
                    && !source[cursor..]
                        .chars()
                        .next()
                        .unwrap_or_default()
                        .is_whitespace()
                {
                    cursor += source[cursor..]
                        .chars()
                        .next()
                        .unwrap_or_default()
                        .len_utf8();
                }
                value = Some(source[start..cursor].to_string());
            }
        }
        attributes.push(DxReactJsxAttribute {
            name: name.to_string(),
            value,
            expression,
        });
    }
    attributes
}

fn find_next_jsx_tag_start(source: &str, mut cursor: usize) -> Option<usize> {
    cursor += source[cursor..].chars().next()?.len_utf8();
    let mut quote = None;
    let mut brace_depth = 0usize;
    let mut escape = false;
    while cursor < source.len() {
        let ch = source[cursor..].chars().next()?;
        if let Some(active_quote) = quote {
            if escape {
                escape = false;
            } else if ch == '\\' {
                escape = true;
            } else if ch == active_quote {
                quote = None;
            }
            cursor += ch.len_utf8();
            continue;
        }
        match ch {
            '"' | '\'' | '`' => quote = Some(ch),
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            '<' if brace_depth == 0 => return Some(cursor),
            _ => {}
        }
        cursor += ch.len_utf8();
    }
    None
}

fn parse_conditional_branch(expression: &str) -> Option<DxReactJsxConditionalBranch> {
    let expression = expression.trim();
    if let Some(question) = expression.find('?') {
        let colon = expression[question + 1..]
            .rfind(':')
            .map(|offset| question + 1 + offset)?;
        let condition = expression[..question].trim();
        let when_true = expression[question + 1..colon].trim();
        let when_false = expression[colon + 1..].trim();
        if !condition.is_empty() && !when_true.is_empty() && !when_false.is_empty() {
            return Some(DxReactJsxConditionalBranch {
                condition: condition.to_string(),
                when_true: when_true.to_string(),
                when_false: Some(when_false.to_string()),
            });
        }
    }
    if let Some(and_index) = expression.find("&&") {
        let condition = expression[..and_index].trim();
        let when_true = expression[and_index + 2..].trim();
        if !condition.is_empty() && !when_true.is_empty() {
            return Some(DxReactJsxConditionalBranch {
                condition: condition.to_string(),
                when_true: when_true.to_string(),
                when_false: None,
            });
        }
    }
    None
}

fn parse_list_iteration(expression: &str) -> Option<DxReactJsxListIteration> {
    let map_index = expression.find(".map(")?;
    let source = expression[..map_index].trim();
    let mapper = &expression[map_index + ".map(".len()..];
    let arrow = mapper.find("=>")?;
    let item_binding = mapper[..arrow]
        .trim()
        .trim_start_matches('(')
        .trim_end_matches(')')
        .split(',')
        .next()
        .unwrap_or_default()
        .trim();
    if source.is_empty() || item_binding.is_empty() {
        return None;
    }
    Some(DxReactJsxListIteration {
        source: source.to_string(),
        item_binding: item_binding.to_string(),
        key_expression: extract_key_expression(expression),
    })
}

fn parse_key_hint(expression: &str) -> Option<DxReactJsxKeyHint> {
    let key_index = expression.find("key=")?;
    let element = expression[..key_index]
        .rfind('<')
        .and_then(|start| {
            expression[start + 1..key_index]
                .split_whitespace()
                .next()
                .map(str::to_string)
        })
        .filter(|element| !element.is_empty() && !element.starts_with('/'))?;
    Some(DxReactJsxKeyHint {
        element,
        value: extract_key_literal(expression),
        expression: extract_key_expression(expression),
    })
}

fn extract_key_literal(expression: &str) -> Option<String> {
    let key_index = expression.find("key=")? + "key=".len();
    let quote = expression[key_index..].chars().next()?;
    if !matches!(quote, '"' | '\'') {
        return None;
    }
    let start = key_index + quote.len_utf8();
    let end = expression[start..]
        .find(quote)
        .map(|offset| start + offset)?;
    Some(expression[start..end].to_string())
}

fn extract_key_expression(expression: &str) -> Option<String> {
    let key_index = expression.find("key={")? + "key=".len();
    let key_source = &expression[key_index..];
    let end = find_balanced_expression(key_source)?;
    Some(key_source[1..end].trim().to_string())
}

fn find_tag_end(source: &str, mut cursor: usize) -> Option<usize> {
    let mut quote = None;
    let mut brace_depth = 0usize;
    while cursor < source.len() {
        let ch = source[cursor..].chars().next()?;
        if let Some(active_quote) = quote {
            if ch == active_quote {
                quote = None;
            }
            cursor += ch.len_utf8();
            continue;
        }
        match ch {
            '"' | '\'' => quote = Some(ch),
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            '>' if brace_depth == 0 => return Some(cursor),
            _ => {}
        }
        cursor += ch.len_utf8();
    }
    None
}

fn find_balanced_expression(source: &str) -> Option<usize> {
    let mut depth = 0usize;
    let mut quote = None;
    let mut escape = false;
    for (index, ch) in source.char_indices() {
        if let Some(active_quote) = quote {
            if escape {
                escape = false;
            } else if ch == '\\' {
                escape = true;
            } else if ch == active_quote {
                quote = None;
            }
            continue;
        }
        match ch {
            '"' | '\'' | '`' => quote = Some(ch),
            '{' => depth += 1,
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

fn skip_whitespace(source: &str, mut cursor: usize) -> usize {
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

fn normalize_jsx_text(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}
