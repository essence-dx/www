# DX Mobile Professional Hardening Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make DX Mobile a professional Android/iOS companion for Zed and DX Agents with real QR pairing, secure credential persistence, read-safe chat access, production output proof, and native build receipts.

**Architecture:** Keep `dx-mobile-app` as the DX WWW app and Tauri shell boundary, with the mobile route owning production-rendered HTML. Put durable device secrets behind native Android Keystore and iOS Keychain adapters, keep browser storage token-free, and route Zed chat reads through the ZeroClaw/DX Agents gateway with redaction and pagination. Do not mark readiness flags as proven until the matching runtime receipt exists.

**Tech Stack:** DX WWW App Router, DX Native/Tauri 2, `tauri-plugin-barcode-scanner`, Rust, Android Kotlin, iOS Swift, ZeroClaw gateway, Node benchmark contracts.

---

## File Structure

- `dx-mobile-app/app/page.tsx`: production-rendered route shell and DOM hooks.
- `dx-mobile-app/public/mobile-companion-runtime.js`: pairing, scanner, session, socket, and token-in-memory runtime.
- `dx-mobile-app/lib/mobile-companion/companion-contract.ts`: public mobile contract and gateway endpoint declarations.
- `dx-mobile-app/server/mobile-companion/readiness.ts`: honest readiness flags and proof status.
- `dx-mobile-app/src-tauri/src/credential_store.rs`: Rust command facade and validation for device credential storage.
- `dx-mobile-app/src-tauri/src/pairing.rs`: QR payload parsing, origin validation, and expiry/token rejection.
- `dx-mobile-app/src-tauri/src/scanner.rs`: scanner command/status facade.
- `dx-mobile-app/src-tauri/plugins/tauri-plugin-dx-mobile-secure-storage/`: native secure-storage plugin owned by DX Mobile.
- `dx-mobile-app/src-tauri/capabilities/mobile.json`: mobile IPC permissions.
- `G:/Dx/agent/crates/zeroclaw-gateway/src/api_pairing.rs`: pairing/device registry gateway contract.
- `G:/Dx/agent/crates/zeroclaw-gateway/src/lib.rs`: route registration and endpoint wiring.
- `G:/Dx/agent/crates/zeroclaw-gateway/src/ws.rs`: live chat WebSocket contract.
- `benchmarks/dx-mobile-app-source-contract.test.cjs`: source-level app contract.
- `benchmarks/cli-www-native-shell-command.test.ts`: native-shell fixture contract.
- `benchmarks/dx-mobile-app-production-output-contract.test.cjs`: production artifact marker contract.
- `dx-mobile-app/README.md`: operator build/pairing instructions and honest proof boundaries.

---

### Task 1: Pair Through The Device Registry Path

**Files:**
- Modify: `dx-mobile-app/public/mobile-companion-runtime.js`
- Modify: `dx-mobile-app/lib/mobile-companion/companion-contract.ts`
- Modify: `benchmarks/dx-mobile-app-source-contract.test.cjs`

- [ ] **Step 1: Write the failing contract**

Require the runtime to pair through `/api/pair` and only keep `/pair` as a legacy fallback if the gateway explicitly rejects JSON pairing.

```js
assert.match(runtime, /deviceRegistryPairEndpoint/);
assert.match(runtime, /fetch\(`\$\{gatewayOrigin\}\/api\/pair`/);
assert.match(runtime, /device_name: "DX Mobile App"/);
assert.match(runtime, /device_type: "mobile"/);
assert.doesNotMatch(runtime, /fetch\(`\$\{gatewayOrigin\}\/pair`[\s\S]*method: "POST"[\s\S]*X-Pairing-Code/);
```

- [ ] **Step 2: Run the failing test**

Run: `node --test benchmarks/dx-mobile-app-source-contract.test.cjs`

Expected before implementation: FAIL on `/api/pair` runtime assertions.

- [ ] **Step 3: Implement registry pairing**

Change `pairWithGateway` to:

