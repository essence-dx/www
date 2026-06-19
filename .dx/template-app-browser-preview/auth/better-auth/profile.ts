"use client";

import { authClient } from "./client";

export type DxBetterAuthProfileInput = {
  name?: string;
  image?: string;
};

export type DxBetterAuthEmailChangeInput = {
  newEmail: string;
  callbackURL?: string;
};

export function updateDxBetterAuthUserProfile(
  input: DxBetterAuthProfileInput,
) {
  return authClient.updateUser(input);
}

export function changeDxBetterAuthEmail(input: DxBetterAuthEmailChangeInput) {
  return authClient.changeEmail(input);
}
