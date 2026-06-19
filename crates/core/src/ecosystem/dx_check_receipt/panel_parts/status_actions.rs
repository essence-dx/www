fn normalize_sha256_hash(value: &str) -> String {
    value
        .strip_prefix("sha256:")
        .unwrap_or(value)
        .trim()
        .to_ascii_lowercase()
}

fn authentication_next_action(
    status: &str,
    refresh_stale: u64,
    refresh_missing: u64,
    dx_style_compatibility_missing: u64,
) -> &'static str {
    if status != "missing-receipt" && refresh_stale > 0 {
        return "Run node examples/template/authentication-receipt-hashes.ts --write after reviewing changed Authentication account or session source surfaces.";
    }

    if status != "missing-receipt" && refresh_missing > 0 {
        return "Restore Authentication receipt-hash refresh metadata so Studio and Zed can show helper freshness without opening raw package-status JSON.";
    }

    if status != "missing-receipt" && dx_style_compatibility_missing > 0 {
        return "Regenerate the Authentication dx-style compatibility row so Studio and Zed can show source-style evidence without claiming live OAuth or session runtime proof.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel Authentication row from package-status, receipt hashes, receipt_hash_refresh helper freshness, selected source markers, and dx-style evidence."
        }
        "stale" => {
            "Regenerate the Authentication receipt and package-status hash manifest after reviewing changed account or session source surfaces."
        }
        "blocked" => {
            "Resolve the app-owned Authentication credentials, callback URLs, cookies, database adapter, email delivery, and hosted-session boundary before claiming runtime proof."
        }
        "unsupported-surface" => {
            "Request only supported Authentication server, route, client, account workflow, session status, provider, or account-management surfaces."
        }
        _ => {
            "Restore the Authentication package-status row and auth-better-auth receipt before trusting package-lane visibility."
        }
    }
}

fn state_management_next_action(
    status: &str,
    refresh_stale: u64,
    refresh_missing: u64,
    dx_style_compatibility_missing: u64,
) -> &'static str {
    if status != "missing-receipt" && refresh_stale > 0 {
        return "Run node examples/template/state-management-receipt-hashes.ts --write after reviewing the changed State Management source surfaces.";
    }

    if status != "missing-receipt" && refresh_missing > 0 {
        return "Restore State Management receipt-hash refresh metadata so Studio and Zed can show helper freshness without opening raw package-status JSON.";
    }

    if status != "missing-receipt" && dx_style_compatibility_missing > 0 {
        return "Regenerate the State Management dx-style compatibility row so Studio and Zed can show source-style evidence without claiming browser storage proof.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel State Management package row from package-status helper freshness, receipt, and dx-style evidence."
        }
        "stale" => "Run dx update state/zustand and review the refreshed State Management receipt.",
        "blocked" => {
            "Resolve the app-owned State Management boundary before claiming release readiness."
        }
        "unsupported-surface" => {
            "Request only supported State Management surfaces or add a real upstream-backed Forge surface."
        }
        _ => {
            "Restore the State Management package receipt before trusting package-lane visibility."
        }
    }
}

fn data_fetching_cache_next_action(
    status: &str,
    refresh_stale: u64,
    refresh_missing: u64,
) -> &'static str {
    if status != "missing-receipt" && refresh_missing > 0 {
        return "Restore the Data Fetching & Cache receipt_hash_refresh helper row so Studio and Zed can show helper freshness beside dx-check metrics.";
    }
    if status != "missing-receipt" && refresh_stale > 0 {
        return "Run node examples/template/data-fetching-cache-receipt-hashes.ts --write after reviewing changed Data Fetching & Cache source files, then rerun dx-check.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel Data Fetching & Cache package row with receipt_hash_refresh helper freshness, source hash evidence, and data-fetching-cache:receipt-hash-refresh markers without claiming live QueryClient execution."
        }
        "stale" => {
            "Regenerate the Data Fetching & Cache dashboard workflow receipt and package-status hash manifest after reviewing changed query dashboard files."
        }
        "blocked" => {
            "Resolve the app-owned query keys, fetchers, retry policy, cache retention, persistence, broadcast sync, and dependency boundary before claiming live QueryClient runtime proof."
        }
        "unsupported-surface" => {
            "Request only supported Data Fetching & Cache dashboard, provider, prefetch, hydration, persistence, or cache-control surfaces."
        }
        _ => {
            "Restore the Data Fetching & Cache package-status row and dashboard workflow receipt before trusting package-lane visibility."
        }
    }
}

