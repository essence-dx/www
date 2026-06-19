use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use super::model::{EnvCheckFormat, EnvCheckStatus, EnvOpenOptions, EnvRecord, EnvScope};
use super::{
    lock_env_view, open_env_view, read_agent_context, read_env_check, reconcile_expired_view,
};

#[test]
fn locked_env_view_does_not_contain_secret_values_after_lock() {
    let project = temp_project("locked-env-view");
    let env_path = project.join(".env");
    fs::write(
        &env_path,
        "DATABASE_URL=env-fixture-database-url\nPUBLIC_SITE_URL=https://dx-www.dev\n",
    )
    .expect("write env viewport");

    let report = lock_env_view(&project, "env-fixture-password").expect("lock env");

    let locked = fs::read_to_string(project.join(".env")).expect("locked env");
    assert!(locked.contains("DX Env Firewall: locked"));
    assert!(!locked.contains("env-fixture-database-url"));
    assert!(!locked.contains("PUBLIC_SITE_URL=https://dx-www.dev"));
    assert_eq!(report.key_count, 2);
    assert!(project.join(".dx/env/local.sr").is_file());
    assert!(project.join(".dx/env/local.machine").is_file());
    assert!(project.join(".dx/env/env.d.ts").is_file());
    let sealed = fs::read_to_string(project.join(".dx/env/local.sr")).expect("sealed sr");
    assert!(!sealed.contains("env-fixture-database-url"));
    assert!(sealed.contains("ciphertext="));
    let contract = fs::read_to_string(project.join(".dx/env/env.d.ts")).expect("env types");
    assert!(contract.contains("declare module \"dx/env\""));
    assert!(contract.contains("readonly DATABASE_URL: string"));
    assert!(contract.contains("readonly PUBLIC_SITE_URL: string"));
}

#[test]
fn unlock_materializes_temporary_view_and_lock_persists_edits() {
    let project = temp_project("unlock-round-trip");
    fs::write(
        project.join(".env"),
        "DATABASE_URL=env-fixture-database-url\nPUBLIC_SITE_URL=https://dx-www.dev\n",
    )
    .expect("write env viewport");

    lock_env_view(&project, "secret").expect("initial lock");
    open_env_view(
        &project,
        EnvOpenOptions {
            password: "secret".to_string(),
            ttl: Duration::from_secs(180),
            json: false,
        },
    )
    .expect("open env");

    let open = fs::read_to_string(project.join(".env")).expect("open env");
    assert!(open.contains("DX Env Firewall: unlocked until"));
    assert!(open.contains("DATABASE_URL=env-fixture-database-url"));

    fs::write(
        project.join(".env"),
        "# DX Env Firewall: unlocked until 2099-01-01T00:00:00Z. Save changes here; DX will validate and reseal.\nDATABASE_URL=env-fixture-database-url-edited\nNEW_SECRET=env-fixture-extra-value\n",
    )
    .expect("write edited viewport");
    lock_env_view(&project, "secret").expect("lock edited env");
    open_env_view(
        &project,
        EnvOpenOptions {
            password: "secret".to_string(),
            ttl: Duration::from_secs(180),
            json: false,
        },
    )
    .expect("open edited env");

    let edited = fs::read_to_string(project.join(".env")).expect("edited open env");
    assert!(edited.contains("DATABASE_URL=env-fixture-database-url-edited"));
    assert!(edited.contains("NEW_SECRET=env-fixture-extra-value"));
}

#[test]
fn locking_an_already_locked_view_preserves_existing_store() {
    let project = temp_project("already-locked-view");
    fs::write(
        project.join(".env"),
        "DATABASE_URL=env-fixture-database-url\nPUBLIC_SITE_URL=https://dx-www.dev\n",
    )
    .expect("write env");
    lock_env_view(&project, "secret").expect("lock env");
    let before = fs::read_to_string(project.join(".dx/env/local.sr")).expect("store before");

    let report = lock_env_view(&project, "secret").expect("lock already locked");
    let after = fs::read_to_string(project.join(".dx/env/local.sr")).expect("store after");

    assert_eq!(report.status, "already-locked");
    assert_eq!(report.key_count, 2);
    assert_eq!(before, after);
}

#[test]
fn expired_unlocked_view_reseals_without_printing_values() {
    let project = temp_project("expired-view");
    fs::write(
        project.join(".env"),
        "STRIPE_SECRET_KEY=env-fixture-stripe-key\n",
    )
    .expect("write env");
    lock_env_view(&project, "secret").expect("lock env");
    open_env_view(
        &project,
        EnvOpenOptions {
            password: "secret".to_string(),
            ttl: Duration::from_secs(1),
            json: false,
        },
    )
    .expect("open env");

    let expired = SystemTime::now() + Duration::from_secs(5);
    let result = reconcile_expired_view(&project, "secret", expired).expect("reconcile");

    assert!(result.resealed);
    let locked = fs::read_to_string(project.join(".env")).expect("locked env");
    assert!(locked.contains("DX Env Firewall: locked"));
    assert!(!locked.contains("env-fixture-stripe-key"));
}

#[test]
fn check_and_agent_context_report_names_scopes_and_redact_values() {
    let project = temp_project("agent-context");
    fs::write(
        project.join(".env"),
        "DATABASE_URL=env-fixture-database-url\nPUBLIC_SITE_URL=https://dx-www.dev\nSTRIPE_SECRET_KEY=env-fixture-stripe-key\n",
    )
    .expect("write env");
    lock_env_view(&project, "secret").expect("lock env");

    let check = read_env_check(&project, EnvCheckFormat::Json).expect("check");
    assert_eq!(check.status, EnvCheckStatus::Current);
    assert_eq!(check.keys.len(), 3);
    assert!(check.keys.iter().any(|key| {
        key.name == "DATABASE_URL" && key.scope == EnvScope::Server && key.value_redacted
    }));
    assert!(check.keys.iter().any(|key| {
        key.name == "PUBLIC_SITE_URL" && key.scope == EnvScope::Public && key.value_redacted
    }));

    let agent = read_agent_context(&project).expect("agent context");
    assert_eq!(agent.keys.len(), 3);
    assert!(agent.keys.iter().all(|key| key.value_redacted));

    let rendered = serde_json::to_string(&agent).expect("agent json");
    assert!(!rendered.contains("env-fixture-database-url"));
    assert!(!rendered.contains("env-fixture-stripe-key"));
    assert!(rendered.contains("DATABASE_URL"));
    assert!(rendered.contains("PUBLIC_SITE_URL"));
}

#[test]
fn env_records_infer_first_class_scopes_from_names() {
    let records = EnvRecord::parse_many(
        "DATABASE_URL=env-fixture-database-url\nPUBLIC_ANALYTICS_ID=pub\nNEXT_PUBLIC_SITE_URL=https://dx-www.dev\nRESEND_API_KEY=env-fixture-resend-key\n",
    )
    .expect("records");

    assert_eq!(records[0].scope, EnvScope::Server);
    assert_eq!(records[1].scope, EnvScope::Public);
    assert_eq!(records[2].scope, EnvScope::Public);
    assert_eq!(records[3].capability.as_deref(), Some("server/email"));
}

fn temp_project(name: &str) -> PathBuf {
    let root =
        std::env::temp_dir().join(format!("dx-www-env-firewall-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("project root");
    root
}
