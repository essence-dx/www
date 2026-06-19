import type {
  ExpressionEditorState,
  ExpressionFieldState,
  ExpressionReference,
} from "../../lib/n8n-studio/types";

function referenceLabel(reference: ExpressionReference) {
  return reference.path ? `${reference.source}.${reference.path}` : reference.source;
}

function ExpressionFieldRow({ field }: { field: ExpressionFieldState }) {
  return (
    <article
      className="n8ns-expression-item"
      data-expression-editor-field={field.fieldName}
      data-expression-mode={field.mode}
    >
      <div className="n8ns-expression-item-header">
        <strong>{field.fieldLabel}</strong>
        <span>{field.valuePath.join(".")}</span>
      </div>
      <code>{field.expression}</code>
      {field.references.length ? (
        <div className="n8ns-expression-reference-list">
          {field.references.map((reference) => (
            <span
              data-expression-reference-kind={reference.kind}
              data-expression-sensitive={String(reference.sensitive)}
              key={`${reference.kind}:${reference.path ?? reference.source}`}
            >
              {referenceLabel(reference)}
            </span>
          ))}
        </div>
      ) : null}
      {field.diagnostics.length ? (
        <div className="n8ns-expression-diagnostics">
          {field.diagnostics.map((diagnostic) => (
            <small
              data-expression-diagnostic={diagnostic.code}
              key={`${diagnostic.code}:${diagnostic.message}`}
            >
              {diagnostic.message}
            </small>
          ))}
        </div>
      ) : (
        <small>{field.previewBoundary.issue}</small>
      )}
    </article>
  );
}

export function ExpressionEditorPanel({
  expressionEditor,
}: {
  expressionEditor: ExpressionEditorState;
}) {
  return (
    <section
      className="n8ns-expression-box"
      data-expression-field-count={expressionEditor.expressionFieldCount}
      data-expression-secret-values={String(expressionEditor.secretsIncluded)}
      data-live-provider-execution={String(expressionEditor.liveProviderExecution)}
      data-studio-surface="expression-editor"
    >
      <div className="n8ns-expression-heading">
        <span>Expression</span>
        <strong>{expressionEditor.expressionFieldCount} active</strong>
      </div>
      {expressionEditor.fields.length ? (
        <div className="n8ns-expression-list">
          {expressionEditor.fields.map((field) => (
            <ExpressionFieldRow field={field} key={field.valuePath.join(".")} />
          ))}
        </div>
      ) : (
        <p className="n8ns-muted">{expressionEditor.issue}</p>
      )}
    </section>
  );
}