fn reactive_store_next_action(
    status: &str,
    refresh_stale: u64,
    refresh_missing: u64,
) -> &'static str {
    if status != "missing-receipt" && refresh_missing > 0 {
        return "Restore the Reactive Store receipt_hash_refresh helper row so Studio and Zed can show helper freshness beside dx-check metrics.";
    }
    if status != "missing-receipt" && refresh_stale > 0 {
        return "Run node examples/template/reactive-store-receipt-hashes.ts --write after reviewing changed Reactive Store files.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel Reactive Store package row from the Forge receipt, source hashes, and receipt_hash_refresh helper freshness."
        }
        "stale" => {
            "Regenerate the Reactive Store package receipt after reviewing changed state files."
        }
        "blocked" => {
            "Resolve the app-owned Reactive Store boundary before claiming release readiness."
        }
        "unsupported-surface" => {
            "Request only supported Reactive Store surfaces or add a real upstream-backed Forge surface."
        }
        _ => "Restore the Reactive Store package receipt before trusting package-lane visibility.",
    }
}

fn forms_next_action(status: &str, refresh_stale: u64, refresh_missing: u64) -> &'static str {
    if status != "missing-receipt" && refresh_missing > 0 {
        return "Restore the Forms receipt_hash_refresh helper row so Studio and Zed can show helper freshness beside dx-check metrics.";
    }
    if status != "missing-receipt" && refresh_stale > 0 {
        return "Run node examples/template/forms-receipt-hashes.ts --write after reviewing changed Forms files.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel Forms package row from package-status, dashboard workflow receipt hash evidence, and receipt_hash_refresh helper freshness."
        }
        "stale" => {
            "Regenerate the Forms dashboard workflow receipt and package-status hash manifest after reviewing changed form files."
        }
        "blocked" => {
            "Resolve the app-owned Forms dependency/runtime boundary before claiming browser submission proof."
        }
        "unsupported-surface" => {
            "Request only supported Forms provider, field, field-array, resolver, or launch-lead surfaces."
        }
        _ => {
            "Restore the Forms package-status row and dashboard workflow receipt before trusting package-lane visibility."
        }
    }
}

fn payments_next_action(
    status: &str,
    receipt_hash_refresh: Option<&DxCheckPanelPackageLaneHashRefreshRow>,
    dx_style_compatibility_missing: u64,
) -> &'static str {
    if status != "missing-receipt" && receipt_hash_refresh.is_none() {
        return "Restore the Payments receipt_hash_refresh helper row so Studio and Zed can show helper freshness beside dx-check metrics.";
    }
    if status != "missing-receipt"
        && receipt_hash_refresh.is_some_and(|helper| {
            helper.status != "current"
                || helper.stale_file_count > 0
                || helper.missing_file_count > 0
        })
    {
        return "Run node examples/template/payments-receipt-hashes.ts --write after reviewing changed Payments files.";
    }
    if status != "missing-receipt" && dx_style_compatibility_missing > 0 {
        return "Regenerate the Payments dx-style compatibility row so Studio and Zed can show source-style evidence without claiming live Stripe runtime proof.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel Payments package row with helper freshness, source hash evidence, and honest Stripe boundary status."
        }
        "stale" => {
            "Regenerate the Payments billing workflow receipt and package-status hash manifest after reviewing changed billing files."
        }
        "blocked" => {
            "Resolve the app-owned Stripe credentials, webhook, checkout, and fulfillment boundary before claiming live payment runtime proof."
        }
        "unsupported-surface" => {
            "Request only supported Payments checkout, payment-intent, embedded-checkout, or billing-workflow surfaces."
        }
        _ => {
            "Restore the Payments package-status row and billing workflow receipt before trusting package-lane visibility."
        }
    }
}

