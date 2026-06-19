"use client";

import { signIn, signUp } from "./client";

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
  image?: string;
  callbackURL?: string;
};

export type DxBetterAuthEmailSignInResult = Awaited<
  ReturnType<typeof signIn.email>
>;

export type DxBetterAuthEmailSignUpResult = Awaited<
  ReturnType<typeof signUp.email>
>;

export async function signInDxBetterAuthEmail(
  input: DxBetterAuthEmailSignInInput,
): Promise<DxBetterAuthEmailSignInResult> {
  const email = normalizeDxBetterAuthEmail(input.email);
  const password = readDxBetterAuthPassword(input.password);

  return signIn.email({
    email,
    password,
    rememberMe: input.rememberMe ?? true,
    ...(input.callbackURL ? { callbackURL: input.callbackURL } : {}),
  });
}

export async function signUpDxBetterAuthEmail(
  input: DxBetterAuthEmailSignUpInput,
): Promise<DxBetterAuthEmailSignUpResult> {
  const email = normalizeDxBetterAuthEmail(input.email);
  const password = readDxBetterAuthPassword(input.password);
  const name = input.name.trim();

  if (!name) {
    throw new Error("DX Authentication sign-up name is required");
  }

  return signUp.email({
    email,
    name,
    password,
    ...(input.image ? { image: input.image } : {}),
    ...(input.callbackURL ? { callbackURL: input.callbackURL } : {}),
  });
}

export function normalizeDxBetterAuthEmail(email: string): string {
  const normalized = email.trim();

  if (!normalized || !normalized.includes("@")) {
    throw new Error("DX Authentication email address is required");
  }

  return normalized;
}

function readDxBetterAuthPassword(password: string): string {
  if (!password) {
    throw new Error("DX Authentication password is required");
  }

  return password;
}
