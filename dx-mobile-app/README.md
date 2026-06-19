# DX Mobile App

DX Native companion powered by DX WWW.

The WWW route is the app UI, and `src-tauri` is the native WebView host
boundary. The same source shape can be materialized for another WWW app with:

```powershell
dx www native-shell --target tauri --project . --write
```

Build the native Android debug app through the DX WWW build lane:

```powershell
dx www build --target android
```

That command writes `.dx/native/android-build-receipt.json` and expects the
arm64 debug APK at
`src-tauri/gen/android/app/build/outputs/apk/arm64/debug/app-arm64-debug.apk`.
The debug APK proof is receipt-backed; release signing, emulator install, and
device smoke proof remain separate follow-up gates.

- `app/page.tsx` mounts the QR pairing and Zed chat companion route.
- `components/mobile-companion/*` renders the real companion surface.
- `public/mobile-companion-runtime.js` connects the route to the DX Agents
  gateway without storing bearer tokens in browser storage.
- `lib/mobile-companion/companion-contract.ts` and
  `server/mobile-companion/readiness.ts` publish the native WWW artifact paths
  and `dx www build --target android` refresh command before native proof is
  claimed.
- `src-tauri/tauri.conf.json` points native dev at `http://127.0.0.1:3000`
  and production at `../.dx/www/output`.
- `src-tauri/src/main.rs` exposes the native companion bridge commands.
- `.dx/native/native-shell-receipt.json` records the WWW-to-native bridge.
- `auth/better-auth/*` remains the Forge-owned Authentication package
  provenance for the app, while live pairing/chat routes are served by the DX
  Agents gateway.
- `styles/*` are authored for DX Style and regenerated into `styles/generated.css`.

There is no phone-frame mock UI. The same route is what DX WWW serves in the
browser and what DX Native hosts in the WebView.

Runtime proof remains separate: gateway pairing, native secure storage,
production output, Android/iOS release signing, emulator install, device smoke
proof, and WebView run receipts are not claimed by this source receipt.

DX Mobile identity validation is contract and readiness owned. Concrete session
IDs, pairing codes, bearer tokens, and agent runtime credentials must not be
rendered into product UI or public DOM attributes.
