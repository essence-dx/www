# DX Forge Package: `www/minimal-starter`

- Variant: `default`
- Version: `0.1.0`
- Upstream: `dx-www/examples/template`
- Generator: `dx-forge/local-source`
- License: `MIT`
- Provenance: `dx-forge-local-source` (verified: `no`)
- Advisory coverage: `missing` via `none` (live: `no`, findings: `0`)
- License review: declared `MIT` (reviewed: `no`)
- Last action: `TrackWrite`
- Risk score: `100`

This package is source-owned. The files below are editable project files, not opaque `node_modules` content. Forge tracks their hashes, treats local edits as reviewable yellow traffic, blocks red/security-sensitive traffic, and updates them through `dx update www/minimal-starter`.

## Package Metadata Review

- Provenance note: Local source was tracked by Forge, but no external upstream provenance is claimed.
- Advisory note: Local source packages do not have live advisory coverage attached by Forge yet.
- License review note: License is recorded from the local package declaration only; no formal DX legal review is claimed.

## Materialized Files

| File | Logical Source | Bytes | Hash |
| --- | --- | ---: | --- |
| `app/page.tsx` | `app/page.tsx` | `651` | `680091d8e63daf0ea482d9427ae3e804d0661c192f3a940de20a9c046b45b1e8` |
| `app/layout.tsx` | `app/layout.tsx` | `381` | `db55d333539d31c2ad3fde13cfb5f807d71a2d67c0bb212b12693820a18cf66e` |
| `styles/theme.css` | `styles/theme.css` | `549` | `d1f26f0a95c6a89406bb31b5e7412f065ed1099832c9285f5fd6f0f271ca1e16` |
| `styles/generated.css` | `styles/generated.css` | `7011` | `6453847bf4cd52d2ae841337303c85200ac7d2abb9aa281ee9b04477784353a8` |
| `styles/globals.css` | `styles/globals.css` | `3640` | `57f00f8d0a8b89778edcdbd85f311477bd4ad315071d0dc69d0a0f83d66c57e0` |
| `components/icons/icon.tsx` | `components/icons/icon.tsx` | `475` | `1a91cff776d4e0b07e2184af991e0ec43214c222fa04f6c00b17df9c541174b4` |
| `lib/utils.ts` | `lib/utils.ts` | `196` | `f53a1ea0634c2cbfbb6275d9cd6f58503bf519d4b7c0ebe06a3c6375b2cf6dec` |
| `public/logo.svg` | `public/logo.svg` | `910` | `2afe23f6d7f297ab8563ee05015907d2d0f1903344c7e8b1b483835f0d78ca3b` |
| `public/icon.svg` | `public/icon.svg` | `869` | `32341047d202fbff170eee4493d889a5c3b79eb377ddeb284da8a0956070c2a4` |
| `public/favicon.svg` | `public/favicon.svg` | `874` | `cfb09e06980b21ac4df558fc791be8cef96c7ab79ab0d8b0eb0d8a95f7c1cd76` |
| `vercel.json` | `vercel.json` | `525` | `feecdba5a814293bb225e931057c2f6eb314a3034479f57b7b1a24edfbc3a081` |
| `.gitignore` | `.gitignore` | `30` | `c7bdb645502d81f18e2ceca7fde809eef64d89d87c29fdf3551bc22d14903d2f` |
| `README.md` | `README.md` | `316` | `e9186ba3ff2f440ba8a7526bc90b290205e4c4d108a388d3762315ae2989e819` |

## Forge Policy

| Traffic | Policy | Decision |
| --- | --- | --- |
| `green` | `source-owned-inputs` | Forge records existing local source files as editable, reviewable project-owned inputs. |
| `green` | `no-lifecycle-execution` | Forge tracking does not run npm install, lifecycle hooks, or upstream scripts. |
