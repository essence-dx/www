# WWW Public API Stability Docs Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make WWW's public developer contract sharper for outside developers by documenting stable surfaces, compatibility boundaries, and migration rules.

**Architecture:** Keep the public trust story in three places: `README.md` for launch-facing summary, `docs/dx-www-developer-contract.md` for project authoring rules, and `docs/api/versioning.md` for stability tiers and release policy. Add one focused TypeScript guard test so the docs cannot drift silently.

**Tech Stack:** Markdown docs, Bun/Node-compatible TypeScript test using built-in `node:test` and `node:assert`.

---

### Task 1: Stabilize The Public Contract

**Files:**
- Modify: `README.md`
- Modify: `docs/dx-www-developer-contract.md`
- Modify: `docs/api/versioning.md`
- Create: `benchmarks/dx-www-public-api-stability-docs.test.ts`

- [x] **Step 1: Document public surfaces**

Add the stable public surfaces: commands, project folders, TSX authoring syntax, DX-native state primitives, style/icons/imports, devtools dev-only behavior, and `.dx` receipts.

- [x] **Step 2: Document compatibility boundaries**

Clarify that React-style authoring is supported while hidden React DOM runtime parity is not the default, and React hook-like syntax must lower to DX-native semantics or fail clearly.

- [x] **Step 3: Document migration and deprecation rules**

Define stability tiers, semver expectations, deprecation windows, and receipt-backed proof requirements.

- [x] **Step 4: Guard the docs**

Add `benchmarks/dx-www-public-api-stability-docs.test.ts` to assert the stability docs mention the exact public API surfaces and compatibility rules.

- [x] **Step 5: Verify**

Run:

```powershell
bun test benchmarks\dx-www-public-api-stability-docs.test.ts
git diff --check
```

