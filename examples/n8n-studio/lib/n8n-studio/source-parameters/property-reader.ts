import type {
  DynamicOptionsBoundary,
  ParameterDisplayOptions,
  ParameterOption,
  ResourceMapperBoundary,
  ResourceLocatorMode,
} from "../types";
import { readBalancedBlock, splitTopLevelEntries } from "./source-blocks";

function unquote(value: string) {
  return value.replace(/^['"`]|['"`]$/g, "");
}

function escapeRegExp(value: string) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function topLevelEntries(source: string) {
  const trimmed = source.trim();
  return splitTopLevelEntries(trimmed.startsWith("{") ? trimmed : `{${trimmed}}`);
}

function stripLeadingComments(value: string) {
  let current = value.trim();

  while (current.startsWith("//") || current.startsWith("/*")) {
    if (current.startsWith("//")) {
      const commentEnd = current.indexOf("\n");
      if (commentEnd === -1) {
        return current;
      }
      current = current.slice(commentEnd + 1).trim();
    } else {
      const commentEnd = current.indexOf("*/");
      if (commentEnd === -1) {
        return current;
      }
      current = current.slice(commentEnd + 2).trim();
    }
  }

  return current;
}

function topLevelPropertyValue(source: string, propertyName: string) {
  const propertyPattern = new RegExp(
    `^(?:(['"\`])${escapeRegExp(propertyName)}\\1|${escapeRegExp(propertyName)})\\s*:`,
  );
  const entry = topLevelEntries(source)
    .map(stripLeadingComments)
    .find((candidate) =>
      propertyPattern.test(candidate),
  );
  if (!entry) {
    return undefined;
  }

  return entry.slice(entry.indexOf(":") + 1).trim();
}

function literalValue(source: string): unknown {
  const value = source.trim();
  if (value === "true") {
    return true;
  }
  if (value === "false") {
    return false;
  }
  if (value === "undefined") {
    return undefined;
  }
  if (/^-?\d+(\.\d+)?$/.test(value)) {
    return Number(value);
  }
  if (/^['"`]/.test(value)) {
    return unquote(value);
  }
  if (value.startsWith("{")) {
    return {};
  }
  if (value.startsWith("[")) {
    return [];
  }

  return value;
}

function stringProperty(source: string, propertyName: string) {
  const value = topLevelPropertyValue(source, propertyName);
  const match = value?.match(/^(['"`])([\s\S]*?)\1/);
  return match?.[2];
}

function booleanProperty(source: string, propertyName: string) {
  const value = topLevelPropertyValue(source, propertyName);
  const match = value?.match(/^(true|false)/);
  return match ? match[1] === "true" : undefined;
}

function literalDefault(source: string) {
  const value = topLevelPropertyValue(source, "default");
  if (!value) {
    return undefined;
  }

  return literalValue(value);
}

function propertyBlock(source: string, propertyName: string, delimiter: "{" | "[") {
  const value = topLevelPropertyValue(source, propertyName);
  if (!value) {
    return undefined;
  }

  const startIndex = value.indexOf(delimiter);
  if (startIndex === -1) {
    return undefined;
  }

  return readBalancedBlock(value, startIndex);
}

export function readPropertyBlock(source: string, propertyName: string, delimiter: "{" | "[") {
  return propertyBlock(source, propertyName, delimiter);
}

function literalArrayValues(source: string): unknown[] {
  return splitTopLevelEntries(`[${source}]`)
    .map(literalValue)
    .filter((value) => value !== undefined);
}

function readDisplayRule(block: string | undefined): Record<string, unknown[]> | undefined {
  if (!block) {
    return undefined;
  }

  const rule: Record<string, unknown[]> = {};
  const keyPattern = /(?:(['"`])([^'"`]+)\1|([A-Za-z0-9_@/.-]+))\s*:\s*\[([\s\S]*?)\]/g;
  for (const match of block.matchAll(keyPattern)) {
    const key = match[2] ?? match[3];
    const values = literalArrayValues(match[4]).filter(
      (value) => !value || typeof value !== "object",
    );
    if (key && values.length > 0) {
      rule[key] = values;
    }
  }

  return Object.keys(rule).length > 0 ? rule : undefined;
}

export function readDisplayOptions(source: string): ParameterDisplayOptions | undefined {
  const displayOptionsBlock = propertyBlock(source, "displayOptions", "{");
  const showBlock = displayOptionsBlock ? propertyBlock(displayOptionsBlock, "show", "{") : undefined;
  const hideBlock = displayOptionsBlock ? propertyBlock(displayOptionsBlock, "hide", "{") : undefined;
  const show = readDisplayRule(showBlock);
  const hide = readDisplayRule(hideBlock);

  if (!show && !hide) {
    return undefined;
  }

  const displayOptions: ParameterDisplayOptions = {};
  if (show) {
    displayOptions.show = show;
  }
  if (hide) {
    displayOptions.hide = hide;
  }

  return displayOptions;
}

export function readStringArrayProperty(
  source: string,
  propertyName: string,
): string[] | undefined {
  const arrayBlock = propertyBlock(source, propertyName, "[");
  if (!arrayBlock) {
    return undefined;
  }

  const values = splitTopLevelEntries(arrayBlock)
    .map((entry) => literalValue(entry))
    .filter((value): value is string => typeof value === "string" && value.length > 0);

  return values.length > 0 ? values : undefined;
}

export function readOptions(source: string): ParameterOption[] | undefined {
  const optionsBlock = propertyBlock(source, "options", "[");
  if (!optionsBlock) {
    return undefined;
  }

  const options = splitTopLevelEntries(optionsBlock)
    .filter((entry) => entry.startsWith("{"))
    .map((entry) => ({
      name: stringProperty(entry, "name") ?? "",
      value: stringProperty(entry, "value") ?? "",
      action: stringProperty(entry, "action"),
      description: stringProperty(entry, "description"),
    }))
    .filter((option) => option.name && option.value);

  return options.length > 0 ? options : undefined;
}

export function readResourceLocatorModes(source: string): ResourceLocatorMode[] | undefined {
  const modesBlock = propertyBlock(source, "modes", "[");
  if (!modesBlock) {
    return undefined;
  }

  const modes = splitTopLevelEntries(modesBlock)
    .filter((entry) => entry.startsWith("{"))
    .map((entry) => {
      const typeOptionsBlock = propertyBlock(entry, "typeOptions", "{");
      const name = stringProperty(entry, "name") ?? "";
      const sourceType = stringProperty(entry, "type");
      return {
        displayName: stringProperty(entry, "displayName") ?? "",
        name,
        type:
          sourceType === "list" || sourceType === "url" || name === "url"
            ? sourceType === "list"
              ? "list"
              : "url"
            : "string",
        placeholder: stringProperty(entry, "placeholder"),
        searchListMethod: typeOptionsBlock
          ? stringProperty(typeOptionsBlock, "searchListMethod")
          : undefined,
        searchable: typeOptionsBlock
          ? booleanProperty(typeOptionsBlock, "searchable")
          : undefined,
      };
    })
    .filter((mode) => mode.displayName && mode.name);

  return modes.length > 0 ? modes : undefined;
}

function readDynamicOptions(source: string): DynamicOptionsBoundary | undefined {
  const typeOptionsBlock = propertyBlock(source, "typeOptions", "{");
  const loadOptionsBlock = typeOptionsBlock
    ? propertyBlock(typeOptionsBlock, "loadOptions", "{")
    : propertyBlock(source, "loadOptions", "{");
  if (loadOptionsBlock) {
    const routingBlock = propertyBlock(loadOptionsBlock, "routing", "{");
    const requestBlock = propertyBlock(routingBlock ?? loadOptionsBlock, "request", "{");
    if (requestBlock) {
      const method = stringProperty(requestBlock, "method");
      const url = stringProperty(requestBlock, "url");
      if (method && url) {
        const responseFilter = loadOptionsBlock.match(
          /pass:\s*(['"`])([\s\S]*?)\1/,
        )?.[2];

        return {
          source: "n8n-type-options-routing",
          loadMethod: `${method}:${url}`,
          request: {
            method,
            url,
          },
          responseFilter,
          providerBoundary: true,
          liveProviderExecution: false,
          secretsIncluded: false,
          issue:
            "Dynamic option loading requires the DX-owned n8n editor-session adapter before provider calls can run.",
        };
      }
    }
  }

  const loadOptionsMethod = typeOptionsBlock
    ? stringProperty(typeOptionsBlock, "loadOptionsMethod")
    : undefined;
  if (!loadOptionsMethod) {
    return undefined;
  }

  return {
    source: "n8n-type-options-load-method",
    loadMethod: loadOptionsMethod,
    request: {
      method: "LOAD_OPTIONS",
      url: `n8n://load-options/${loadOptionsMethod}`,
    },
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    issue:
      "Dynamic option loading requires the DX-owned n8n editor-session adapter before provider calls can run.",
  };
}

function readResourceMapperBoundary(source: string): ResourceMapperBoundary | undefined {
  const typeOptionsBlock = propertyBlock(source, "typeOptions", "{");
  const resourceMapperBlock = typeOptionsBlock
    ? propertyBlock(typeOptionsBlock, "resourceMapper", "{")
    : undefined;
  if (!resourceMapperBlock) {
    return undefined;
  }

  const resourceMapperMethod = stringProperty(
    resourceMapperBlock,
    "resourceMapperMethod",
  );
  if (!resourceMapperMethod) {
    return undefined;
  }

  const fieldWordsBlock = propertyBlock(resourceMapperBlock, "fieldWords", "{");

  return {
    resourceMapperMethod,
    mode: stringProperty(resourceMapperBlock, "mode"),
    loadOptionsDependsOn:
      readStringArrayProperty(typeOptionsBlock ?? "", "loadOptionsDependsOn") ?? [],
    fieldWords: fieldWordsBlock
      ? {
          singular: stringProperty(fieldWordsBlock, "singular"),
          plural: stringProperty(fieldWordsBlock, "plural"),
        }
      : undefined,
    addAllFields: booleanProperty(resourceMapperBlock, "addAllFields"),
    multiKeyMatch: booleanProperty(resourceMapperBlock, "multiKeyMatch"),
    providerBoundary: true,
    liveProviderExecution: false,
    secretsIncluded: false,
    issue:
      "Resource mapping requires the DX-owned n8n editor-session adapter before provider schema calls can run.",
  };
}

export const sourcePropertyReader = {
  stringProperty,
  booleanProperty,
  literalDefault,
  readPropertyBlock,
  readDisplayOptions,
  readStringArrayProperty,
  readOptions,
  readResourceLocatorModes,
  readDynamicOptions,
  readResourceMapperBoundary,
};
