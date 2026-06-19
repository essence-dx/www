"use client";

import { createAuthClient } from "better-auth/react";

export type DxBetterAuthClientEnv = Record<string, string | undefined>;
export type DxBetterAuthClientOptions = Parameters<typeof createAuthClient>[0];

export function defaultBetterAuthClientEnv(): DxBetterAuthClientEnv {
  return (globalThis as unknown as { process?: { env?: DxBetterAuthClientEnv } }).process?.env ?? {};
}

export function readBetterAuthClientBaseURL(
  env: DxBetterAuthClientEnv = defaultBetterAuthClientEnv(),
): string | undefined {
  return env.NEXT_PUBLIC_BETTER_AUTH_URL?.trim() || env.BETTER_AUTH_URL?.trim() || undefined;
}

export function createDxAuthClient(options?: DxBetterAuthClientOptions) {
  const baseURL = options?.baseURL ?? readBetterAuthClientBaseURL();

  return createAuthClient({
    ...options,
    ...(baseURL ? { baseURL } : {}),
  });
}

export const authClient = createDxAuthClient();
export const {
  signIn,
  signUp,
  signOut,
  useSession,
  getSession,
  listSessions,
  revokeSession,
  revokeOtherSessions,
  revokeSessions,
  requestPasswordReset,
  resetPassword,
  changePassword,
  sendVerificationEmail,
  listAccounts,
  linkSocial,
  unlinkAccount,
  getAccessToken,
  updateUser,
  changeEmail,
  deleteUser,
} = authClient;
