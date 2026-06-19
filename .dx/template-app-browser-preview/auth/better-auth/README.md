# Authentication

DX Forge Authentication is a source-owned Better Auth slice for the default DX-WWW template.

- Public package name: Authentication
- Package id: auth/better-auth
- Upstream provenance: better-auth 1.6.11
- Provider surface: email/password plus Google OAuth
- Runtime state: missing-config until the host app supplies Better Auth and Google provider environment variables

The package materializes front-facing files for client actions, session helpers, account management, profile updates, route handlers, dashboard readiness, and the selected Google provider. It does not claim a live OAuth redirect or hosted session without app credentials.
