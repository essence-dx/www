import { headers } from "next/headers";

import type { DxBetterAuth } from "./route";

export type DxBetterAuthSessionList = Awaited<
  ReturnType<DxBetterAuth["api"]["listSessions"]>
>;

export type DxBetterAuthSessionMutation = Awaited<
  ReturnType<DxBetterAuth["api"]["revokeSession"]>
>;

export async function listDxBetterAuthSessions(
  auth: DxBetterAuth,
): Promise<DxBetterAuthSessionList> {
  return auth.api.listSessions({
    headers: await headers(),
  });
}

export async function revokeDxBetterAuthSession(
  auth: DxBetterAuth,
  token: string,
): Promise<DxBetterAuthSessionMutation> {
  const safeToken = token.trim();

  if (!safeToken) {
    throw new Error("DX Authentication session token is required");
  }

  return auth.api.revokeSession({
    headers: await headers(),
    body: {
      token: safeToken,
    },
  });
}

export async function revokeOtherDxBetterAuthSessions(
  auth: DxBetterAuth,
): Promise<DxBetterAuthSessionMutation> {
  return auth.api.revokeOtherSessions({
    headers: await headers(),
  });
}

export async function revokeAllDxBetterAuthSessions(
  auth: DxBetterAuth,
): Promise<DxBetterAuthSessionMutation> {
  return auth.api.revokeSessions({
    headers: await headers(),
  });
}
