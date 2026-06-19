"use client";

import { changeEmail, updateUser } from "./client";
import { normalizeDxBetterAuthEmail } from "./email-password";

export type DxBetterAuthProfileUpdateInput = {
  name?: string;
  image?: string | null;
  additionalFields?: Record<string, unknown>;
};

export type DxBetterAuthEmailChangeInput = {
  newEmail: string;
  callbackURL?: string;
};

export type DxBetterAuthProfileUpdateResult = Awaited<
  ReturnType<typeof updateUser>
>;

export type DxBetterAuthEmailChangeResult = Awaited<
  ReturnType<typeof changeEmail>
>;

export async function updateDxBetterAuthUserProfile(
  input: DxBetterAuthProfileUpdateInput,
): Promise<DxBetterAuthProfileUpdateResult> {
  const profile = readDxBetterAuthProfileUpdate(input);

  return updateUser({
    ...(profile.name !== undefined ? { name: profile.name } : {}),
    ...(profile.image !== undefined ? { image: profile.image } : {}),
    ...profile.additionalFields,
  });
}

export async function changeDxBetterAuthEmail(
  input: DxBetterAuthEmailChangeInput,
): Promise<DxBetterAuthEmailChangeResult> {
  return changeEmail({
    newEmail: normalizeDxBetterAuthEmail(input.newEmail),
    ...(input.callbackURL ? { callbackURL: input.callbackURL } : {}),
  });
}

function readDxBetterAuthProfileUpdate(input: DxBetterAuthProfileUpdateInput): {
  name?: string;
  image?: string | null;
  additionalFields: Record<string, unknown>;
} {
  const trimmedName = input.name?.trim();
  const name = trimmedName || undefined;
  const image = readDxBetterAuthProfileImage(input.image);
  const additionalFields = readDxBetterAuthAdditionalProfileFields(
    input.additionalFields,
  );

  if (
    name === undefined &&
    image === undefined &&
    Object.keys(additionalFields).length === 0
  ) {
    throw new Error("DX Authentication profile update requires a field to update");
  }

  return {
    ...(name !== undefined ? { name } : {}),
    ...(image !== undefined ? { image } : {}),
    additionalFields,
  };
}

function readDxBetterAuthProfileImage(
  image: string | null | undefined,
): string | null | undefined {
  if (image === undefined || image === null) {
    return image;
  }

  const safeImage = image.trim();

  return safeImage || null;
}

function readDxBetterAuthAdditionalProfileFields(
  additionalFields: Record<string, unknown> | undefined,
): Record<string, unknown> {
  if (!additionalFields) {
    return {};
  }

  if (Object.prototype.hasOwnProperty.call(additionalFields, "email")) {
    throw new Error(
      "DX Authentication profile updates cannot change email; use changeDxBetterAuthEmail",
    );
  }

  return additionalFields;
}
