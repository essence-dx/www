import {
  readBalancedBlock,
  readConstAssignmentBlock,
  splitTopLevelEntries,
} from "./source-blocks";

type SearchFilterTarget = {
  resource: string;
};

function entryKeyValue(entry: string) {
  const separatorIndex = entry.indexOf(":");
  if (separatorIndex === -1) {
    return undefined;
  }

  const key = entry
    .slice(0, separatorIndex)
    .trim()
    .replace(/^['"`]|['"`]$/g, "");
  const value = entry.slice(separatorIndex + 1).trim();

  return key ? { key, value } : undefined;
}

function stringValue(source: string) {
  return source.trim().match(/^['"`]([\s\S]*?)['"`]/)?.[1];
}

function stringMapFromObjectBlock(block: string) {
  return Object.fromEntries(
    splitTopLevelEntries(block)
      .map(entryKeyValue)
      .filter((entry): entry is { key: string; value: string } => Boolean(entry))
      .map(({ key, value }) => [key, stringValue(value) ?? ""])
      .filter(([, value]) => value),
  ) as Record<string, string>;
}

function literalStringsFromArrayBlock(
  block: string,
  referenceValues: Record<string, string[]> = {},
) {
  return splitTopLevelEntries(block).flatMap((entry) => {
    const referenceName = entry.trim().match(/^\.\.\.typeConditions\.([A-Za-z0-9_]+)/)?.[1];
    if (referenceName) {
      return referenceValues[referenceName] ?? [];
    }

    const value = stringValue(entry);
    return value ? [value] : [];
  });
}

function stringArrayMapFromObjectBlock(
  block: string,
  referenceValues: Record<string, string[]> = {},
) {
  return Object.fromEntries(
    splitTopLevelEntries(block)
      .map(entryKeyValue)
      .filter((entry): entry is { key: string; value: string } => Boolean(entry))
      .map(({ key, value }) => {
        const arrayStart = value.indexOf("[");
        if (arrayStart === -1) {
          return [key, []];
        }

        return [
          key,
          literalStringsFromArrayBlock(
            readBalancedBlock(value, arrayStart),
            referenceValues,
          ),
        ];
      }),
  ) as Record<string, string[]>;
}

function conditionLabel(value: string) {
  return value
    .split("_")
    .filter(Boolean)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ");
}

function optionEntries(values: string[]) {
  return values
    .map(
      (value) => `{
name: '${conditionLabel(value)}',
value: '${value}',
}`,
    )
    .join(",\n");
}

function conditionEntry(type: string, conditions: string[], returnType?: string) {
  const returnTypeRule = returnType ? `,\nreturnType: ['${returnType}']` : "";

  return `{
displayName: 'Condition',
name: 'condition',
type: 'options',
displayOptions: {
show: {
type: ['${type}']${returnTypeRule},
},
},
options: [
${optionEntries(conditions)}
],
default: '',
description: 'The value of the property to filter by',
}`;
}

function returnTypeEntry(returnTypes: string[]) {
  return `{
displayName: 'Return Type',
name: 'returnType',
type: 'options',
displayOptions: {
show: {
type: ['formula'],
},
},
options: [
${optionEntries(returnTypes)}
],
default: '',
description: 'The formula return type',
}`;
}

function conditionEntriesFromGenericFunctions(genericFunctionsSource: string) {
  const types = stringMapFromObjectBlock(
    readConstAssignmentBlock(genericFunctionsSource, "types", "{"),
  );
  const typeConditions = stringArrayMapFromObjectBlock(
    readConstAssignmentBlock(genericFunctionsSource, "typeConditions", "{"),
  );
  const formulaConditions = stringArrayMapFromObjectBlock(
    readConstAssignmentBlock(genericFunctionsSource, "formula", "{"),
    typeConditions,
  );
  const typedConditionEntries = Object.entries(types).flatMap(
    ([type, conditionType]) => {
      const conditions = typeConditions[conditionType] ?? [];
      return conditions.length > 0 ? [conditionEntry(type, conditions)] : [];
    },
  );
  const formulaReturnTypes = Object.keys(formulaConditions);
  const formulaEntries = formulaReturnTypes.flatMap((returnType) => {
    const conditions = formulaConditions[returnType] ?? [];
    return conditions.length > 0
      ? [conditionEntry("formula", conditions, returnType)]
      : [];
  });

  return [
    ...typedConditionEntries,
    returnTypeEntry(formulaReturnTypes),
    ...formulaEntries,
  ];
}

function filterEntries(
  filtersDescriptionSource: string,
  genericFunctionsSource: string,
) {
  const conditions = conditionEntriesFromGenericFunctions(genericFunctionsSource);
  const filtersBlock = readConstAssignmentBlock(
    filtersDescriptionSource,
    "filters",
    "[",
  );

  return splitTopLevelEntries(filtersBlock).flatMap((entry) => {
    const normalized = entry.trim();
    if (normalized === "...conditions") {
      return conditions;
    }

    return normalized.startsWith("{") ? [normalized] : [];
  });
}

function getSearchFiltersArrayBlock(genericFunctionsSource: string) {
  const functionIndex = genericFunctionsSource.search(
    /export\s+function\s+getSearchFilters\b/,
  );
  if (functionIndex === -1) {
    throw new Error("Missing Notion getSearchFilters source");
  }

  const returnIndex = genericFunctionsSource.indexOf("return", functionIndex);
  const arrayStart = genericFunctionsSource.indexOf("[", returnIndex);
  if (returnIndex === -1 || arrayStart === -1) {
    throw new Error("Missing Notion getSearchFilters return array");
  }

  return readBalancedBlock(genericFunctionsSource, arrayStart);
}

function searchFilterTargets(descriptionSources: string[]) {
  const targets = descriptionSources.flatMap((source) =>
    [...source.matchAll(/\.\.\.getSearchFilters\(\s*['"]([^'"]+)['"]\s*\)/g)]
      .map((match) => ({ resource: match[1] })),
  );
  const seen = new Set<string>();

  return targets.filter((target) => {
    if (seen.has(target.resource)) {
      return false;
    }
    seen.add(target.resource);
    return true;
  });
}

function searchFilterEntry(
  entry: string,
  target: SearchFilterTarget,
  filters: string[],
) {
  return entry
    .replace(/\bresource:\s*\[resource\]/g, `resource: ['${target.resource}']`)
    .replace(
      /values:\s*\[\s*\.\.\.filters\(\s*getConditions\(\)\s*\)\s*\]/g,
      `values: [
${filters.join(",\n")}
]`,
    );
}

export function sourceEntriesFromNotionSearchFilterCalls(
  genericFunctionsSource: string,
  filtersDescriptionSource: string,
  descriptionSources: string[],
) {
  const searchFilterEntries = splitTopLevelEntries(
    getSearchFiltersArrayBlock(genericFunctionsSource),
  ).filter((entry) => entry.trim().startsWith("{"));
  const filters = filterEntries(filtersDescriptionSource, genericFunctionsSource);

  return searchFilterTargets(descriptionSources).flatMap((target) =>
    searchFilterEntries.map((entry) =>
      searchFilterEntry(entry, target, filters),
    ),
  );
}
