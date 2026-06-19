export type DxGoogleOAuthEnv = Record<string, string | undefined>;

export type DxGoogleOAuthConfig = {
  clientId: string;
  clientSecret: string;
  redirectUri: string;
  scopes: string[];
  stateCookieName: string;
  allowedRedirectOrigin?: string;
};

const DEFAULT_SCOPES = ["openid", "email", "profile"];

export function defaultGoogleOAuthEnv(): DxGoogleOAuthEnv {
  return (globalThis as unknown as { process?: { env?: DxGoogleOAuthEnv } }).process?.env ?? {};
}

export function readGoogleOAuthConfig(env: DxGoogleOAuthEnv = defaultGoogleOAuthEnv()): DxGoogleOAuthConfig {
  const clientId = requiredEnv(env, "GOOGLE_CLIENT_ID");
  const clientSecret = requiredEnv(env, "GOOGLE_CLIENT_SECRET");
  const redirectUri = requiredEnv(env, "GOOGLE_REDIRECT_URI");
  const scopes = (env.GOOGLE_OAUTH_SCOPES ?? DEFAULT_SCOPES.join(" "))
    .split(/[,\s]+/)
    .map((scope) => scope.trim())
    .filter(Boolean);

  return {
    clientId,
    clientSecret,
    redirectUri,
    scopes: scopes.length > 0 ? scopes : DEFAULT_SCOPES,
    stateCookieName: env.DX_GOOGLE_STATE_COOKIE ?? "dx_google_oauth_state",
    allowedRedirectOrigin: env.DX_GOOGLE_ALLOWED_REDIRECT_ORIGIN,
  };
}

export function buildGoogleAuthorizationUrl(config: DxGoogleOAuthConfig, state: string): string {
  const url = new URL("https://accounts.google.com/o/oauth2/v2/auth");
  url.searchParams.set("client_id", config.clientId);
  url.searchParams.set("redirect_uri", config.redirectUri);
  url.searchParams.set("response_type", "code");
  url.searchParams.set("scope", config.scopes.join(" "));
  url.searchParams.set("state", state);
  url.searchParams.set("access_type", "offline");
  url.searchParams.set("prompt", "consent");
  return url.toString();
}

export function createGoogleOAuthState(bytes = 24): string {
  const random = new Uint8Array(bytes);
  globalThis.crypto.getRandomValues(random);
  return Array.from(random, (value) => value.toString(16).padStart(2, "0")).join("");
}

export function serializeGoogleStateCookie(name: string, state: string): string {
  return `${name}=${state}; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=600`;
}

export function parseCookies(header: string | null): Record<string, string> {
  return Object.fromEntries(
    (header ?? "")
      .split(";")
      .map((part) => part.trim())
      .filter(Boolean)
      .map((part) => {
        const index = part.indexOf("=");
        return index === -1 ? [part, ""] : [part.slice(0, index), part.slice(index + 1)];
      }),
  );
}

export function assertSafeRedirect(requestUrl: URL, config: DxGoogleOAuthConfig): void {
  if (!config.allowedRedirectOrigin) {
    return;
  }
  if (requestUrl.origin !== config.allowedRedirectOrigin) {
    throw new Error(`Google OAuth callback origin ${requestUrl.origin} is not allowed`);
  }
}

function requiredEnv(env: DxGoogleOAuthEnv, name: string): string {
  const value = env[name]?.trim();
  if (!value) {
    throw new Error(`Missing required Google OAuth env var: ${name}`);
  }
  return value;
}
