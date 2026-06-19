"use client";

import { authClient } from "./client";

export function listDxBetterAuthSessions() {
  return authClient.listSessions();
}

export function revokeDxBetterAuthSession(token: string) {
  return authClient.revokeSession({ token });
}

export function revokeOtherDxBetterAuthSessions() {
  return authClient.revokeOtherSessions();
}
