# DX-WWW Zed Panel V1 Receipt Compatibility Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make DX-WWW accept the current CLI `dx.check.zed_panel.v1` Zed panel schema while preserving backward compatibility for existing unversioned receipts.

**Architecture:** Keep DX-WWW as a receipt consumer only. Update the receipt reader in `G:\Dx\www\core\src\ecosystem\dx_check_receipt.rs` so `dx.check.zed_panel.v1` is the current schema and `dx.check.zed_panel` remains accepted as a legacy machine receipt. Do not change `.sr`, `.machine`, launch scoring, or CLI receipt emission in this slice.

**Tech Stack:** Rust, `serde_json`, existing DX-WWW `dx_check_receipt` unit tests, focused Cargo test with `-j 1`.

---

## File Structure

- `G:\Dx\www\core\src\ecosystem\dx_check_receipt.rs`: update schema constants, add a tiny schema support helper, update the embedded current sample to v1, and add a focused legacy compatibility test.
- `G:\Dx\www\benchmarks\dx-check-zed-panel-schema-compat.test.cjs`: lightweight source guard for the schema compatibility contract when the large Rust compile graph is too expensive for an immediate red/green loop.

## Task 1: Current V1 Receipt Support

**Files:**
- Modify: `G:\Dx\www\core\src\ecosystem\dx_check_receipt.rs`

- [ ] **Step 1: Write the failing current-schema assertion**

In `dx_check_latest_panel_reads_zed_receipt`, keep the existing assertion:

```rust
assert_eq!(zed.schema_version, DX_CHECK_ZED_PANEL_SCHEMA_VERSION);
```

Then update the embedded `sample_receipt()` JSON from:

```json
"schema_version": "dx.check.zed_panel"
```

to:

```json
"schema_version": "dx.check.zed_panel.v1"
```

- [ ] **Step 2: Run the focused test and confirm RED**

Run:

```powershell
cargo test -j 1 --lib dx_check_latest_panel_reads_zed_receipt
```

Expected: fail with `Malformed` / unsupported Zed panel schema because the reader still only accepts `dx.check.zed_panel`.

- [ ] **Step 3: Implement current v1 support**

Change:

```rust
pub const DX_CHECK_ZED_PANEL_SCHEMA_VERSION: &str = "dx.check.zed_panel";
```

to:

```rust
pub const DX_CHECK_ZED_PANEL_SCHEMA_VERSION: &str = "dx.check.zed_panel.v1";
const DX_CHECK_ZED_PANEL_LEGACY_SCHEMA_VERSION: &str = "dx.check.zed_panel";
```

Add:

```rust
fn is_supported_zed_panel_schema(schema_version: &str) -> bool {
    matches!(
        schema_version,
        DX_CHECK_ZED_PANEL_SCHEMA_VERSION | DX_CHECK_ZED_PANEL_LEGACY_SCHEMA_VERSION
    )
}
```

Use the helper in `read_dx_check_latest_panel` instead of strict equality.

- [ ] **Step 4: Run the focused test and confirm GREEN**

Run:

```powershell
cargo test -j 1 --lib dx_check_latest_panel_reads_zed_receipt
```

Expected: pass.

## Task 2: Legacy Receipt Compatibility

**Files:**
- Modify: `G:\Dx\www\core\src\ecosystem\dx_check_receipt.rs`

- [ ] **Step 1: Write the failing legacy compatibility test**

Add a test next to `dx_check_latest_panel_reads_zed_receipt`:

```rust
#[test]
fn dx_check_latest_panel_still_reads_legacy_unversioned_zed_receipt() {
    let dir = TempDir::new().expect("temp dir");
    let receipt_path = dx_check_latest_receipt_path(dir.path());
    fs::create_dir_all(receipt_path.parent().expect("receipt parent")).expect("receipt dir");
    fs::write(
        &receipt_path,
        sample_receipt().replace(
            DX_CHECK_ZED_PANEL_SCHEMA_VERSION,
            DX_CHECK_ZED_PANEL_LEGACY_SCHEMA_VERSION,
        ),
    )
    .expect("legacy receipt");

    let report = read_dx_check_latest_panel(dir.path());

    assert_eq!(report.status, DxCheckLatestPanelStatus::Ready);
    assert_eq!(
        report.zed.expect("legacy zed panel").schema_version,
        DX_CHECK_ZED_PANEL_LEGACY_SCHEMA_VERSION
    );
}
```

- [ ] **Step 2: Run the focused legacy test and confirm RED before helper support**

Run:

```powershell
cargo test -j 1 --lib dx_check_latest_panel_still_reads_legacy_unversioned_zed_receipt
```

Expected before helper support: fail because only v1 is accepted. If the helper is already present from Task 1, this test should pass and confirms the backwards-compatible implementation.

- [ ] **Step 3: Keep legacy support minimal**

No conversion or rewriting is needed. Preserve the original receipt schema string in `report.zed.schema_version` so consumers can see provenance.

## Task 3: Focused Verification

**Files:**
- Verify only DX-WWW receipt reader scope.

- [ ] **Step 1: Run lightweight source guard**

Run:

```powershell
node --test .\benchmarks\dx-check-zed-panel-schema-compat.test.cjs
```

Expected: pass after implementation; before implementation, fail because the current schema constant/helper is missing.

- [ ] **Step 2: Format check**

Run:

```powershell
cargo fmt --check
```

- [ ] **Step 3: Focused tests**

Run:

```powershell
cargo test -j 1 --lib dx_check_latest_panel_reads_zed_receipt
cargo test -j 1 --lib dx_check_latest_panel_still_reads_legacy_unversioned_zed_receipt
```

- [ ] **Step 4: Lightweight compile check**

Run:

```powershell
cargo check -j 1 --message-format short
git diff --check
```

Expected: pass. If repo-wide compile is blocked by unrelated dirty lanes, report that exact blocker and keep the focused test evidence.

## Self-Review

- Spec coverage: accepts current CLI `dx.check.zed_panel.v1`, preserves legacy unversioned JSON receipt compatibility, and does not alter serializer `.sr` / `.machine` contracts.
- Placeholder scan: no placeholders remain.
- Type consistency: existing `DxCheckZedPanel.schema_version` remains a `String`; schema acceptance is isolated to one helper.
