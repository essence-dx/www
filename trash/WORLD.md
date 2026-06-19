# DX WWW World Connection Map

Created: 2026-06-02

This file lists the outside services, protocols, and validation gates that WWW
and Forge should support so a DX WWW app can connect to the whole production web
stack without losing the DX identity: source-owned files, explicit adapter
boundaries, safe secrets, receipts, `.sr` serializer artifacts, generated
`.machine` contracts, and clear proof.

The goal is not to hardcode every service into templates. The goal is to make
Forge packages and WWW adapters powerful enough that users can connect the tools
they already use, while WWW keeps the richest first-class experience.

## Top Verification Targets

The world map below is intentionally broad, but the first launch does not need
handwritten support for every provider. WWW needs a strong adapter contract,
Forge package lifecycle, typed env/secrets, receipts, and replayable validation.
After that, the ecosystem can grow through official integrations, verified Forge
packages, and community-maintained adapters without turning templates into a
hardcoded provider catalog.

These are the first three targets per category that WWW creators should verify
directly. They represent the provider shape for that category, so later
integrations can plug into the same contracts.

| Category | First Three Targets To Verify | Why These First |
| --- | --- | --- |
| Database and data platform | PostgreSQL, Neon, Turso/libSQL | Covers standard SQL, serverless Postgres, and local-first/serverless SQLite. |
| ORM, query builder, and schema tooling | DX ORM / Forge database, Drizzle, Prisma | Covers the native DX path plus the two ecosystems developers will compare first. |
| Authentication and identity | DX Auth / Forge auth, Better Auth, Clerk | Covers native auth, source-owned auth, and popular hosted identity. |
| Authorization and policy | PostgreSQL RLS, OpenFGA, Casbin | Covers database policy, relationship-based authorization, and embedded policy checks. |
| Payments, billing, and commerce | Stripe, Lemon Squeezy, Paddle | Covers global payments, indie software commerce, and merchant-of-record billing. |
| Object storage, uploads, media, and CDN | AWS S3, Cloudflare R2, Vercel Blob | Covers the industry baseline, low-egress object storage, and deploy-platform storage. |
| Search and discovery | Meilisearch, Typesense, Algolia | Covers self-hosted search, fast open-source search, and hosted commercial search. |
| Vector search and AI data | pgvector, Pinecone, MongoDB Atlas Vector Search | Covers Postgres-native vectors, dedicated vector infrastructure, and document-vector hybrid search. |
| AI, LLM, agents, and structured output | OpenAI, Anthropic, Google Gemini | Covers the models and SDK behavior most teams will evaluate first. |
| Realtime, collaboration, and presence | WebSocket/SSE, Supabase Realtime, Ably | Covers native web protocols, database-driven realtime, and hosted realtime infrastructure. |
| Queues, jobs, cron, and workflows | Cloudflare Queues, Upstash QStash, Temporal | Covers edge queues, HTTP task delivery, and durable workflow orchestration. |
| Cache, config, feature flags, and rate limits | Redis/Valkey, Upstash Redis, Cloudflare KV | Covers local/server cache, serverless Redis, and edge key-value config. |
| Analytics, product data, and experimentation | PostHog, Plausible, Vercel Analytics | Covers product analytics, privacy-friendly analytics, and deployment-native web vitals. |
| Observability, logs, errors, and traces | OpenTelemetry, Sentry, Datadog | Covers the open standard, application errors, and enterprise observability. |
| CMS, content, docs, and marketing sites | Content collections/MDX, Sanity, Strapi | Covers source-owned content, hosted structured content, and self-hosted CMS. |
| Email, SMS, push, and notifications | Resend, Twilio, Firebase Cloud Messaging | Covers transactional email, SMS/phone messaging, and push notifications. |
| Deployment, hosting, edge, and cloud | Vercel, Cloudflare, Fly.io | Covers frontend deployment, edge runtime, and portable server deployment. |
| Security, secrets, compliance, and supply chain | DX Env Firewall, GitHub Advanced Security, Sigstore | Covers native secret safety, code scanning, and signed provenance. |
| Developer platforms and team tools | GitHub, Linear, Notion | Covers code hosting, issue/project operations, and workspace documentation. |
| Internationalization, localization, maps, and forms | FormatJS/Intl, Google Maps, React Hook Form shape | Covers browser-native i18n, maps/geocoding, and form validation expectations. |

