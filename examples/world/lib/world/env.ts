import type { WorldEnvRequirement, WorldIntegration } from "./contracts";

declare const process:
  | {
      env?: Record<string, string | undefined>;
    }
  | undefined;

export type WorldEnvSnapshot = ReadonlyMap<string, "present" | "missing">;

export function readWorldEnv(source: Record<string, string | undefined> = process?.env ?? {}): WorldEnvSnapshot {
  const entries = Object.entries(source)
    .filter(([name]) => /^[A-Z][A-Z0-9_]*$/.test(name))
    .map(([name, value]) => [name, value ? "present" : "missing"] as const);

  return new Map(entries);
}

export function requiredEnvNames(integration: WorldIntegration): readonly string[] {
  return integration.env.filter((item) => item.required).map((item) => item.name);
}

export function presentEnvNames(
  requirements: readonly WorldEnvRequirement[],
  snapshot: WorldEnvSnapshot,
): readonly string[] {
  return requirements.filter((item) => snapshot.get(item.name) === "present").map((item) => item.name);
}

export function missingEnvNames(
  requirements: readonly WorldEnvRequirement[],
  snapshot: WorldEnvSnapshot,
): readonly string[] {
  return requirements
    .filter((item) => item.required)
    .filter((item) => snapshot.get(item.name) !== "present")
    .map((item) => item.name);
}
