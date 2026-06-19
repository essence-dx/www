# DX Forge Package: `auth/better-auth`

- Variant: `default`
- Version: `1.6.11-dx.9`
- Upstream: `better-auth`
- Generator: `dx-forge/better-auth`
- License: `MIT`
- Provenance: `dx-forge-curated-registry` (verified: `no`)
- Advisory coverage: `curated-fixture` via `dx-forge-curated-advisory-fixture` (live: `no`, findings: `0`)
- License review: declared `MIT` (reviewed: `no`)
- Last action: `AddWrite`
- Risk score: `85`

This package is source-owned. The files below are editable project files, not opaque `node_modules` content. Forge tracks their hashes, treats local edits as reviewable yellow traffic, blocks red/security-sensitive traffic, and updates them through `dx update auth/better-auth`.

## Package Metadata Review

- Provenance note: DX inspected the local upstream better-auth source mirror and package export map for betterAuth(), createAuthClient(), Next.js handlers/cookies, adapters, and plugins; this is curated Authentication launch metadata, not SLSA or live upstream provenance.
- Advisory note: Curated DX Forge advisory fixture records no known advisory findings for this Authentication launch slice based on upstream better-auth, but it is not a live advisory feed and does not audit the deployed database adapter, session policy, or OAuth provider credentials.
- License review note: License is recorded from the curated package declaration only; no formal DX legal review is claimed.

## Google OAuth Contract

Forge owns the starter files for the OAuth handoff, but it does not own your production identity policy. Review and edit these values before deployment.

- Required env vars: `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`, `GOOGLE_REDIRECT_URI`.
- Optional env vars: `GOOGLE_OAUTH_SCOPES`, `DX_GOOGLE_STATE_COOKIE`, `DX_GOOGLE_ALLOWED_REDIRECT_ORIGIN`.
- Local callback example: `http://localhost:3000/auth/better-auth/callback`.
- Owned source files: `auth/better-auth/config.ts`, `auth/better-auth/route.ts`, `auth/better-auth/callback.ts`, `auth/better-auth/.env.example`, and `auth/better-auth/README.md`.
- Application-owned work: connect the callback token response to your session store, rotate secrets outside the repo, and set production redirect origins explicitly.

## Better Auth Contract

Forge owns the launch slice around Better Auth's public APIs, but it does not own your database adapter, production identity policy, or secret rotation.

- Required env vars: `BETTER_AUTH_SECRET` and `BETTER_AUTH_URL`.
- Optional env vars: `BETTER_AUTH_TRUSTED_ORIGINS`, `BETTER_AUTH_APP_NAME`, `BETTER_AUTH_EMAIL_PASSWORD_ENABLED`, `NEXT_PUBLIC_BETTER_AUTH_URL`, `GITHUB_CLIENT_ID`, `GITHUB_CLIENT_SECRET`, `GOOGLE_CLIENT_ID`, and `GOOGLE_CLIENT_SECRET`.
- Owned source files: `auth/better-auth/options.ts`, `auth/better-auth/server.ts`, `auth/better-auth/client.ts`, `auth/better-auth/route.ts`, `auth/better-auth/metadata.ts`, `auth/better-auth/.env.example`, and `auth/better-auth/README.md`.
- Application-owned work: provide the Better Auth database adapter, keep secrets outside the repo, review trusted origins, and mount the route helper from your framework route file.

## Materialized Files