### Integration Ownership Model

- **Official WWW integrations:** the first three targets in each category should
  get first-party adapter contracts, docs, validation receipts, and example
  projects.
- **Verified Forge packages:** community or partner packages can become verified
  when they pass connection, schema, security, runtime, and receipt gates.
- **Community adapters:** the long tail should be easy to add through stable
  adapter traits/contracts, generated `.machine` metadata, serializer-backed
  `.sr` state, and replayable proof commands.
- **No template hardcoding:** templates should expose capabilities and examples,
  not provider lock-in. A user should be able to start with WWW and connect to
  their existing stack without rewriting the framework.
- **WWW as the strongest host:** DX Style, DX Icons, Forge, receipts, `dx check`,
  env safety, and adapter validation should work outside WWW too, but WWW should
  remain the place where the whole DX ecosystem feels native.

### DX Env Firewall

WWW should make env security native. Developers can keep a familiar `.env`
workflow, but `.env` is a temporary sealed viewport rather than the durable
source of truth. Local values are stored in `.dx/env/local.sr`, encrypted with a
password-derived key, compiled to `.dx/env/local.machine`, and exposed through a
generated typed contract at `.dx/env/env.d.ts`.

The operating model is:

- `dx env lock --password-env DX_ENV_PASSWORD` reads the current `.env` viewport, encrypts
  values into `.dx/env/local.sr`, writes `.dx/env/local.machine`, generates
  `.dx/env/env.d.ts`, writes a redacted receipt, and reseals `.env`.
- `dx env open --password-env DX_ENV_PASSWORD --ttl-seconds 180` materializes an editable
  `.env` view for a short time.
- `dx env reconcile --password-env DX_ENV_PASSWORD` reseals an expired unlocked
  viewport after saving validated changes.
- `dx env check --json` proves key names, scopes, capabilities, machine
  contract freshness, and redaction status without printing values.
- `dx env agent-context --json` gives AI workers the env shape, missing/current
  status, and safe capability map without revealing secrets.

## Core Principle

Every world integration should prove these things:

- **Connection:** the app can connect using safe env/secrets, TLS, region, pool,
  and runtime boundaries.
- **Schema:** database tables, auth tables, queues, buckets, indexes, and
  provider resources are represented in source-owned contracts.
- **Migration:** setup, migration, rollback, drift detection, and seed data are
  explicit.
- **Runtime:** dev, build, preview, production, edge, server, and browser
  boundaries are clear.
- **Security:** secrets are not hardcoded, webhooks are verified, auth/RLS/RBAC
  rules are explicit, and PII boundaries are documented.
- **Proof:** `.dx/receipts/**`, `.sr`, `.machine`, source manifests, replay
  commands, and provider evidence describe what was actually validated.
- **No fake support:** if live credentials or provider access are missing, the
  integration is `preview-only / not writable / not live-validated`.

## Priority Lanes

### 1. Database And Data Platform

WWW/Forge should support direct database connection, schema generation,
migration receipts, connection-pool receipts, health checks, and query
diagnostics.

**Relational databases**

- PostgreSQL
- MySQL
- MariaDB
- SQLite
- libSQL
- SQL Server
- CockroachDB
- SingleStore
- TiDB
- Oracle Database

**Postgres platforms**

- Neon
- Supabase
- AWS RDS PostgreSQL
- AWS Aurora PostgreSQL
- Google Cloud SQL PostgreSQL
- Azure Database for PostgreSQL
- Railway Postgres
- Render Postgres
- Aiven Postgres
- Crunchy Bridge
- Tembo
- Timescale
- Xata
- Nile

**MySQL and compatible platforms**

- PlanetScale
- AWS RDS MySQL
- AWS Aurora MySQL
- Google Cloud SQL MySQL
- Azure Database for MySQL
- Railway MySQL
- Aiven MySQL
- MariaDB Cloud
- SingleStore
- TiDB Cloud

**SQLite and local-first platforms**

- SQLite
- Turso
- Cloudflare D1
- libSQL
- PGlite
- LiteFS
- rqlite
- Fly.io LiteFS deployments

