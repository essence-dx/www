function matchingClose(open: string) {
  return open === "{" ? "}" : "]";
}

export function readBalancedBlock(source: string, startIndex: number) {
  const open = source[startIndex];
  const close = matchingClose(open);
  let depth = 0;
  let quote: string | undefined;
  let escaped = false;

  for (let index = startIndex; index < source.length; index += 1) {
    const character = source[index];

    if (quote) {
      escaped = character === "\\" && !escaped;
      if (character === quote && !escaped) {
        quote = undefined;
      } else if (character !== "\\") {
        escaped = false;
      }
      continue;
    }

    if (character === "'" || character === '"' || character === "`") {
      quote = character;
      continue;
    }

    if (character === open) {
      depth += 1;
    } else if (character === close) {
      depth -= 1;
      if (depth === 0) {
        return source.slice(startIndex, index + 1);
      }
    }
  }

  throw new Error("Unable to read balanced source block");
}

export function readExportedAssignmentBlock(
  source: string,
  name: string,
  delimiter: "{" | "[",
) {
  const assignment = source.indexOf(`export const ${name}`);
  if (assignment === -1) {
    throw new Error(`Missing exported parameter source: ${name}`);
  }

  const equalsIndex = source.indexOf("=", assignment);
  const startIndex = source.indexOf(delimiter, equalsIndex);
  if (startIndex === -1) {
    throw new Error(`Missing ${delimiter} block for exported parameter source: ${name}`);
  }

  return readBalancedBlock(source, startIndex);
}

export function readConstAssignmentBlock(
  source: string,
  name: string,
  delimiter: "{" | "[",
) {
  const assignmentPattern = new RegExp(`(?:export\\s+)?const\\s+${name}\\b`);
  const assignment = source.search(assignmentPattern);
  if (assignment === -1) {
    throw new Error(`Missing const parameter source: ${name}`);
  }

  const equalsIndex = source.indexOf("=", assignment);
  const startIndex = source.indexOf(delimiter, equalsIndex);
  if (startIndex === -1) {
    throw new Error(`Missing ${delimiter} block for const parameter source: ${name}`);
  }

  return readBalancedBlock(source, startIndex);
}

export function splitTopLevelEntries(block: string) {
  const content = block.slice(1, -1);
  const entries: string[] = [];
  let start = 0;
  let depth = 0;
  let quote: string | undefined;
  let escaped = false;

  for (let index = 0; index < content.length; index += 1) {
    const character = content[index];

    if (quote) {
      escaped = character === "\\" && !escaped;
      if (character === quote && !escaped) {
        quote = undefined;
      } else if (character !== "\\") {
        escaped = false;
      }
      continue;
    }

    if (character === "'" || character === '"' || character === "`") {
      quote = character;
      continue;
    }

    if (character === "{" || character === "[") {
      depth += 1;
    } else if (character === "}" || character === "]") {
      depth -= 1;
    } else if (character === "," && depth === 0) {
      const entry = content.slice(start, index).trim();
      if (entry) {
        entries.push(entry);
      }
      start = index + 1;
    }
  }

  const lastEntry = content.slice(start).trim();
  if (lastEntry) {
    entries.push(lastEntry);
  }

  return entries;
}
