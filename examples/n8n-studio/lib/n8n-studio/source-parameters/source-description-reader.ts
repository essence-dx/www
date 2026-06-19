import type { NodeParameterDefinition } from "../node-types/types";
import { createParameterDefinitionFromSource } from "./parameter-definition";
import { sourcePropertyReader } from "./property-reader";
import {
  readBalancedBlock,
  readConstAssignmentBlock,
  splitTopLevelEntries,
} from "./source-blocks";

export type SharedSourceReferenceOptions = {
  sharedSource?: string;
  sharedObjectNames?: string[];
  sharedArrayPropertyNames?: string[];
  sharedArraySpreadNames?: string[];
  localObjectSource?: string;
  localObjectNames?: string[];
};

export type SourceArrayEntriesOptions = SharedSourceReferenceOptions & {
  source: string;
  arrayName: string;
  localArraySpreadNames?: string[];
};

export type VersionDescriptionParameterOptions = {
  leadingConstObjectNames?: string[];
};

function withoutBlockComments(entry: string) {
  return entry.replace(/\/\*[\s\S]*?\*\//g, "").trim();
}

function normalizedObjectEntries(block: string) {
  return splitTopLevelEntries(block)
    .map(withoutBlockComments)
    .filter((entry) => entry.startsWith("{"));
}

function objectContent(block: string | undefined) {
  return block?.trim().replace(/^\{\s*/, "").replace(/\s*\}$/, "").trim() ?? "";
}

function localSpreadNames(entry: string) {
  return splitTopLevelEntries(entry)
    .map((part) => part.trim())
    .filter((part) => part.startsWith("..."))
    .map((part) => part.match(/^\.\.\.([A-Za-z0-9_]+)/)?.[1])
    .filter((name): name is string => Boolean(name));
}

function identifierEntry(entry: string) {
  return entry.trim().match(/^([A-Za-z0-9_]+)$/)?.[1];
}

function topLevelPropertyPattern(propertyName: string) {
  return new RegExp(`^(?:(['"\`])${propertyName}\\1|${propertyName})\\s*:`);
}

function entryWithoutTopLevelProperties(
  entry: string,
  propertyNames: string[] = [],
) {
  return splitTopLevelEntries(entry)
    .map((part) => part.trim())
    .filter((part) => !part.startsWith("..."))
    .filter(
      (part) =>
        !propertyNames.some((propertyName) =>
          topLevelPropertyPattern(propertyName).test(part),
        ),
    )
    .join(",\n");
}

function readSharedObject(name: string, sharedSource: string | undefined) {
  return sharedSource
    ? readConstAssignmentBlock(sharedSource, name, "{")
    : undefined;
}

function readLocalObject(
  name: string,
  localObjectSource: string | undefined,
) {
  return localObjectSource
    ? readConstAssignmentBlock(localObjectSource, name, "{")
    : undefined;
}

function readReferencedObject(
  name: string,
  options: SharedSourceReferenceOptions,
) {
  if (options.localObjectNames?.includes(name)) {
    return readLocalObject(name, options.localObjectSource);
  }

  if (options.sharedObjectNames?.includes(name)) {
    return readSharedObject(name, options.sharedSource);
  }

  return undefined;
}

function readSharedArray(name: string, sharedSource: string | undefined) {
  return sharedSource
    ? readConstAssignmentBlock(sharedSource, name, "[")
    : undefined;
}

function expandObjectSpreads(
  entry: string,
  options: SharedSourceReferenceOptions,
) {
  const spreadBlocks = localSpreadNames(entry)
    .map((name) => readReferencedObject(name, options))
    .filter((block): block is string => Boolean(block));
  if (spreadBlocks.length === 0) {
    return entry;
  }

  const sourceParts = [
    entryWithoutTopLevelProperties(entry),
    ...spreadBlocks.map(objectContent),
  ].filter(Boolean);

  return expandSharedArrayPropertyReferences(
    `{${sourceParts.join(",\n")}}`,
    options,
  );
}

function expandObjectSpreadBlock(
  block: string,
  options: SharedSourceReferenceOptions,
) {
  const expandedNestedBlock = `{${
    expandNestedObjectSpreads(block.slice(1, -1), options)
  }}`;

  return expandObjectSpreads(expandedNestedBlock, options);
}

function expandNestedObjectSpreads(
  source: string,
  options: SharedSourceReferenceOptions,
) {
  let expanded = "";
  let index = 0;
  let quote: string | undefined;
  let escaped = false;

  while (index < source.length) {
    const character = source[index];

    if (quote) {
      expanded += character;
      escaped = character === "\\" && !escaped;
      if (character === quote && !escaped) {
        quote = undefined;
      } else if (character !== "\\") {
        escaped = false;
      }
      index += 1;
      continue;
    }

    if (character === "'" || character === '"' || character === "`") {
      quote = character;
      expanded += character;
      index += 1;
      continue;
    }

    if (character === "{") {
      const block = readBalancedBlock(source, index);
      expanded += expandObjectSpreadBlock(block, options);
      index += block.length;
      continue;
    }

    expanded += character;
    index += 1;
  }

  return expanded;
}

function expandSharedObjectIdentifiers(
  source: string,
  options: SharedSourceReferenceOptions,
) {
  return (options.sharedObjectNames ?? []).reduce((expandedSource, name) => {
    const pattern = `([\\[,])\\s*${name}\\s*(?=,|\\])`;
    if (!new RegExp(pattern).test(expandedSource)) {
      return expandedSource;
    }

    const parameterBlock = readSharedObject(name, options.sharedSource);
    if (!parameterBlock) {
      return expandedSource;
    }

    return expandedSource.replace(
      new RegExp(pattern, "g"),
      (_match, prefix: string) => `${prefix}\n${parameterBlock}`,
    );
  }, source);
}

function expandSharedArrayPropertyReferences(
  source: string,
  options: SharedSourceReferenceOptions,
) {
  return (options.sharedArrayPropertyNames ?? []).reduce((expandedSource, name) => {
    const pattern = `(:\\s*)${name}\\b`;
    if (!new RegExp(pattern).test(expandedSource)) {
      return expandedSource;
    }

    const parameterBlock = readSharedArray(name, options.sharedSource);
    if (!parameterBlock) {
      return expandedSource;
    }

    return expandedSource.replace(
      new RegExp(pattern, "g"),
      (_match, prefix: string) => `${prefix}${parameterBlock}`,
    );
  }, source);
}

function expandSharedReferences(
  source: string,
  options: SharedSourceReferenceOptions,
) {
  return expandNestedObjectSpreads(
    expandSharedArrayPropertyReferences(
      expandSharedObjectIdentifiers(source, options),
      options,
    ),
    options,
  );
}

function sharedArraySpreadEntries(
  spreadName: string,
  options: SharedSourceReferenceOptions,
) {
  if (!options.sharedArraySpreadNames?.includes(spreadName)) {
    return [];
  }

  const sharedArrayBlock = readSharedArray(spreadName, options.sharedSource);
  return sharedArrayBlock ? normalizedObjectEntries(sharedArrayBlock) : [];
}

function localArraySpreadEntries(
  spreadName: string,
  options: SourceArrayEntriesOptions,
  seen: Set<string>,
) {
  if (!options.localArraySpreadNames?.includes(spreadName)) {
    return undefined;
  }

  return sourceEntriesForArrayWithSeen(
    {
      ...options,
      arrayName: spreadName,
    },
    seen,
  );
}

function sourceEntriesForArrayWithSeen(
  options: SourceArrayEntriesOptions,
  seen: Set<string>,
) {
  if (seen.has(options.arrayName)) {
    return [];
  }
  seen.add(options.arrayName);

  const block = readConstAssignmentBlock(options.source, options.arrayName, "[");
  const referenceOptions = {
    ...options,
    localObjectSource: options.localObjectSource ?? options.source,
  };

  return splitTopLevelEntries(block).flatMap((entry) => {
    const normalizedEntry = withoutBlockComments(entry);
    const spreadName = normalizedEntry.match(/^\.\.\.([A-Za-z0-9_]+)/)?.[1];
    if (spreadName) {
      const localEntries = localArraySpreadEntries(spreadName, options, seen);
      if (localEntries) {
        return localEntries;
      }

      return sharedArraySpreadEntries(spreadName, referenceOptions).map((sharedEntry) =>
        expandSharedReferences(sharedEntry, referenceOptions),
      );
    }

    const identifier = identifierEntry(normalizedEntry);
    if (
      identifier &&
      (referenceOptions.sharedObjectNames?.includes(identifier) ||
        referenceOptions.localObjectNames?.includes(identifier))
    ) {
      const referencedObject = readReferencedObject(identifier, referenceOptions);
      return referencedObject
        ? [expandSharedReferences(referencedObject, referenceOptions)]
        : [];
    }

    const expandedEntry = expandSharedReferences(normalizedEntry, referenceOptions);
    return expandedEntry.startsWith("{") ? [expandedEntry] : [];
  });
}

export function sourceEntriesForArray(options: SourceArrayEntriesOptions) {
  return sourceEntriesForArrayWithSeen(options, new Set<string>());
}

function mergedDisplayOptionsBlock(
  entry: string,
  displayOptionsBlock: string,
) {
  const entryDisplayOptionsBlock = sourcePropertyReader.readPropertyBlock(
    entry,
    "displayOptions",
    "{",
  );
  const fieldShow = entryDisplayOptionsBlock
    ? sourcePropertyReader.readPropertyBlock(entryDisplayOptionsBlock, "show", "{")
    : undefined;
  const fieldHide = entryDisplayOptionsBlock
    ? sourcePropertyReader.readPropertyBlock(entryDisplayOptionsBlock, "hide", "{")
    : undefined;
  const operationShow = sourcePropertyReader.readPropertyBlock(
    displayOptionsBlock,
    "show",
    "{",
  );
  const operationHide = sourcePropertyReader.readPropertyBlock(
    displayOptionsBlock,
    "hide",
    "{",
  );
  const showContent = [objectContent(fieldShow), objectContent(operationShow)]
    .filter(Boolean)
    .join(",\n");
  const hideContent = [objectContent(fieldHide), objectContent(operationHide)]
    .filter(Boolean)
    .join(",\n");
  const sections = [
    showContent ? `show: {${showContent}}` : "",
    hideContent ? `hide: {${hideContent}}` : "",
  ].filter(Boolean);

  return `{${sections.join(",\n")}}`;
}

export function applyN8nDisplayOptions(
  entry: string,
  displayOptionsBlock: string,
) {
  return `{
displayOptions: ${mergedDisplayOptionsBlock(entry, displayOptionsBlock)},
${entryWithoutTopLevelProperties(entry, ["displayOptions"])}
}`;
}

export function sourceParametersFromEntries(entries: string[]) {
  return entries
    .map(createParameterDefinitionFromSource)
    .filter((parameter): parameter is NodeParameterDefinition => Boolean(parameter));
}

export function sourceParametersFromVersionDescription(
  source: string,
  options: VersionDescriptionParameterOptions = {},
) {
  const versionBlock = readConstAssignmentBlock(source, "versionDescription", "{");
  const propertiesBlock = sourcePropertyReader.readPropertyBlock(
    versionBlock,
    "properties",
    "[",
  );
  const leadingParameters = sourceParametersFromEntries(
    (options.leadingConstObjectNames ?? []).map((name) =>
      readConstAssignmentBlock(source, name, "{"),
    ),
  );
  const versionProperties = propertiesBlock
    ? sourceParametersFromEntries(normalizedObjectEntries(propertiesBlock))
    : [];

  return [...leadingParameters, ...versionProperties];
}

export function credentialsFromVersionDescription(source: string) {
  const versionBlock = readConstAssignmentBlock(source, "versionDescription", "{");
  const credentialsBlock = sourcePropertyReader.readPropertyBlock(
    versionBlock,
    "credentials",
    "[",
  );
  if (!credentialsBlock) {
    return [];
  }

  return normalizedObjectEntries(credentialsBlock)
    .map((entry) => ({
      name: sourcePropertyReader.stringProperty(entry, "name") ?? "",
      required: sourcePropertyReader.booleanProperty(entry, "required") ?? false,
    }))
    .filter((credential) => credential.name);
}