**Document and NoSQL databases**

- MongoDB Atlas
- Firebase Firestore
- Firebase Realtime Database
- AWS DynamoDB
- Couchbase
- Fauna
- Appwrite Database
- Convex
- SurrealDB
- RedisJSON

**Validation gates**

- `DATABASE_URL` / provider-specific env validation.
- Direct vs pooled connection mode.
- Serverless-safe connection policy.
- TLS/SSL mode validation.
- Region and latency receipt.
- Migration dry-run and apply receipts.
- Schema drift receipt.
- Seed and fixture receipt.
- Rollback plan.
- RLS/RBAC policy receipt.
- Query smoke test.
- Transaction test.
- Prepared statement compatibility.
- Edge-runtime compatibility.
- Dev/prod environment separation.
- Read replica / branching / sync mode validation.
- Backup and restore note.

### 2. ORM, Query Builder, And Schema Tooling

WWW has its own Drizzle-inspired data direction, but Forge should connect to the
tools developers already trust. The framework should validate schema ownership,
types, migrations, generated files, and runtime boundaries.

**ORMs and query builders**

- DX ORM / Forge database package
- Drizzle ORM
- Prisma
- Kysely
- TypeORM
- Sequelize
- MikroORM
- Knex
- SQLx
- SeaORM
- Diesel
- Ent
- Slonik
- Postgres.js
- node-postgres
- mysql2
- better-sqlite3
- libSQL client

**Validation gates**

- Schema source file detected.
- Generated type output path.
- Migration folder detected.
- Migration dry-run.
- Runtime driver boundary.
- Edge/server/browser import boundary.
- SQL injection safety notes.
- Prepared statement support.
- Transaction support.
- Connection cleanup.
- Read/write query receipts.
- Model-to-table mapping receipt.
- Forge package provenance.

### 3. Authentication And Identity

Forge already has better-auth-inspired authentication. WWW should make auth a
first-class framework capability with secure sessions, cookies, CSRF, OAuth,
passkeys, organizations, roles, permissions, and provider webhooks.

**Auth frameworks and providers**

- DX Auth / Forge auth package
- Better Auth
- Auth.js / NextAuth
- Clerk
- Auth0
- Supabase Auth
- Firebase Auth
- WorkOS
- Stytch
- Descope
- Logto
- FusionAuth
- Ory
- Keycloak
- Amazon Cognito
- Zitadel
- Okta
- Microsoft Entra ID

**OAuth and social providers**

- Google
- GitHub
- Microsoft
- Apple
- Discord
- Slack
- X / Twitter
- LinkedIn
- GitLab
- Bitbucket
- Notion
- Linear
- Twitch
- Spotify

**Enterprise identity**

- OIDC
- OAuth 2.0
- SAML
- SCIM
- LDAP bridge
- SSO
- MFA
- Passkeys / WebAuthn
- Magic links
- TOTP
- Backup codes
- Device sessions

**Validation gates**

- Session storage schema.
- Cookie flags: `HttpOnly`, `Secure`, `SameSite`.
- CSRF protection.
- OAuth callback URL receipt.
- Redirect allowlist.
- Provider env validation.
- Webhook signature verification.
- User/session/account/verification table receipt.
- Password reset flow.
- Email verification flow.
- Passkey registration and login flow.
- Organization/team membership.
- Role and permission mapping.
- RLS claim mapping for database providers.
- Logout and session revocation.
- Rate limit and abuse protection.
- Audit log receipt.

### 4. Authorization And Policy

Auth is not enough. WWW should make permissions explicit and source-owned.

**Authorization systems**

- DX policy package
- Supabase RLS
- PostgreSQL RLS
- OpenFGA
- AuthZed / SpiceDB
- Oso
- Permit.io
- Cerbos
- Casbin
- Keycloak authorization
- WorkOS RBAC/FGA

**Validation gates**

- RBAC model.
- ABAC model.
- Organization and team scopes.
- Resource ownership.
- Policy fixtures.
- Deny-by-default proof.
- Route-handler authorization check.
- Server-action authorization check.
- Database RLS mapping.
- Admin override audit trail.
- Permission matrix receipt.

### 5. Payments, Billing, And Commerce