fn backend_platform_client_next_action(status: &str) -> &'static str {
    match status {
        "present" => {
            "Render the DX Studio/check-panel Backend Platform Client package row with receipt_hash_refresh helper freshness, selected Supabase source markers, and SHA-256 evidence."
        }
        "stale" => {
            "Run node examples/template/backend-platform-client-receipt-hashes.ts --write after reviewing changed Backend Platform Client files."
        }
        "blocked" => {
            "Resolve the app-owned hosted Supabase credentials, RLS, provider setup, and runtime boundary before claiming hosted Supabase runtime proof."
        }
        "unsupported-surface" => {
            "Request only supported Backend Platform Client profile, schema-query, Auth, Storage, Realtime, RPC, or Edge Function surfaces."
        }
        _ => {
            "Restore the Backend Platform Client package-status row and dashboard workflow receipt before trusting package-lane visibility."
        }
    }
}

fn realtime_app_database_next_action(
    status: &str,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_missing: u64,
) -> &'static str {
    if status != "missing-receipt" && receipt_hash_refresh_missing > 0 {
        return "Restore the Realtime App Database receipt_hash_refresh helper row so Studio and Zed can show helper freshness beside dx-check metrics.";
    }
    if status != "missing-receipt" && receipt_hash_refresh_stale > 0 {
        return "Run node examples/template/realtime-app-database-receipt-hashes.ts --write after reviewing changed realtime files so Studio and Zed can show current helper freshness.";
    }
    if status != "missing-receipt" && dx_style_compatibility_missing > 0 {
        return "Regenerate the Realtime App Database dx-style compatibility row so Studio and Zed can show source-style evidence without claiming hosted Instant runtime proof.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel Realtime App Database package row from package-status, dashboard receipt, hash, receipt_hash_refresh helper freshness, and dx-style evidence without claiming hosted Instant runtime proof."
        }
        "stale" => {
            "Regenerate the Realtime App Database dashboard workflow receipt and package-status hash manifest after reviewing changed realtime files."
        }
        "blocked" => {
            "Resolve the app-owned Instant app id, rules, auth policy, storage, streams, or Sync Table boundary before claiming hosted realtime runtime proof."
        }
        "unsupported-surface" => {
            "Request only supported Realtime App Database todos, presence, auth, storage, streams, Sync Table, or dashboard workflow surfaces."
        }
        _ => {
            "Restore the Realtime App Database package-status row and dashboard workflow receipt before trusting package-lane visibility."
        }
    }
}

fn validation_schemas_next_action(
    status: &str,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
) -> &'static str {
    if status != "missing-receipt" && receipt_hash_refresh_stale > 0 {
        return "Run node examples/template/validation-schemas-receipt-hashes.ts --write after reviewing changed Validation & Schemas files.";
    }
    if status != "missing-receipt" && receipt_hash_refresh_missing > 0 {
        return "Restore package-status receipt_hash_refresh for Validation & Schemas so Studio and Zed can show helper freshness beside dx-check metrics.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel Validation & Schemas package row from package-status, dashboard receipt, hash, and helper freshness evidence."
        }
        "stale" => {
            "Regenerate the Validation & Schemas dashboard settings receipt and package-status hash manifest after reviewing changed schema files."
        }
        "blocked" => {
            "Resolve the app-owned Validation & Schemas dependency/runtime boundary before claiming live Validation & Schemas runtime proof."
        }
        "unsupported-surface" => {
            "Request only supported Validation & Schemas schema, parse, error, JSON Schema, or dashboard settings surfaces."
        }
        _ => {
            "Restore the Validation & Schemas package-status row and dashboard settings receipt before trusting package-lane visibility."
        }
    }
}

