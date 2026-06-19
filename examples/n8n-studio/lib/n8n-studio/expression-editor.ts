import type {
  ExpressionAnalysis,
  ExpressionDiagnostic,
  ExpressionEditorState,
  ExpressionFieldState,
  ExpressionReference,
  ExpressionReferenceKind,
  ParameterField,
  WorkflowDocument,
  WorkflowNode,
} from "./types";

const expressionReferencePattern =
  /\$(json|node|parameter|workflow|env|credentials)((?:\.[A-Za-z_$][\w$]*|\[['"][^'"]+['"]\]|\["[^"]+"\])*)/g;

function expressionBlocks(value: string) {
  const trimmedValue = value.trim();
  if (trimmedValue.startsWith("={{") && trimmedValue.endsWith("}}")) {
    return {
      mode: "full-expression" as const,
      blocks: [trimmedValue.slice(3, -2).trim()],
    };
  }

  const blocks = [...value.matchAll(/\{\{([\s\S]*?)\}\}/g)].map((match) =>
    match[1].trim(),
  );
  return {
    mode: blocks.length ? ("template-expression" as const) : ("literal" as const),
    blocks,
  };
}

function normalizeReferencePath(pathExpression: string) {
  if (!pathExpression) {
    return undefined;
  }

  const path = pathExpression
    .replace(/\[['"]([^'"]+)['"]\]/g, ".$1")
    .replace(/\["([^"]+)"\]/g, ".$1")
    .replace(/^\./, "");

  return path || undefined;
}

function diagnosticForReference(
  reference: ExpressionReference,
): ExpressionDiagnostic | undefined {
  if (!reference.sensitive) {
    return undefined;
  }

  return {
    severity: "warning",
    code: "secret-reference-blocked",
    message: `${reference.source} is visible as an expression reference only; secret values are never evaluated in Studio state.`,
  };
}

function referencesForBlock(block: string): ExpressionReference[] {
  return [...block.matchAll(expressionReferencePattern)].map((match) => {
    const kind = match[1] as ExpressionReferenceKind;
    return {
      kind,
      source: `$${kind}`,
      path: normalizeReferencePath(match[2] ?? ""),
      sensitive: kind === "credentials" || kind === "env",
    };
  });
}

function expressionDiagnostics(
  expression: string,
  references: ExpressionReference[],
): ExpressionDiagnostic[] {
  const diagnostics = references
    .map(diagnosticForReference)
    .filter((diagnostic): diagnostic is ExpressionDiagnostic => Boolean(diagnostic));

  if (expression.includes("{{") && !expression.includes("}}")) {
    diagnostics.push({
      severity: "error",
      code: "unclosed-expression",
      message: "Expression contains an opening marker without a closing marker.",
    });
  }

  return diagnostics;
}

export function analyzeN8nExpression(value: string): ExpressionAnalysis {
  const { blocks, mode } = expressionBlocks(value);
  const expressionBody = blocks.join("\n");
  const references = blocks.flatMap(referencesForBlock);

  return {
    mode,
    expression: value,
    expressionBody,
    references,
    diagnostics: expressionDiagnostics(value, references),
  };
}

function valueIsExpression(value: unknown): value is string {
  return typeof value === "string" && analyzeN8nExpression(value).mode !== "literal";
}

function expressionPreviewBoundary() {
  return {
    status: "source-only" as const,
    providerBoundary: true as const,
    liveProviderExecution: false as const,
    secretsIncluded: false as const,
    issue:
      "Source-owned expression preview records references and diagnostics; live item evaluation requires governed execution or pinned-data receipts.",
  };
}

function expressionFieldState(field: ParameterField): ExpressionFieldState | undefined {
  if (!field.valuePath || !field.expressionEnabled || !valueIsExpression(field.value)) {
    return undefined;
  }

  const analysis = analyzeN8nExpression(field.value);
  return {
    fieldName: field.name,
    fieldLabel: field.label,
    valuePath: field.valuePath,
    expression: analysis.expression,
    expressionBody: analysis.expressionBody,
    mode: analysis.mode,
    references: analysis.references,
    diagnostics: analysis.diagnostics,
    previewBoundary: expressionPreviewBoundary(),
  };
}

function flattenParameterFields(fields: ParameterField[]): ParameterField[] {
  return fields.flatMap((field) => [
    field,
    ...(field.childFields ? flattenParameterFields(field.childFields) : []),
    ...(field.collectionItems?.flatMap((item) =>
      flattenParameterFields(item.fields),
    ) ?? []),
  ]);
}

export function applyExpressionStateToFields(
  fields: ParameterField[],
  expressionFields: ExpressionFieldState[],
): ParameterField[] {
  const expressionByPath = new Map(
    expressionFields.map((field) => [field.valuePath.join("."), field]),
  );

  return fields.map((field) => ({
    ...field,
    expressionState: field.valuePath
      ? expressionByPath.get(field.valuePath.join("."))
      : undefined,
    childFields: field.childFields
      ? applyExpressionStateToFields(field.childFields, expressionFields)
      : undefined,
    collectionItems: field.collectionItems?.map((item) => ({
      ...item,
      fields: applyExpressionStateToFields(item.fields, expressionFields),
    })),
  }));
}

export function createExpressionEditorState({
  parameters,
  selectedNode,
}: {
  document: WorkflowDocument;
  parameters: ParameterField[];
  selectedNode: WorkflowNode;
}): ExpressionEditorState {
  const fields = flattenParameterFields(parameters)
    .map(expressionFieldState)
    .filter((field): field is ExpressionFieldState => Boolean(field));
  const diagnosticCount = fields.reduce(
    (count, field) => count + field.diagnostics.length,
    0,
  );

  return {
    schema: "dx.n8n-studio.expression-editor",
    selectedNodeId: selectedNode.id,
    selectedNodeName: selectedNode.name,
    nodeType: selectedNode.type,
    expressionFieldCount: fields.length,
    diagnosticCount,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    fields,
    issue:
      fields.length > 0
        ? "Expression references are parsed from source-owned parameter state; live evaluation stays receipt-gated."
        : "No expression-enabled parameter values are active on the selected node.",
  };
}

export function createEmptyExpressionEditorState(
  selectedNodeId: string,
): ExpressionEditorState {
  return {
    schema: "dx.n8n-studio.expression-editor",
    selectedNodeId,
    expressionFieldCount: 0,
    diagnosticCount: 0,
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    fields: [],
    issue: "Selected node is not available for expression analysis.",
  };
}
