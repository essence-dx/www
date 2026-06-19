# DX-WWW Game-Changing Ideas Report

Generated from the local `benchmarks/binary-web-lab` experiment.

## Honest Thesis

DX-WWW should not try to be "React but in WASM." That would lose too many tiny-page and ecosystem cases.

The bigger invention is an adaptive web compiler that understands the shape of a page and ships the cheapest valid representation:

- Static HTML/CSS when the page is static.
- Micro-JS when a tiny interaction is smaller than WASM.
- Template/data packets when UI repeats.
- Columnar packets when repeated slots form table-like data.
- Semantic codecs when data has predictable structure.
- Patch streams when only part of the UI changes.
- Server fragments when server-owned state is cheaper than client runtime.

## Lab Signals

### 1. Template/Data Packets

For a 1200-row dashboard:

- HTML string: `203,444` raw / `3,912` Brotli.
- JSON graph: `547,068` raw / `10,870` Brotli.
- DX template slots: `43,628` raw / `2,675` Brotli.

Verdict: strong. This is the foundation.

### 2. Columnar Slot Encoding

For the same dashboard:

- DX template slots: `43,628` raw.
- DX columnar slots: `21,655` raw.

Verdict: strong for dashboards, tables, grids, CMS collections, pricing pages, ecommerce product lists, admin apps, docs navs, and repeated cards.

### 3. Semantic Codecs

For generated dashboard data:

- DX semantic codec: `22` raw / `26` Brotli.

Verdict: extremely strong, but not universal. Use only when the compiler can prove the shape: numeric ranges, repeated prefixes, enum sets, date sequences, price columns, route lists, or generated CMS fields.

### 4. Viewport-First Delivery

For a 1200-row table where only 40 rows are initially visible:

- DX viewport-40 packet: `638` raw / `199` Brotli.

Verdict: strong if it is built into the compiler and not left to manual virtualization. This attacks perceived speed.

### 5. Binary Patch Streams

For 12 changed rows:

- HTML row fragments: `1,187` raw / `147` Brotli.
- JSON cell patch: `1,152` raw / `163` Brotli.
- DX cell patch: `145` raw / `134` Brotli.

For a 600-row bulk status update:

- HTML row fragments: `102,020` raw / `2,512` Brotli.
- JSON range op: `74` raw / `66` Brotli.
- DX range op: `10` raw / `14` Brotli.

Verdict: this is the live-app wedge. UI updates should be shipped as intent/opcodes, not component trees.

## The Out-of-the-Box Product Idea

Build DX-WWW as a meaning-aware web compiler:

1. It reads source-owned components.
2. It detects repeated structures and data shapes.
3. It compiles templates, slots, patches, and semantic codecs.
4. It chooses the smallest delivery mode per page and per interaction.
5. It produces an explainable profile report: "This page is 82% smaller than the Next.js baseline because rows were encoded as columnar enum slots and range patches."

That report is part of the product. Developers and business owners can both understand it.

## Where DX-WWW Can Beat Current Frameworks

- WordPress-style sites with plugin bloat.
- Agency and creator websites.
- CMS-driven marketing sites.
- Docs sites with repeated components.
- Admin dashboards.
- CRUD apps.
- Ecommerce category/product-list pages.
- SaaS settings/billing/team pages.
- Sites where most UI is repeated structure with different content.

## Where DX-WWW Will Not Automatically Win

- Tiny static pages unless the compiler emits no runtime.
- Tiny interactive pages unless the compiler emits micro-JS.
- Arbitrary rich apps that need large third-party browser SDKs.
- Highly custom canvas/WebGL/editor/map apps where the payload is dominated by domain libraries.
- Cases where semantic compression guesses incorrectly.

## Architecture To Build Next

- `DxDeliveryPlanner`: chooses static, micro-JS, template-slots, columnar-slots, semantic-codec, patch-stream, range-op, or server-fragment.
- `DxTemplate`: stable cached component structure.
- `DxSlotSchema`: typed slot layout for text, numbers, booleans, attrs, classes, enums, and event bindings.
- `DxColumnBatch`: columnar repeated slot data.
- `DxSemanticCodec`: proven compact representation for ranges, enums, prefixes, dates, prices, routes, and generated fields.
- `DxPatchStream`: compact runtime update operations.
- Browser benchmark harness: apply packets to real DOM templates and measure paint, scripting, memory, and interaction time.

## Strategic Verdict

The billion-dollar angle is not "we have a binary format."

The angle is "DX can make websites smaller because it understands the product structure better than generic JavaScript frameworks do."

That is a real innovation path. It is also hard. The next proof must be one real vertical slice that emits these modes from source, runs in the browser, and beats good baselines without hand-written benchmark shortcuts.

## External Signals

- Google Search is already using Compression Dictionary Transport for repeated HTML and reported average HTML payload savings in Chrome users, with bigger savings for dictionary-compressed results: https://developer.chrome.com/blog/search-compression-dictionaries
- The Speculation Rules API can make future navigations faster, but it is not baseline everywhere and must be used as progressive enhancement: https://developer.mozilla.org/en-US/docs/Web/API/Speculation_Rules_API
- WASM should use streaming instantiation when it is actually needed: https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiateStreaming_static