fn type_safe_api_next_action(
    status: &str,
    refresh_stale: u64,
    refresh_missing: u64,
) -> &'static str {
    if status == "unsupported-surface" {
        return "Request only supported Type-Safe API router, procedure, route handler, dashboard workflow, launch health, or typed-client surfaces.";
    }
    if status == "blocked" {
        return "Resolve the app-owned Type-Safe API dependency/runtime/auth/service boundary before claiming live tRPC route proof.";
    }
    if status != "missing-receipt" && refresh_missing > 0 {
        return "Restore the Type-Safe API receipt_hash_refresh helper row so Studio and Zed can show helper freshness beside dx-check metrics.";
    }
    if status != "missing-receipt" && refresh_stale > 0 {
        return "Run node examples/template/type-safe-api-receipt-hashes.ts --write after reviewing Type-Safe API source changes, then rerun dx-check.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel Type-Safe API package row with receipt_hash_refresh helper freshness and source hash evidence."
        }
        "stale" => {
            "Regenerate the Type-Safe API dashboard workflow receipt and package-status hash manifest after reviewing changed router, client, launch, or dashboard files."
        }
        "blocked" => {
            "Resolve the app-owned Type-Safe API dependency/runtime/auth/service boundary before claiming live tRPC route proof."
        }
        "unsupported-surface" => {
            "Request only supported Type-Safe API router, procedure, route handler, dashboard workflow, launch health, or typed-client surfaces."
        }
        _ => {
            "Restore the Type-Safe API package-status row and dashboard workflow receipt before trusting package-lane visibility."
        }
    }
}

fn motion_animation_next_action(
    status: &str,
    refresh_stale: u64,
    refresh_missing: u64,
) -> &'static str {
    if status != "missing-receipt" && refresh_missing > 0 {
        return "Restore the Motion & Animation receipt_hash_refresh helper row so Studio and Zed can show helper freshness beside dx-check metrics.";
    }
    if status != "missing-receipt" && refresh_stale > 0 {
        return "Run node examples/template/motion-receipt-hashes.ts --write after reviewing changed Motion & Animation source files, then rerun dx-check.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel Motion package row with receipt_hash_refresh helper freshness, source hash evidence, and motion-animation:receipt-hash-refresh markers without claiming live Motion browser animation proof."
        }
        "stale" => {
            "Regenerate the Motion & Animation dashboard workflow receipt and package-status hash manifest after reviewing changed launch choreography files."
        }
        "blocked" => {
            "Resolve the app-owned Motion & Animation route choreography, reduced-motion policy, accessibility QA, animation budgets, and browser runtime proof boundary before claiming live Motion readiness."
        }
        "unsupported-surface" => {
            "Request only supported Motion & Animation provider, layout, reorder, motion-value, scroll-progress, or dashboard-workflow surfaces."
        }
        _ => {
            "Restore the Motion & Animation package-status row and dashboard workflow receipt before trusting package-lane visibility."
        }
    }
}

fn ui_components_next_action(
    status: &str,
    receipt_hash_refresh: Option<&DxCheckPanelPackageLaneHashRefreshRow>,
) -> &'static str {
    if receipt_hash_refresh.is_none() {
        return "Restore the UI Components receipt_hash_refresh helper row so Studio and Zed can show helper freshness beside dx-check metrics.";
    }
    if receipt_hash_refresh.is_some_and(|helper| {
        helper.status != "current" || helper.stale_file_count > 0 || helper.missing_file_count > 0
    }) {
        return "Run the UI Components receipt hash helper after reviewing changed shadcn-ui or Radix-derived files.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel UI Components package row with receipt_hash_refresh helper freshness and source hash evidence."
        }
        "stale" => {
            "Regenerate the UI Components dashboard controls receipt and package-status hash manifest after reviewing changed front-facing UI files."
        }
        "blocked" => {
            "Resolve the app-owned UI Components dashboard persistence, accessibility, registry sync, and governed browser UI runtime proof boundary before claiming runtime readiness."
        }
        "unsupported-surface" => {
            "Request only supported UI Components button, badge, card, field, input, item, label, separator, textarea, or dashboard-control surfaces."
        }
        _ => {
            "Restore the UI Components package-status row and dashboard controls receipt before trusting package-lane visibility."
        }
    }
}

