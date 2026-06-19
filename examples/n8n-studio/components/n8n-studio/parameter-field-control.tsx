"use client";

import { coerceParameterInputValue } from "../../lib/n8n-studio/parameter-value-coercion";
import type {
  ExpressionEditorState,
  ExpressionFieldState,
  ParameterField,
  ParameterValuePath,
} from "../../lib/n8n-studio/types";

export type ParameterFieldControlProps = {
  expressionEditor?: ExpressionEditorState;
  field: ParameterField;
  depth?: number;
  onAddCollectionItem?: (collectionPath: ParameterValuePath) => void;
  onRemoveCollectionItem?: (
    collectionPath: ParameterValuePath,
    itemIndex: number,
  ) => void;
  onUpdateParameterValue?: (valuePath: ParameterValuePath, value: unknown) => void;
};

function formatParameterValue(value: unknown) {
  if (Array.isArray(value)) {
    return value.join(", ");
  }
  if (value && typeof value === "object") {
    return JSON.stringify(value);
  }
  return String(value ?? "");
}

function formatFieldMetadata(field: ParameterField) {
  return [
    field.options?.map((option) => option.name).join(", "),
    field.credentialTypes?.join(", "),
    field.resourceLocatorModes?.map((mode) => mode.displayName).join(", "),
  ].filter((value): value is string => Boolean(value));
}

function renderChildTemplate(
  field: ParameterField,
  props: ParameterFieldControlProps,
) {
  if (!field.childFields?.length || field.collectionItems?.length) {
    return null;
  }

  return (
    <div className="n8ns-child-field-grid" data-collection-field={field.name}>
      {field.childFields.map((childField) => (
        <ParameterFieldControl
          depth={(props.depth ?? 0) + 1}
          expressionEditor={props.expressionEditor}
          field={childField}
          key={childField.name}
          onAddCollectionItem={props.onAddCollectionItem}
          onRemoveCollectionItem={props.onRemoveCollectionItem}
          onUpdateParameterValue={props.onUpdateParameterValue}
        />
      ))}
    </div>
  );
}

function renderCollectionItems(
  field: ParameterField,
  props: ParameterFieldControlProps,
) {
  if (!field.collectionItems?.length && !props.onAddCollectionItem) {
    return null;
  }

  return (
    <div className="n8ns-collection-field" data-collection-field={field.name}>
      {props.onAddCollectionItem && field.valuePath ? (
        <button
          data-collection-action="add-item"
          onClick={() => props.onAddCollectionItem?.(field.valuePath ?? [])}
          type="button"
        >
          Add item
        </button>
      ) : null}
      {field.collectionItems?.map((item) => (
        <fieldset
          className="n8ns-collection-item"
          data-collection-item={item.key}
          key={item.key}
        >
          <legend>{item.label}</legend>
          {item.fields.map((childField) => (
            <ParameterFieldControl
              depth={(props.depth ?? 0) + 1}
              expressionEditor={props.expressionEditor}
              field={childField}
              key={childField.name}
              onAddCollectionItem={props.onAddCollectionItem}
              onRemoveCollectionItem={props.onRemoveCollectionItem}
              onUpdateParameterValue={props.onUpdateParameterValue}
            />
          ))}
          {props.onRemoveCollectionItem ? (
            <button
              data-collection-action="remove-item"
              onClick={() =>
                props.onRemoveCollectionItem?.(item.collectionPath, item.itemIndex)
              }
              type="button"
            >
              Remove item
            </button>
          ) : null}
        </fieldset>
      ))}
    </div>
  );
}

