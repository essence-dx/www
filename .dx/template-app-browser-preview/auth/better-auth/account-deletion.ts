"use client";

import { authClient } from "./client";

export const DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION = "delete my account";

export type DxBetterAuthDeleteAccountInput = {
  confirmation: typeof DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION;
  password?: string;
  callbackURL?: string;
};

export function deleteDxBetterAuthAccount(
  input: DxBetterAuthDeleteAccountInput,
) {
  return authClient.deleteUser({
    password: input.password,
    callbackURL: input.callbackURL,
  });
}
