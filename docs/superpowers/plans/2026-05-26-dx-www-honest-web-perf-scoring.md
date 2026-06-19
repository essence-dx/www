# Honest Web Performance Scoring Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Prevent DX-WWW orchestration and serializer artifacts from awarding or implying measured Lighthouse proof when web performance evidence is partial, source-only, or audit-only.

**Architecture:** Keep this slice in DX-WWW because the faulty aggregate scoring is in the DX-WWW extension-list orchestrator, not the DX Check engine crate. Preserve existing JSON receipts and command behavior, but make score projection stricter and add explicit serializer-native estimated/proof fields. Do not run Lighthouse, start servers, or alter the DX CLI launch receipt pipeline in this slice.

**Tech Stack:** Rust (`dx-www` crate), `serde_json`, existing DX serializer `.sr` writer, focused `cargo test -j 1`.

---

### Task 1: Harden Orchestrator Web-Perf Score Projection

**Files:**
- Modify: `G:\Dx\www\dx-www\src\cli\extension_orchestrator.rs`

- [ ] **Step 1: Write failing unit tests**

Add a `#[cfg(test)]` module at the bottom of `extension_orchestrator.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn web_perf_score_requires_complete_measured_total_for_full_credit() {
        let complete = json!({
            "measurement_status": "measured-from-lighthouse-json",
            "score_completeness": { "complete": true },
            "scores": { "total": 376 }
        });
        assert_eq!(web_perf_score(&complete, 100), 94);
    }

    #[test]
    fn web_perf_score_rejects_url_only_source_proof_without_measurements() {
        let source_only = json!({
            "measurement_status": "not-measured-cdp-runtime-not-attached",
            "score_completeness": { "complete": false },
            "scores": { "total": null }
        });
        assert_eq!(web_perf_score(&source_only, 100), 0);
    }

    #[test]
    fn web_perf_score_rejects_partial_or_audit_only_reports_without_totals() {
        let partial = json!({
            "measurement_status": "partial-lighthouse-json-missing-score-categories",
            "score_completeness": { "complete": false },
            "scores": { "total": null }
        });
        assert_eq!(web_perf_score(&partial, 100), 0);

        let audit_only = json!({
            "measurement_status": "audit-only-rule-findings",
            "findings": ["missing Lighthouse proof"],
            "diagnostics": [{ "id": "web-perf-proof-missing" }]
        });
        assert_eq!(web_perf_score(&audit_only, 100), 0);
    }
}
```

- [ ] **Step 2: Verify RED**

Run:

```powershell
$env:CARGO_TARGET_DIR='F:\Temp\dx-www-cargo-target'; $env:TEMP='F:\Temp\dx-www-cargo-temp'; $env:TMP='F:\Temp\dx-www-cargo-temp'; cargo test -j 1 -p dx-www web_perf_score_rejects_partial_or_audit_only_reports_without_totals
```

Expected: FAIL because the current fallback returns `max_score`.

- [ ] **Step 3: Implement minimal scoring fix**

Change `web_perf_score` to:

```rust
fn web_perf_score(report: &Value, max_score: u64) -> u64 {
    let complete = report
        .get("score_completeness")
        .and_then(|value| value.get("complete"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if complete {
        return report
            .get("scores")
            .and_then(|scores| scores.get("total"))
            .and_then(Value::as_u64)
            .map(|total| (total / 4).min(max_score))
            .unwrap_or(0);
    }

    0
}
```

- [ ] **Step 4: Verify GREEN**

Run the three `web_perf_score_*` unit tests with `-j 1`.

### Task 2: Make Web-Perf `.sr` Artifact Proof Status Explicit

**Files:**
- Modify: `G:\Dx\www\dx-www\src\cli\serializer_artifacts.rs`
- Modify: `G:\Dx\www\dx-www\src\cli\public_framework_tools.rs`

- [ ] **Step 1: Write failing source-shape benchmark**

Update `benchmarks/public-framework-tools.test.ts` to assert that web performance artifacts include:

```text
"score_estimated"
("raw_lighthouse_json", web_perf_raw_lighthouse_json(report))
fn web_perf_raw_lighthouse_json(report: &Value) -> String
```

- [ ] **Step 2: Verify RED**

Run:

```powershell
node --test benchmarks/public-framework-tools.test.ts
```

Expected: FAIL because `score_estimated` and the helper are absent.

- [ ] **Step 3: Implement serializer helper**

Add `sr_null()` in `serializer_artifacts.rs`:

```rust
pub(super) fn sr_null() -> String {
    "null".to_string()
}
```

Import it in `public_framework_tools.rs` and add:

```rust
fn web_perf_score_estimated(report: &Value) -> bool {
    !report
        .get("score_completeness")
        .and_then(|value| value.get("complete"))
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn web_perf_raw_lighthouse_json(report: &Value) -> String {
    report
        .get("input")
        .and_then(Value::as_str)
        .map(sr_string)
        .unwrap_or_else(sr_null)
}
```

Use those in `write_web_perf_sr_artifact`.

- [ ] **Step 4: Verify GREEN**

Run the focused benchmark again.

### Task 3: Keep Doctor and Lighthouse Device Proof Honest

**Files:**
- Modify: `G:\Dx\www\dx-www\src\cli\public_framework_tools.rs`

- [ ] **Step 1: Add failing tests**

Add focused tests proving URL-only receipts score `0`, `web_perf_report_measured` requires `score_completeness.complete=true` plus a numeric total, and a single Lighthouse import cannot be labeled as `--device both`.

- [ ] **Step 2: Verify RED**

Run:

```powershell
$env:CARGO_TARGET_DIR='F:\Temp\dx-www-cargo-target'; $env:TEMP='F:\Temp\dx-www-cargo-temp'; $env:TMP='F:\Temp\dx-www-cargo-temp'; cargo test -j 1 -p dx-www web_perf_
```

Expected: FAIL before implementation because URL-only receipts still receive credit and `both` imports are not rejected.

- [ ] **Step 3: Implement measured-proof gating**

Add `web_perf_receipt_proof(project)` and `web_perf_report_measured(report)` helpers, pass the proof into `framework_risk_register`, and keep `performance_claims_unmeasured` active unless the receipt is actually measured. Return `0` from `web_perf_500_score` for URL-only reports.

- [ ] **Step 4: Guard single Lighthouse proof from both-device claims**

Reject `--device both` for `--lighthouse` and `--from-lighthouse` until a dual mobile/desktop receipt aggregation path exists.

### Task 4: Final Verification

- [ ] Run:

```powershell
$env:CARGO_TARGET_DIR='F:\Temp\dx-www-cargo-target'; $env:TEMP='F:\Temp\dx-www-cargo-temp'; $env:TMP='F:\Temp\dx-www-cargo-temp'; cargo fmt --check
```

- [ ] Run:

```powershell
$env:CARGO_TARGET_DIR='F:\Temp\dx-www-cargo-target'; $env:TEMP='F:\Temp\dx-www-cargo-temp'; $env:TMP='F:\Temp\dx-www-cargo-temp'; cargo check -j 1 -p dx-www --message-format short
```

- [ ] Run:

```powershell
git diff --check
```

- [ ] Confirm no server was started and no launch JSON receipt schema was changed.
