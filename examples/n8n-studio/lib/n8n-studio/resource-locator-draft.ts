import type { ResourceLocatorDraftState, ResourceLocatorMode } from "./types";

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function readStringProperty(value: unknown, propertyName: string) {
  if (!isRecord(value)) {
    return undefined;
  }

  const propertyValue = value[propertyName];
  if (typeof propertyValue === "string") {
    return propertyValue;
  }
  if (typeof propertyValue === "number") {
    return String(propertyValue);
  }
  return undefined;
}

function readSelectedValue(value: unknown) {
  if (typeof value === "string") {
    return value;
  }
  return readStringProperty(value, "value") ?? "";
}

function activeModeName(value: unknown, modes: ResourceLocatorMode[]) {
  const mode = readStringProperty(value, "mode");
  if (mode && modes.some((candidate) => candidate.name === mode)) {
    return mode;
  }
  return modes[0]?.name ?? "id";
}

export function createResourceLocatorDraft(
  fieldName: string,
  value: unknown,
  modes: ResourceLocatorMode[] | undefined,
): ResourceLocatorDraftState | undefined {
  if (!modes?.length) {
    return undefined;
  }

  const activeMode = activeModeName(value, modes);
  const mode = modes.find((candidate) => candidate.name === activeMode);
  const selectedValue = readSelectedValue(value);
  const selectedLabel = readStringProperty(value, "cachedResultName");

  return {
    activeMode,
    query: selectedLabel ?? selectedValue,
    selectedValue,
    selectedLabel,
    listSearchMethod: mode?.searchListMethod,
    searchable: Boolean(mode?.searchable && mode.searchListMethod),
    listBoundary: {
      status: "source-only",
      providerBoundary: true,
      liveProviderExecution: false,
      issue: `Dynamic ${fieldName} list/search requires the n8n editor-session adapter.`,
    },
  };
}
