# All-Size Framework Comparison

Generated: 2026-05-18T08:46:42.492Z

## Method

- Actual tiny route: production/minimal counter builds measured for DX-WWW, Next.js, Svelte, Astro, and HTMX.
- All-size rows: compiler packet lab over equivalent generated page/update shapes. These are payload/compiler results, not full Lighthouse/browser-app scores.
- Current latest npm versions checked during this run: next 16.2.6, react 19.2.6, svelte 5.55.7, astro 6.3.3, htmx 2.0.10.

## Matrix

| Scenario | Best current baseline | DX-WWW current/adaptive path | Winner | Honest verdict |
| --- | --- | --- | --- | --- |
| Tiny interactive counter, actual browser route | Astro: 2,722 B raw / 1,007 B gzip / 729 B Brotli / 1.465 ms route median | DX-WWW: 1,867 B raw / 919 B gzip / 657 B Brotli / 0.63 ms route median | DX-WWW current micro route | DX-WWW is 9.9% smaller than Astro on Brotli and ships no WASM for this tiny interaction; DX packet lab is 15.51x smaller than Astro but is not yet the shipped route. |
| Tiny interactive counter, React framework baseline | Next.js: 636,239 B raw / 187,903 B gzip / 162,325 B Brotli / 20.361 ms route median | DX-WWW: 1,867 B raw / 919 B gzip / 657 B Brotli / 0.63 ms route median | DX-WWW current demo | Current DX-WWW demo is 247.07x smaller than Next.js on Brotli. |
| Tiny interactive counter, compiled JS baseline | Svelte: 47,774 B raw / 18,437 B gzip / 16,439 B Brotli / 3.491 ms route median | DX-WWW: 1,867 B raw / 919 B gzip / 657 B Brotli / 0.63 ms route median | DX-WWW current demo | Current DX-WWW demo is 25.02x smaller than Svelte on Brotli. |
| Static/repeated docs, 160 sections | html-string: 36,349 B raw / 786 B gzip / 463 B Brotli / 17420 ns packet access | dx-template-data: 21,991 B raw / 620 B gzip / 391 B Brotli / 930 ns packet access | DX-WWW adaptive template/data | DX-WWW smaller by 15.6% Brotli |
| Repeated marketing cards, 180 cards | html-string: 40,617 B raw / 876 B gzip / 527 B Brotli / 19335 ns packet access | dx-template-data: 18,271 B raw / 597 B gzip / 433 B Brotli / 1505 ns packet access | DX-WWW adaptive template/data | DX-WWW smaller by 17.8% Brotli |
| Large dashboard, full 1200-row data | html-string: 203,444 B raw / 7,876 B gzip / 3,912 B Brotli / 93285 ns packet access | dx-template-data: 43,628 B raw / 6,256 B gzip / 2,675 B Brotli / 13280 ns packet access | DX-WWW template/data on Brotli; columnar wins raw/access | DX-WWW smaller by 31.6% Brotli; columnar is 89.4% smaller raw but worse than template on Brotli for this synthetic data. |
| Large dashboard, initial 40-row viewport | html-string: 203,444 B raw / 7,876 B gzip / 3,912 B Brotli / 93285 ns packet access | dx-viewport-40: 638 B raw / 267 B gzip / 199 B Brotli / 182 ns packet access | DX-WWW viewport packet | DX-WWW smaller by 94.9% Brotli |
| 12-row live update | html-row-fragments: 1,187 B raw / 187 B gzip / 147 B Brotli / 457 ns packet access | dx-cell-patch: 145 B raw / 153 B gzip / 134 B Brotli / 85 ns packet access | DX-WWW patch stream, but only slightly after Brotli | DX-WWW smaller by 8.8% Brotli |
| 600-row bulk update | json-range-op: 74 B raw / 90 B gzip / 66 B Brotli / 560 ns packet access | dx-range-op: 10 B raw / 30 B gzip / 14 B Brotli / 5 ns packet access | DX-WWW range op | DX-WWW smaller by 78.8% Brotli |

## Honest Verdict

- Today: Current DX-WWW now beats Astro, Next.js, Svelte, and HTMX on the tiny measured route by using the micro-JS/no-WASM path.
- If wired correctly: The adaptive compiler direction is genuinely strong for repeated UI, dashboards, partial updates, source-owned package registries, and framework hosting, but it must keep static/micro-JS/no-WASM paths automatic for tiny pages.
- Non-tech pitch: Non-technical buyers will care if this becomes cheaper hosting, faster dashboards, smaller websites, safer editable packages, and easier maintenance. They will not care about binary packets by name.
- Biggest flaw: The compiler packet wins are not yet the complete product. The missing layer is production runtime selection, DOM apply benchmarking, DX registry/versioning workflow, framework-compatible integrations, and proof on real websites.

## Demanding Product Read

- The invention is real enough to be worth building, because the payload wins appear exactly where modern apps hurt: repeated UI, dashboards, updates, and dependency packaging.
- It is not yet a Next.js killer as a product. It is currently a compiler thesis with promising packet evidence and an early demo.
- To become a billion-dollar-grade platform, the next proof must be a real website builder flow: compile a shadcn-style app, emit static/micro/wasm plans, deploy it, compare real browser metrics, and show safe editable dependency updates.