```js
async function pairWithGateway(origin, code) {
  const gatewayOrigin = validateGatewayOrigin(origin);
  const response = await fetch(`${gatewayOrigin}/api/pair`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      code,
      device_name: "DX Mobile App",
      device_type: "mobile",
    }),
  });

  if (!response.ok) {
    const detail = await response.text().catch(() => response.statusText);
    throw new Error(`Pairing failed (${response.status}): ${detail || response.statusText}`);
  }

  return response.json();
}
```

- [ ] **Step 4: Verify pairing contract**

Run: `node --test benchmarks/dx-mobile-app-source-contract.test.cjs benchmarks/cli-www-native-shell-command.test.ts`

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add -- dx-mobile-app/public/mobile-companion-runtime.js dx-mobile-app/lib/mobile-companion/companion-contract.ts benchmarks/dx-mobile-app-source-contract.test.cjs
git commit -m "Route DX Mobile pairing through device registry"
```

---

### Task 2: Add Native Secure Credential Storage

**Files:**
- Create: `dx-mobile-app/src-tauri/plugins/tauri-plugin-dx-mobile-secure-storage/Cargo.toml`
- Create: `dx-mobile-app/src-tauri/plugins/tauri-plugin-dx-mobile-secure-storage/build.rs`
- Create: `dx-mobile-app/src-tauri/plugins/tauri-plugin-dx-mobile-secure-storage/src/lib.rs`
- Create: `dx-mobile-app/src-tauri/plugins/tauri-plugin-dx-mobile-secure-storage/src/mobile.rs`
- Create: `dx-mobile-app/src-tauri/plugins/tauri-plugin-dx-mobile-secure-storage/android/src/main/java/com/dx/mobile/storage/SecureStoragePlugin.kt`
- Create: `dx-mobile-app/src-tauri/plugins/tauri-plugin-dx-mobile-secure-storage/ios/Sources/SecureStoragePlugin.swift`
- Modify: `dx-mobile-app/src-tauri/Cargo.toml`
- Modify: `dx-mobile-app/src-tauri/src/credential_store.rs`
- Modify: `dx-mobile-app/src-tauri/src/lib.rs`
- Modify: `dx-mobile-app/server/mobile-companion/readiness.ts`

- [ ] **Step 1: Write failing source guards**

Add assertions to `benchmarks/dx-mobile-app-source-contract.test.cjs`:

```js
assert.match(tauriCargo, /tauri-plugin-dx-mobile-secure-storage/);
assert.match(tauriLib, /tauri_plugin_dx_mobile_secure_storage::init\(\)/);
assert.doesNotMatch(tauriCredentialStore, /disabled-no-plaintext-fallback/);
assert.match(tauriCredentialStore, /secure-native-keychain-keystore/);
```

- [ ] **Step 2: Create the plugin crate**

Use the repo-local Tauri plugin pattern from `G:/Dx/native/examples/api/src-tauri/tauri-plugin-sample`. The plugin commands must be exactly `store`, `load`, and `clear`.

- [ ] **Step 3: Implement Android storage**

Use Android Keystore AES-GCM with a non-exportable app key and store ciphertext in private `SharedPreferences`. The raw bearer token must never appear in the XML file.

- [ ] **Step 4: Implement iOS storage**

Use Keychain generic password with `kSecAttrAccessibleWhenUnlockedThisDeviceOnly` and service `com.essencefromexistence.dx.mobile`.

- [ ] **Step 5: Wire Rust facade**

`credential_store_status()` should return:

```rust
SecureCredentialStoreStatus {
    available: true,
    durable: true,
    adapter: "secure-native-keychain-keystore",
    token_persistence: "native-secure-storage",
    reason: None,
}
```

- [ ] **Step 6: Verify locally**

Run:

```powershell
cargo fmt --manifest-path .\dx-mobile-app\src-tauri\Cargo.toml -- --check
cargo check --manifest-path .\dx-mobile-app\src-tauri\Cargo.toml -j 1
cargo test --manifest-path .\dx-mobile-app\src-tauri\Cargo.toml -j 1
node --test .\benchmarks\dx-mobile-app-source-contract.test.cjs
```

- [ ] **Step 7: Commit**

```powershell
git add -- dx-mobile-app/src-tauri dx-mobile-app/server/mobile-companion/readiness.ts benchmarks/dx-mobile-app-source-contract.test.cjs
git commit -m "Add native secure storage for DX Mobile"
```

---

### Task 3: Prove Production WWW Output

**Files:**
- Create: `benchmarks/dx-mobile-app-production-output-contract.test.cjs`
- Modify: `benchmarks/cli-www-native-shell-command.test.ts`

- [ ] **Step 1: Add production output test**

Create a test that assumes `dx www build` has run and validates `dx-mobile-app/.dx/www/output/app/index.html`:

```js
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const htmlPath = path.resolve(__dirname, "../dx-mobile-app/.dx/www/output/app/index.html");

