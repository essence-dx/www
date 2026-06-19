"use client";

import { signIn } from "./client";

export type DxBetterAuthSocialProvider = "google";

export type DxBetterAuthSocialIdToken = {
  token: string;
  nonce?: string;
  accessToken?: string;
  refreshToken?: string;
  expiresAt?: number;
  user?: {
    name?: {
      firstName?: string;
      lastName?: string;
    };
    email?: string;
  };
};

export type DxBetterAuthSocialSignInInput = {
  provider: DxBetterAuthSocialProvider;
  callbackURL?: string;
  errorCallbackURL?: string;
  newUserCallbackURL?: string;
  disableRedirect?: boolean;
  scopes?: string[];
  requestSignUp?: boolean;
  loginHint?: string;
  idToken?: DxBetterAuthSocialIdToken;
  additionalData?: Record<string, unknown>;
};

export type DxBetterAuthSocialSignInResult = Awaited<
  ReturnType<typeof signIn.social>
>;

export async function signInDxBetterAuthSocial(
  input: DxBetterAuthSocialSignInInput,
): Promise<DxBetterAuthSocialSignInResult> {
  return signIn.social({
    provider: readDxBetterAuthSocialProvider(input.provider),
    ...(input.callbackURL ? { callbackURL: input.callbackURL } : {}),
    ...(input.errorCallbackURL
      ? { errorCallbackURL: input.errorCallbackURL }
      : {}),
    ...(input.newUserCallbackURL
      ? { newUserCallbackURL: input.newUserCallbackURL }
      : {}),
    ...(input.disableRedirect === undefined
      ? {}
      : { disableRedirect: input.disableRedirect }),
    ...(input.scopes?.length ? { scopes: input.scopes } : {}),
    ...(input.requestSignUp === undefined
      ? {}
      : { requestSignUp: input.requestSignUp }),
    ...(input.loginHint ? { loginHint: input.loginHint } : {}),
    ...(input.idToken ? { idToken: input.idToken } : {}),
    ...(input.additionalData ? { additionalData: input.additionalData } : {}),
  });
}

export function readDxBetterAuthSocialProvider(
  provider: DxBetterAuthSocialProvider,
): DxBetterAuthSocialProvider {
  if (provider !== "google") {
    throw new Error("DX Authentication social provider must be google");
  }

  return provider;
}