fn database_orm_next_action(
    status: &str,
    receipt_hash_refresh: Option<&DxCheckPanelPackageLaneHashRefreshRow>,
    dx_style_compatibility_missing: u64,
) -> &'static str {
    if status != "missing-receipt" && receipt_hash_refresh.is_none() {
        return "Restore the Database ORM receipt_hash_refresh helper row so Studio and Zed can show helper freshness beside dx-check metrics.";
    }
    if status != "missing-receipt"
        && receipt_hash_refresh.is_some_and(|helper| {
            helper.status != "current"
                || helper.stale_file_count > 0
                || helper.missing_file_count > 0
        })
    {
        return "Run node examples/template/database-orm-receipt-hashes.ts --write after reviewing changed Drizzle source files so Studio and Zed can show current helper freshness.";
    }
    if status != "missing-receipt" && dx_style_compatibility_missing > 0 {
        return "Regenerate the Database ORM dx-style compatibility row so Studio and Zed can show source-style evidence without claiming live SQLite read proof.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel Database ORM package row from package-status, dashboard receipt, source hash evidence, receipt_hash_refresh helper freshness, and dx-style evidence without claiming live SQLite read proof."
        }
        "stale" => {
            "Regenerate the Database ORM dashboard workflow receipt and package-status hash manifest after reviewing changed Drizzle source files."
        }
        "blocked" => {
            "Resolve the app-owned Database ORM database path, migration, replica health, dependency, or authorization boundary before claiming runtime proof."
        }
        "unsupported-surface" => {
            "Request only supported Database ORM SQLite schema, migration, replica-routing, and dashboard workflow surfaces."
        }
        _ => {
            "Restore the Database ORM package-status row and dashboard workflow receipt before trusting package-lane visibility."
        }
    }
}

fn documentation_system_next_action(status: &str) -> &'static str {
    match status {
        "present" => {
            "Render the DX Studio/check-panel Documentation System package row from package-status, dashboard receipt, hash, and dx-style evidence."
        }
        "stale" => {
            "Regenerate the Documentation System dashboard workflow receipt and package-status hash manifest after reviewing changed docs, launch, dashboard, or Forge files."
        }
        "blocked" => {
            "Resolve the app-owned Documentation System dependency/runtime boundary before claiming live Fumadocs renderer runtime proof."
        }
        "unsupported-surface" => {
            "Request only supported Documentation System docs route, dashboard workflow, LLM export, OpenAPI, or search surfaces."
        }
        _ => {
            "Restore the Documentation System package-status row and dashboard workflow receipt before trusting package-lane visibility."
        }
    }
}

fn markdown_mdx_content_next_action(
    status: &str,
    dx_style_compatibility_missing: u64,
    materialized_source_missing: u64,
) -> &'static str {
    if status != "missing-receipt" && materialized_source_missing > 0 {
        return "Regenerate the Markdown & MDX Content materializedSource row so Studio and Zed can show receipt-helper execution evidence without claiming live Markdown/MDX renderer proof.";
    }
    if status != "missing-receipt" && dx_style_compatibility_missing > 0 {
        return "Regenerate the Markdown & MDX Content dx-style compatibility row so Studio and Zed can show source-style evidence without claiming live Markdown/MDX renderer proof.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel Markdown & MDX Content package row from package-status, receipt, hash, dx-style, and materialized-source evidence."
        }
        "stale" => {
            "Regenerate the Markdown & MDX Content package receipt and package-status hash manifest after reviewing changed renderer, provider, server compile, or receipt-helper files."
        }
        "blocked" => {
            "Resolve the app-owned Markdown & MDX Content runtime dependency, raw HTML, sanitizer, plugin, content trust, or trusted MDX execution boundary before claiming runtime proof."
        }
        "unsupported-surface" => {
            "Request only supported Markdown & MDX Content renderer, MDX provider, server compile, or receipt-helper surfaces."
        }
        _ => {
            "Restore the Markdown & MDX Content package-status row and package receipt before trusting package-lane visibility."
        }
    }
}