WWW should support products, subscriptions, checkout, invoices, usage, seats,
tax, refunds, entitlement sync, and verified webhooks.

**Payments and billing**

- Stripe
- Paddle
- Lemon Squeezy
- Polar
- PayPal
- Braintree
- Adyen
- Chargebee
- Recurly
- RevenueCat
- Square
- Razorpay
- Mollie

**Commerce**

- Shopify
- Medusa
- Saleor
- WooCommerce
- BigCommerce
- Stripe products/prices

**Validation gates**

- Checkout session route.
- Customer mapping.
- Product/price mapping.
- Subscription status mapping.
- Entitlement receipt.
- Webhook signature verification.
- Idempotency key handling.
- Event replay safety.
- Refund/cancel/update flows.
- Usage-based billing counters.
- Tax/VAT notes.
- Portal/session links.
- Server-only secret boundary.
- Test-mode vs live-mode receipt.

### 6. Object Storage, Uploads, Media, And CDN

WWW should support user uploads, private/public assets, image transforms,
presigned URLs, multipart uploads, CDN cache headers, and receipts.

**Object storage**

- AWS S3
- Cloudflare R2
- Vercel Blob
- Supabase Storage
- Firebase Storage
- Google Cloud Storage
- Azure Blob Storage
- Backblaze B2
- DigitalOcean Spaces
- MinIO
- Wasabi

**Upload and media platforms**

- UploadThing
- Cloudinary
- ImageKit
- Imgix
- Mux
- Bunny Storage/CDN
- Filestack
- Transloadit
- S3 multipart uploads

**Validation gates**

- Bucket/container env validation.
- Public vs private asset mode.
- Presigned upload URL.
- Presigned download URL.
- Multipart upload support.
- MIME and extension validation.
- Image metadata receipt.
- Size limits.
- Cache-control receipt.
- CDN purge/invalidation command.
- Signed URL expiration.
- Virus/malware scan hook.
- User ownership policy.

### 7. Search And Discovery

WWW should support site search, app search, semantic search, indexing receipts,
and stale-index detection.

**Search engines**

- Algolia
- Typesense
- Meilisearch
- Elasticsearch
- OpenSearch
- Vespa
- Solr
- ZincSearch
- Sonic

**Validation gates**

- Index schema.
- Source-to-index mapping.
- Index write receipt.
- Stale index check.
- Facet/filter proof.
- Typo tolerance proof.
- Permission-filtered search.
- Search key scope.
- Reindex command.

### 8. Vector Search And AI Data

WWW should make RAG, embeddings, memory, and vector indexes easy without hiding
which provider owns data.

**Vector databases and vector-capable stores**

- pgvector
- Supabase Vector
- Neon Postgres with pgvector
- MongoDB Vector Search
- Pinecone
- Weaviate
- Qdrant
- Milvus
- Zilliz
- Chroma
- Upstash Vector
- Redis Vector
- Elasticsearch vector search
- OpenSearch vector search
- LanceDB
- Turso AI/embeddings paths

**Validation gates**

- Embedding model mapping.
- Vector dimension check.
- Index creation receipt.
- Metadata filter support.
- Hybrid search support.
- Similarity query smoke.
- RAG source citation receipt.
- Delete/update vector sync.
- PII/vector retention notes.
- Provider cost/rate-limit notes.

### 9. AI, LLM, Agents, And Structured Output

WWW should support AI as a source-owned runtime capability: streaming,
structured output, tools, eval receipts, vector memory, and provider adapters.

**AI providers and SDKs**

- OpenAI
- Anthropic
- Google Gemini
- Mistral
- Cohere
- Groq
- Together AI
- Perplexity
- DeepSeek
- xAI
- Replicate
- Hugging Face
- Ollama
- LM Studio
- Vercel AI SDK
- LangChain
- LlamaIndex

**Validation gates**

- Provider env validation.
- Model catalog receipt.
- Streaming response proof.
- Tool/function calling proof.
- Structured output schema.
- JSON repair/failure behavior.
- Token/cost tracking.
- Rate-limit retry policy.
- Safety filter receipt.
- Prompt/source provenance.
- Eval command.
- RAG citation receipt.
- Secret redaction.

### 10. Realtime, Collaboration, And Presence

