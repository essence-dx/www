import type {
  EditorSessionRequestPlan,
  ParameterField,
  ParameterValuePath,
} from "./types";

function valuePathKey(valuePath: ParameterValuePath | undefined) {
  return valuePath?.map(String).join(".");
}

function fieldPath(field: ParameterField) {
  return valuePathKey(field.valuePath) ?? field.name;
}

function planCanOverlayOptions(plan: EditorSessionRequestPlan) {
  return (
    plan.status === "configured" &&
    plan.kind === "dynamic-node-parameters" &&
    Boolean(plan.resolvedOptions?.length)
  );
}

function planCanOverlayResourceLocator(plan: EditorSessionRequestPlan) {
  return (
    plan.status === "configured" &&
    plan.kind === "resource-locator-search" &&
    Boolean(plan.resolvedOptions?.length)
  );
}

function matchesDynamicBoundary(
  nodeType: string | undefined,
  field: ParameterField,
  plan: EditorSessionRequestPlan,
) {
  if (!nodeType || !plan.dynamicLoadBoundary || !plan.loadMethod) {
    return false;
  }

  return plan.dynamicLoadBoundary === `${nodeType}.${fieldPath(field)}.${plan.loadMethod}`;
}

function matchesFieldAndMethod(field: ParameterField, plan: EditorSessionRequestPlan) {
  if (plan.fieldName !== field.name) {
    return false;
  }

  if (!plan.loadMethod) {
    return true;
  }

  return (
    field.dynamicOptions?.loadMethod === plan.loadMethod ||
    field.resourceLocatorDraft?.listSearchMethod === plan.loadMethod ||
    field.resourceLocatorModes?.some(
      (mode) => mode.searchListMethod === plan.loadMethod,
    ) === true
  );
}

function matchingDynamicOptionPlan(
  field: ParameterField,
  plans: EditorSessionRequestPlan[],
  nodeType: string | undefined,
) {
  return plans.find(
    (plan) =>
      planCanOverlayOptions(plan) &&
      (matchesDynamicBoundary(nodeType, field, plan) ||
        matchesFieldAndMethod(field, plan)),
  );
}

function matchingResourceLocatorPlan(
  field: ParameterField,
  plans: EditorSessionRequestPlan[],
) {
  return plans.find(
    (plan) =>
      planCanOverlayResourceLocator(plan) && matchesFieldAndMethod(field, plan),
  );
}

function overlayField(
  field: ParameterField,
  plans: EditorSessionRequestPlan[],
  nodeType: string | undefined,
): ParameterField {
  const dynamicOptionPlan = matchingDynamicOptionPlan(field, plans, nodeType);
  const resourceLocatorPlan = matchingResourceLocatorPlan(field, plans);
  const childFields = field.childFields?.map((childField) =>
    overlayField(childField, plans, nodeType),
  );
  const collectionItems = field.collectionItems?.map((item) => ({
    ...item,
    fields: item.fields.map((childField) =>
      overlayField(childField, plans, nodeType),
    ),
  }));

  return {
    ...field,
    ...(dynamicOptionPlan?.resolvedOptions
      ? { options: dynamicOptionPlan.resolvedOptions }
      : {}),
    ...(resourceLocatorPlan?.resolvedOptions && field.resourceLocatorDraft
      ? {
          resourceLocatorDraft: {
            ...field.resourceLocatorDraft,
            resolvedOptions: resourceLocatorPlan.resolvedOptions,
            resolvedQuery:
              resourceLocatorPlan.resolvedQuery ??
              field.resourceLocatorDraft.query,
            nextPageToken: resourceLocatorPlan.nextPageToken,
          },
        }
      : {}),
    ...(childFields ? { childFields } : {}),
    ...(collectionItems ? { collectionItems } : {}),
  };
}

export function applyConfiguredEditorSessionPlansToParameters(
  fields: ParameterField[],
  plans: EditorSessionRequestPlan[],
  nodeType?: string,
): ParameterField[] {
  return fields.map((field) => overlayField(field, plans, nodeType));
}
