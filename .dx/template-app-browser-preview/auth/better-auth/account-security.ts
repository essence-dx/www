"use client";

import { authClient } from "./client";

export type DxBetterAuthPasswordResetInput = {
  email: string;
  redirectTo?: string;
};

export type DxBetterAuthPasswordChangeInput = {
  currentPassword: string;
  newPassword: string;
  revokeOtherSessions?: boolean;
};

export function requestDxBetterAuthPasswordReset(
  input: DxBetterAuthPasswordResetInput,
) {
  return authClient.forgetPassword(input);
}

export function changeDxBetterAuthPassword(
  input: DxBetterAuthPasswordChangeInput,
) {
  return authClient.changePassword(input);
}
