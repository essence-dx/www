function unique(values) {
  return [...new Set(values)];
}

function comparableReceipt(value) {
  if (Array.isArray(value)) {
    return value.map(comparableReceipt);
  }

  if (!value || typeof value !== "object") {
    return value;
  }

  const result = {};
  for (const [key, child] of Object.entries(value)) {
    if (/^checked[A-Z]/.test(key)) {
      continue;
    }
    if (
      key === "generatedAt" &&
      typeof value.kind === "string" &&
      value.kind.startsWith("dx.nextRust.vendorBoundary")
    ) {
      continue;
    }
    result[key] = comparableReceipt(child);
  }
  return result;
}

function findMismatches(actual, expected, prefix = "") {
  if (JSON.stringify(actual) === JSON.stringify(expected)) {
    return [];
  }

  if (
    !actual ||
    !expected ||
    typeof actual !== "object" ||
    typeof expected !== "object" ||
    Array.isArray(actual) ||
    Array.isArray(expected)
  ) {
    return [prefix || "<root>"];
  }

  const keys = unique([...Object.keys(actual), ...Object.keys(expected)]).sort();
  return keys.flatMap((key) =>
    findMismatches(actual[key], expected[key], prefix ? `${prefix}.${key}` : key),
  );
}

module.exports = {
  comparableReceipt,
  findMismatches,
};
