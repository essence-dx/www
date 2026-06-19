"use client";

import { deleteUser } from "./client";

export const DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION =
  "delete my account" as const;

export type DxBetterAuthDeleteAccountInput = {
  confirmation: typeof DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION;
  callbackURL?: string;
  password?: string;
  token?: string;
  allowFreshSessionDelete?: boolean;
};

export type DxBetterAuthDeleteAccountResult = Awaited<
  ReturnType<typeof deleteUser>
>;

export async function deleteDxBetterAuthAccount(
  input: DxBetterAuthDeleteAccountInput,
): Promise<DxBetterAuthDeleteAccountResult> {
  readDxBetterAuthDeleteConfirmation(input.confirmation);

  const safePassword = readOptionalDxBetterAuthSecret(input.password);
  const safeToken = readOptionalDxBetterAuthSecret(input.token);

  if (!safePassword && !safeToken && input.allowFreshSessionDelete !== true) {
    throw new Error(
      "DX Authentication account deletion requires a password, token, or explicit fresh-session opt-in",
    );
  }

  return deleteUser({
    ...(input.callbackURL ? { callbackURL: input.callbackURL } : {}),
    ...(safePassword ? { password: safePassword } : {}),
    ...(safeToken ? { token: safeToken } : {}),
  });
}

function readDxBetterAuthDeleteConfirmation(
  confirmation: DxBetterAuthDeleteAccountInput["confirmation"],
): void {
  if (confirmation !== DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION) {
    throw new Error(
      `Type "${DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION}" to delete the account`,
    );
  }
}

function readOptionalDxBetterAuthSecret(
  value: string | undefined,
): string | undefined {
  const safeValue = value?.trim();

  return safeValue || undefined;
}
