export const metadata = {
  title: "DX Shader, the generative shader studio",
  description:
    "Create looping abstract shader art with WebGL2, curated controls, share codes, and deterministic exports.",
} as const;

export default function HomePage() {
  return (
    <main className="dx-shader-root" data-dx-route="/" data-dx-template="dx-shader">
      <header className="topbar">
        <div className="brand">
          <svg className="brand-mark" viewBox="0 0 20 20" aria-hidden="true">
            <path
              d="M10 1 L19 10 L10 19 L1 10 Z"
              fill="none"
              stroke="currentColor"
              strokeWidth="1.4"
            />
            <path
              d="M10 5.5 L14.5 10 L10 14.5 L5.5 10 Z"
              fill="currentColor"
            />
          </svg>
          <span className="brand-name">DX Shader</span>
          <span className="brand-sub">generative shader studio</span>
        </div>

        <div className="topbar-center">
          <div
            className="segmented"
            id="dx-shader-aspect-control"
            role="group"
            aria-label="Canvas aspect ratio"
          />
        </div>

        <div className="topbar-actions">
          <button
            className="btn btn-primary"
            id="btn-random"
            title="Randomize everything (R)"
            type="button"
          >
            <svg viewBox="0 0 16 16" aria-hidden="true">
              <rect
                x="1.5"
                y="1.5"
                width="13"
                height="13"
                rx="3"
                fill="none"
                stroke="currentColor"
                strokeWidth="1.5"
              />
              <circle cx="5.4" cy="5.4" r="1.25" fill="currentColor" />
              <circle cx="10.6" cy="10.6" r="1.25" fill="currentColor" />
              <circle cx="10.6" cy="5.4" r="1.25" fill="currentColor" />
              <circle cx="5.4" cy="10.6" r="1.25" fill="currentColor" />
            </svg>
            Randomize
          </button>
          <div className="topbar-divider" />
          <button
            className="btn"
            id="btn-export-png"
            title="Save still image (S)"
            type="button"
          >
            <svg viewBox="0 0 16 16" aria-hidden="true">
              <rect
                x="1.5"
                y="1.5"
                width="13"
                height="13"
                rx="2"
                fill="none"
                stroke="currentColor"
                strokeWidth="1.5"
              />
              <circle cx="5.5" cy="5.5" r="1.5" fill="currentColor" />
              <path
                d="M2 12 L6 8 L9 11 L11.5 8.5 L14 11"
                fill="none"
                stroke="currentColor"
                strokeWidth="1.5"
              />
            </svg>
            Image
          </button>
          <button
            className="btn"
            id="btn-export-video"
            title="Record looping video"
            type="button"
          >
            <svg viewBox="0 0 16 16" aria-hidden="true">
              <rect
                x="1.5"
                y="3.5"
                width="9"
                height="9"
                rx="2"
                fill="none"
                stroke="currentColor"
                strokeWidth="1.5"
              />
              <path
                d="M10.5 7 L14.5 4.5 V11.5 L10.5 9"
                fill="currentColor"
              />
            </svg>
            Video
          </button>
          <button
            className="btn"
            id="btn-export-gif"
            title="Render seamless looping GIF"
            type="button"
          >
            <svg viewBox="0 0 16 16" aria-hidden="true">
              <path
                d="M13.5 8 a5.5 5.5 0 1 1 -1.6 -3.9"
                fill="none"
                stroke="currentColor"
                strokeWidth="1.5"
              />
              <path
                d="M13.8 1.6 V4.4 H11"
                fill="none"
                stroke="currentColor"
                strokeWidth="1.5"
              />
            </svg>
            GIF
          </button>
          <button
            className="btn"
            id="btn-set"
            title="Generate a consistent set of variations"
            type="button"
          >
            <svg viewBox="0 0 16 16" aria-hidden="true">
              <rect
                x="1.5"
                y="1.5"
                width="5.5"
                height="5.5"
                rx="1.5"
                fill="none"
                stroke="currentColor"
                strokeWidth="1.4"
              />
              <rect
                x="9"
                y="1.5"
                width="5.5"
                height="5.5"
                rx="1.5"
                fill="none"
                stroke="currentColor"
                strokeWidth="1.4"
              />
              <rect
                x="1.5"
                y="9"
                width="5.5"
                height="5.5"
                rx="1.5"
                fill="none"
                stroke="currentColor"
                strokeWidth="1.4"
              />
              <rect
                x="9"
                y="9"
                width="5.5"
                height="5.5"
                rx="1.5"
                fill="none"
                stroke="currentColor"
                strokeWidth="1.4"
              />
            </svg>
            Set
          </button>
          <div className="topbar-divider" />
          <a className="btn btn-ghost" href="/dx-shader/docs.html" title="Documentation">
            <svg viewBox="0 0 16 16" aria-hidden="true">
              <path
                d="M3 2 H10 L13 5 V14 H3 Z"
                fill="none"
                stroke="currentColor"
                strokeWidth="1.4"
              />
              <path
                d="M5.5 7.5 H10.5 M5.5 10 H10.5"
                stroke="currentColor"
                strokeWidth="1.2"
              />
            </svg>
            Docs
          </a>
        </div>
      </header>

      <div className="app">
        <section className="stage" id="stage">
          <div
            className="shader-loader"
            data-loader-mode="auto"
            data-loader-state="preparing"
            id="shader-boot-loader"
            role="status"
            aria-live="polite"
          >
            <div className="shader-loader-panel">
              <div
                className="shader-loader-mark"
                data-dx-icon-search="pack:dx lucide:loader-circle"
              >
                <dx-icon
                  className="shader-loader-dx"
                  name="pack:dx"
                  aria-hidden="true"
                />
                <dx-icon
                  className="shader-loader-spinner"
                  data-lucide-icon="loader-circle"
                  name="lucide:loader-circle"
                  aria-hidden="true"
                />
              </div>
              <div className="shader-loader-copy">
                <span className="shader-loader-title">Preparing shader engine</span>
                <span className="shader-loader-detail" id="shader-boot-status">
                  Checking GPU readiness
                </span>
              </div>
              <button
                className="shader-loader-action"
                hidden="hidden"
                id="shader-boot-start"
                type="button"
              >
                Start shader
              </button>
              <div className="shader-loader-shimmer" aria-hidden="true" />
            </div>
          </div>
          <div className="canvas-frame" id="canvas-frame">
            <div className="canvas-shell" id="canvas-shell">
              <div
                className="shader-static-fallback"
                id="shader-static-fallback"
                aria-hidden="true"
              >
                <span className="shader-static-title">DX Shader</span>
                <span className="shader-static-detail">Safe shader preview</span>
              </div>
              <noscript>
                <div className="shader-static-fallback shader-static-fallback-noscript">
                  <span className="shader-static-title">DX Shader</span>
                  <span className="shader-static-detail">
                    JavaScript is required to start the live shader.
                  </span>
                </div>
              </noscript>
              <canvas id="view"></canvas>
            </div>
          </div>
          <div className="stage-meta">
            <div className="meta-left">
              <button
                className="icon-btn"
                id="btn-play"
                title="Play / pause (Space)"
                type="button"
              >
                <svg id="icon-pause" viewBox="0 0 14 14">
                  <rect
                    x="2.5"
                    y="2"
                    width="3"
                    height="10"
                    rx="1"
                    fill="currentColor"
                  />
                  <rect
                    x="8.5"
                    y="2"
                    width="3"
                    height="10"
                    rx="1"
                    fill="currentColor"
                  />
                </svg>
                <svg id="icon-play" viewBox="0 0 14 14">
                  <path d="M3.5 2 L12 7 L3.5 12 Z" fill="currentColor" />
                </svg>
              </button>
              <span className="meta-item mono" id="meta-mode">
                ...
              </span>
              <span className="meta-sep" />
              <span className="meta-item mono dim" id="meta-seed">
                seed 0000
              </span>
            </div>
            <div className="meta-right">
              <span className="meta-item mono dim" id="meta-loop">
                4.0s loop
              </span>
              <span className="meta-sep" />
              <span className="meta-item mono dim" id="meta-res">
                0x0
              </span>
              <span className="meta-sep" />
              <span className="meta-item mono dim" id="meta-fps">
                60 fps
              </span>
            </div>
          </div>
        </section>

        <aside className="rail" id="rail" />
      </div>

      <div className="overlay is-hidden" id="overlay" hidden="hidden">
        <div className="overlay-card">
          <div className="overlay-title" id="overlay-title">
            Rendering
          </div>
          <div className="overlay-detail mono" id="overlay-detail">
            ...
          </div>
          <div className="progress">
            <div className="progress-fill" id="overlay-bar" />
          </div>
          <button className="btn overlay-cancel" id="overlay-cancel" type="button">
            Cancel
          </button>
        </div>
      </div>

      <div className="toast mono is-hidden" id="toast" hidden="hidden"></div>

      <script defer src="/dx-shader/js/palettes.js"></script>
      <script defer src="/dx-shader/js/shaders.js"></script>
      <script defer src="/dx-shader/js/engine.js"></script>
      <script defer src="/dx-shader/js/ui.js"></script>
      <script defer src="/dx-shader/js/main.js"></script>
    </main>
  );
}
