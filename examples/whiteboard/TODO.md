# Whiteboard Continuation

The whiteboard example is source-owned WWW code and should stay small, typed, and DX-native.

## Current Follow-Ups

- Keep source-runtime evidence fresh when the example is changed: canvas rendering, store-backed pointer/wheel controller flows, keyboard shortcuts, import, export, and local persistence; track browser-level receipts separately.
- Add collaboration adapters only after the local document model remains stable under focused tests.
- Keep remote image upload, remote URL image ingestion, CDN/media storage,
  media-library browsing, validated external library import, and remote
  share-link adapters as explicit future features; local image file import is
  limited to validated embedded data URLs on source-owned image elements.
- Promote local share receipts to real remote share adapters only when an adapter can attach an exact URL and receipt.
- Keep smart obstacle avoidance, automatic diagram layout, and browser-level
  connector-route receipts as future work; deterministic source-level elbow
  routing is separate from those claims.
- Keep frame nesting, drag-to-nest/autocapture, browser-level frame/presentation/outline receipts,
  and advanced group-aware transforms as future work; first-class frame
  membership, multi-element templates, frame presentation navigation, and
  frame-grouped outline navigation are now source-owned metadata/reducer
  contracts.
- Add browser E2E evidence for pointer drawing, marquee selection, wheel pan/zoom, and keyboard shortcuts once the WWW browser-controller path can record stable receipts.
- Add browser E2E evidence for measurement panel visual alignment and focus behavior once the WWW browser-controller path can record stable receipts.
- Add a small browser receipt controller for production-grade pointer, wheel, and keyboard evidence; until then, docs should claim only source-level DOM event adapters and focused source tests.
- Keep `.dx` receipts current after `dx check` and avoid committing raw secrets or user-local cache data.

## Guardrails

- Do not introduce React runtime imports or dummy compatibility hooks.
- Keep hand-authored TypeScript files focused; split files before they become difficult to review.
- Preserve the public `lib/whiteboard/scene.ts` and `lib/whiteboard/input/input-runtime.ts` import surfaces when moving internals.
