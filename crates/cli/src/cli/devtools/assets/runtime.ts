// DX Devtools injected runtime manifest.
// Rust serves /_dx/devtools/runtime.js by concatenating the ordered files in ./runtime/.
// Keep this file small so the source tree points developers to the split runtime instead of hiding a mega asset.
export const DX_DEVTOOLS_RUNTIME_FRAGMENTS = [
  "runtime/part-01-boot.ts",
  "runtime/part-02-protocol.ts",
  "runtime/part-03-controls.ts",
  "runtime/part-04-render.ts",
  "runtime/part-05-events.ts",
] as const;
