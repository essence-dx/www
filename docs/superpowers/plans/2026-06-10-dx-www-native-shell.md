# DX WWW Native Shell Bridge Plan

## Goal

Make DX WWW able to materialize a source-owned DX Native/Tauri shell for an existing WWW app, without fake phone UI, package installs, or native builds.

## Constraints

- Keep `dx dev`, browser automation, full Rust builds, and broad readiness gates off while CPU is saturated.
- Touch `cli_core_impl.rs` only for narrow command dispatch.
- Do not stage unrelated dirty work in `examples/` or pre-existing modified files.
- Treat `src-tauri` as the native WebView host boundary; the WWW route remains the app UI.

## Steps

1. Add a failing source contract for `dx www native-shell`.
2. Add `native_shell_options.rs` for parsing `--target tauri`, `--project`, `--plan`, `--write`, `--force`, `--format`, `--output`, and native identity options.
3. Add `native_shell_command.rs` for validation, plan rendering, source materialization, and `.dx/native/native-shell-receipt.json`.
4. Wire `dx www native-shell` and `dx www native` in the CLI with help text.
5. Align `dx-mobile-app` to the reusable contract: port agreement, receipt, honest auth readiness, typed layout, and no unimplemented visible controls.
6. Run lightweight focused source checks. Defer native compile/package proof unless CPU drops.
7. Stage only the owned paths, commit, and push to `origin/dev` if the remote remains compatible.
