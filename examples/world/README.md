# DX WWW World Example

`examples/world` is the template-level integration lab for connecting WWW to
the wider production stack without hardcoding providers into the Rust framework.

It maps the first three verification targets from each `WORLD.md` category into
typed contracts, env requirements, receipt expectations, and route-level status
surfaces. It also includes a TypeScript live-connection runner that performs
safe read-only probes when credentials are present.

## Routes

- `/` shows the world integration cockpit.
- `/integrations` shows all provider cards grouped by category.
- `/readiness` shows env and receipt readiness.
- `/api/world/status` returns redacted machine-readable status.
- `/api/world/provider` returns a redacted provider-readiness contract.
- `/api/world/firebase-crud` exposes the guarded Firebase Firestore CRUD route contract.
- `/api/world/live` returns the source-owned live runner and receipt contract.
- `/api/world/neon-crud` exposes the guarded Neon database CRUD route contract.
- `/api/world/supabase-crud` exposes the guarded Supabase Storage CRUD route contract.
- `/api/world/turso-crud` exposes the guarded Turso/libSQL database CRUD route contract.

## Operating Model

- Provider values are never hardcoded in source.
- Env names, scopes, and capabilities are declared in TypeScript contracts.
- `.env` values should be managed through DX Env Firewall.
- Live provider checks run through explicit read-only adapters.
- Framework-level ideas discovered here go into root `PLAN.md` first.

## Live Connection Runner

Run from the repository root:

```powershell
bun test benchmarks\examples-world-live-connections.test.ts
```

The runner lives at:

```text
examples/world/lib/world/connections/runner.ts
```

It writes a generated, ignored receipt:

```text
examples/world/.dx/receipts/world/live-connections.json
```

Current probes are safe by default:

- They inspect env presence, not raw values.
- They use read-only HTTP endpoints or local CLI identity checks.
- They never create accounts, deploy apps, send messages, create payments, or mutate provider state.
- They refuse to write a receipt if a raw provider env value appears in the output.

The Vercel CLI probe can validate the local authenticated account even when
`VERCEL_TOKEN` is absent. Provider HTTP probes validate only when the relevant
Env Firewall keys are present.

## Turso CRUD Proof

Turso is wired through the libSQL HTTP pipeline, matching the existing safe
`SELECT 1` provider probe without installing SDK packages.

Required server env:

```text
TURSO_DATABASE_URL
TURSO_AUTH_TOKEN
```

Optional provider metadata env:

```text
TURSO_ORGANIZATION
TURSO_DATABASE_NAME
```

Focused proof:

```powershell
bun test benchmarks\examples-world-turso-crud.test.ts
```

Live mutation is opt-in. The CRUD adapter creates a temporary table, inserts a
row, reads it, updates it, reads it again, deletes it, and drops the temporary
table through `/v2/pipeline`. The receipt never includes the auth token.

The route contract is guarded:

```text
POST /api/world/turso-crud
DX_WORLD_ALLOW_TURSO_CRUD=1
x-dx-world-confirm: turso-database-crud
```

## Firebase CRUD Proof

Firebase is wired through Firestore REST so the example does not require
`firebase`, `firebase-admin`, `node_modules`, or bundled SDK code.

Required app config env:

```text
FIREBASE_PROJECT_ID
FIREBASE_API_KEY
```

Optional config and auth env:

```text
FIREBASE_AUTH_DOMAIN
FIREBASE_STORAGE_BUCKET
FIREBASE_MESSAGING_SENDER_ID
FIREBASE_APP_ID
FIREBASE_MEASUREMENT_ID
FIREBASE_AUTH_ID_TOKEN
GOOGLE_APPLICATION_CREDENTIALS
```

Focused proof:

```powershell
bun test benchmarks\examples-world-firebase-crud.test.ts
```

Live mutation is opt-in. The CRUD adapter creates a temporary document, reads it,
updates it, and deletes it through Firestore REST. If Firestore rules reject the
public config, the receipt stays `blocked` and does not pretend CRUD succeeded.

The route contract is guarded:

