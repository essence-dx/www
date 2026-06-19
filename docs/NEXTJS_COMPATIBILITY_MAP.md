# Next.js Compatibility Map For DX WWW

Date: 2026-05-22

## Official Upstream Mirror

Official upstream mirror:

- Source: `https://github.com/vercel/next.js.git`
- Local mirror: `G:\WWW\inspirations\nextjs`
- Inspected commit: `f3f56ecec2f3f8cefa0f0a1323ea406740251d5c`
- License: MIT License, preserved at `G:\WWW\inspirations\nextjs\license.md`

DX WWW studies the official Next.js source for compatibility behavior, but keeps the DX architecture source-owned: one source-owned `dx` file, Forge packages, dx-style, dx-check, and DX Studio/Zed preview. Do not use Turborepo as DX architecture, and do not make package-manager black boxes part of the public DX workflow.

## Feature Map

| Next.js surface | DX WWW launch direction | Reuse decision | Notes |
| --- | --- | --- | --- |
| App Router | TSX-first `app/` authoring with `layout.tsx`, `page.tsx`, route groups, and dynamic segments | Adapter boundary | Reuse the public file conventions and behavior model. Do not import Next's runtime graph, bundler, or Turborepo assumptions. |
| layouts | Source-owned layout/template/page composition | Adapter boundary | DX should compose deterministic source trees and expose Studio edit maps. Full React reconciliation remains separate work. |
| route handlers | `app/api/**/route.ts` with method exports, safe helper support, and route handler request maps | Adapter boundary | DX already executes safe source-owned route handlers without `node_modules`; dynamic route params and query values are passed through DX request maps. This is not unbounded Route Handler runtime coverage. |
| metadata | `export const metadata` plus safe literal and request-bound `generateMetadata` return compatibility | Adapter boundary | Static metadata extraction is useful. DX can read safe literal `generateMetadata` return objects, `alternates.canonical`, and a safe request-bound `generateMetadata` subset for `params`/`searchParams` values supplied by the DX request map; arbitrary dynamic metadata and metadata streaming remain explicit DX-owned follow-ups. |
| server/client components | React-familiar TSX with compiler-owned client island lowering | Adapter boundary | Keep `"use client"` support honest. Full React Server Components runtime is not copied. |
| server actions | Source-owned server action edges and explicit endpoints | Adapter boundary | Match authoring ergonomics, but keep execution through DX action receipts and Forge boundaries. |
| redirects/not-found | `redirect()`, `permanentRedirect()`, and `notFound()` compatibility markers | Reuse directly for constants and semantics only | Next uses `NEXT_REDIRECT` and `NEXT_HTTP_ERROR_FALLBACK;404` sentinel digests. DX mirrors the control-flow intent and emits safe metadata/head hints without importing Node runtime internals. |
| image/font/script boundaries | DX-owned boundaries for images, fonts, scripts, and asset receipts | Adapter boundary | Next's optimized loaders depend on bundler/runtime assumptions. DX should expose app-owned configuration and static export receipts. |
| middleware | Future request interception surface | Adapter boundary | Next middleware is valuable as a behavior target, but DX should use Rust/server-native routing and explicit edge/runtime receipts. |
| static export | Fast source-owned static output and deploy manifest | Adapter boundary | Study Next export metadata shape. Keep DX static export free of package-manager execution. |
| cache/revalidate | Route-level cache and revalidation receipts | Adapter boundary | Reuse public concepts like `revalidate`, `dynamic`, and cache modes. Do not copy the internal incremental cache stack. |

## Reuse Directly

- Public App Router file conventions.
- Public navigation helper names where they are part of the authoring API: `redirect()`, `permanentRedirect()`, and `notFound()`.
- Sentinel digest names as compatibility metadata: `NEXT_REDIRECT` and `NEXT_HTTP_ERROR_FALLBACK`.
- MIT license/provenance references in docs and source comments where behavior is upstream-informed.

## Adapter Boundary

- React rendering, layout/page composition, client islands, metadata, route handlers, server actions, image/font/script handling, middleware, static export, and cache/revalidate.
- These should feel familiar to Next.js developers, but be implemented as DX-owned Rust/TSX/compiler behavior.

## Not Useful For DX Architecture

Not useful for DX architecture:

- Turborepo as the public architecture.
- Next's package-manager install assumptions.
- Webpack/Turbopack internals as a direct dependency path.
- Node lifecycle scripts as the default package route.
- Hidden `node_modules` resolution as the normal template story.

## Current Implemented Slice

The first upstream-informed compatibility slice is App Router navigation control-flow metadata:

