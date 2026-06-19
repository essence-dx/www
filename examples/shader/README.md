# DX Shader

DX-WWW shader studio adapted from [Leonxlnx/lumenshaders](https://github.com/Leonxlnx/lumenshaders), a dependency-free WebGL2 generative shader studio.

## What This Example Contains

- `app/page.tsx` owns the DX-WWW App Router route and studio shell.
- `styles/dx-shader.css` owns the DX Shader instrument-panel design system.
- `public/dx-shader/js/` serves the dependency-free shader runtime in the browser.
- `tests/*.ts` guard the DX loader, brand, and safe shader boot contract.
- `public/dx-shader/assets/` provides the favicon and touch icon used by the route.
- `public/dx-shader/docs.html` keeps the DX Shader documentation available from the Docs control.
- `public/dx-shader/docs.css` keeps docs-only styles out of the HTML document.
- `public/dx-shader/fonts/` self-hosts JetBrains Mono for the default UI font.

The upstream reference clone lives at `G:\Dx\www\inspirations\lumenshaders`.

## Runtime Safety

The DX-WWW page renders a DX icon loader before WebGL starts. Edge, low-core,
low-memory, data-saver, and reduced-motion devices wait for an explicit shader
start instead of booting WebGL immediately.

The live renderer now uses a hard pixel budget, reason-based suspension for
visibility/export/modal ownership, idle dirty-frame rendering, deferred ordered
runtime scripts, lazy export codec loading, local system fonts, and a cheap
default shell without large persistent backdrop blurs.

Constrained renderers skip the startup stagger animation before any intro
classes are applied, avoiding extra compositor and style churn during boot.
They also cap live playback draw cadence at 30 FPS, reducing GPU fragment work
without changing still exports.

During live playback, sustained low FPS lowers the live canvas pixel budget and
recovers cautiously only after the frame rate stabilizes, with a short cooldown
between live canvas resizes.

Constrained devices also use a lower live pixel budget and cheaper live shader
uniforms before WebGL starts. The engine suspends on WebGL context loss and
reports recovery back to the loader instead of continuing a broken render loop.
Export paths estimate device-safe memory before resizing the canvas or capturing
frames, yield back to the browser during long work, and cancel modal preview/set
generation when the dialog closes.

The copied files under `public/dx-shader/js/` are browser-delivered runtime
artifacts adapted from the upstream project. New project-owned tests and source
should use `.ts` or `.tsx`. Keep those public runtime files as `.js` until the
project has a real transpile step for browser-delivered scripts.

The shell uses a Vercel-inspired neutral theme with dark mode as the default
and a `prefers-color-scheme: light` override for light mode.

## Run

Only run locally after performance-safe approval on the shared machine:

```powershell
cd G:\Dx\www\examples\shader
G:\Dx\bin\dx-www.exe dev --host 127.0.0.1 --port 3001
```

The port can be changed if another DX server is already using it.

## Attribution

Lumenshaders is MIT licensed. The adapted runtime and assets retain the upstream license in `public/dx-shader/LICENSE`.
