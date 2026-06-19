import { toNextJsHandler } from "better-auth/next-js";

import type { createDxBetterAuth } from "./server";

export type DxBetterAuth = ReturnType<typeof createDxBetterAuth>;

export function createDxBetterAuthRouteHandlers(auth: DxBetterAuth) {
  return toNextJsHandler(auth);
}
