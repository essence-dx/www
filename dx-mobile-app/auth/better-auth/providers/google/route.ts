import {
  buildGoogleAuthorizationUrl,
  createGoogleOAuthState,
  defaultGoogleOAuthEnv,
  readGoogleOAuthConfig,
  serializeGoogleStateCookie,
  type DxGoogleOAuthEnv,
} from "./config";

export function handleGoogleAuthStart(
  _request: Request,
  env: DxGoogleOAuthEnv = defaultGoogleOAuthEnv(),
): Response {
  const config = readGoogleOAuthConfig(env);
  const state = createGoogleOAuthState();
  const response = Response.redirect(buildGoogleAuthorizationUrl(config, state), 302);
  response.headers.append("Set-Cookie", serializeGoogleStateCookie(config.stateCookieName, state));
  return response;
}

export const GET = handleGoogleAuthStart;