```text
POST /api/world/firebase-crud
DX_WORLD_ALLOW_FIREBASE_CRUD=1
x-dx-world-confirm: firebase-firestore-crud
```

The current supplied Firebase Web config reaches Firestore REST, but the project
returns HTTP 403 for unauthenticated document CRUD. That is a secure default.
Live CRUD needs either security rules that allow the test path, a Firebase Auth
ID token, or a future server-owned Admin/service-account adapter.

## Neon CRUD Proof

Neon is wired through the official HTTP SQL shape used by Neon serverless
clients, but the example does not install or bundle a driver package.

Accepted server env:

```text
NEON_DATABASE_URL
DATABASE_URL
```

Optional provider metadata env:

```text
NEON_API_KEY
NEON_PROJECT_ID
```

Focused proof:

```powershell
bun test benchmarks\examples-world-neon-crud.test.ts
```

Live mutation is opt-in. The CRUD adapter creates a temporary table, inserts a
row, reads it, updates it, reads it again, deletes it, and drops the temporary
table. The receipt never includes the connection string or password.

The route contract is guarded:

```text
POST /api/world/neon-crud
DX_WORLD_ALLOW_NEON_CRUD=1
x-dx-world-confirm: neon-database-crud
```

## Supabase CRUD Proof

Supabase is wired through source-owned TypeScript adapters, not hardcoded Rust
framework behavior and not committed secrets.

Required server env:

```text
SUPABASE_URL
SUPABASE_SECRET_KEY
```

Optional compatibility env:

```text
NEXT_PUBLIC_SUPABASE_URL
NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY
SUPABASE_SERVICE_ROLE_KEY
```

Focused proof:

```powershell
bun test benchmarks\examples-world-supabase-crud.test.ts
```

Live mutation is intentionally separate from the default read-only runner. The
CRUD adapter creates a temporary private bucket, creates an object, reads it,
updates it, deletes the object, and deletes the bucket. The receipt includes
only status codes, env names, bucket/object labels, and redacted metadata.

The route contract is guarded:

```text
POST /api/world/supabase-crud
DX_WORLD_ALLOW_SUPABASE_CRUD=1
x-dx-world-confirm: supabase-storage-crud
```

Table CRUD is not claimed from project API keys alone. Database table CRUD needs
an existing exposed table plus RLS policy, a database connection string, or a
provider/migration token that can create the schema safely.

## Current Proof

- Covers the top three targets from every `WORLD.md` category.
- Includes real live probes for Vercel CLI/API, Turso SQL-over-HTTP, OpenAI,
  Anthropic, Gemini, Stripe, GitHub, Linear, Notion, Cloudflare, Resend, and
  configured-readiness probes for providers that need Forge adapters.
- Includes a guarded Firebase Firestore CRUD adapter and a safe read-only
  Firestore document probe. Current live Firebase config is reachable but
  blocked by Firestore rules for unauthenticated CRUD.
- Includes a guarded Neon database CRUD adapter and a safe read-only Neon HTTP
  SQL `SELECT 1` probe.
- Includes a guarded Supabase Storage CRUD adapter and a safe read-only
  Supabase Storage bucket-access probe.
- Includes a guarded Turso/libSQL database CRUD adapter using the HTTP pipeline.
- Builds as a normal WWW app with no `node_modules` requirement.
- Emits source-owned route-handler receipts for `/api/world/status`, `/api/world/provider`, and `/api/world/live`.
- Keeps API responses redacted and separates catalog readiness from live runner receipts.
- `dx check` is green; missing Forge manifest/package-lock notes are expected because this example does not install provider packages yet.

## Live Provider Boundary

This example does not create provider accounts, mutate remote resources, or claim
live validation without credentials. A provider becomes live only when DX Env
Firewall has the required keys and a replayable provider receipt exists.

WWW route handlers currently expose the live runner contract with inline JSON.
Direct route-handler execution of imported TypeScript helpers, server-only env
injection, bounded fetch, and provider receipt import are tracked in root
`PLAN.md` before they are promoted into framework-level behavior.
