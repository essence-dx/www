import {
  assertSafeRedirect,
  defaultGoogleOAuthEnv,
  parseCookies,
  readGoogleOAuthConfig,
  type DxGoogleOAuthEnv,
} from "./config";

export type DxGoogleTokenResponse = {
  access_token?: string;
  expires_in?: number;
  id_token?: string;
  refresh_token?: string;
  scope?: string;
  token_type?: string;
  error?: string;
  error_description?: string;
};

export async function handleGoogleAuthCallback(
  request: Request,
  env: DxGoogleOAuthEnv = defaultGoogleOAuthEnv(),
  fetcher: typeof fetch = fetch,
): Promise<Response> {
  const config = readGoogleOAuthConfig(env);
  const url = new URL(request.url);
  assertSafeRedirect(url, config);

  const error = url.searchParams.get("error");
  if (error) {
    return json({ ok: false, error }, 400);
  }

  const code = url.searchParams.get("code");
  const state = url.searchParams.get("state");
  const cookies = parseCookies(request.headers.get("cookie"));
  if (!code || !state || cookies[config.stateCookieName] !== state) {
    return json({ ok: false, error: "invalid_google_oauth_state" }, 400);
  }

  const token = await exchangeGoogleCode(config, code, fetcher);
  if (token.error) {
    return json({ ok: false, error: token.error, description: token.error_description }, 400);
  }

  return json({
    ok: true,
    token,
    next: "Store this token response in your application session layer.",
  });
}

export const GET = handleGoogleAuthCallback;

async function exchangeGoogleCode(
  config: ReturnType<typeof readGoogleOAuthConfig>,
  code: string,
  fetcher: typeof fetch,
): Promise<DxGoogleTokenResponse> {
  const body = new URLSearchParams({
    client_id: config.clientId,
    client_secret: config.clientSecret,
    code,
    grant_type: "authorization_code",
    redirect_uri: config.redirectUri,
  });
  const response = await fetcher("https://oauth2.googleapis.com/token", {
    method: "POST",
    headers: { "content-type": "application/x-www-form-urlencoded" },
    body,
  });
  return response.json() as Promise<DxGoogleTokenResponse>;
}

function json(body: unknown, status = 200): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { "content-type": "application/json; charset=utf-8" },
  });
}
