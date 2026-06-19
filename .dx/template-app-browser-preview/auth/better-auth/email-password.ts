"use client";

import { authClient } from "./client";

export type DxBetterAuthEmailSignInInput = {
  email: string;
  password: string;
  rememberMe?: boolean;
  callbackURL?: string;
};

export type DxBetterAuthEmailSignUpInput = {
  email: string;
  name: string;
  password: string;
  callbackURL?: string;
};

export function signInDxBetterAuthEmail(input: DxBetterAuthEmailSignInInput) {
  return authClient.signIn.email(input);
}

export function signUpDxBetterAuthEmail(input: DxBetterAuthEmailSignUpInput) {
  return authClient.signUp.email(input);
}