WWW should support live apps without making realtime a template hack.

**Realtime systems**

- WebSocket
- Server-Sent Events
- WebRTC data channels
- Supabase Realtime
- Firebase Realtime Database
- Firestore listeners
- Ably
- Pusher
- Liveblocks
- PartyKit
- Socket.io
- Convex realtime
- Yjs
- Automerge
- Cloudflare Durable Objects
- NATS

**Validation gates**

- Channel naming.
- Authenticated subscription.
- Presence proof.
- Broadcast proof.
- Replay/resume behavior.
- Backpressure handling.
- Disconnect/reconnect behavior.
- Multi-tab behavior.
- Collaboration conflict policy.
- Server-only publish boundary.

### 11. Queues, Jobs, Cron, And Workflows

WWW should validate async work as a first-class production surface.

**Queues and schedulers**

- Upstash QStash
- Upstash Workflow
- Upstash Kafka
- Cloudflare Queues
- Cloudflare Workflows
- AWS SQS
- AWS SNS
- AWS EventBridge
- AWS Step Functions
- Google Pub/Sub
- Google Cloud Tasks
- Azure Service Bus
- Inngest
- Trigger.dev
- Temporal
- BullMQ
- RabbitMQ
- Kafka
- Confluent
- Redpanda
- NATS
- Celery
- Sidekiq

**Validation gates**

- Queue/topic env validation.
- Producer route.
- Consumer handler.
- Retry policy.
- Dead-letter queue.
- Idempotency key.
- Cron schedule receipt.
- Delayed job proof.
- Workflow step receipt.
- Local replay command.
- Provider replay command.

### 12. Cache, Config, Feature Flags, And Rate Limits

WWW should know the difference between cache, config, feature flags, and durable
state.

**Cache and key-value**

- Redis
- Valkey
- Upstash Redis
- Dragonfly
- KeyDB
- Cloudflare KV
- Vercel KV / Upstash
- Momento
- DynamoDB cache patterns
- Memcached

**Feature flags and config**

- Vercel Edge Config
- LaunchDarkly
- Statsig
- Unleash
- GrowthBook
- PostHog Feature Flags
- ConfigCat
- Split
- Cloudflare Workers bindings/secrets

**Validation gates**

- Cache key namespace.
- TTL proof.
- Invalidation receipt.
- Stale-while-revalidate behavior.
- Rate-limit policy.
- Feature flag default.
- Flag targeting proof.
- Edge/runtime compatibility.
- Config drift receipt.

### 13. Analytics, Product Data, And Experimentation

WWW should support analytics without shipping random scripts blindly.

**Analytics**

- PostHog
- Plausible
- Fathom
- Google Analytics
- Mixpanel
- Amplitude
- Segment
- RudderStack
- Heap
- Matomo
- Vercel Analytics

**Validation gates**

- Script injection policy.
- Consent mode.
- Server-side event route.
- Client event binding.
- Privacy/redaction policy.
- Experiment assignment receipt.
- UTM/campaign capture.
- No-JS fallback.
- Production-only or dev-only mode.

### 14. Observability, Logs, Errors, And Traces

WWW should prove errors, traces, logs, replay, source maps, and release metadata
cleanly.

**Observability platforms**

- Sentry
- PostHog
- OpenTelemetry
- Datadog
- New Relic
- Grafana
- Prometheus
- Honeycomb
- Axiom
- Better Stack
- Logtail
- Logflare
- Highlight
- LogRocket
- Vercel Observability
- SigNoz

**Validation gates**

- Release name.
- Environment name.
- Source map upload.
- Error capture.
- Performance span.
- Route handler span.
- Server action span.
- Log drain.
- PII scrubbing.
- Session replay privacy.
- Health check endpoint.
- Alert receipt.

### 15. CMS, Content, Docs, And Marketing Sites

WWW should support content-heavy sites and plain HTML while keeping source-owned
styles/icons/checks.

**CMS and content**

- Sanity
- Contentful
- Strapi
- Payload
- Directus
- Hygraph
- Storyblok
- Prismic
- WordPress
- Ghost
- DatoCMS
- TinaCMS
- Markdown
- MDX
- Plain HTML

**Validation gates**

