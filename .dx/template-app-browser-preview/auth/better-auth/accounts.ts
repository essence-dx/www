"use client";

import { authClient } from "./client";
import type { DxBetterAuthSocialProvider } from "./social";

export type DxBetterAuthLinkAccountInput = {
  provider: DxBetterAuthSocialProvider;
  callbackURL?: string;
  errorCallbackURL?: string;
};

export function listDxBetterAuthAccounts() {
  return authClient.listAccounts();
}

export function linkDxBetterAuthSocialAccount(
  input: DxBetterAuthLinkAccountInput,
) {
  return authClient.linkSocial(input);
}

export function unlinkDxBetterAuthAccount(providerId: string) {
  return authClient.unlinkAccount({ providerId });
}