test("DX Mobile production artifact contains the real companion screen", () => {
  assert.ok(fs.existsSync(htmlPath), "run `dx www build` before this contract");
  const html = fs.readFileSync(htmlPath, "utf8");
  assert.match(html, /data-dx-mobile-app="companion"/);
  assert.match(html, /name="agent_alias"/);
  assert.match(html, /data-dx-chat-send="\/ws\/chat"/);
  assert.match(html, /data-dx-chat-list-endpoint="\/api\/sessions"/);
  assert.match(html, /data-dx-icon="status:check"/);
  assert.match(html, /\/mobile-companion-runtime\.js/);
  assert.doesNotMatch(html, /mobile-device-frame|phone-mock|iphone-frame|bezel|notch/);
});
```

- [ ] **Step 2: Verify build plus artifact**

Run:

```powershell
Set-Location G:\Dx\www\dx-mobile-app
dx www build
Set-Location G:\Dx\www
node --test .\benchmarks\dx-mobile-app-production-output-contract.test.cjs
```

Expected: PASS.

- [ ] **Step 3: Commit**

```powershell
git add -- benchmarks/dx-mobile-app-production-output-contract.test.cjs benchmarks/cli-www-native-shell-command.test.ts
git commit -m "Prove DX Mobile production output"
```

---

### Task 4: Add Gateway Session Read Contracts

**Files:**
- Modify: `G:/Dx/agent/crates/zeroclaw-gateway/src/api_pairing.rs`
- Modify: `G:/Dx/agent/crates/zeroclaw-gateway/src/lib.rs`
- Modify: `G:/Dx/agent/crates/zeroclaw-gateway/src/openapi.rs`
- Modify: `dx-mobile-app/public/mobile-companion-runtime.js`
- Modify: `dx-mobile-app/lib/mobile-companion/companion-contract.ts`

- [ ] **Step 1: Add gateway tests**

Add tests proving:
- `/api/pair` creates a device registry row.
- `/api/devices/me/capabilities` succeeds after `/api/pair`.
- `/api/sessions` returns redacted session summaries.
- `/api/sessions/:session_key/messages` caps message count and redacts tool payloads.

- [ ] **Step 2: Add OpenAPI coverage**

Document `/api/pair`, `/api/devices`, `/api/devices/me/capabilities`, `/api/sessions`, and `/api/sessions/{session_key}/messages`.

- [ ] **Step 3: Verify gateway**

Run from `G:/Dx/agent`:

```powershell
cargo test -p zeroclaw-gateway -j 1 api_pairing
cargo test -p zeroclaw-gateway -j 1 sessions
```

- [ ] **Step 4: Commit**

```powershell
git add -- G:/Dx/agent/crates/zeroclaw-gateway/src/api_pairing.rs G:/Dx/agent/crates/zeroclaw-gateway/src/lib.rs G:/Dx/agent/crates/zeroclaw-gateway/src/openapi.rs
git commit -m "Document and verify DX Mobile gateway sessions"
```

---

### Task 5: Add Mobile Runtime State Machine

**Files:**
- Modify: `dx-mobile-app/public/mobile-companion-runtime.js`
- Modify: `dx-mobile-app/app/page.tsx`
- Modify: `dx-mobile-app/styles/mobile-companion/forms.css`
- Modify: `benchmarks/dx-mobile-app-source-contract.test.cjs`

- [ ] **Step 1: Add state contract**

Assert scanner and pairing states exist:

```js
assert.match(runtime, /pairingState/);
assert.match(runtime, /scannerState/);
assert.match(runtime, /socketState/);
assert.match(runtime, /permission-denied/);
assert.match(runtime, /paired-session-only/);
assert.match(runtime, /connected-live-chat/);
```

- [ ] **Step 2: Implement explicit states**

Use these exact state values:

```js
const pairingState = {
  idle: "idle",
  parsing: "parsing",
  pairing: "pairing",
  pairedSessionOnly: "paired-session-only",
  pairedDurable: "paired-durable",
  failed: "failed",
};
```

- [ ] **Step 3: Verify no silent failures**

Every `catch` path must call `writeStatus(error.message, "danger")` or a more specific warning. No empty catches around pairing, credential storage, session loading, or socket connect.

- [ ] **Step 4: Commit**

```powershell
git add -- dx-mobile-app/public/mobile-companion-runtime.js dx-mobile-app/app/page.tsx dx-mobile-app/styles/mobile-companion/forms.css benchmarks/dx-mobile-app-source-contract.test.cjs
git commit -m "Add explicit DX Mobile runtime states"
```

---

### Task 6: Android And iOS Build Receipts

**Files:**
- Modify: `dx-mobile-app/README.md`
- Modify: `dx-mobile-app/.dx/native/native-shell-receipt.json`
- Create: `dx-mobile-app/.dx/receipts/native/android-debug-build.json`
- Create: `dx-mobile-app/.dx/receipts/native/ios-source-build-plan.json`

- [ ] **Step 1: Android debug build**

Run only when CPU is low:

```powershell
Set-Location G:\Dx\www\dx-mobile-app
dx www build
cargo tauri android build --debug -j 1
```

Expected: Android debug APK or explicit SDK/toolchain blocker.

- [ ] **Step 2: Android runtime smoke**

On an emulator/device:

```powershell
adb install -r .\src-tauri\gen\android\app\build\outputs\apk\universal\debug\app-universal-debug.apk
adb shell monkey -p com.essencefromexistence.dx.mobile 1
adb logcat -d | Select-String "DX Mobile"
```

- [ ] **Step 3: iOS receipt**

On macOS only, run:

```bash
cd /Volumes/Work/Dx/www/dx-mobile-app
dx www build
cargo tauri ios build --debug
```

If Windows is the only available host, record iOS as `source-ready-not-device-built` instead of proven.

- [ ] **Step 4: Commit receipts**

```powershell
git add -- dx-mobile-app/README.md dx-mobile-app/.dx/native/native-shell-receipt.json dx-mobile-app/.dx/receipts/native
git commit -m "Record DX Mobile native build receipts"
```

---

## Final Verification

Run from `G:/Dx/www`:

```powershell
node --check .\dx-mobile-app\public\mobile-companion-runtime.js
node --test .\benchmarks\dx-mobile-app-source-contract.test.cjs .\benchmarks\cli-www-native-shell-command.test.ts .\benchmarks\dx-android-build-lane-contract.test.ts .\benchmarks\dx-mobile-app-production-output-contract.test.cjs
cargo fmt --manifest-path .\dx-mobile-app\src-tauri\Cargo.toml -- --check
cargo check --manifest-path .\dx-mobile-app\src-tauri\Cargo.toml -j 1
cargo clippy --manifest-path .\dx-mobile-app\src-tauri\Cargo.toml -j 1 -- -D warnings
cargo test --manifest-path .\dx-mobile-app\src-tauri\Cargo.toml -j 1
```

Run from `G:/Dx/www/dx-mobile-app`:

```powershell
dx www build
```

Before final completion:
- `nativeSecureStorageProven` is true only after Android/iOS storage proof exists.
- `qrScannerRuntimeProven` is true only after mobile camera scan proof exists.
- `androidReleaseProven` is true only after a real Android build receipt exists.
- iOS remains source-ready until built on macOS.

