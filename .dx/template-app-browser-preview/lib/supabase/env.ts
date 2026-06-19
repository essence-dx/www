export type DxSupabaseEnv = Record<string, string | undefined>;

export type DxSupabasePublicConfig = {
  url: string;
  publishableKey: string;
  isLocal: boolean;
};

export function defaultSupabaseEnv(): DxSupabaseEnv {
  return (globalThis as unknown as { process?: { env?: DxSupabaseEnv } }).process?.env ?? {};
}

export function readSupabasePublicConfig(
  env: DxSupabaseEnv = defaultSupabaseEnv(),
): DxSupabasePublicConfig {
  const url = assertSupabasePublicUrl(requiredEnv(env, "NEXT_PUBLIC_SUPABASE_URL"));
  const publishableKey = assertSupabasePublishableKey(
    requiredEnv(env, "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY"),
  );

  return {
    url,
    publishableKey,
    isLocal: isLocalSupabaseUrl(url),
  };
}

export function isLocalSupabaseUrl(value: string): boolean {
  try {
    const url = new URL(value);
    return url.hostname === "localhost" || url.hostname === "127.0.0.1";
  } catch {
    return false;
  }
}

export function requiredEnv(env: DxSupabaseEnv, key: string): string {
  const value = env[key]?.trim();
  if (!value) {
    throw new Error(`Missing required Supabase env var: ${key}`);
  }
  return value;
}

export function assertSupabasePublicUrl(value: string): string {
  let url: URL;
  try {
    url = new URL(value);
  } catch {
    throw new Error("NEXT_PUBLIC_SUPABASE_URL must be a valid URL.");
  }

  if (!["http:", "https:"].includes(url.protocol)) {
    throw new Error("NEXT_PUBLIC_SUPABASE_URL must use http or https.");
  }

  return value;
}

export function assertSupabasePublishableKey(value: string): string {
  if (/service[_-]?role|secret/i.test(value)) {
    throw new Error(
      "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY must be a publishable key, not a service-role secret.",
    );
  }

  return value;
}
