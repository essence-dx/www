import type {
  ExpressionEditorState,
  ParameterField,
  ParameterValuePath,
  WorkflowNode,
} from "../../lib/n8n-studio/types";
import { ExpressionEditorPanel } from "./expression-editor-panel";
import { ParameterFieldControl } from "./parameter-field-control";

export type ParameterInspectorProps = {
  expressionEditor: ExpressionEditorState;
  node: WorkflowNode;
  onAddCollectionItem?: (collectionPath: ParameterValuePath) => void;
  onRemoveCollectionItem?: (
    collectionPath: ParameterValuePath,
    itemIndex: number,
  ) => void;
  onUpdateParameterValue?: (valuePath: ParameterValuePath, value: unknown) => void;
  parameters: ParameterField[];
};

export function ParameterInspector({
  expressionEditor,
  node,
  onAddCollectionItem,
  onRemoveCollectionItem,
  onUpdateParameterValue,
  parameters,
}: ParameterInspectorProps) {
  return (
    <section className="n8ns-panel" data-studio-surface="node-parameters">
      <div className="n8ns-panel-header">
        <div>
          <p className="n8ns-eyebrow">Selected node</p>
          <h2>{node.name}</h2>
        </div>
        <span className="n8ns-badge">Parameters</span>
      </div>
      <div className="n8ns-field-stack">
        {parameters.map((field) => (
          <ParameterFieldControl
            expressionEditor={expressionEditor}
            field={field}
            key={field.name}
            onAddCollectionItem={onAddCollectionItem}
            onRemoveCollectionItem={onRemoveCollectionItem}
            onUpdateParameterValue={onUpdateParameterValue}
          />
        ))}
      </div>
      <ExpressionEditorPanel expressionEditor={expressionEditor} />
    </section>
  );
}