- `redirect("/target")` emits a DX control-flow record with `NEXT_REDIRECT`, status `307`, type `replace`, and a safe server-component-style refresh meta hint.
- `permanentRedirect("/target")` emits the same record with status `308`.
- `notFound()` emits a DX control-flow record with `NEXT_HTTP_ERROR_FALLBACK;404` and a safe `noindex` meta hint.
- Navigation helper scanning now ignores comments and string literals, accepts normal whitespace calls such as `redirect ( "/target", "push" )`, and accepts the safe Next-familiar redirect type forms `"push"`, `"replace"`, `RedirectType.push`, and `RedirectType.replace`; arbitrary dynamic redirect destinations remain adapter-boundary work.

This is not full Next.js runtime parity. DX does not yet execute arbitrary component imports, throw/catch React control-flow errors through a real renderer, reconcile provider trees, or set the actual page HTTP status from App Router page rendering. Those remain explicit next actions.

The next source-owned metadata slice now reads safe literal `generateMetadata()` return objects and a safe request-bound `generateMetadata()` subset:

- `title`, `description`, and `canonical` are accepted when returned as direct string literals.
- `alternates: { canonical: "..." }` is accepted as the familiar Next-shaped canonical field.
- `params.slug`, `params["slug"]`, `searchParams.preview`, `searchParams["preview"]`, and template literals composed only from those request bindings are recorded when values are supplied by the DX App Router request map.
- `generateMetadata()` safe optional chaining reads such as `params?.slug`, `params?.["slug"]`, `searchParams?.preview`, and `(await searchParams)?.preview` resolve from the same request map when the value is known.
- Safe parameter root aliases such as `generateMetadata({ params: routeParams, searchParams: queryParams })`, `routeParams.slug`, and `queryParams?.preview` resolve from the same request map while recording the canonical `params.*` / `searchParams.*` binding.
- Safe props object destructuring such as `generateMetadata(props)`, `const { params: routeParams, searchParams: queryParams } = props`, `routeParams.slug`, and `queryParams?.preview` resolves from the same request map without executing arbitrary metadata code.
- Safe destructured aliases with quoted string defaults such as `const { slug = "latest" } = params` and `const { preview: previewMode = "off" } = searchParams` resolve from the request map or the literal fallback without executing destructuring defaults.
- Safe const-arrow generateMetadata exports such as `export const generateMetadata = async ({ params, searchParams }) => { return { ... }; }` resolve the same known request values without importing Next runtime, React/RSC, Turbopack, or `node_modules`.
- Unresolved request-bound metadata is recorded as adapter-boundary receipt evidence instead of executing user code.
- Arbitrary dynamic metadata expressions, streamed metadata, cookies, headers, and imported helpers remain adapter-boundary work.
- This is source-owned metadata extraction for known request values; dynamic metadata execution and streaming metadata remain bounded follow-ups.

The page-prop binding slice keeps familiar App Router page props source-owned:

- Direct page reads such as `params.slug`, `params["slug"]`, `searchParams.preview`, and `searchParams["preview"]` resolve from the DX request map.
- The safe optional chaining reads `params?.slug`, `params?.["slug"]`, `searchParams?.preview`, and `(await searchParams)?.preview` resolve from the same request map when the value is known.
- The async page params and searchParams aliases slice records safe local aliases such as `const { slug } = params`, `const { slug } = await params`, and `const preview = (await searchParams)?.query` in `dx.tsx.requestPropBindings`.
- The unresolved page-prop aliases are recorded as `missing-request-prop-value` evidence in the same receipt instead of disappearing or forcing arbitrary request-code execution.
- The receipt records `adapter_boundary=source-owned-next-app-router-request-props`, `next_runtime_required=false`, `react_rsc_required=false`, `arbitrary_request_code_execution=false`, and `node_modules_required=false` so this slice stays visibly outside a Next/RSC runtime takeover.
- These aliases feed the bounded TSX static render surface only from known route/query values; this is not full React Server Component execution or full Next.js runtime parity.

The route-handler request slice now keeps dynamic `app/api/**/route.ts` matches source-owned:

- `app/api/posts/[slug]/route.ts` captures `slug` into a DX request map before the safe route-handler interpreter runs.
- Query values are passed as `request.searchParams` map evidence alongside `request.params`.
- The interpreter can read `request.params`, `request.params.slug`, `request.searchParams`, and `request.searchParams.preview` without importing Next's runtime or executing package lifecycle scripts.
- The safe route-handler context params slice accepts direct reads such as `context.params.slug`, `context.params`, `{ params }`, and `params.slug` in the DX safe interpreter.
- The async route-handler params aliases slice recognizes safe local reads such as `const { slug } = await params` and `const slugAlias = context.params.slug`; aliases resolve only from the DX route map.
- Dev responses include DX route-handler receipt headers with schema `dx.next.appRouteHandlerReceipt` plus numeric `format=1`, request-map counts, source path evidence, explicit `node_modules_required=false`, `x-dx-route-handler-source-owned=true`, and external-runtime headers set to false for Studio/Zed inspection.
- Unbounded context-argument coverage, `NextRequest`, `nextUrl`, streaming bodies, and middleware handoff remain adapter-boundary work.
