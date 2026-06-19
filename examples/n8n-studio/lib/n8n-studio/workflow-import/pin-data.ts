import { isRecord } from "./records";

export function sanitizePinData(pinData: unknown, keptNodeNames: Set<string>) {
  if (!isRecord(pinData)) {
    return {};
  }

  return Object.fromEntries(
    Object.entries(pinData).filter(
      ([nodeName, items]) => keptNodeNames.has(nodeName) && Array.isArray(items),
    ),
  );
}
