"use client";

import { authClient } from "./client";

export type DxBetterAuthSocialProvider = "google";

export const dxBetterAuthSocialProviders = [
  {
    provider: "google",
    label: "Google",
    requiredEnv: ["GOOGLE_CLIENT_ID", "GOOGLE_CLIENT_SECRET"],
  },
] as const;

export type DxBetterAuthSocialSignInInput = {
  provider: DxBetterAuthSocialProvider;
  callbackURL?: string;
  errorCallbackURL?: string;
  newUserCallbackURL?: string;
};

export function signInDxBetterAuthSocial(
  input: DxBetterAuthSocialSignInInput,
) {
  return authClient.signIn.social(input);
}