function resourceLocatorValue(mode: string, value: string) {
  return { mode, value };
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function renderResourceLocatorControl(
  field: ParameterField,
  canUpdateValue: boolean,
  handleInputChange: (value: unknown) => void,
) {
  const draft = field.resourceLocatorDraft;
  if (!draft || !field.resourceLocatorModes?.length) {
    return null;
  }

  const activeMode = field.resourceLocatorModes.find(
    (mode) => mode.name === draft.activeMode,
  );

  return (
    <div className="n8ns-resource-locator" data-resource-locator-field={field.name}>
      <select
        aria-label={`${field.label} locator mode`}
        data-resource-locator-mode={field.name}
        disabled={!canUpdateValue}
        onChange={(event) => handleInputChange(resourceLocatorValue(event.target.value, ""))}
        value={draft.activeMode}
      >
        {field.resourceLocatorModes.map((mode) => (
          <option key={mode.name} value={mode.name}>
            {mode.displayName}
          </option>
        ))}
      </select>
      <input
        aria-label={`${field.label} locator value`}
        data-resource-locator-selected-value={field.name}
        onChange={
          canUpdateValue
            ? (event) =>
                handleInputChange(
                  resourceLocatorValue(draft.activeMode, event.target.value),
                )
            : undefined
        }
        placeholder={activeMode?.placeholder}
        readOnly={!canUpdateValue}
        value={draft.selectedValue}
      />
      {draft.listSearchMethod ? (
        <div
          className="n8ns-resource-locator-boundary"
          data-live-provider-execution={String(
            draft.listBoundary.liveProviderExecution,
          )}
          data-resource-locator-search-boundary={draft.listSearchMethod}
        >
          <input
            aria-label={`${field.label} resource search`}
            readOnly
            value={draft.query}
          />
          <small>{draft.listBoundary.issue}</small>
          {draft.resolvedOptions?.length ? (
            <div
              className="n8ns-field-meta"
              data-resource-locator-resolved-options={field.name}
            >
              {draft.resolvedOptions.map((option) => (
                <small key={`${option.value}-${option.name}`}>{option.name}</small>
              ))}
            </div>
          ) : null}
        </div>
      ) : null}
    </div>
  );
}

function optionValues(value: unknown) {
  return Array.isArray(value)
    ? value.filter((item): item is string => typeof item === "string")
    : [];
}

function renderOptionsControl(
  field: ParameterField,
  canUpdateValue: boolean,
  handleInputChange: (value: unknown) => void,
) {
  if (!field.options?.length) {
    return null;
  }

  if (field.type === "multiOptions") {
    return (
      <select
        aria-label={field.label}
        data-parameter-options-field={field.name}
        disabled={!canUpdateValue}
        multiple
        onChange={
          canUpdateValue
            ? (event) =>
                handleInputChange(
                  Array.from(event.target.selectedOptions).map(
                    (option) => option.value,
                  ),
                )
            : undefined
        }
        value={optionValues(field.value)}
      >
        {field.options.map((option) => (
          <option key={`${option.value}-${option.name}`} value={option.value}>
            {option.name}
          </option>
        ))}
      </select>
    );
  }

  return (
    <select
      aria-label={field.label}
      data-parameter-options-field={field.name}
      disabled={!canUpdateValue}
      onChange={
        canUpdateValue
          ? (event) => handleInputChange(event.target.value)
          : undefined
      }
      value={formatParameterValue(field.value)}
    >
      {field.options.map((option) => (
        <option key={`${option.value}-${option.name}`} value={option.value}>
          {option.name}
        </option>
      ))}
    </select>
  );
}

function renderNoticeControl(field: ParameterField) {
  if (field.type !== "notice") {
    return null;
  }

  return (
    <output
      aria-label={field.label}
      className="n8ns-parameter-notice"
      data-parameter-notice-field={field.name}
    >
      {formatParameterValue(field.value)}
    </output>
  );
}

function resourceMapperValue(value: unknown) {
  if (!isRecord(value)) {
    return {
      mappingMode: "defineBelow",
      value: null,
    };
  }

  return {
    mappingMode:
      typeof value.mappingMode === "string" ? value.mappingMode : "defineBelow",
    value: value.value ?? null,
  };
}

function renderResourceMapperControl(
  field: ParameterField,
  canUpdateValue: boolean,
  handleInputChange: (value: unknown) => void,
) {
  if (field.type !== "resourceMapper") {
    return null;
  }

  const mapperValue = resourceMapperValue(field.value);

  return (
    <div className="n8ns-resource-mapper" data-resource-mapper-field={field.name}>
      <select
        aria-label={`${field.label} mapping mode`}
        data-resource-mapper-mode={field.name}
        disabled={!canUpdateValue}
        onChange={(event) =>
          handleInputChange({
            ...mapperValue,
            mappingMode: event.target.value,
          })
        }
        value={mapperValue.mappingMode}
      >
        <option value="defineBelow">Map Each Column Below</option>
        <option value="autoMapInputData">Auto-Map Input Data to Columns</option>
      </select>
      <textarea
        aria-label={`${field.label} mapped value`}
        data-resource-mapper-value={field.name}
        onChange={
          canUpdateValue
            ? (event) =>
                handleInputChange({
                  ...mapperValue,
                  value: event.target.value || null,
                })
            : undefined
        }
        readOnly={!canUpdateValue}
        rows={3}
        value={formatParameterValue(mapperValue.value)}
      />
    </div>
  );
}

function renderResourceMapperBoundary(field: ParameterField) {
  const mapper = field.resourceMapper;
  if (field.type !== "resourceMapper" || !mapper) {
    return null;
  }

  return (
    <div
      className="n8ns-resource-mapper-boundary"
      data-live-provider-execution={String(mapper.liveProviderExecution)}
      data-resource-mapper-field={field.name}
      data-resource-mapper-method={mapper.resourceMapperMethod}
    >
      <small>{mapper.fieldWords?.plural ?? "fields"}</small>
      <code>{mapper.resourceMapperMethod}</code>
      {mapper.loadOptionsDependsOn.length ? (
        <small>{mapper.loadOptionsDependsOn.join(", ")}</small>
      ) : null}
      <small>{mapper.issue}</small>
    </div>
  );
}

function valuePathKey(valuePath: ParameterValuePath | undefined) {
  return valuePath?.join(".");
}

function expressionStateForField(
  field: ParameterField,
  expressionEditor: ExpressionEditorState | undefined,
) {
  const fieldPath = valuePathKey(field.valuePath);
  if (!fieldPath) {
    return field.expressionState;
  }

  return (
    field.expressionState ??
    expressionEditor?.fields.find((expressionField) => {
      return valuePathKey(expressionField.valuePath) === fieldPath;
    })
  );
}

function renderExpressionState(expressionState: ExpressionFieldState | undefined) {
  if (!expressionState) {
    return null;
  }

  return (
    <div
      className="n8ns-field-expression"
      data-expression-field={expressionState.fieldName}
      data-expression-mode={expressionState.mode}
      data-live-provider-execution={String(
        expressionState.previewBoundary.liveProviderExecution,
      )}
    >
      <code>{expressionState.expression}</code>
      {expressionState.references.length ? (
        <small>
          {expressionState.references
            .map((reference) =>
              reference.path ? `${reference.source}.${reference.path}` : reference.source,
            )
            .join(", ")}
        </small>
      ) : (
        <small>{expressionState.previewBoundary.issue}</small>
      )}
    </div>
  );
}

export function ParameterFieldControl({
  expressionEditor,
  field,
  depth = 0,
  onAddCollectionItem,
  onRemoveCollectionItem,
  onUpdateParameterValue,
}: ParameterFieldControlProps) {
  const metadata = formatFieldMetadata(field);
  const canUpdateValue = Boolean(onUpdateParameterValue && field.valuePath);
  const expressionState = expressionStateForField(field, expressionEditor);
  const handleInputChange = (value: unknown) => {
    onUpdateParameterValue?.(
      field.valuePath ?? [],
      coerceParameterInputValue(field, value),
    );
  };

  return (
    <div
      className="n8ns-field"
      data-expression-field={expressionState?.fieldName}
      data-field-depth={depth}
      data-field-type={field.type}
    >
      <label className="n8ns-field-input">
        <span>{field.label}</span>
        {field.type === "notice" ? (
          renderNoticeControl(field)
        ) : field.type === "boolean" ? (
          <input
            aria-label={field.label}
            checked={field.value === true}
            onChange={
              canUpdateValue ? (event) => handleInputChange(event.target.checked) : undefined
            }
            readOnly={!canUpdateValue}
            type="checkbox"
          />
        ) : field.type === "resourceLocator" ? (
          renderResourceLocatorControl(field, canUpdateValue, handleInputChange)
        ) : field.type === "resourceMapper" ? (
          renderResourceMapperControl(field, canUpdateValue, handleInputChange)
        ) : (field.type === "options" || field.type === "multiOptions") &&
          field.options?.length ? (
          renderOptionsControl(field, canUpdateValue, handleInputChange)
        ) : (
          <input
            aria-label={field.label}
            inputMode={field.type === "number" ? "decimal" : undefined}
            onChange={
              canUpdateValue ? (event) => handleInputChange(event.target.value) : undefined
            }
            readOnly={!canUpdateValue}
            type={field.type === "number" ? "number" : "text"}
            value={formatParameterValue(field.value)}
          />
        )}
      </label>
      {field.description ? (
        <small className="n8ns-field-description">{field.description}</small>
      ) : null}
      {metadata.length ? (
        <div className="n8ns-field-meta">
          {metadata.map((value) => (
            <small key={value}>{value}</small>
          ))}
        </div>
      ) : null}
      {renderResourceMapperBoundary(field)}
      {field.renderingBoundary === "complex-source-field" ? (
        <small className="n8ns-field-boundary">Complex source field</small>
      ) : null}
      {renderExpressionState(expressionState)}
      {renderCollectionItems(field, {
        depth,
        expressionEditor,
        field,
        onAddCollectionItem,
        onRemoveCollectionItem,
        onUpdateParameterValue,
      })}
      {renderChildTemplate(field, {
        depth,
        expressionEditor,
        field,
        onAddCollectionItem,
        onRemoveCollectionItem,
        onUpdateParameterValue,
      })}
    </div>
  );
}
