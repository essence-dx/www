# Authentication Google Provider

This provider plugin belongs under the Authentication package at `auth/better-auth/providers/google`.

Required app-owned environment:

- `GOOGLE_CLIENT_ID`
- `GOOGLE_CLIENT_SECRET`

The default template exposes readiness and missing-config state only. It does not start OAuth until those credentials are provided by the host app.
