"use client";

import { createAuthClient } from "better-auth/react";

function readClientBaseURL() {
  if (typeof process === "undefined") {
    return "/api/auth";
  }

  return process.env.NEXT_PUBLIC_BETTER_AUTH_URL ?? "/api/auth";
}

export const authClient = createAuthClient({
  baseURL: readClientBaseURL(),
});

export const useSession = authClient.useSession;

export function signOut(options?: Parameters<typeof authClient.signOut>[0]) {
  return authClient.signOut(options);
}

export function listSessions() {
  return authClient.listSessions();
}

export function revokeOtherSessions() {
  return authClient.revokeOtherSessions();
}
