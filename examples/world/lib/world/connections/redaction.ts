import type { WorldConnectionEnvStatus } from "./contracts";

const SECRET_NAME_PATTERN = /(TOKEN|SECRET|KEY|PASSWORD|DATABASE_URL|AUTH|SID|DSN)/i;

export function connectionEnvStatus(
  requiredEnv: readonly string[],
  optionalEnv: readonly string[],
  env: Record<string, string | undefined>,
): WorldConnectionEnvStatus {
  const allNames = [...requiredEnv, ...optionalEnv];
  const presentEnv = allNames.filter((name) => Boolean(env[name]));
  const missingEnv = requiredEnv.filter((name) => !env[name]);

  return {
    requiredEnv,
    optionalEnv,
    presentEnv,
    missingEnv,
  };
}

export function readableEndpointLabel(value: string): string {
  try {
    const url = new URL(value);
    return `${url.origin}${url.pathname}`;
  } catch {
    return value.replace(/[?#].*$/, "");
  }
}

export function redactedEnvLabel(name: string, env: Record<string, string | undefined>): string {
  if (!env[name]) {
    return `${name}:missing`;
  }

  return SECRET_NAME_PATTERN.test(name) ? `${name}:present-redacted` : `${name}:present`;
}

export function hasLeakedEnvValue(value: unknown, env: Record<string, string | undefined>): boolean {
  const serialized = JSON.stringify(value);

  if (!serialized) {
    return false;
  }

  return Object.entries(env).some(([name, secret]) => {
    if (!SECRET_NAME_PATTERN.test(name)) {
      return false;
    }

    if (!secret || secret.length < 4) {
      return false;
    }

    return serialized.includes(secret);
  });
}
