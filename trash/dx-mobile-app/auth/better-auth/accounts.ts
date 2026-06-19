"use client";

import {
  getAccessToken,
  linkSocial,
  listAccounts,
  unlinkAccount,
} from "./client";
import {
  readDxBetterAuthSocialProvider,
  type DxBetterAuthSocialIdToken,
  type DxBetterAuthSocialProvider,
} from "./social";

export type DxBetterAuthLinkedAccountsResult = Awaited<
  ReturnType<typeof listAccounts>
>;

export type DxBetterAuthAccountLinkInput = {
  provider: DxBetterAuthSocialProvider;
  callbackURL?: string;
  errorCallbackURL?: string;
  disableRedirect?: boolean;
  scopes?: string[];
  requestSignUp?: boolean;
  idToken?: Omit<DxBetterAuthSocialIdToken, "expiresAt" | "user"> & {
    scopes?: string[];
  };
  additionalData?: Record<string, unknown>;
};

export type DxBetterAuthAccountUnlinkInput = {
  providerId: DxBetterAuthSocialProvider;
  accountId?: string;
};

export type DxBetterAuthAccessTokenInput = {
  providerId: DxBetterAuthSocialProvider;
  accountId?: string;
};

export type DxBetterAuthAccountLinkResult = Awaited<
  ReturnType<typeof linkSocial>
>;

export type DxBetterAuthAccountUnlinkResult = Awaited<
  ReturnType<typeof unlinkAccount>
>;

export type DxBetterAuthAccessTokenResult = Awaited<
  ReturnType<typeof getAccessToken>
>;

export async function listDxBetterAuthAccounts(): Promise<DxBetterAuthLinkedAccountsResult> {
  return listAccounts();
}

export async function linkDxBetterAuthSocialAccount(
  input: DxBetterAuthAccountLinkInput,
): Promise<DxBetterAuthAccountLinkResult> {
  return linkSocial({
    provider: readDxBetterAuthSocialProvider(input.provider),
    ...(input.callbackURL ? { callbackURL: input.callbackURL } : {}),
    ...(input.errorCallbackURL
      ? { errorCallbackURL: input.errorCallbackURL }
      : {}),
    ...(input.disableRedirect === undefined
      ? {}
      : { disableRedirect: input.disableRedirect }),
    ...(input.scopes?.length ? { scopes: input.scopes } : {}),
    ...(input.requestSignUp === undefined
      ? {}
      : { requestSignUp: input.requestSignUp }),
    ...(input.idToken ? { idToken: input.idToken } : {}),
    ...(input.additionalData ? { additionalData: input.additionalData } : {}),
  });
}

export async function unlinkDxBetterAuthSocialAccount(
  input: DxBetterAuthAccountUnlinkInput,
): Promise<DxBetterAuthAccountUnlinkResult> {
  return unlinkAccount({
    providerId: readDxBetterAuthSocialProvider(input.providerId),
    ...(input.accountId ? { accountId: input.accountId } : {}),
  });
}

export async function getDxBetterAuthAccessToken(
  input: DxBetterAuthAccessTokenInput,
): Promise<DxBetterAuthAccessTokenResult> {
  return getAccessToken({
    providerId: readDxBetterAuthSocialProvider(input.providerId),
    ...(input.accountId ? { accountId: input.accountId } : {}),
  });
}
