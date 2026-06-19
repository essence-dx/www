import {
  readSupabasePublicConfig,
  type DxSupabasePublicConfig,
} from "@/lib/supabase/env";
import {
  getDxSupabaseCurrentProfile,
  upsertDxSupabaseProfile,
  type DxSupabaseProfile,
  type DxSupabaseProfileInput,
} from "@/lib/supabase/profiles";

export type {
  DxSupabaseProfile,
  DxSupabaseProfileInput,
} from "@/lib/supabase/profiles";

export type DxSupabaseProfileConfigStatus =
  | {
      kind: "ready";
      config: DxSupabasePublicConfig;
      message: string;
    }
  | {
      kind: "missing-config";
      message: string;
    };

export type DxSupabaseProfileUpsertReceipt = {
  status: "ready-to-submit" | "missing-config";
  userId: string;
  operation: string;
  input: DxSupabaseProfileInput;
  boundary: string;
};

export type DxSupabaseProfileField = {
  key: Extract<keyof DxSupabaseProfileInput, "fullName" | "username" | "website">;
  label: string;
  inputType: "text" | "url";
  autoComplete: string;
};

export type DxSupabaseProfilesReadModel = {
  kind: "ready";
  table: "profiles";
  select: "id, full_name, username, website";
  operation: "supabase.from('profiles').select('id, full_name, username, website')";
  rows: readonly DxSupabaseProfile[];
  message: string;
};

export const dxSupabaseProfileApi = {
  readCurrent: getDxSupabaseCurrentProfile,
  upsert: upsertDxSupabaseProfile,
} as const;

export const dxSupabaseProfileFields = [
  {
    key: "fullName",
    label: "Full name",
    inputType: "text",
    autoComplete: "name",
  },
  {
    key: "username",
    label: "Username",
    inputType: "text",
    autoComplete: "username",
  },
  {
    key: "website",
    label: "Website",
    inputType: "url",
    autoComplete: "url",
  },
] as const satisfies readonly DxSupabaseProfileField[];

export const dxSupabaseLocalProfile: DxSupabaseProfile = {
  id: "00000000-0000-4000-8000-000000000001",
  updatedAt: null,
  username: "essencedx",
  fullName: "essencefromexistence",
  avatarUrl: null,
  website: "https://dx.local/profile",
};

export const dxSupabaseLocalProfiles: readonly DxSupabaseProfile[] = [
  dxSupabaseLocalProfile,
  {
    id: "00000000-0000-4000-8000-000000000002",
    updatedAt: null,
    username: "friday",
    fullName: "Friday",
    avatarUrl: null,
    website: "https://dx.local/friday",
  },
] as const;

export const dxSupabaseInitialProfileDraft: DxSupabaseProfileInput = {
  username: dxSupabaseLocalProfile.username,
  fullName: dxSupabaseLocalProfile.fullName,
  avatarUrl: dxSupabaseLocalProfile.avatarUrl,
  website: dxSupabaseLocalProfile.website,
};

export function readDxSupabaseProfileConfigStatus(): DxSupabaseProfileConfigStatus {
  try {
    const config = readSupabasePublicConfig();
    return {
      kind: "ready",
      config,
      message: config.isLocal
        ? "Local Supabase is configured for profile writes."
        : "Hosted Supabase public config is present for profile writes.",
    };
  } catch (error) {
    return {
      kind: "missing-config",
      message: error instanceof Error ? error.message : "Supabase config is missing.",
    };
  }
}

export function readDxSupabaseProfilesReadModel(): DxSupabaseProfilesReadModel {
  return {
    kind: "ready",
    table: "profiles",
    select: "id, full_name, username, website",
    operation: "supabase.from('profiles').select('id, full_name, username, website')",
    rows: dxSupabaseLocalProfiles,
    message:
      "Local profiles fixture is ready. Hosted Supabase queries stay behind app-owned env config.",
  };
}

export function createDxSupabaseProfilePreview(
  profile: DxSupabaseProfile,
  input: DxSupabaseProfileInput,
): DxSupabaseProfile {
  return {
    ...profile,
    username: input.username ?? null,
    fullName: input.fullName ?? null,
    avatarUrl: input.avatarUrl ?? null,
    website: input.website ?? null,
  };
}

export function updateDxSupabaseProfileDraft(
  current: DxSupabaseProfileInput,
  field: keyof DxSupabaseProfileInput,
  value: string,
): DxSupabaseProfileInput {
  return {
    ...current,
    [field]: value.trim() ? value : null,
  };
}

export function createDxSupabaseProfileUpsertReceipt(
  status: DxSupabaseProfileConfigStatus,
  profile: DxSupabaseProfile,
  input: DxSupabaseProfileInput,
): DxSupabaseProfileUpsertReceipt {
  return {
    status: status.kind === "ready" ? "ready-to-submit" : "missing-config",
    userId: profile.id,
    operation: `${dxSupabaseProfileApi.upsert.name}(userId, input)`,
    input,
    boundary:
      status.kind === "ready"
        ? "User action can submit through the app-owned Supabase project."
        : "Set NEXT_PUBLIC_SUPABASE_URL and NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY before live writes.",
  };
}
