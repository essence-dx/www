export function LaunchPage() {
  return (
    <html lang="en" className="dark [scroll-behavior:smooth]">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <meta
          name="description"
          content="DX-WWW is a CSS-first, binary-capable web framework for tiny, fast, production websites and app routes."
        />
        <meta name="application-name" content="DX-WWW" />
        <meta name="author" content="DX-WWW" />
        <meta property="og:type" content="website" />
        <meta property="og:title" content="DX-WWW - Binary-capable web framework" />
        <meta
          property="og:description"
          content="A static-first framework that ships generated CSS by default and binary packets only when they win."
        />
        <meta property="og:site_name" content="DX-WWW" />
        <meta name="twitter:card" content="summary" />
        <meta name="twitter:title" content="DX-WWW - Binary-capable web framework" />
        <meta
          name="twitter:description"
          content="Tiny routes, source-owned styling, and selective binary packets for production websites."
        />
        <meta name="theme-color" content="#030303" />
        <title>DX-WWW - Binary Web Launch Site</title>
        <link rel="preload" href="/dx-style.generated.css" as="style" />
        <link rel="stylesheet" href="/dx-style.generated.css" />
        <script src="/site.js" defer></script>
      </head>
      <body className="m-0 min-h-screen bg-[#030303] text-zinc-50 antialiased [color-scheme:dark] [font-family:JetBrains_Mono,ui-monospace,SFMono-Regular,Consolas,monospace]">
        <a
          className="sr-only focus:not-sr-only focus:fixed focus:left-4 focus:top-4 focus:z-50 focus:rounded-md focus:bg-zinc-50 focus:px-3 focus:py-2 focus:text-zinc-950"
          href="#main"
        >
          Skip to product
        </a>

        <header className="sticky top-0 z-40 border-b border-zinc-900 bg-[#030303] px-4 py-3 md:px-8">
          <div className="mx-auto flex max-w-7xl items-center justify-between gap-4">
            <a className="flex items-center gap-3 text-zinc-50 no-underline" href="/" aria-label="DX-WWW home">
              <span className="inline-flex h-9 w-9 items-center justify-center rounded-md border border-zinc-700 bg-zinc-950 text-sm font-semibold text-zinc-50 shadow-lg">
                DX
              </span>
              <span className="flex flex-col leading-tight">
                <strong className="text-sm font-semibold">DX-WWW</strong>
                <span className="text-xs text-zinc-500">Binary-capable web</span>
              </span>
            </a>

            <nav className="hidden flex-wrap items-center justify-end gap-4 text-sm text-zinc-400 md:flex" aria-label="Primary navigation">
              <a className="text-zinc-400 no-underline hover:text-zinc-50" href="#proof">Proof</a>
              <a className="text-zinc-400 no-underline hover:text-zinc-50" href="#runtime">Runtime</a>
              <a className="text-zinc-400 no-underline hover:text-zinc-50" href="#dx-style">DX Style</a>
              <a className="text-zinc-400 no-underline hover:text-zinc-50" href="#launch">Launch</a>
              <a className="text-zinc-400 no-underline hover:text-zinc-50" href="#status">Status</a>
              <a className="rounded-md border border-zinc-800 px-3 py-2 text-zinc-200 no-underline hover:border-zinc-600 hover:text-zinc-50" href="/fair-counter">
                Tiny route
              </a>
            </nav>
          </div>
        </header>

        <main id="main">
          <section className="border-b border-zinc-900 px-4 py-10 md:px-8 md:py-14" aria-labelledby="hero-title">
            <div className="mx-auto max-w-7xl">
              <div className="mb-8 flex flex-col gap-3 rounded-lg border border-zinc-800 bg-zinc-950 p-4 text-sm text-zinc-400 md:flex-row md:items-center md:justify-between">
                <div className="flex items-center gap-2">
                  <span className="h-2 w-2 rounded-full bg-emerald-400" aria-hidden="true"></span>
                  <span>DX-WWW local launch server</span>
                </div>
                <strong className="font-medium text-zinc-100" id="route-status">checking route</strong>
              </div>

              <div className="grid gap-6 lg:grid-cols-2">
                <section className="rounded-lg border border-zinc-800 bg-[#070707] p-6 shadow-2xl md:p-8" aria-labelledby="hero-title">
                  <p className="m-0 text-xs font-semibold uppercase tracking-normal text-emerald-400">Launch build</p>
                  <h1 className="m-0 mt-4 text-6xl font-semibold tracking-normal text-zinc-50 md:text-8xl" id="hero-title">
                    DX-WWW
                  </h1>
                  <p className="mt-5 max-w-3xl text-lg leading-8 text-zinc-300">
                    A static-first web framework that ships tiny HTML, route-local behavior,
                    generated CSS by default, and binary packets only when they truly win.
                  </p>

                  <div className="mt-7 flex flex-col gap-3 sm:flex-row">
                    <a className="inline-flex items-center justify-center rounded-md bg-zinc-50 px-5 py-3 text-sm font-semibold text-zinc-950 no-underline hover:bg-zinc-200" href="/fair-counter">
                      Open 668 B route
                    </a>
                    <a className="inline-flex items-center justify-center rounded-md border border-zinc-800 px-5 py-3 text-sm font-semibold text-zinc-100 no-underline hover:border-zinc-600" href="#proof">
                      See benchmark proof
                    </a>
                  </div>

                  <dl className="mt-8 grid gap-3 md:grid-cols-3" aria-label="Current fair counter benchmark">
                    <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4">
                      <dt className="text-xs text-zinc-500">DX-WWW Brotli</dt>
                      <dd className="m-0 mt-2 text-2xl font-semibold text-zinc-50" data-metric="brotli">668 B</dd>
                    </div>
                    <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4">
                      <dt className="text-xs text-zinc-500">Requests</dt>
                      <dd className="m-0 mt-2 text-2xl font-semibold text-zinc-50">1</dd>
                    </div>
                    <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4">
                      <dt className="text-xs text-zinc-500">Median local load</dt>
                      <dd className="m-0 mt-2 text-2xl font-semibold text-zinc-50" data-metric="latency">4.41 ms</dd>
                    </div>
                  </dl>
                </section>

                <aside className="rounded-lg border border-zinc-800 bg-zinc-950 p-5 shadow-2xl" aria-label="Route compiler preview">
                  <div className="flex items-center justify-between gap-3 border-b border-zinc-800 pb-4">
                    <div>
                      <p className="m-0 text-xs uppercase tracking-normal text-zinc-500">Compiler plan</p>
                      <h2 className="m-0 mt-2 text-xl font-semibold text-zinc-50" data-route-title>Tiny route compiler plan</h2>
                    </div>
                    <span className="rounded-full border border-emerald-500 bg-emerald-950 px-3 py-1 text-xs text-emerald-300" data-runtime>
                      Micro JS
                    </span>
                  </div>

                  <div className="mt-4 flex flex-wrap gap-2" role="tablist" aria-label="Route scenarios">
                    <button
                      className="rounded-md border border-zinc-50 bg-zinc-50 px-3 py-2 text-xs font-semibold text-zinc-950 cursor-pointer"
                      type="button"
                      data-scenario="tiny"
                      aria-selected="true"
                    >
                      Tiny
                    </button>
                    <button
                      className="rounded-md border border-zinc-800 bg-zinc-950 px-3 py-2 text-xs font-semibold text-zinc-300 cursor-pointer"
                      type="button"
                      data-scenario="content"
                      aria-selected="false"
                    >
                      Content
                    </button>
                    <button
                      className="rounded-md border border-zinc-800 bg-zinc-950 px-3 py-2 text-xs font-semibold text-zinc-300 cursor-pointer"
                      type="button"
                      data-scenario="dashboard"
                      aria-selected="false"
                    >
                      Dashboard
                    </button>
                  </div>

                  <pre className="mt-5 overflow-auto rounded-lg border border-zinc-800 bg-[#050505] p-4 text-sm leading-7 text-zinc-300 whitespace-pre-wrap" data-plan>
route "/fair-counter"
mode static_html + micro_js
style default: generated_css
binary sidecar: optional
requests: 1
ship before interaction
                  </pre>

                  <div className="mt-5 grid gap-3">
                    <div>
                      <div className="mb-2 flex justify-between text-xs text-zinc-500">
                        <span>Raw payload</span>
                        <strong className="font-medium text-zinc-300" data-size="raw">1.90 KB</strong>
                      </div>
                      <div className="h-2 overflow-hidden rounded-full bg-zinc-900" data-bar="raw" role="meter" aria-valuemin="0" aria-valuemax="200" aria-valuenow="1.90">
                        <span className="block h-full w-[1%] rounded-full bg-sky-400"></span>
                      </div>
                    </div>
                    <div>
                      <div className="mb-2 flex justify-between text-xs text-zinc-500">
                        <span>Gzip payload</span>
                        <strong className="font-medium text-zinc-300" data-size="gzip">926 B</strong>
                      </div>
                      <div className="h-2 overflow-hidden rounded-full bg-zinc-900" data-bar="gzip" role="meter" aria-valuemin="0" aria-valuemax="200" aria-valuenow="0.93">
                        <span className="block h-full w-[1%] rounded-full bg-violet-400"></span>
                      </div>
                    </div>
                    <div>
                      <div className="mb-2 flex justify-between text-xs text-zinc-500">
                        <span>Brotli payload</span>
                        <strong className="font-medium text-zinc-300" data-size="brotli">668 B</strong>
                      </div>
                      <div className="h-2 overflow-hidden rounded-full bg-zinc-900" data-bar="brotli" role="meter" aria-valuemin="0" aria-valuemax="200" aria-valuenow="0.67">
                        <span className="block h-full w-[1%] rounded-full bg-emerald-400"></span>
                      </div>
                    </div>
                  </div>
                </aside>
              </div>
            </div>
          </section>

          <section className="border-b border-zinc-900 px-4 py-12 md:px-8" id="proof" aria-labelledby="proof-title">
            <div className="mx-auto max-w-7xl">
              <div className="flex flex-col gap-4 md:flex-row md:items-end md:justify-between">
                <div>
                  <p className="m-0 text-xs font-semibold uppercase tracking-normal text-sky-400">Measured locally</p>
                  <h2 className="m-0 mt-3 text-3xl font-semibold tracking-normal text-zinc-50" id="proof-title">
                    Fair counter benchmark
                  </h2>
                </div>
                <p className="m-0 max-w-2xl text-sm leading-6 text-zinc-400" id="benchmark-updated">
                  Benchmark report loading.
                </p>
              </div>

              <div className="mt-6 overflow-auto rounded-lg border border-zinc-800">
                <table className="w-full min-w-[720px] [border-collapse:collapse]">
                  <thead className="bg-zinc-950 text-left text-xs uppercase tracking-normal text-zinc-500">
                    <tr>
                      <th className="border-b border-zinc-800 px-4 py-3 font-medium">Framework</th>
                      <th className="border-b border-zinc-800 px-4 py-3 font-medium">Raw</th>
                      <th className="border-b border-zinc-800 px-4 py-3 font-medium">Gzip</th>
                      <th className="border-b border-zinc-800 px-4 py-3 font-medium">Brotli</th>
                      <th className="border-b border-zinc-800 px-4 py-3 font-medium">Median load</th>
                      <th className="border-b border-zinc-800 px-4 py-3 font-medium">Requests</th>
                    </tr>
                  </thead>
                  <tbody className="text-sm text-zinc-300">
                    <tr className="bg-[#070707]">
                      <td className="border-b border-zinc-900 px-4 py-4 font-semibold text-zinc-50">DX-WWW</td>
                      <td className="border-b border-zinc-900 px-4 py-4" data-metric="raw">1.90 KB</td>
                      <td className="border-b border-zinc-900 px-4 py-4" data-metric="gzip">926 B</td>
                      <td className="border-b border-zinc-900 px-4 py-4">668 B</td>
                      <td className="border-b border-zinc-900 px-4 py-4">4.41 ms</td>
                      <td className="border-b border-zinc-900 px-4 py-4">1</td>
                    </tr>
                    <tr>
                      <td className="border-b border-zinc-900 px-4 py-4">Astro 6</td>
                      <td className="border-b border-zinc-900 px-4 py-4">2.72 KB</td>
                      <td className="border-b border-zinc-900 px-4 py-4">1.01 KB</td>
                      <td className="border-b border-zinc-900 px-4 py-4">729 B</td>
                      <td className="border-b border-zinc-900 px-4 py-4">2.81 ms</td>
                      <td className="border-b border-zinc-900 px-4 py-4">1</td>
                    </tr>
                    <tr>
                      <td className="border-b border-zinc-900 px-4 py-4">Svelte 5</td>
                      <td className="border-b border-zinc-900 px-4 py-4">24.82 KB</td>
                      <td className="border-b border-zinc-900 px-4 py-4">10.29 KB</td>
                      <td className="border-b border-zinc-900 px-4 py-4">9.11 KB</td>
                      <td className="border-b border-zinc-900 px-4 py-4">10.64 ms</td>
                      <td className="border-b border-zinc-900 px-4 py-4">3</td>
                    </tr>
                    <tr>
                      <td className="border-b border-zinc-900 px-4 py-4">HTMX 2</td>
                      <td className="border-b border-zinc-900 px-4 py-4">53.91 KB</td>
                      <td className="border-b border-zinc-900 px-4 py-4">17.56 KB</td>
                      <td className="border-b border-zinc-900 px-4 py-4">15.72 KB</td>
                      <td className="border-b border-zinc-900 px-4 py-4">4.36 ms</td>
                      <td className="border-b border-zinc-900 px-4 py-4">2</td>
                    </tr>
                    <tr>
                      <td className="px-4 py-4">Next.js 16 + React 19</td>
                      <td className="px-4 py-4">634.90 KB</td>
                      <td className="px-4 py-4">187.43 KB</td>
                      <td className="px-4 py-4">161.92 KB</td>
                      <td className="px-4 py-4">30.25 ms</td>
                      <td className="px-4 py-4">9</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>
          </section>

          <section className="border-b border-zinc-900 px-4 py-12 md:px-8" id="runtime" aria-labelledby="runtime-title">
            <div className="mx-auto grid max-w-7xl gap-4 md:grid-cols-3">
              <div className="md:col-span-1">
                <p className="m-0 text-xs font-semibold uppercase tracking-normal text-violet-400">Runtime policy</p>
                <h2 className="m-0 mt-3 text-3xl font-semibold tracking-normal text-zinc-50" id="runtime-title">
                  Ship the smallest runtime that can honestly do the job.
                </h2>
              </div>
              <div className="grid gap-4 md:col-span-2 md:grid-cols-3">
                <article className="rounded-lg border border-zinc-800 bg-zinc-950 p-5">
                  <h3 className="m-0 text-base font-semibold text-zinc-50">Static first</h3>
                  <p className="mb-0 mt-3 text-sm leading-6 text-zinc-400">Content pages stay HTML and generated CSS until interaction is needed.</p>
                </article>
                <article className="rounded-lg border border-zinc-800 bg-zinc-950 p-5">
                  <h3 className="m-0 text-base font-semibold text-zinc-50">Micro JS</h3>
                  <p className="mb-0 mt-3 text-sm leading-6 text-zinc-400">Tiny islands use route-local scripts instead of a framework runtime tax.</p>
                </article>
                <article className="rounded-lg border border-zinc-800 bg-zinc-950 p-5">
                  <h3 className="m-0 text-base font-semibold text-zinc-50">Binary when it wins</h3>
                  <p className="mb-0 mt-3 text-sm leading-6 text-zinc-400">DX serializers and WASM remain available for richer state packets.</p>
                </article>
              </div>
            </div>
          </section>

          <section className="border-b border-zinc-900 px-4 py-12 md:px-8" id="dx-style" aria-labelledby="style-title">
            <div className="mx-auto grid max-w-7xl gap-5 lg:grid-cols-2">
              <div>
                <p className="m-0 text-xs font-semibold uppercase tracking-normal text-emerald-400">DX Style</p>
                <h2 className="m-0 mt-3 text-3xl font-semibold tracking-normal text-zinc-50" id="style-title">
                  React-like source, Tailwind-style classes, generated CSS output.
                </h2>
                <p className="mt-4 max-w-2xl text-sm leading-6 text-zinc-400">
                  This page is authored in LaunchPage.dx.jsx with className syntax. The browser receives normal HTML and generated CSS.
                </p>
              </div>
              <div className="grid gap-3 md:grid-cols-2">
                <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-5">
                  <p className="m-0 text-xs text-zinc-500">Style status</p>
                  <strong className="mt-2 block text-lg text-zinc-50" id="style-status">loading</strong>
                </div>
                <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-5">
                  <p className="m-0 text-xs text-zinc-500">Generated CSS</p>
                  <strong className="mt-2 block text-lg text-zinc-50" id="style-css-size">checking</strong>
                </div>
                <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-5">
                  <p className="m-0 text-xs text-zinc-500">Theme tokens</p>
                  <strong className="mt-2 block text-lg text-zinc-50" id="theme-token-count">0</strong>
                </div>
                <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-5">
                  <p className="m-0 text-xs text-zinc-500">Binary sidecar</p>
                  <strong className="mt-2 block text-lg text-zinc-50" id="style-packet-size">optional</strong>
                </div>
                <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-5 md:col-span-2">
                  <p className="m-0 text-xs text-zinc-500">Recipes</p>
                  <strong className="mt-2 block text-lg text-zinc-50" id="recipe-count">0</strong>
                </div>
              </div>
            </div>
          </section>

          <section className="border-b border-zinc-900 px-4 py-12 md:px-8" id="launch" aria-labelledby="launch-title">
            <div className="mx-auto max-w-7xl">
              <p className="m-0 text-xs font-semibold uppercase tracking-normal text-amber-300">Launch path</p>
              <h2 className="m-0 mt-3 text-3xl font-semibold tracking-normal text-zinc-50" id="launch-title">
                The billion-dollar version needs proof beyond tiny demos.
              </h2>
              <div className="mt-6 grid gap-4 md:grid-cols-3">
                <article className="rounded-lg border border-zinc-800 bg-zinc-950 p-5">
                  <span className="text-xs text-zinc-500">01</span>
                  <h3 className="m-0 mt-3 text-base font-semibold text-zinc-50">Website builder output</h3>
                  <p className="mb-0 mt-3 text-sm leading-6 text-zinc-400">Beat WordPress-style pages on payload, hosting cost, and editable component ownership.</p>
                </article>
                <article className="rounded-lg border border-zinc-800 bg-zinc-950 p-5">
                  <span className="text-xs text-zinc-500">02</span>
                  <h3 className="m-0 mt-3 text-base font-semibold text-zinc-50">Framework workbench</h3>
                  <p className="mb-0 mt-3 text-sm leading-6 text-zinc-400">Make React-like authoring, routing, data, style generation, and deployment feel boringly reliable.</p>
                </article>
                <article className="rounded-lg border border-zinc-800 bg-zinc-950 p-5">
                  <span className="text-xs text-zinc-500">03</span>
                  <h3 className="m-0 mt-3 text-base font-semibold text-zinc-50">Enterprise proof</h3>
                  <p className="mb-0 mt-3 text-sm leading-6 text-zinc-400">Win one real dashboard, one docs site, and one commerce surface before claiming framework dominance.</p>
                </article>
              </div>
            </div>
          </section>

          <section className="px-4 py-12 md:px-8" id="status" aria-labelledby="status-title">
            <div className="mx-auto flex max-w-7xl flex-col gap-5 rounded-lg border border-zinc-800 bg-zinc-950 p-6 md:flex-row md:items-center md:justify-between">
              <div>
                <p className="m-0 text-xs font-semibold uppercase tracking-normal text-zinc-500">Current status</p>
                <h2 className="m-0 mt-3 text-2xl font-semibold tracking-normal text-zinc-50" id="status-title">
                  Promising framework core, not yet a finished Next.js replacement.
                </h2>
              </div>
              <a className="inline-flex items-center justify-center rounded-md bg-zinc-50 px-5 py-3 text-sm font-semibold text-zinc-950 no-underline hover:bg-zinc-200" href="/fair-counter">
                Re-test tiny route
              </a>
            </div>
          </section>
        </main>
      </body>
    </html>
  );
}
