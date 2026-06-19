import { Icon } from "@/components/icons/icon";

export function LandingPageContent() {
  return (
<main className="dx-landing" data-dx-route="/" data-dx-source="dx-website-startpage" data-dx-forge="website-conversion-dx-landing" data-dx-surface="migration/tsx-source" data-dx-template="tsx-landing" data-dx-style-surface="dx-landing" data-dx-check-surface="dx-landing" data-forge-package="www/landing-page" data-dx-hot-reload-target="route:/" data-route="/" data-source="dx-website-startpage" data-evidence="WWW ecosystem landing" data-surface-map="forge/source-surfaces/dx-landing.json" data-visual-audit="forge/visual-audits/dx-landing.json">
<div className="dx-scrollbar" aria-hidden="true"><span></span></div>

    <header className="site-nav" data-dx-section="navigation">
      <a className="brand-pill" href="/" aria-label="DX home">
        <img className="brand-logo" src="/logo.svg" alt="Dx WWW" loading="eager" decoding="async" />
      </a>
      <nav className="nav-links" aria-label="DX landing sections">
        <a href="#www-platform">WWW</a>
        <a href="#token-revolution">Serializer</a>
        <a href="#providers-token">Providers</a>
        <a href="#forge">Forge</a>
        <a href="#check">Check</a>
      </nav>
    </header>

    <section className="landing-hero">
      <div className="hero-video-carousel dx-carousel" data-dx-carousel="hero-media" data-dx-carousel-autoplay="true" aria-label="8K nature video carousel">
        <div className="video-carousel-track" data-dx-carousel-track="true">
          <article className="video-slide" data-dx-carousel-slide="true">
            <a className="video-preview-card" href="https://www.youtube.com/watch?v=u1GSweKRC9U" target="_blank" rel="noopener noreferrer" aria-label="Open 8K nature preview video">
              <Icon name="pack:play" className="video-play-icon" />
              <div><span>8K Nature</span><strong>Wildlife preview surface</strong></div>
            </a>
          </article>
          <article className="video-slide" data-dx-carousel-slide="true">
            <a className="video-preview-card" href="https://www.youtube.com/watch?v=oJZhvdtiPUw" target="_blank" rel="noopener noreferrer" aria-label="Open ocean HDR preview video">
              <Icon name="pack:play" className="video-play-icon" />
              <div><span>Ocean HDR</span><strong>Media-rich product surface</strong></div>
            </a>
          </article>
          <article className="video-slide" data-dx-carousel-slide="true">
            <a className="video-preview-card" href="https://www.youtube.com/watch?v=rHYuYdDqCmk" target="_blank" rel="noopener noreferrer" aria-label="Open travel 8K preview video">
              <Icon name="pack:play" className="video-play-icon" />
              <div><span>Travel 8K</span><strong>Fullscreen creator canvas</strong></div>
            </a>
          </article>
        </div>
        <div className="carousel-controls" aria-label="Media carousel controls">
          <div className="carousel-dots" role="tablist" aria-label="Media previews">
            <button type="button" data-dx-carousel-dot="true" data-dx-carousel-index="0" aria-label="Show 8K Nature preview"></button>
            <button type="button" data-dx-carousel-dot="true" data-dx-carousel-index="1" aria-label="Show Ocean HDR preview"></button>
            <button type="button" data-dx-carousel-dot="true" data-dx-carousel-index="2" aria-label="Show Travel 8K preview"></button>
          </div>
        </div>
      </div>
      <h1>The Developer Experience You Actually Deserve.</h1>
      <p className="hero-copy">
        WWW is the web framework and operating stack for DX: a Rust-first runtime, source-owned style system,
        Forge package workflow, Check quality gates, local AI, browser automation, and automation-ready workflows.
      </p>

      <div className="platform-grid" aria-label="DX platform downloads">
        <a href="/download" className="platform-card">
          <img src="/public/icons/platform-svgl/apple.svg" className="platform-icon" alt="" aria-hidden="true" loading="lazy" decoding="async" />
          <span>macOS</span><small>Native app</small>
        </a>
        <a href="/download" className="platform-card">
          <img src="/public/icons/platform-svgl/windows.svg" className="platform-icon" alt="" aria-hidden="true" loading="lazy" decoding="async" />
          <span>Windows</span><small>Native app</small>
        </a>
        <a href="/download" className="platform-card">
          <img src="/public/icons/platform-svgl/linux.svg" className="platform-icon" alt="" aria-hidden="true" loading="lazy" decoding="async" />
          <span>Linux</span><small>Native app</small>
        </a>
        <a href="/download" className="platform-card">
          <img src="/public/icons/platform-svgl/android.svg" className="platform-icon" alt="" aria-hidden="true" loading="lazy" decoding="async" />
          <span>Android</span><small>Mobile app</small>
        </a>
        <a href="/download" className="platform-card">
          <img src="/public/icons/platform-svgl/apple.svg" className="platform-icon" alt="" aria-hidden="true" loading="lazy" decoding="async" />
          <span>iOS</span><small>Mobile app</small>
        </a>
        <a href="/download" className="platform-card">
          <img src="/public/icons/platform-svgl/chrome.svg" className="platform-icon" alt="" aria-hidden="true" loading="lazy" decoding="async" />
          <span>ChromeOS</span><small>Web app</small>
        </a>
        <a href="/download" className="platform-card">
          <img src="/public/icons/platform-svgl/firefox.svg" className="platform-icon" alt="" aria-hidden="true" loading="lazy" decoding="async" />
          <span>Browser</span><small>Remote console</small>
        </a>
        <a href="/download" className="platform-card">
          <img src="/public/icons/platform-svgl/rust.svg" className="platform-icon" alt="" aria-hidden="true" loading="lazy" decoding="async" />
          <span>CLI</span><small>dx toolchain</small>
        </a>
        {/*
        <a href="/download" className="platform-card">
          <img src="/public/icons/platform-svgl/vercel.svg" className="platform-icon" alt="" aria-hidden="true" loading="lazy" decoding="async" />
          <span>More</span><small>Future tools</small>
        </a>
        */}
      </div>

      <div className="hero-actions">
        <a href="/download" className="primary-action dx-icon-download-action" data-icon-source="dx-icons" aria-label="Download DX with DX-powered icons">
          <Icon name="pack:motion" className="action-icon dx-action-icon" />
          <span>Get Started Free</span>
        </a>
        <a href="#token-revolution" className="secondary-action">See Serializer</a>
      </div>

      <div className="hero-image-grid" aria-label="DX visual system">
        <figure><img src="/public/thumbnails/rainbow.png" alt="DX colorful workflow preview" width="1024" height="1024" loading="lazy" decoding="async" /><figcaption>Visual builder</figcaption></figure>
        <figure><img src="/public/thumbnails/indigo.png" alt="DX forge package workflow preview" width="1024" height="1024" loading="lazy" decoding="async" /><figcaption>Forge workflow</figcaption></figure>
        <figure><img src="/public/thumbnails/teal.png" alt="DX browser automation preview" width="1024" height="1024" loading="lazy" decoding="async" /><figcaption>Automation</figcaption></figure>
      </div>

      <div className="hero-proof-grid">
        <article>Rust-first WWW runtime with source-owned style and no template node_modules</article>
        <article>Forge packages, Providers, Token tools, and automation-ready workflows</article>
        <article>Check, Serializer, and quality receipts for measurable product evidence</article>
      </div>

      <div className="stat-grid">
        <article><strong>180+</strong><span>LLM Providers</span></article>
        <article><strong>90%</strong><span>Local Agent Speed Achieved</span></article>
        <article><strong>0</strong><span>Template node_modules</span></article>
      </div>

      <div className="motion-icon-rail" aria-label="Animated DX capability icons">
        <span style="--icon-delay: 0s"><Icon name="pack:ui-components" className="capability-icon" />WWW</span>
        <span style="--icon-delay: .08s"><Icon name="pack:state" className="capability-icon" />Forge</span>
        <span style="--icon-delay: .16s"><Icon name="pack:state" className="capability-icon" />Tokens</span>
        <span style="--icon-delay: .24s"><Icon name="pack:ui-components" className="capability-icon" />Style</span>
        <span style="--icon-delay: .32s"><Icon name="pack:play" className="capability-icon" />Browser</span>
        <span style="--icon-delay: .4s"><Icon name="pack:three-scene" className="capability-icon" />3D</span>
      </div>
    </section>

    {/*
    <section className="landing-section" id="showcases">
      <p className="eyebrow">DX Thesis</p>
      <h2>Game-Changing Features That Set DX Apart</h2>
      <p className="section-copy">Every feature is designed to go viral. This is not incremental; it is a paradigm shift.</p>
      <div className="feature-banner-grid">
        <article className="feature-banner">
          <img src="/public/thumbnails/amber.png" alt="Forge storage workflow preview" width="1024" height="1024" loading="lazy" decoding="async" />
          <div><span>Forge</span><h3>Unlimited Free Storage for Every Media Type</h3><p>Video to YouTube, images to Pinterest, audio to SoundCloud, 3D to Sketchfab, code to Git providers.</p></div>
        </article>
        <article className="feature-banner">
          <img src="/public/thumbnails/blue.png" alt="Traffic security workflow preview" width="1024" height="1024" loading="lazy" decoding="async" />
          <div><span>Safe Autonomy</span><h3>AI That Acts Fast Without Compromising Control</h3><p>DX keeps agent actions policy-aware, reviewable, and recoverable without slowing down safe work.</p></div>
        </article>
        <article className="feature-banner">
          <img src="/public/thumbnails/cyan.png" alt="DX check quality preview" width="1024" height="1024" loading="lazy" decoding="async" />
          <div><span>Check</span><h3>500-Point Quality Score for Every Project</h3><p>Security scanning, code checks, media checks, and clear improvement reports.</p></div>
        </article>
        <article className="feature-banner">
          <img src="/public/thumbnails/emerald.png" alt="Media engine preview" width="1024" height="1024" loading="lazy" decoding="async" />
          <div><span>Media Engine</span><h3>5,000+ Fonts, 1M+ Icons, 20+ Providers, Built-In Asset Tools</h3><p>Fetch from Unsplash, Pexels, YouTube, Vimeo, Spotify, Sketchfab, then transform, version, and collaborate.</p></div>
        </article>
        <article className="feature-banner">
          <img src="/public/thumbnails/fuchsia.png" alt="Works everywhere preview" width="1024" height="1024" loading="lazy" decoding="async" />
          <div><span>Works Everywhere</span><h3>9+ Native Platforms, Every Browser, Every IDE, Every Creative Tool</h3><p>macOS, Windows, Linux, Android, iOS, ChromeOS, watchOS, tvOS, remote web console, and extensions.</p></div>
        </article>
        <article className="feature-banner">
          <img src="/public/thumbnails/green.png" alt="AI provider freedom preview" width="1024" height="1024" loading="lazy" decoding="async" />
          <div><span>180+ AI Providers</span><h3>Any Model. Any Provider. Even Offline.</h3><p>Broad provider coverage, unlimited local models, and hybrid cloud/offline switching.</p></div>
        </article>
      </div>
    </section>

    <section className="landing-section" id="story-engine">
      <p className="eyebrow">The DX Story Engine</p>
      <h2>A single runtime for local speed, provider freedom, and production workflows.</h2>
      <div className="story-grid">
        <article><small>Step 01</small><h3>Rust Core + WWW Runtime</h3><strong>Up to 70% lower RAM pressure</strong><p>DX runs generation, routing, package receipts, and automation workflows without the dependency overhead that slows traditional web stacks.</p></article>
        <article><small>Step 02</small><h3>Token Stack: Serializer + Tokenizers</h3><strong>30-90% token savings</strong><p>Compact, AI-readable payloads reduce cost and latency across multi-step workflows.</p></article>
        <article><small>Step 03</small><h3>Always-On Workflows</h3><strong>No offline lockout</strong><p>Switch between cloud and local execution while preserving workflow state and output continuity.</p></article>
        <article><small>Step 04</small><h3>180+ Providers + Tool Connectors</h3><strong>One connected runtime</strong><p>Models, tools, communication apps, and media pipelines share context so work moves end-to-end faster.</p></article>
      </div>
    </section>

    <section className="landing-section theme-section" id="style-system">
      <p className="eyebrow">Style Theme System</p>
      <h2>Clean in light mode, cinematic in dark mode, native to the user's system.</h2>
      <p className="section-copy">WWW should feel sharp in a browser and in static export. The landing uses system-aware style variables, a manual switcher, readable contrast, and motion that stays light enough for live demos.</p>
      <div className="theme-proof-grid">
        <article>
          <span>Light</span>
          <strong>White surface, black type, restrained borders.</strong>
          <p>Clean clarity for product claims, comparison tables, docs, and system sections.</p>
        </article>
        <article>
          <span>Dark</span>
          <strong>Black canvas, soft glow, high signal cards.</strong>
          <p>DX energy for the face, style story, agents, Forge, and local AI.</p>
        </article>
        <article>
          <span>System</span>
          <strong>Follows the operating system by default.</strong>
          <p>Users can switch manually without breaking the WWW export or the static bundle.</p>
        </article>
      </div>
    </section>

    <section className="landing-section" id="what-is-dx">
      <p className="eyebrow">What Is DX?</p>
      <h2>A unified development experience platform for developers, creators, and teams.</h2>
      <div className="card-grid three">
        <article>AI generation, tool calling, media creation, and workflow integration are one cohesive experience.</article>
        <article>Generate code, analyze data, run research, and produce media with one context and one mental model.</article>
        <article>Manage everything from the browser, then keep the same workflow across native desktop, mobile, and companion apps.</article>
      </div>
      <div className="card-grid two">
        <article><small>Workflow</small><strong>One clean surface for building, editing, checking, and shipping.</strong></article>
        <article><small>Extensions</small><strong>Browsers, IDEs, Figma, Photoshop, DaVinci Resolve, and more.</strong></article>
      </div>
    </section>

    <section className="landing-section" id="command-center">
      <p className="eyebrow">DX Command Center</p>
      <h2>Move from landing-page overview to operational evidence in one click.</h2>
      <div className="card-grid three">
        <article><small>Product Workflows</small><strong>Assistant flows</strong><p>Explore ask, agent, research execution patterns and connected context flows.</p></article>
        <article><small>Integration Surface</small><strong>MCP and providers</strong><p>Validate routing, provider coverage, and tool-call interfaces across clients.</p></article>
        <article><small>Docs + API Readiness</small><strong>Operational setup</strong><p>Go deeper into architecture notes, setup patterns, and workflow documentation.</p></article>
      </div>
    </section>

    */}

    <section className="landing-section" id="www-platform">
      <p className="eyebrow">The WWW Platform</p>
      <h2>A Rust-first web stack where framework, style, packages, checks, and automation share one contract.</h2>
      <div className="editor-shell dx-carousel" data-dx-carousel="platform-evidence" data-dx-carousel-autoplay="true" data-dx-carousel-interval="4600" aria-label="WWW platform evidence carousel">
        <aside className="editor-rail left-rail">
          <span>Source</span>
          <button type="button" data-dx-carousel-dot="true" data-dx-carousel-index="0">Routes</button>
          <button type="button" data-dx-carousel-dot="true" data-dx-carousel-index="1">Styles</button>
          <button type="button" data-dx-carousel-dot="true" data-dx-carousel-index="2">Forge</button>
          <button type="button" data-dx-carousel-dot="true" data-dx-carousel-index="3">Receipts</button>
        </aside>
        <div className="editor-stage">
          <div className="screen-dock" role="tablist" aria-label="WWW route evidence preview">
            <button type="button" data-dx-carousel-dot="true" data-dx-carousel-index="0" aria-label="Show route evidence"></button>
            <button type="button" data-dx-carousel-dot="true" data-dx-carousel-index="1" aria-label="Show style evidence"></button>
            <button type="button" data-dx-carousel-dot="true" data-dx-carousel-index="2" aria-label="Show Forge evidence"></button>
            <button type="button" data-dx-carousel-dot="true" data-dx-carousel-index="3" aria-label="Show check evidence"></button>
          </div>
          <div className="screen-stack" data-dx-carousel-viewport="true">
            <div className="screen-card-track" data-dx-carousel-track="true">
              <article className="screen-card primary-screen" data-dx-carousel-slide="true">
                <small>WWW Runtime</small>
                <strong>Routes, pages, layouts, assets, and manifests built as one system</strong>
                <p>Every product surface carries source metadata, route identity, theme tokens, and quality gates without template-local dependency chaos.</p>
              </article>
              <article className="screen-card preview-screen" data-dx-carousel-slide="true">
                <small>Style</small>
                <strong>Theme values, responsive sections, icons, and motion stay inspectable</strong>
                <p>WWW routes stay visual-edit ready, so future studio tools can make precise changes without hardcoded guesses.</p>
              </article>
              <article className="screen-card forge-screen" data-dx-carousel-slide="true">
                <small>Forge</small>
                <strong>Source-owned package slices show what entered the app and why</strong>
                <p>Forge turns packages, media, adapters, and provenance into reviewable app source instead of hiding the real surface inside dependency folders.</p>
              </article>
              <article className="screen-card check-screen" data-dx-carousel-slide="true">
                <small>Check + receipts</small>
                <strong>Route quality, assets, accessibility notes, and release evidence stay machine-readable</strong>
                <p>Checks, serializer files, and receipts make the project explain itself to humans, tools, and agents before any public release.</p>
              </article>
            </div>
          </div>
          <div className="carousel-controls platform-carousel-controls" aria-label="Platform carousel controls">
            <button type="button" className="carousel-control" data-dx-carousel-prev="true" aria-label="Previous platform evidence">Prev</button>
            <button type="button" className="carousel-control" data-dx-carousel-next="true" aria-label="Next platform evidence">Next</button>
          </div>
        </div>
        <aside className="editor-rail right-rail">
          <span>Quality</span>
          <button type="button" data-dx-carousel-dot="true" data-dx-carousel-index="3">Check</button>
          <button type="button" data-dx-carousel-dot="true" data-dx-carousel-index="2">Forge</button>
          <button type="button" data-dx-carousel-dot="true" data-dx-carousel-index="1">Tokens</button>
          <button type="button" data-dx-carousel-dot="true" data-dx-carousel-index="0">Deploy</button>
        </aside>
      </div>
      <div className="card-grid three">
        <article><small>Style</small><strong>Theme tokens, layout primitives, motion, and icon surfaces stay source-owned and export-safe.</strong></article>
        <article><small>Forge</small><strong>Package slices, media assets, provenance, and rollback receipts are part of the app workflow.</strong></article>
        <article><small>Check</small><strong>Routes, assets, accessibility notes, and product evidence can be scored before public release.</strong></article>
      </div>
    </section>

    {/*
    <section className="landing-section" id="www-studio">
      <p className="eyebrow">WWW + Forge Studio</p>
      <h2>React ecosystem power without the black-hole template workflow.</h2>
      <p className="section-copy">WWW templates are built from source-owned Forge packages and verifiable project evidence, so the app can describe itself instead of hiding behind black-box dependencies.</p>
      <div className="package-chip-grid" aria-label="WWW package proof">
        <span>Better Auth</span><span>Stripe</span><span>React Hook Form</span><span>Zod</span>
        <span>Zustand</span><span>TanStack Query</span><span>Motion</span><span>3D Scene</span>
        <span>react-markdown</span><span>Fumadocs</span><span>Supabase</span><span>Drizzle</span>
        <span>InstantDB</span><span>tRPC</span><span>AI Routes</span>
      </div>
      <div className="card-grid three">
        <article><small>No template node_modules</small><strong>Forge materializes small, inspectable front-facing files instead of dumping dependency chaos into every new project.</strong></article>
        <article><small>Source Edit Contract</small><strong>Stable data-dx markers connect route sections to real source files, tokens, assets, and media slots.</strong></article>
        <article><small>Template</small><strong>Auth, payments, forms, validation, state, query, docs, markdown, 3D, automation, and backend surfaces appear together.</strong></article>
      </div>
    </section>

    */}

    <section className="landing-section" id="providers-token">
      <p className="eyebrow">Providers + Token</p>
      <h2>Model routing and token budgets stay measurable instead of hidden.</h2>
      <div className="signal-strip">
        <article><small>Provider Catalog</small><strong>Local, free, and premium providers share one visible readiness surface.</strong><p>Users can start fast without exposing internal routing details.</p></article>
        <article><small>Routing Policy</small><strong>Choose the right model class for the task instead of wasting premium calls by default.</strong><p>DX keeps provider choice practical, inspectable, and cost-aware.</p></article>
        <article><small>Token Stack</small><strong>Serializer receipts</strong><p>Receipts show prompt, output, tool, and saved-token estimates instead of hiding costs.</p></article>
      </div>
      <div className="card-grid three">
        <article><small>First Run</small><strong>Detect ready providers, ask before importing credentials, and keep Skip visible on every step.</strong></article>
        <article><small>Cost Control</small><strong>Show when Serializer, smaller context, or local execution saved real tokens.</strong></article>
        <article><small>No Lock-In</small><strong>Keep local and cloud provider choices swappable through receipts and config.</strong></article>
      </div>
    </section>

    {/*
    <section className="landing-section" id="agents-automation-browser">
      <p className="eyebrow">Agents, Automations, Browser Control</p>
      <h2>Agent runs, automation surfaces, and visual browser checks share one evidence path.</h2>
      <div className="card-grid three">
        <article><small>Agents</small><strong>CLI-first agents with connection status, automation entrypoints, and background receipts.</strong></article>
        <article><small>Automations</small><strong>Connector metadata becomes a DX automation surface with credential, workflow, and receipt views.</strong></article>
        <article><small>Check Panel</small><strong>Manual and AI-driven browser control can power visual QA, Chrome flows, e2e evidence, and deploy confidence.</strong></article>
      </div>
      <div className="playbook-grid">
        <article><small>Step 1</small><p>Connect model, browser, repo, and sources.</p></article>
        <article><small>Step 2</small><p>Generate or edit UI through WWW routes and source markers.</p></article>
        <article><small>Step 3</small><p>Run visual checks and browser automations.</p></article>
        <article><small>Step 4</small><p>Forge backs up risky operations and records receipts.</p></article>
      </div>
    </section>

    <section className="landing-section" id="built-on-rust">
      <p className="eyebrow">Built on Rust. Not Node.js. Not Electron.</p>
      <h2>Native-grade responsiveness across platforms.</h2>
      <div className="card-grid three">
        <article><small>Speed</small><strong>Near-native performance on every operation.</strong></article>
        <article><small>Efficiency</small><strong>Designed to save RAM and stay responsive under heavy workloads.</strong></article>
        <article><small>Toolchain</small><strong>Rust-powered routing, static export, package manifests, and quality checks.</strong></article>
      </div>
    </section>

    <section className="landing-section" id="generate-anything">
      <p className="eyebrow">Generate Literally Anything</p>
      <h2>If you can name it, DX can generate it.</h2>
      <table className="dx-table">
        <thead><tr><th>Category</th><th>Capabilities</th></tr></thead>
        <tbody>
          <tr><td>Text & Code</td><td>Code generation, completion, refactor, and review</td></tr>
          <tr><td>Images</td><td>Image generation, editing, and token-efficient image workflows</td></tr>
          <tr><td>Video</td><td>Video generation and processing pipelines</td></tr>
          <tr><td>Audio & Music</td><td>Sound design, composition, and voice synthesis</td></tr>
          <tr><td>3D / AR / VR</td><td>3D, AR, and VR asset and scene generation</td></tr>
          <tr><td>Documents & PDFs</td><td>PDFs, specs, reports, and document generation</td></tr>
          <tr><td>Charts & Data</td><td>Visualizations, dashboards, and analysis</td></tr>
          <tr><td>Tool Calling</td><td>MCP, ACP, A2A, and DX DCP-compatible tool execution</td></tr>
          <tr><td>Conversation</td><td>Real-time voice interaction and STT/TTS</td></tr>
        </tbody>
      </table>
    </section>

    */}

    <section className="landing-section" id="token-revolution">
      <p className="eyebrow">Serializer LLM Format</p>
      <h2>DX Serializer `.sr` receipts carry the same data with fewer tokens than JSON.</h2>
      <p className="section-copy">Same Check receipt data, counted with cl100k_base: JSON uses 45 tokens, DX Serializer uses 31 tokens, saving 14 tokens or 31.1%.</p>
      <div className="serializer-compare-grid" aria-label="DX Serializer .sr format versus JSON token comparison">
        <article className="serializer-panel">
          <div className="code-panel-header"><span>JSON</span><strong>45 tokens</strong></div>
          <div className="code-surface json-code" role="img" aria-label="JSON example">
<span className="code-line"><span className="code-punctuation">{"{"}</span></span>
<span className="code-line"><span className="code-key">"tool"</span><span className="code-punctuation">:</span><span className="code-string">"check"</span><span className="code-punctuation">,</span></span>
<span className="code-line"><span className="code-key">"score"</span><span className="code-punctuation">:</span><span className="code-number">472</span><span className="code-punctuation">,</span></span>
<span className="code-line"><span className="code-key">"route"</span><span className="code-punctuation">:</span><span className="code-string">"/"</span><span className="code-punctuation">,</span></span>
<span className="code-line"><span className="code-key">"status"</span><span className="code-punctuation">:</span><span className="code-string">"ready"</span><span className="code-punctuation">,</span></span>
<span className="code-line"><span className="code-key">"findings"</span><span className="code-punctuation">: [</span></span>
<span className="code-line"><span className="code-punctuation">{"{"}</span><span className="code-key">"type"</span><span className="code-punctuation">:</span><span className="code-string">"asset"</span><span className="code-punctuation">,</span><span className="code-key">"count"</span><span className="code-punctuation">:</span><span className="code-number">0</span><span className="code-punctuation">{"}"},</span></span>
<span className="code-line"><span className="code-punctuation">{"{"}</span><span className="code-key">"type"</span><span className="code-punctuation">:</span><span className="code-string">"style"</span><span className="code-punctuation">,</span><span className="code-key">"count"</span><span className="code-punctuation">:</span><span className="code-number">1</span><span className="code-punctuation">{"}"}</span></span>
<span className="code-line code-indent-1"><span className="code-punctuation">]</span></span>
<span className="code-line"><span className="code-punctuation">{"}"}</span></span>
          </div>
        </article>
        <article className="serializer-panel serializer-panel-strong">
          <div className="code-panel-header"><span>DX Serializer .sr</span><strong>31 tokens</strong></div>
          <div className="code-surface sr-code" role="img" aria-label="DX Serializer .sr example">
<span className="code-line"><span className="code-key">tool</span><span className="code-punctuation">:</span><span className="code-string">check</span></span>
<span className="code-line"><span className="code-key">score</span><span className="code-punctuation">:</span><span className="code-number">472</span></span>
<span className="code-line"><span className="code-key">route</span><span className="code-punctuation">:</span><span className="code-string">/</span></span>
<span className="code-line"><span className="code-key">status</span><span className="code-punctuation">:</span><span className="code-string">ready</span></span>
<span className="code-line"><span className="code-key">findings</span><span className="code-punctuation">[</span><span className="code-number">2</span><span className="code-punctuation">]{"{"}</span><span className="code-key">type</span><span className="code-punctuation">,</span><span className="code-key">count</span><span className="code-punctuation">{"}"}:</span></span>
<span className="code-line"><span className="code-string">asset</span><span className="code-punctuation">,</span><span className="code-number">0</span></span>
<span className="code-line"><span className="code-string">style</span><span className="code-punctuation">,</span><span className="code-number">1</span></span>
          </div>
        </article>
      </div>
      <div className="card-grid three">
        <article><small>Measured Saving</small><strong>14 fewer cl100k tokens on this small Check receipt, with the same fields and findings.</strong></article>
        <article><small>Serializer Rows</small><strong>Repeated object keys collapse into one typed row header, so arrays stop wasting context.</strong></article>
        <article><small>DX Path</small><strong>Humans read Serializer receipts; tools can still use generated machine files when speed matters.</strong></article>
      </div>
    </section>

    {/*
    <section className="landing-section" id="works-everywhere">
      <p className="eyebrow">Works Everywhere</p>
      <h2>Native apps and extensions across the development and creative workflow.</h2>
      <table className="dx-table">
        <thead><tr><th>Platform</th><th>App Type</th><th>Status</th></tr></thead>
        <tbody>
          <tr><td>macOS</td><td>Native desktop app</td><td>Ready</td></tr>
          <tr><td>Windows</td><td>Native desktop app</td><td>Ready</td></tr>
          <tr><td>Linux</td><td>Native desktop app</td><td>Ready</td></tr>
          <tr><td>Android</td><td>Mobile app</td><td>Ready</td></tr>
          <tr><td>iOS</td><td>Mobile app</td><td>Ready</td></tr>
          <tr><td>ChromeOS</td><td>Native / web app</td><td>Ready</td></tr>
          <tr><td>Browser</td><td>Remote web console + extension</td><td>Ready</td></tr>
          <tr><td>Developer tools</td><td>CLI, browser console, future connectors</td><td>Ready</td></tr>
          <tr><td>Design and media tools</td><td>Connectors</td><td>Ready</td></tr>
        </tbody>
      </table>
    </section>

    <section className="landing-section" id="free-ai">
      <p className="eyebrow">Free AI Access</p>
      <h2>Any provider, even offline. Own your workflow.</h2>
      <div className="card-grid three">
        <article><small>Online</small><strong>Connect to 180+ LLM providers, open-source models, and self-hosted endpoints.</strong></article>
        <article><small>Offline</small><strong>Run capable local models without internet and without token limits.</strong></article>
        <article><small>Hybrid</small><strong>Switch between cloud and local models based on runtime conditions.</strong></article>
      </div>
      <div className="card-grid two">
        <article><small>Integrations</small><strong>Provider and tool connectors, Cloud CLI skills, WhatsApp, Telegram, Discord, and more.</strong></article>
        <article><small>Forge</small><strong>Version control for code and viral-ready media with bring-your-own storage connectors.</strong></article>
      </div>
    </section>

    <section className="landing-section" id="competitive">
      <p className="eyebrow">Competitive Positioning</p>
      <h2>Technical differences that matter in production workflows.</h2>
      <table className="dx-table">
        <thead><tr><th>Feature</th><th>DX</th><th>Competitors</th></tr></thead>
        <tbody>
          <tr><td>Core Language</td><td>Rust + WWW</td><td>Node.js app stacks</td></tr>
          <tr><td>Token Efficiency</td><td>30-90% savings with Serializer and tokenizers</td><td>No end-to-end optimization</td></tr>
          <tr><td>Serialization</td><td>Serializer, 70-90% savings</td><td>Raw JSON payloads</td></tr>
          <tr><td>Offline Support</td><td>Unlimited and free</td><td>Internet plus paid tiers</td></tr>
          <tr><td>AI Provider Support</td><td>180+ providers + local models</td><td>Locked to 1-3 providers</td></tr>
          <tr><td>Connectors</td><td>Provider and tool connectors + Cloud CLI skills</td><td>Limited integrations</td></tr>
          <tr><td>Media Generation</td><td>Text, images, video, 3D/AR/VR, audio, and docs</td><td>Mostly code only</td></tr>
          <tr><td>Safe Autonomy</td><td>Policy-aware automation with recoverable actions</td><td>Manual review or all-or-nothing</td></tr>
          <tr><td>Platform Coverage</td><td>Desktop, mobile, ChromeOS, companion OS, extensions</td><td>1-2 platforms, limited connectors</td></tr>
        </tbody>
      </table>
    </section>

    */}

    <section className="landing-section" id="forge">
      <p className="eyebrow">Forge Storage Strategy</p>
      <h2>Code and every media type get resilient storage plus distribution-ready workflows.</h2>
      <table className="dx-table">
        <thead><tr><th>Asset Type</th><th>Primary Storage Route</th></tr></thead>
        <tbody>
          <tr><td>Video</td><td>YouTube unlisted or draft</td></tr>
          <tr><td>Images</td><td>Pinterest libraries</td></tr>
          <tr><td>Audio</td><td>SoundCloud or Spotify-like platforms</td></tr>
          <tr><td>3D / AR / VR</td><td>Sketchfab-like storage endpoints</td></tr>
          <tr><td>Code + Docs</td><td>GitHub, GitLab, Bitbucket, single or multi-target</td></tr>
        </tbody>
      </table>
      <div className="playbook-grid">
        <article><small>Forge Step 1</small><p>Generate assets in one DX flow.</p></article>
        <article><small>Forge Step 2</small><p>Route each media type to platform-specific storage.</p></article>
        <article><small>Forge Step 3</small><p>Version every iteration and preserve rollback history.</p></article>
        <article><small>Forge Step 4</small><p>Promote from draft to publish-ready channels.</p></article>
      </div>
    </section>

    {/*
    <section className="landing-section" id="traffic-security">
      <p className="eyebrow">Safe Autonomy</p>
      <h2>Your AI agent stays autonomous and safe.</h2>
      <div className="card-grid three">
        <article><small>Fast</small><strong>Harmless actions stay low-friction.</strong><p>DX keeps everyday work moving without extra ceremony.</p></article>
        <article><small>Clear</small><strong>Risky actions surface the right warning.</strong><p>Users see what matters without drowning in prompts.</p></article>
        <article><small>Recoverable</small><strong>High-risk changes stay reversible.</strong><p>DX is designed around trust, recovery, and professional control.</p></article>
      </div>
      <div className="playbook-grid">
        <article><small>Security Step 1</small><p>Classify action risk in real time.</p></article>
        <article><small>Security Step 2</small><p>Apply the right automation policy.</p></article>
        <article><small>Security Step 3</small><p>Protect sensitive values before outbound calls.</p></article>
        <article><small>Security Step 4</small><p>Create safety snapshot on high-risk operations.</p></article>
      </div>
    </section>

    */}

    <section className="landing-section" id="check">
      <p className="eyebrow">Check</p>
      <h2>500-point code + media quality score that helps you improve.</h2>
      <div className="score-row" aria-label="Check point bands">
        <span>0-100</span><span>101-200</span><span>201-300</span><span>301-400</span><span>401-500</span>
      </div>
      <div className="card-grid three">
        <article>Security and vulnerability scanning.</article>
        <article>Code + media linting for quality consistency.</article>
        <article>Actionable reports with prioritized improvements.</article>
      </div>
    </section>

    <section className="landing-section" id="ecosystem-tools">
      <p className="eyebrow">Media, Serializer, i18n, Driven, DCP</p>
      <h2>Real DX tools for multimodal creation, receipts, and agent workflows.</h2>
      <div className="card-grid three">
        <article>Media owns audio, video, image, and 3D asset tooling as its own DX root package.</article>
        <article>Serializer owns compact `.sr` receipts and generated machine contracts for fast tools and smaller AI context.</article>
        <article>i18n owns localization workflows and future Lingo-compatible translation paths.</article>
        <article>Driven owns repeatable AI development workflows without chaotic prompt juggling.</article>
        <article>DCP owns secure agent/tool communication beside MCP, ACP, and A2A.</article>
        <article>Metasearch, Providers, and Token tools keep search, model routing, and context budgets measurable.</article>
      </div>
    </section>

    {/*
    <section className="landing-section" id="built-in-tools">
      <p className="eyebrow">DX Root Toolchain</p>
      <h2>The landing page now maps to the actual DX toolchain.</h2>
      <div className="card-grid three">
        <article><small>WWW</small><strong>React-familiar TSX routes, App Router shape, source-owned dev/build/check receipts.</strong></article>
        <article><small>Forge</small><strong>Package materialization, source slices, trust scoring, manifests, and receipts.</strong></article>
        <article><small>Style + Icons</small><strong>Generated atomic CSS, theme tokens, event-class styling, and first-party icon search.</strong></article>
        <article><small>Check</small><strong>500-point project quality checks with web performance, source, route, and evidence gates.</strong></article>
        <article><small>Serializer</small><strong>Human-readable, token-efficient, fast serialization with 70-90% smaller payloads than JSON.</strong></article>
        <article><small>Build + JS + Py</small><strong>DX-owned build, JavaScript, and Python lanes for faster local developer tooling.</strong></article>
        <article><small>Media + i18n</small><strong>Media tooling and localization workflows for product assets, language, and voice.</strong></article>
        <article><small>Providers + Token</small><strong>AI provider catalog, model routing, token accounting, and context-saving receipts.</strong></article>
        <article><small>DCP + Driven</small><strong>Secure agent communication and repeatable execution strategy for real sub-agent work.</strong></article>
      </div>
    </section>
    */}

    {/*
    <section className="landing-section" id="extensions">
      <p className="eyebrow">Extensions</p>
      <h2>DX everywhere you already work.</h2>
      <table className="dx-table">
        <thead><tr><th>Category</th><th>Supported Tools</th></tr></thead>
        <tbody>
          <tr><td>Browsers</td><td>Chrome, Safari, Firefox, Edge, Arc, Brave, Opera</td></tr>
          <tr><td>Developer Surfaces</td><td>CLI, browser extension, hosted dashboard, and future editor connectors</td></tr>
          <tr><td>Design Tools</td><td>Figma, Adobe Photoshop, Adobe Illustrator, Sketch, Canva</td></tr>
          <tr><td>Video Editors</td><td>DaVinci Resolve, Adobe Premiere Pro, Final Cut Pro</td></tr>
          <tr><td>Communication</td><td>WhatsApp, Telegram, Discord, Slack, Microsoft Teams</td></tr>
        </tbody>
      </table>
    </section>

    */}

    <section className="landing-section" id="benchmarks">
      <p className="eyebrow">Measured DX Evidence</p>
      <h2>Only receipt-backed measurements make it onto this page.</h2>
      <div className="bench-grid">
        <article className="benchmark-card benchmark-card-wide">
          <span>WWW</span>
          <strong>657 B Brotli and 0.63 ms on the tiny interactive route.</strong>
          <p>The tiny counter route ships 657 B Brotli from WWW, compared with Astro at 729 B, Svelte at 16.05 KB, and Next.js at 158.52 KB in the same receipt set.</p>
          <div className="benchmark-chip-row"><small>WWW 657 B</small><small>Astro 729 B</small><small>Svelte 16.05 KB</small><small>Next.js 158.52 KB</small></div>
        </article>
        <article className="benchmark-card">
          <span>JS</span>
          <strong>DX machine reads reduce parse time across small, medium, and large payloads.</strong>
          <p>The clean June 1 run measured 40 runs with startup excluded: small payloads moved from 14.143 ms to 7.383 ms, medium from 30.103 ms to 13.94 ms, and large from 70.668 ms to 27.903 ms.</p>
          <div className="benchmark-chip-row"><small>small 7.383 ms</small><small>medium 13.94 ms</small><small>large 27.903 ms</small></div>
        </article>
        <article className="benchmark-card">
          <span>Build</span>
          <strong>Build has a governed receipt, with public speed claims still gated.</strong>
          <p>The Build receipt keeps upstream timing marked pending, so this page avoids speed claims until final measurements exist.</p>
          <div className="benchmark-chip-row"><small>timing pending</small><small>upstream pending</small><small>no speed claim</small></div>
        </article>
        <article className="benchmark-card">
          <span>Native</span>
          <strong>Native has machine-cache evidence; startup claims stay locked.</strong>
          <p>The current Native receipts do not claim Tauri startup timing yet, so the page keeps that comparison out of the public story.</p>
          <div className="benchmark-chip-row"><small>cache evidence</small><small>startup pending</small><small>no speed claim</small></div>
        </article>
        <article className="benchmark-card">
          <span>Py</span>
          <strong>Local native-JIT starts at 0.047 s; official Python still wins the broad suite.</strong>
          <p>The May 30 report records startup at 0.047 s versus 0.068 s for the official default, while official Python still leads on int_loop, function_calls, json_regex, and decimal_math.</p>
          <div className="benchmark-chip-row"><small>startup 0.047 s</small><small>official 0.068 s</small><small>broad suite: official</small></div>
        </article>
      </div>
      <p className="section-copy">DX is strongest where the receipts are already clean: WWW route output and JS machine reads. Build, Native, and Py stay honest until their measurement gates are complete.</p>
    </section>

    {/*
    <section className="landing-section" id="testimonials">
      <p className="eyebrow">Developer Testimonials</p>
      <h2>Builder evidence points.</h2>
      <div className="card-grid three">
        <article>"DX cut our token costs by 60% on day one. The compact Serializer receipts alone were worth the switch." <small>- Lena M., Staff Engineer @ Arcforge</small></article>
        <article>"Finally an AI tool that works offline. Deployed DX on a remote rig with no internet, full capability." <small>- Riku S., DevOps Lead @ Shellgrid</small></article>
        <article>"Forge is underrated. We now version all design exports alongside code with zero extra tooling." <small>- Priya K., Senior Developer @ Byteplane</small></article>
      </div>
    </section>

    */}

    <section className="landing-section" id="comparison-table">
      <p className="eyebrow">Comparison Table</p>
      <h2>WWW versus the old web stack.</h2>
      <table className="dx-table">
        <thead><tr><th>Feature</th><th>WWW</th><th>Next.js-style apps</th><th>Vite + npm apps</th></tr></thead>
        <tbody>
          <tr><td>Runtime Shape</td><td>Rust-first WWW</td><td>Node.js server/app router</td><td>Node.js dev server</td></tr>
          <tr><td>Styling</td><td>Style tokens and source-owned CSS</td><td>Tailwind/config-heavy setup</td><td>Plugin-dependent CSS setup</td></tr>
          <tr><td>Package Model</td><td>Forge source slices and receipts</td><td>node_modules dependency graph</td><td>node_modules dependency graph</td></tr>
          <tr><td>Quality Gates</td><td>Check route, asset, and product evidence</td><td>App-specific scripts</td><td>App-specific scripts</td></tr>
          <tr><td>Token Efficiency</td><td>Serializer receipts</td><td>Usually external</td><td>Usually external</td></tr>
          <tr><td>Automation</td><td>Agents, browser checks, and Forge backups</td><td>Custom integration work</td><td>Custom integration work</td></tr>
          <tr><td>Media Workflow</td><td>Code + media assets with Forge provenance</td><td>App-owned manual setup</td><td>App-owned manual setup</td></tr>
          <tr><td>Static Export</td><td>Production-ready static bundle path</td><td>Config-dependent</td><td>Config-dependent</td></tr>
        </tbody>
      </table>
    </section>

    <section className="landing-cta" id="start-building">
      <p className="eyebrow">Start Building</p>
      <h2>Build with the DX ecosystem.</h2>
      <p>Use WWW, Forge, Style, Icons, Check, Serializer, Build, JS, Py, Media, Providers, Token tools, DCP, and Driven from one system.</p>
      <a href="mailto:hello@dx.ai?subject=DX%20Ecosystem" className="primary-action">Start With DX</a>
    </section>

    <footer className="dx-footer border-t border-border overflow-hidden" data-dx-section="footer">
      <div className="footer-topline flex flex-wrap items-center justify-between gap-4">
        <a className="footer-brand" href="/" aria-label="DX home footer">
          <img className="brand-logo" src="/logo.svg" alt="Dx WWW" loading="lazy" decoding="async" />
          <span>Enhanced Development Experience</span>
        </a>
      </div>

      <div className="footer-callout flex flex-wrap items-center justify-between gap-4 rounded-lg border border-border bg-card p-5">
        <span className="text-xs font-black uppercase text-muted-foreground">DX System Stack</span>
        <strong>WWW. Forge. Style. Icons. Check. Serializer. Build. JS. Py. Providers. DCP.</strong>
      </div>

      <div className="footer-theme-row flex flex-wrap items-center justify-between gap-4">
        <div>
          <span>Theme</span>
          <strong>Clean light mode. WWW dark mode.</strong>
        </div>
        <div className="theme-switcher" aria-label="Theme selector">
          <button type="button" data-theme-choice="light">Light</button>
          <button type="button" data-theme-choice="dark">Dark</button>
          <button type="button" data-theme-choice="system">System</button>
        </div>
      </div>

      <div className="footer-link-grid grid gap-8 py-10 md:grid-cols-3">
        <nav className="flex flex-col gap-2" aria-label="Product links">
          <strong>Product</strong>
          <a href="#www-platform">WWW Platform</a>
          <a href="#token-revolution">Serializer</a>
          <a href="#providers-token">Providers + Token</a>
          <a href="#benchmarks">Evidence</a>
        </nav>
        <nav className="flex flex-col gap-2" aria-label="Platform links">
          <strong>Platform</strong>
          <a href="#forge">Forge</a>
          <a href="#check">Check</a>
          <a href="#ecosystem-tools">Ecosystem Tools</a>
          <a href="#benchmarks">Evidence</a>
        </nav>
        <nav className="flex flex-col gap-2" aria-label="Evidence links">
          <strong>Evidence</strong>
          <a href="#token-revolution">JSON vs Serializer</a>
          <a href="#comparison-table">Comparison</a>
          <a href="#benchmarks">Evidence</a>
          <a href="#start-building">Start Building</a>
        </nav>
      </div>

      <div className="footer-status-strip flex flex-wrap justify-between gap-2" aria-label="DX system points">
        <span className="rounded-full border border-border px-3 py-2 text-xs font-black uppercase text-muted-foreground">WWW</span>
        <span className="rounded-full border border-border px-3 py-2 text-xs font-black uppercase text-muted-foreground">Forge</span>
        <span className="rounded-full border border-border px-3 py-2 text-xs font-black uppercase text-muted-foreground">Serializer</span>
        <span className="rounded-full border border-border px-3 py-2 text-xs font-black uppercase text-muted-foreground">Icons</span>
        <span className="rounded-full border border-border px-3 py-2 text-xs font-black uppercase text-muted-foreground">Build / JS / Py</span>
        <span className="rounded-full border border-border px-3 py-2 text-xs font-black uppercase text-muted-foreground">Providers + DCP</span>
      </div>

      <div className="footer-mega" aria-label="DX footer identity">
        <strong className="footer-mega-word block font-black uppercase">I use Dx BTW</strong>
        <strong className="footer-mega-word footer-mega-word-long block font-black uppercase">
          Enhanced Development Experience
        </strong>
      </div>
    </footer>
    <script src="/public/dx-landing-runtime.js" defer></script>
</main>
  );
}
