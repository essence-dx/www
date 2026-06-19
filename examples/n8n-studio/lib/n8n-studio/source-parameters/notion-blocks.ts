import { readBalancedBlock, splitTopLevelEntries } from "./source-blocks";

type BlockCallTarget = {
  resource: string;
  operation: string;
};

const nestedArrayNames = [
  "colors",
  "annotation",
  "typeMention",
  "typeEquation",
  "typeText",
];

function constAssignmentIndex(source: string, name: string) {
  return source.search(new RegExp(`(?:export\\s+)?const\\s+${name}\\b`));
}

function readConstArrayBlock(source: string, name: string) {
  const assignment = constAssignmentIndex(source, name);
  if (assignment === -1) {
    throw new Error(`Missing Notion block array source: ${name}`);
  }

  const equalsIndex = source.indexOf("=", assignment);
  const startIndex = source.indexOf("[", equalsIndex);
  if (startIndex === -1) {
    throw new Error(`Missing array block for Notion block source: ${name}`);
  }

  return readBalancedBlock(source, startIndex);
}

function readArrowArrayBlock(source: string, name: string) {
  const assignment = constAssignmentIndex(source, name);
  if (assignment === -1) {
    throw new Error(`Missing Notion block function source: ${name}`);
  }

  const arrowIndex = source.indexOf("=>", assignment);
  const startIndex = source.indexOf("[", arrowIndex);
  if (arrowIndex === -1 || startIndex === -1) {
    throw new Error(`Missing arrow array for Notion block source: ${name}`);
  }

  return readBalancedBlock(source, startIndex);
}

function arrayContent(block: string) {
  return block.trim().replace(/^\[\s*/, "").replace(/\s*\]$/, "").trim();
}

function entriesFromArrowArray(source: string, name: string) {
  return splitTopLevelEntries(readArrowArrayBlock(source, name)).filter((entry) =>
    entry.trim().startsWith("{"),
  );
}

function displayOptionsFor(
  blockType: string,
  richText?: boolean,
) {
  const richTextSection =
    richText === undefined ? "" : `,\nrichText: [${richText ? "true" : "false"}]`;

  return `{
show: {
type: ['${blockType}']${richTextSection},
},
}`;
}

function replaceDisplayOptions(entry: string, displayOptions: string) {
  return entry.replace(/\bdisplayOptions\s*,/g, `displayOptions: ${displayOptions},`);
}

function replaceTypeParameter(entry: string, blockType: string) {
  return entry.replace(/\btype:\s*\[type\]/g, `type: ['${blockType}']`);
}

function expandKnownArrayReferences(source: string, blocksSource: string) {
  return nestedArrayNames.reduce((expandedSource, name) => {
    const block = readConstArrayBlock(blocksSource, name);
    const content = arrayContent(block);

    return expandedSource
      .replace(new RegExp(`\\.\\.\\.${name}\\b`, "g"), content)
      .replace(new RegExp(`(:\\s*)${name}\\b`, "g"), `$1${block}`);
  }, source);
}

function entriesFromTypeFunction(
  blocksSource: string,
  name: string,
  blockType: string,
) {
  return entriesFromArrowArray(blocksSource, name).map((entry) =>
    replaceTypeParameter(entry, blockType),
  );
}

function entriesFromDisplayOptionsFunction(
  blocksSource: string,
  name: string,
  displayOptions: string,
) {
  return entriesFromArrowArray(blocksSource, name).map((entry) =>
    expandKnownArrayReferences(
      replaceDisplayOptions(entry, displayOptions),
      blocksSource,
    ),
  );
}

function blockTypeEntries(blocksSource: string, blockType: string) {
  if (blockType === "to_do") {
    return [
      ...entriesFromTypeFunction(blocksSource, "todo", blockType),
      ...entriesFromDisplayOptionsFunction(
        blocksSource,
        "richText",
        displayOptionsFor(blockType),
      ),
      ...entriesFromDisplayOptionsFunction(
        blocksSource,
        "textContent",
        displayOptionsFor(blockType, false),
      ),
      ...entriesFromDisplayOptionsFunction(
        blocksSource,
        "text",
        displayOptionsFor(blockType, true),
      ),
    ];
  }

  if (blockType === "child_page") {
    return entriesFromTypeFunction(blocksSource, "title", blockType);
  }

  if (blockType === "image") {
    return entriesFromTypeFunction(blocksSource, "imageBlock", blockType);
  }

  return [
    ...entriesFromDisplayOptionsFunction(
      blocksSource,
      "richText",
      displayOptionsFor(blockType),
    ),
    ...entriesFromDisplayOptionsFunction(
      blocksSource,
      "textContent",
      displayOptionsFor(blockType, false),
    ),
    ...entriesFromDisplayOptionsFunction(
      blocksSource,
      "text",
      displayOptionsFor(blockType, true),
    ),
  ];
}

function blockCallTargets(sources: string[]) {
  const targets = sources.flatMap((source) =>
    [...source.matchAll(/\.\.\.blocks\(\s*['"]([^'"]+)['"]\s*,\s*['"]([^'"]+)['"]\s*\)/g)]
      .map((match) => ({
        resource: match[1],
        operation: match[2],
      })),
  );
  const seen = new Set<string>();

  return targets.filter((target) => {
    const key = `${target.resource}:${target.operation}`;
    if (seen.has(key)) {
      return false;
    }
    seen.add(key);
    return true;
  });
}

function blockUiEntry(blocksSource: string, target: BlockCallTarget) {
  const [entry] = entriesFromArrowArray(blocksSource, "blocks");

  return expandKnownArrayReferences(
    entry
      .replace(/\bresource:\s*\[resource\]/g, `resource: ['${target.resource}']`)
      .replace(/\boperation:\s*\[operation\]/g, `operation: ['${target.operation}']`)
      .replace(/\.\.\.block\(\s*['"]([^'"]+)['"]\s*\)/g, (_match, blockType: string) =>
        blockTypeEntries(blocksSource, blockType).join(",\n"),
      ),
    blocksSource,
  );
}

export function sourceEntriesFromNotionBlockCalls(
  blocksSource: string,
  descriptionSources: string[],
) {
  return blockCallTargets(descriptionSources).map((target) =>
    blockUiEntry(blocksSource, target),
  );
}
