"use client";

import {
  changePassword,
  requestPasswordReset,
  resetPassword,
  sendVerificationEmail,
} from "./client";
import { normalizeDxBetterAuthEmail } from "./email-password";

export type DxBetterAuthPasswordResetRequestInput = {
  email: string;
  redirectTo?: string;
};

export type DxBetterAuthPasswordResetInput = {
  token: string;
  newPassword: string;
};

export type DxBetterAuthPasswordChangeInput = {
  currentPassword: string;
  newPassword: string;
  revokeOtherSessions?: boolean;
};

export type DxBetterAuthVerificationEmailInput = {
  email: string;
  callbackURL?: string;
};

export type DxBetterAuthPasswordResetRequestResult = Awaited<
  ReturnType<typeof requestPasswordReset>
>;

export type DxBetterAuthPasswordResetResult = Awaited<
  ReturnType<typeof resetPassword>
>;

export type DxBetterAuthPasswordChangeResult = Awaited<
  ReturnType<typeof changePassword>
>;

export type DxBetterAuthVerificationEmailResult = Awaited<
  ReturnType<typeof sendVerificationEmail>
>;

export async function requestDxBetterAuthPasswordReset(
  input: DxBetterAuthPasswordResetRequestInput,
): Promise<DxBetterAuthPasswordResetRequestResult> {
  const email = normalizeDxBetterAuthEmail(input.email);

  return requestPasswordReset({
    email,
    ...(input.redirectTo ? { redirectTo: input.redirectTo } : {}),
  });
}

export async function resetDxBetterAuthPassword(
  input: DxBetterAuthPasswordResetInput,
): Promise<DxBetterAuthPasswordResetResult> {
  const token = readDxBetterAuthResetToken(input.token);
  const newPassword = readDxBetterAuthPasswordValue(
    input.newPassword,
    "new password",
  );

  return resetPassword({
    token,
    newPassword,
  });
}

export async function changeDxBetterAuthPassword(
  input: DxBetterAuthPasswordChangeInput,
): Promise<DxBetterAuthPasswordChangeResult> {
  const currentPassword = readDxBetterAuthPasswordValue(
    input.currentPassword,
    "current password",
  );
  const newPassword = readDxBetterAuthPasswordValue(
    input.newPassword,
    "new password",
  );

  return changePassword({
    currentPassword,
    newPassword,
    revokeOtherSessions: input.revokeOtherSessions ?? true,
  });
}

export async function sendDxBetterAuthVerificationEmail(
  input: DxBetterAuthVerificationEmailInput,
): Promise<DxBetterAuthVerificationEmailResult> {
  const email = normalizeDxBetterAuthEmail(input.email);

  return sendVerificationEmail({
    email,
    ...(input.callbackURL ? { callbackURL: input.callbackURL } : {}),
  });
}

function readDxBetterAuthPasswordValue(
  value: string,
  label: string,
): string {
  if (!value) {
    throw new Error(`DX Authentication ${label} is required`);
  }

  return value;
}

function readDxBetterAuthResetToken(token: string): string {
  const safeToken = token.trim();

  if (!safeToken) {
    throw new Error("DX Authentication reset token is required");
  }

  return safeToken;
}
