import { headers } from "next/headers";

import { createDxBetterAuth } from "./server";

export async function readDxBetterAuthSession() {
  const auth = createDxBetterAuth();

  return auth.api.getSession({
    headers: await headers(),
  });
}

export async function requireDxBetterAuthSession() {
  const session = await readDxBetterAuthSession();

  if (!session) {
    throw new Error("Authentication session is required");
  }

  return session;
}