- Content source config.
- Type/schema generation.
- Draft/preview route.
- Revalidation behavior.
- Image asset mapping.
- SEO metadata.
- Sitemap.
- RSS.
- Content freshness receipt.
- Plain HTML style/icon scan.

### 16. Email, SMS, Push, And Notifications

WWW should connect communication channels with strict secrets and verified
webhooks.

**Email**

- Resend
- SendGrid
- Postmark
- Mailgun
- AWS SES
- Brevo
- Loops
- Customer.io
- ConvertKit
- React Email

**SMS, voice, push, notifications**

- Twilio
- Vonage
- Plivo
- Firebase Cloud Messaging
- Expo Push
- OneSignal
- Novu
- Knock
- Courier

**Validation gates**

- Sender/domain verification.
- Template source path.
- Preview render.
- Delivery webhook signature.
- Bounce/complaint handling.
- Unsubscribe handling.
- SMS opt-in policy.
- Push token storage.
- Notification preference schema.

### 17. Deployment, Hosting, Edge, And Cloud

WWW should run anywhere, but adapters must prove what each platform supports.

**Hosting and edge**

- Vercel
- Cloudflare Workers/Pages
- Netlify
- Fly.io
- Railway
- Render
- AWS
- Google Cloud
- Azure
- DigitalOcean
- Heroku
- Koyeb
- Deno Deploy
- Bun Deploy
- Fastly Compute
- Akamai
- Docker
- Kubernetes
- Nginx
- Caddy

**Validation gates**

- Build output contract.
- Static/no-JS output.
- Server handler output.
- Edge/server runtime mapping.
- Environment variable mapping.
- Route handler support.
- WebSocket/SSE support.
- Cache header support.
- Asset upload.
- Preview URL.
- Hosted smoke test.
- Provider receipt.
- Rollback command.

### 18. Security, Secrets, Compliance, And Supply Chain

WWW/Forge must make secure defaults feel normal.

**Security tools**

- GitHub Advanced Security
- Snyk
- Socket
- Semgrep
- Trivy
- Dependabot
- OSSF Scorecard
- Doppler
- Infisical
- 1Password
- HashiCorp Vault
- AWS Secrets Manager
- Google Secret Manager
- Azure Key Vault
- Cloudflare Secrets Store

**Validation gates**

- Secret source map.
- Secret redaction.
- Dependency provenance.
- License receipt.
- Advisory receipt.
- CSP policy.
- CORS policy.
- CSRF policy.
- Rate limit.
- Webhook signature checks.
- Audit log.
- SAST/SCA receipt.

### 19. Developer Platforms And Team Tools

Forge packages should connect to developer workflows and business apps.

**Developer and team APIs**

- GitHub
- GitLab
- Bitbucket
- Linear
- Jira
- Notion
- Slack
- Discord
- Trello
- Asana
- Airtable
- Google Workspace
- Microsoft Graph
- Calendly
- Zoom
- Figma
- Vercel
- Cloudflare

**Validation gates**

- OAuth app config.
- Webhook signature.
- Event subscription.
- Rate limits.
- Pagination.
- Sync cursor.
- Conflict handling.
- Audit receipt.
- Scoped token storage.

### 20. Internationalization, Localization, Maps, And Forms

WWW should support real global apps, not just English dashboards.

**i18n and localization**

- FormatJS
- Lingui
- i18next
- next-intl-inspired patterns
- Crowdin
- Lokalise
- Phrase
- PO files
- ICU messages

**Maps and geospatial**

- Mapbox
- Google Maps
- MapTiler
- HERE
- OpenStreetMap
- PostGIS

**Forms and validation**

- DX Forms package
- Zod
- Valibot
- ArkType
- Yup
- React Hook Form compatibility
- Conform
- Vest

**Validation gates**

- Locale route support.
- Message extraction.
- Missing translation check.
- RTL check.
- Timezone/currency formatting.
- Geocoding key boundary.
- Form schema receipt.
- Server validation receipt.
- Error message mapping.

## Forge Package Contract For World Integrations

Every Forge world package should declare:

