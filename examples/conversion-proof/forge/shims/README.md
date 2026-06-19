# Launch Runtime Shims

These shims keep the DX-WWW conversion proof honest. They let the UI proof name missing runtime boundaries without pretending that hosted Supabase operations, Convex realtime execution, React hydration, Radix state machines, or Next.js routing were implemented.

The adapters return skipped, non-success results by design. A future worker can replace a shim only after adding the real DX-WWW-owned runtime code, permission boundaries, and verification receipts.