fn ai_sdk_next_action(
    status: &str,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_missing: u64,
) -> &'static str {
    if status != "missing-receipt" && receipt_hash_refresh_missing > 0 {
        return "Restore the AI SDK receipt_hash_refresh helper row so Studio and Zed can show helper freshness beside dx-check metrics.";
    }
    if status != "missing-receipt" && receipt_hash_refresh_stale > 0 {
        return "Run node examples/template/ai-sdk-receipt-hashes.ts --write after reviewing changed AI SDK launch assistant files, then rerun dx-check.";
    }
    if status != "missing-receipt" && dx_style_compatibility_missing > 0 {
        return "Regenerate the AI SDK dx-style compatibility row so Studio and Zed can show source-style evidence without claiming live model streaming or browser proof.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel AI SDK package row from package-status, launch assistant receipt, hash, receipt_hash_refresh helper freshness, and dx-style evidence without claiming live model streaming or browser proof."
        }
        "stale" => {
            "Regenerate the AI SDK launch assistant receipt and package-status hash manifest after reviewing changed assistant files."
        }
        "blocked" => {
            "Resolve the app-owned AI SDK provider credentials, gateway routing, model safety, persistence, rate limits, and billing controls before claiming runtime proof."
        }
        "unsupported-surface" => {
            "Request only supported AI SDK chat-route, dashboard-readiness, assistant-component, or launch-assistant surfaces."
        }
        _ => {
            "Restore the AI SDK package-status row and launch assistant receipt before trusting package-lane visibility."
        }
    }
}

fn internationalization_next_action(
    status: &str,
    refresh_stale: u64,
    refresh_missing: u64,
    dx_style_compatibility_missing: u64,
) -> &'static str {
    if status != "missing-receipt" && refresh_missing > 0 {
        return "Restore the Internationalization receipt_hash_refresh helper row so Studio and Zed can show helper freshness beside dx-check metrics.";
    }
    if status != "missing-receipt" && refresh_stale > 0 {
        return "Run node examples/template/internationalization-receipt-hashes.ts --write after reviewing changed Internationalization source files, then rerun dx-check.";
    }
    if status != "missing-receipt" && dx_style_compatibility_missing > 0 {
        return "Regenerate the Internationalization dx-style compatibility row so Studio and Zed can show source-style evidence without claiming live locale routing proof.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel Internationalization row from package-status, source hash evidence, receipt_hash_refresh helper freshness, and dx-style evidence without claiming live locale routing proof."
        }
        "stale" => {
            "Regenerate the Internationalization dashboard locale workflow receipt and package-status hash manifest after reviewing changed locale files."
        }
        "blocked" => {
            "Resolve the app-owned Internationalization locale routing, translation QA, SEO, or dependency boundary before claiming runtime proof."
        }
        "unsupported-surface" => {
            "Request only supported Internationalization dashboard locale and message-contract surfaces."
        }
        _ => {
            "Restore the Internationalization package-status row and dashboard workflow receipt before trusting package-lane visibility."
        }
    }
}