- `package_id`
- `category`
- `provider`
- `source_files`
- `generated_files`
- `env_vars`
- `secrets`
- `runtime_boundary`
- `browser_boundary`
- `server_boundary`
- `edge_boundary`
- `dev_support`
- `build_support`
- `preview_support`
- `production_support`
- `live_validation_required`
- `local_validation_available`
- `receipt_paths`
- `serializer_receipt_paths`
- `machine_contract_paths`
- `rollback_plan`
- `docs_path`

## Receipt Types We Need

- `dx.forge.world.connection`
- `dx.forge.world.schema`
- `dx.forge.world.migration`
- `dx.forge.world.auth`
- `dx.forge.world.webhook`
- `dx.forge.world.storage`
- `dx.forge.world.queue`
- `dx.forge.world.search`
- `dx.forge.world.vector`
- `dx.forge.world.ai`
- `dx.forge.world.observability`
- `dx.forge.world.deploy`
- `dx.forge.world.security`
- `dx.forge.world.provider-live-proof`
- `dx.forge.world.preview-only`

## First Launch Priority

The first serious world-connection pass should prioritize:

1. **Postgres:** Neon, Supabase, generic Postgres, AWS/Railway/Render/Aiven.
2. **SQLite/libSQL:** Turso, Cloudflare D1, local SQLite.
3. **Auth:** DX Auth, Better Auth, Clerk, Auth.js, Supabase Auth, Auth0.
4. **ORM:** DX ORM, Drizzle, Prisma, Kysely.
5. **Payments:** Stripe, Lemon Squeezy, Polar, Paddle.
6. **Storage:** S3, R2, Vercel Blob, Supabase Storage, UploadThing.
7. **Cache/queue:** Redis, Upstash Redis, Upstash QStash, Cloudflare Queues.
8. **Observability:** Sentry, OpenTelemetry, PostHog, Axiom.
9. **AI/vector:** OpenAI, Anthropic, Vercel AI SDK, pgvector, Pinecone,
   Qdrant, MongoDB Vector Search, Upstash Vector.
10. **Deploy:** Vercel, Cloudflare, Netlify, Fly.io, Railway, Render.

This covers the majority of modern SaaS, AI app, dashboard, content, commerce,
and internal-tool projects without forcing one provider stack on users.

## Source Notes

The current list was refreshed against public docs and ecosystem signals on
2026-06-02. Exact provider APIs, pricing, limits, and popular rankings must be
refreshed before release-facing claims.

- Neon documents standard Postgres connection strings, direct and pooled
  connections, and language/framework guides:
  <https://neon.com/docs/get-started/connect-neon>
- Turso documents libSQL embedded replicas, local reads, cloud-primary writes,
  sync, and production considerations:
  <https://docs.turso.tech/features/embedded-replicas/introduction>
- Supabase documents database functions, edge functions, and Postgres RLS:
  <https://supabase.com/docs/guides/database/functions>
  <https://supabase.com/docs/guides/database/postgres/row-level-security>
- Better Auth documents database adapters, auth tables, migrations, plugin
  schemas, TypeScript, and passkeys:
  <https://better-auth.com/docs/concepts/database>
- Drizzle documents dialect-specific, serverless-ready SQL support and major
  database drivers:
  <https://orm.drizzle.team/docs/overview>
  <https://orm.drizzle.team/docs/connect-overview>
- Prisma documents supported self-hosted and managed/serverless databases:
  <https://docs.prisma.io/docs/orm/core-concepts/supported-databases>
- Cloudflare documents Workers bindings for D1, Durable Objects, KV, Queues,
  R2, Vectorize, secrets, and workflows:
  <https://developers.cloudflare.com/workers/configuration/bindings/>
- Vercel documents integrations, Blob, Edge Config, and marketplace connection
  patterns:
  <https://vercel.com/docs/integrations>
  <https://vercel.com/docs/storage/vercel-blob/using-blob-sdk>
  <https://vercel.com/docs/edge-config/>
- Upstash documents serverless Redis, Vector, QStash, Workflow, global
  replication, and REST access:
  <https://upstash.com/docs>
- Stripe documents subscriptions and webhook-driven billing flows:
  <https://docs.stripe.com/billing/subscriptions/webhooks>
- MongoDB documents Atlas Vector Search for semantic search, hybrid search,
  filtering, and RAG:
  <https://www.mongodb.com/docs/vector-search/>