| File | Logical Source | Bytes | Hash |
| --- | --- | ---: | --- |
| `auth/better-auth/options.ts` | `js/auth/better-auth/options.ts` | `2543` | `7c7b5ad9f2a2402d0d486d11dcd3fc4b30925fd5d9e38e3d14fe96f922760d1a` |
| `auth/better-auth/server.ts` | `js/auth/better-auth/server.ts` | `1216` | `34cd8ededca31b536f04155340bf611b65ebbcbed254a8ca7b8f23cf8b9048d6` |
| `auth/better-auth/client.ts` | `js/auth/better-auth/client.ts` | `1263` | `d2d8138c5157e44725fe9b4b38918f21699855a9a861ff78ac67ab318547b00e` |
| `auth/better-auth/email-password.ts` | `js/auth/better-auth/email-password.ts` | `1919` | `9b4330fac97ae61e3fa9b237d98101f3efc2a904788d52ad3134f435d5f56d74` |
| `auth/better-auth/social.ts` | `js/auth/better-auth/social.ts` | `2034` | `78482fb99089b01fdf5d7fcbacec1333ff58456bab8306bd5949f7f09a0a7931` |
| `auth/better-auth/accounts.ts` | `js/auth/better-auth/accounts.ts` | `2730` | `18bde753aa1093cc4482c1a14b98ce4c2195ea546a2ceee938b0be5c2fb41d36` |
| `auth/better-auth/profile.ts` | `js/auth/better-auth/profile.ts` | `2635` | `aedf2d7491c4b056e9563ccfd44b3a384ae4b5c27b52d72716b1ff64d28a4de7` |
| `auth/better-auth/account-deletion.ts` | `js/auth/better-auth/account-deletion.ts` | `1682` | `dad80da8c1eeec7767eea820f0098aa323bdf14e45fe84d3858001556949b1ba` |
| `auth/better-auth/account-security.ts` | `js/auth/better-auth/account-security.ts` | `2924` | `06a69a6102990146b775099171c4874fc2eb81848d3d8a951b67848f69da6341` |
| `auth/better-auth/route.ts` | `js/auth/better-auth/route.ts` | `280` | `8e4fa36ee94770e5f8db97d5581e545fc6510f6e578ff1eb4be0f3072c03a774` |
| `auth/better-auth/session.ts` | `js/auth/better-auth/session.ts` | `630` | `d58bcd1f5ef2fd1f83f05cf8fb7ace37a348429587652e07ab250f6427dca295` |
| `auth/better-auth/session-management.ts` | `js/auth/better-auth/session-management.ts` | `1236` | `d6bd6d50a4cc24c9cd35d389880cd1e0ae629b811861a9f71e81944cc49be722` |
| `auth/better-auth/dashboard.ts` | `js/auth/better-auth/dashboard.ts` | `6219` | `60dcd968350f80085323ea36be3a0ce69d7af5dbf05c8325bfbdf6f1a2c70f5f` |
| `auth/better-auth/metadata.ts` | `js/auth/better-auth/metadata.ts` | `5666` | `7060aa000c63c0ec0f3964516dce8c08afcb4a2fab1d094879d7b16f4c5d5939` |
| `auth/better-auth/.env.example` | `js/auth/better-auth/.env.example` | `430` | `968a6cc874a04a3b88eaddcdc54e7ab43b23810fe105bdc522f40cb4aa7962f2` |
| `auth/better-auth/README.md` | `js/auth/better-auth/README.md` | `5552` | `ade08a003db7fc4d16717d58c04e41ab22cd7e0bfffe91b34bb8215292565b16` |
| `auth/better-auth/providers/google/config.ts` | `js/auth/better-auth/providers/google/config.ts` | `3023` | `0fb43278598b8a764cb77314b2774f17a34c8a6b3ce90342120308d033e8eea5` |
| `auth/better-auth/providers/google/route.ts` | `js/auth/better-auth/providers/google/route.ts` | `654` | `967a804da380265c0fceca421eda9f6d63ce75ecf183f5d5f49d46317321579a` |
| `auth/better-auth/providers/google/callback.ts` | `js/auth/better-auth/providers/google/callback.ts` | `2228` | `649f4fa54b5c4640c3f8f336ba09126ada9def8e35206751a953172cb63a9ec4` |
| `auth/better-auth/providers/google/.env.example` | `js/auth/better-auth/providers/google/.env.example` | `340` | `9e6fd3f02b089ebbf44c1089e178fee660fd6fd74dda888374e28f2916ca0fba` |
| `auth/better-auth/providers/google/README.md` | `js/auth/better-auth/providers/google/README.md` | `429` | `535cda5f1f57abdd8a7c74cbac2f97ae84ed4e2f446a0ba1a4bd58c0c9efc2b6` |

## Forge Policy

| Traffic | Policy | Decision |
| --- | --- | --- |
| `green` | `no-lifecycle-execution` | Forge add does not run npm install, lifecycle hooks, or upstream scripts. |
| `yellow` | `package-provenance-recorded` | Forge records package provenance source `dx-forge-curated-registry` with external verification `no`. |
| `yellow` | `package-advisory-boundary` | Forge records advisory coverage `curated-fixture` from provider `dx-forge-curated-advisory-fixture` with live coverage `no` and finding count `0`. |
| `yellow` | `package-license-review-boundary` | Forge records declared license `MIT` with formal review `no`. |
| `green` | `auth-google-env-contract` | auth/better-auth materializes `.env.example` for GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET, GOOGLE_REDIRECT_URI, GOOGLE_OAUTH_SCOPES, DX_GOOGLE_STATE_COOKIE, and DX_GOOGLE_ALLOWED_REDIRECT_ORIGIN; secrets remain outside Forge receipts. |
| `green` | `auth-google-redirect-contract` | auth/better-auth documents the local callback URL `http://localhost:3000/auth/better-auth/callback` and keeps redirect/origin values editable in project source. |
| `green` | `auth-google-source-ownership` | Forge owns the starter config, start route, callback handler, env example, and README files; the application owns session storage, secret rotation, and production redirect policy. |
| `green` | `better-auth-database-boundary` | auth/better-auth materializes a Better Auth server factory that requires the application to pass a real database adapter; Forge does not hide session storage behind a dummy in-memory default. |
| `green` | `better-auth-next-handler-contract` | auth/better-auth uses Better Auth's public `toNextJsHandler` and `nextCookies` APIs for route-handler integration and keeps the app-owned auth instance explicit. |
| `green` | `better-auth-discovery-metadata` | auth/better-auth includes `metadata.ts` so DX CLI, Zed, and launch templates can discover package id, upstream package, required dependency, and primary helper names without scanning generated source text. |