fn three_scene_system_next_action(
    status: &str,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_missing: u64,
) -> &'static str {
    if status != "missing-receipt" && receipt_hash_refresh_missing > 0 {
        return "Restore the 3D Scene System receipt_hash_refresh helper row so Studio and Zed can show helper freshness beside hash and dx-style evidence.";
    }
    if status != "missing-receipt" && receipt_hash_refresh_stale > 0 {
        return "Run node examples/template/3d-scene-system-receipt-hashes.ts --write after reviewing changed 3D Scene System source files, then rerun dx-check.";
    }
    if status != "missing-receipt" && dx_style_compatibility_missing > 0 {
        return "Regenerate the 3D Scene System dx-style compatibility row so Studio and Zed can show source-style evidence without claiming live browser/WebGL proof.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel 3D Scene System package row from package-status, hash, receipt_hash_refresh helper freshness, and dx-style evidence without claiming live browser/WebGL proof."
        }
        "stale" => {
            "Regenerate the 3D Scene System dashboard workflow receipt and package-status hash manifest after reviewing changed scene files."
        }
        "blocked" => {
            "Resolve the app-owned 3D Scene System browser/WebGL runtime, dependency, asset, shader, or WebXR boundary before claiming runtime proof."
        }
        "unsupported-surface" => {
            "Request only supported 3D Scene System launch-scene, dashboard workflow, renderer handoff, R3F/Drei adapter, or Web Preview runtime surfaces."
        }
        _ => {
            "Restore the 3D Scene System package-status row and dashboard workflow receipt before trusting package-lane visibility."
        }
    }
}

fn webassembly_bridge_next_action(
    status: &str,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_missing: u64,
) -> &'static str {
    if status != "missing-receipt" && receipt_hash_refresh_missing > 0 {
        return "Restore the WebAssembly Bridge receipt_hash_refresh helper row so Studio and Zed can show helper freshness beside hash and dx-style evidence.";
    }
    if status != "missing-receipt" && receipt_hash_refresh_stale > 0 {
        return "Run node examples/template/webassembly-bridge-receipt-hashes.ts --write after reviewing changed WebAssembly Bridge source files, then rerun dx-check.";
    }
    if status != "missing-receipt" && dx_style_compatibility_missing > 0 {
        return "Regenerate the WebAssembly Bridge dx-style compatibility row so Studio and Zed can show source-style evidence without claiming live generated-Wasm or browser style proof.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel WebAssembly Bridge package row from package-status, hash, receipt_hash_refresh helper freshness, and dx-style evidence without claiming live generated-Wasm or browser style proof."
        }
        "stale" => {
            "Regenerate the WebAssembly Bridge dashboard workflow receipt and package-status hash manifest after reviewing changed wasm files."
        }
        "blocked" => {
            "Resolve the app-owned Rust crate exports, wasm32 build artifact, generated JavaScript glue, CSP, MIME serving, and performance/security review before claiming WebAssembly Bridge runtime proof."
        }
        "unsupported-surface" => {
            "Request only supported WebAssembly Bridge loader, React hook, dashboard workflow, launch local-compute, or readiness workflow surfaces."
        }
        _ => {
            "Restore the WebAssembly Bridge package-status row and dashboard workflow receipt before trusting package-lane visibility."
        }
    }
}

fn automation_connectors_next_action(
    status: &str,
    receipt_hash_refresh_stale: u64,
    receipt_hash_refresh_missing: u64,
    dx_style_compatibility_missing: u64,
    upstream_runtime_boundary_missing: u64,
) -> &'static str {
    if status != "missing-receipt" && receipt_hash_refresh_missing > 0 {
        return "Restore the Automation Connectors receipt_hash_refresh helper row so Studio and Zed can show helper freshness beside hash, dx-style, and runtime-boundary evidence.";
    }
    if status != "missing-receipt" && receipt_hash_refresh_stale > 0 {
        return "Run node examples/template/automation-connectors-receipt-hashes.ts --write after reviewing changed Automation Connectors source files, then rerun dx-check.";
    }
    if status != "missing-receipt" && upstream_runtime_boundary_missing > 0 {
        return "Restore the Automation Connectors inspected upstream files and public API metadata before claiming upstream runtime-boundary visibility.";
    }
    if status != "missing-receipt" && dx_style_compatibility_missing > 0 {
        return "Regenerate the Automation Connectors dx-style compatibility row so Studio and Zed can show source-style evidence without claiming live n8n runtime proof.";
    }

    match status {
        "present" => {
            "Render the DX Studio/check-panel Automation Connectors package row from package-status, hash, receipt_hash_refresh helper freshness, dx-style, and upstream runtime-boundary evidence without claiming live n8n execution."
        }
        "stale" => {
            "Regenerate the Automation Connectors dashboard workflow receipt and package-status hash manifest after reviewing changed connector files."
        }
        "blocked" => {
            "Resolve the app-owned provider credentials, OAuth callbacks, webhook registration, operator approval, and live n8n workflow execution before claiming Automation Connectors runtime proof."
        }
        "unsupported-surface" => {
            "Request only supported Automation Connectors launch workflow, connector readiness, dashboard handoff, source-guard, dx-style, or lower dx-check surfaces."
        }
        _ => {
            "Restore the Automation Connectors package-status row and dashboard workflow receipt before trusting package-lane visibility."
        }
    }
}

