# Authentication

This package materializes a source-owned Authentication slice based on upstream better-auth APIs for a Next-style launch app without running package-manager lifecycle scripts or creating `node_modules`.

## Owned Surface

- `options.ts` reads `BETTER_AUTH_*` and Google provider env values.
- `server.ts` creates an upstream better-auth server with a caller-provided database adapter.
- `client.ts` creates the React auth client and exports the common client plus session-management actions.
- `email-password.ts` exposes guarded client helpers around upstream better-auth `signIn.email` and `signUp.email`.
- `social.ts` exposes guarded client helpers around upstream better-auth `signIn.social` for Google launches.
- `accounts.ts` exposes guarded client helpers around linked accounts, account linking, unlinking, and OAuth access-token refresh.
- `profile.ts` exposes guarded client helpers around `updateUser` and `changeEmail`.
- `account-deletion.ts` exposes a confirmation-gated helper around `deleteUser`.
- `account-security.ts` exposes guarded client helpers around password reset, password change, and verification email APIs.
- `route.ts` exposes a typed helper around `toNextJsHandler(auth)`.
- `session.ts` exposes server-side session helpers around `auth.api.getSession({ headers })`.
- `session-management.ts` exposes server-side helpers for `listSessions`, `revokeSession`, `revokeOtherSessions`, and `revokeSessions`.
- `metadata.ts` gives DX CLI and host UIs a stable package-discovery record with aliases, source mirror/provenance, exported files, required env, app-owned boundaries, and receipt paths.

## Next Route Example

```ts
import { createDxBetterAuthRouteHandlers } from "@/auth/better-auth/route";
import { createDxBetterAuth } from "@/auth/better-auth/server";
import { database } from "@/server/database";

export const auth = createDxBetterAuth({ database });
export const { GET, POST } = createDxBetterAuthRouteHandlers(auth);
```

## Server Session Example

```ts
import { getDxBetterAuthSession } from "@/auth/better-auth/session";
import { auth } from "@/app/api/auth/[...all]/route";

export async function LaunchAccountMenu() {
  const session = await getDxBetterAuthSession(auth);
  return <span>{session?.user.email ?? "Signed out"}</span>;
}
```

## Session Management Example

```ts
import { listDxBetterAuthSessions } from "@/auth/better-auth/session-management";
import { auth } from "@/app/api/auth/[...all]/route";

export async function LaunchSessionCount() {
  const sessions = await listDxBetterAuthSessions(auth);
  return <span>{sessions.length} active sessions</span>;
}
```

## Client Session Actions Example

```ts
"use client";

import { listSessions, revokeOtherSessions } from "@/auth/better-auth/client";

export async function refreshLaunchSessions() {
  const sessions = await listSessions();
  return sessions;
}

export async function signOutOtherLaunchDevices() {
  await revokeOtherSessions();
}
```

## Email Password Example

```ts
"use client";

import { signInDxBetterAuthEmail } from "@/auth/better-auth/email-password";

export async function signInFromLaunchForm(email: string, password: string) {
  return signInDxBetterAuthEmail({
    email,
    password,
    callbackURL: "/launch",
  });
}
```

## Social Sign-In Example

```ts
"use client";

import { signInDxBetterAuthSocial } from "@/auth/better-auth/social";

export async function signInWithGoogle() {
  return signInDxBetterAuthSocial({
    provider: "google",
    callbackURL: "/launch",
  });
}
```

## Linked Accounts Example

```ts
"use client";

import {
  linkDxBetterAuthSocialAccount,
  listDxBetterAuthAccounts,
} from "@/auth/better-auth/accounts";

export async function refreshLinkedAccounts() {
  return listDxBetterAuthAccounts();
}

export async function linkGoogleForLaunch() {
  return linkDxBetterAuthSocialAccount({
    provider: "google",
    callbackURL: "/launch",
  });
}
```

## Profile Example

```ts
"use client";

import {
  changeDxBetterAuthEmail,
  updateDxBetterAuthUserProfile,
} from "@/auth/better-auth/profile";

export async function updateLaunchProfile(name: string, newEmail?: string) {
  await updateDxBetterAuthUserProfile({ name });

  if (newEmail) {
    await changeDxBetterAuthEmail({
      newEmail,
      callbackURL: "/launch",
    });
  }
}
```

## Account Deletion Example

```ts
"use client";

import {
  deleteDxBetterAuthAccount,
  DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION,
} from "@/auth/better-auth/account-deletion";

export async function deleteLaunchAccount(password: string) {
  return deleteDxBetterAuthAccount({
    confirmation: DX_BETTER_AUTH_DELETE_ACCOUNT_CONFIRMATION,
    password,
    callbackURL: "/goodbye",
  });
}
```

## Account Security Example

```ts
"use client";

import { requestDxBetterAuthPasswordReset } from "@/auth/better-auth/account-security";

export async function requestLaunchPasswordReset(email: string) {
  return requestDxBetterAuthPasswordReset({
    email,
    redirectTo: "/reset-password",
  });
}
```

Keep `BETTER_AUTH_SECRET` outside source control, review trusted origins before production, and choose the database adapter in application code so the launch template can pair this slice with Drizzle, Turso, Postgres, or another upstream better-auth adapter. Applications still own profile field policy, email-change enablement, verification email delivery/callback URLs, delete-account email delivery, delete-token expiry, before/after deletion hooks, account export UX, and production auth policy.
