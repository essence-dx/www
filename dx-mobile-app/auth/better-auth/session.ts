import { headers } from "next/headers";

import type { DxBetterAuth } from "./route";

export type DxBetterAuthSession = Awaited<
  ReturnType<DxBetterAuth["api"]["getSession"]>
>;

export async function getDxBetterAuthSession(
  auth: DxBetterAuth,
): Promise<DxBetterAuthSession> {
  return auth.api.getSession({
    headers: await headers(),
  });
}

export async function requireDxBetterAuthSession(
  auth: DxBetterAuth,
): Promise<NonNullable<DxBetterAuthSession>> {
  const session = await getDxBetterAuthSession(auth);

  if (!session) {
    throw new Error("DX Authentication session required");
  }

  return session;
}