fn json_text<'a>(value: &'a serde_json::Value, path: &[&str]) -> Option<&'a str> {
    value_at(value, path).and_then(|value| value.as_str())
}

fn json_u64(value: &serde_json::Value, path: &[&str]) -> Option<u64> {
    value_at(value, path).and_then(|value| value.as_u64())
}

fn json_bool(value: &serde_json::Value, path: &[&str]) -> Option<bool> {
    value_at(value, path).and_then(|value| value.as_bool())
}

fn json_string_array(value: &serde_json::Value, path: &[&str]) -> Vec<String> {
    value_at(value, path)
        .and_then(|value| value.as_array())
        .map(|values| {
            values
                .iter()
                .filter_map(|value| value.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn json_array_contains_all(value: &serde_json::Value, path: &[&str], required: &[&str]) -> bool {
    let values = json_string_array(value, path);
    required
        .iter()
        .all(|required_value| values.iter().any(|value| value == required_value))
}

fn json_array_len(value: &serde_json::Value, path: &[&str]) -> u64 {
    value_at(value, path)
        .and_then(|value| value.as_array())
        .map(|values| values.len() as u64)
        .unwrap_or(0)
}

fn value_at<'a>(value: &'a serde_json::Value, path: &[&str]) -> Option<&'a serde_json::Value> {
    path.iter()
        .try_fold(value, |current, key| current.get(*key))
}

fn panel_tone(status: &str, blocker_count: u32, warning_count: u32) -> String {
    if blocker_count > 0 || matches!(status, "blocked" | "fail" | "failed" | "error") {
        "danger".to_string()
    } else if warning_count > 0 || matches!(status, "warning" | "warn" | "partial") {
        "warning".to_string()
    } else {
        "success".to_string()
    }
}

fn default_weight_profile() -> String {
    DX_CHECK_WEIGHT_PROFILE.to_string()
}

fn default_bucket_weight() -> u16 {
    100
}

fn default_scoring_config_report() -> DxCheckScoringConfigReport {
    DxCheckScoringConfigReport {
        schema_version: "dx.check.scoring_config".to_string(),
        active_profile: DX_CHECK_WEIGHT_PROFILE.to_string(),
        status: "default".to_string(),
        config_present: false,
        config_paths_checked: vec![
            ".dx/check/config.json".to_string(),
            ".dx/check/config.toml".to_string(),
            "dx.check.json".to_string(),
            "dx.check.toml".to_string(),
        ],
        config_path: None,
        configured_profile: None,
        active_bucket_weights: vec![
            bucket_weight("code-quality", "Code Quality"),
            bucket_weight("structure", "Structure"),
            bucket_weight("web-performance", "Web Performance"),
            bucket_weight("dx-framework-health", "DX Framework Health"),
            bucket_weight("test-readiness", "Test and Launch Readiness"),
        ],
        configured_bucket_weights: Vec::new(),
        configured_total_weight: None,
        applies_to_score: true,
        ignored_reason: None,
        next_action:
            "No scoring config was found; dx-check is using the launch-default 500-point profile."
                .to_string(),
    }
}

fn bucket_weight(id: &str, label: &str) -> DxCheckPanelBucketWeight {
    DxCheckPanelBucketWeight {
        id: id.to_string(),
        label: label.to_string(),
        weight: 100,
    }
}
