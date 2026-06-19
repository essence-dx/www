//! Approval-gated R2/S3 object HEAD execution for Forge.

use anyhow::{Context, Result, bail};
use object_store::{Error as ObjectStoreError, ObjectStore, path::Path as ObjectPath};

use super::forge_registry::{
    DxForgeR2Config, DxForgeRemoteObjectHeadExecutionApproval,
    DxForgeRemoteObjectHeadExecutionCheck, DxForgeRemoteObjectHeadExecutionReceipt,
    DxForgeRemoteObjectHeadExecutionStatus, DxForgeRemoteObjectMetadataCheck,
    DxForgeRemoteObjectMetadataPlan, DxForgeRemoteProviderKind,
};

/// Execute approved R2/S3 object HEAD checks from environment configuration.
///
/// This is read-only metadata probing. It never writes, deletes, fetches blobs,
/// installs packages, or serializes secret values.
pub async fn execute_r2_remote_object_head_checks_from_env(
    metadata_plan: &DxForgeRemoteObjectMetadataPlan,
    approval: DxForgeRemoteObjectHeadExecutionApproval,
) -> Result<DxForgeRemoteObjectHeadExecutionReceipt> {
    validate_live_head_approval(metadata_plan, &approval)?;
    let config = DxForgeR2Config::from_env()
        .context("Cloudflare R2/S3-compatible config is required for live HEAD checks")?;
    execute_r2_remote_object_head_checks_with_config(&config, metadata_plan, approval).await
}

/// Execute approved R2/S3 object HEAD checks against an explicit config.
///
/// Callers must still pass approval with `network_allowed=true`; this keeps
/// tests, Zed panels, and CLI flows from accidentally turning a dry-run plan
/// into a live cloud read.
pub async fn execute_r2_remote_object_head_checks_with_config(
    config: &DxForgeR2Config,
    metadata_plan: &DxForgeRemoteObjectMetadataPlan,
    approval: DxForgeRemoteObjectHeadExecutionApproval,
) -> Result<DxForgeRemoteObjectHeadExecutionReceipt> {
    validate_live_head_approval(metadata_plan, &approval)?;
    let store = config.store()?;
    let mut checks = Vec::with_capacity(metadata_plan.checks.len());

    for check in &metadata_plan.checks {
        checks.push(head_check(&store, check).await.with_context(|| {
            format!(
                "read R2/S3 HEAD metadata for Forge object `{}`",
                check.object_key
            )
        })?);
    }

    let mut warnings = metadata_plan.warnings.clone();
    warnings.push(
        "approved live R2/S3 HEAD execution is metadata-only: no writes, deletes, blob fetches, installs, or secret values are allowed".to_string(),
    );

    Ok(DxForgeRemoteObjectHeadExecutionReceipt {
        schema_version: "dx.forge.remote_object_head_execution_receipt".to_string(),
        source_plan_schema: metadata_plan.schema_version.clone(),
        provider_kind: metadata_plan.provider_kind,
        package_id: metadata_plan.package_id.clone(),
        version: metadata_plan.version.clone(),
        provider_mode: approval.provider_mode.trim().to_string(),
        approval_required: true,
        approved: true,
        approved_by: Some(approval.approved_by.trim().to_string()),
        dry_run: false,
        network_allowed: true,
        write_allowed: false,
        checks,
        warnings,
        next_actions: vec![
            "evaluate the measured HEAD receipt before remote install".to_string(),
            "block remote install when required objects are missing or byte-mismatched".to_string(),
            "keep R2/S3 writes behind a separate explicit approval path".to_string(),
        ],
    })
}

fn validate_live_head_approval(
    metadata_plan: &DxForgeRemoteObjectMetadataPlan,
    approval: &DxForgeRemoteObjectHeadExecutionApproval,
) -> Result<()> {
    if metadata_plan.provider_kind != DxForgeRemoteProviderKind::S3CompatibleObjectStorage {
        bail!("live HEAD execution only supports S3-compatible object storage plans");
    }
    if !approval.network_allowed {
        bail!("live R2/S3 HEAD execution requires explicit network approval");
    }
    if approval.approved_by.trim().is_empty() {
        bail!("live R2/S3 HEAD execution requires a non-empty approval operator");
    }
    if approval.provider_mode.trim().is_empty() {
        bail!("live R2/S3 HEAD execution requires a provider mode label");
    }
    if metadata_plan.write_allowed {
        bail!("live R2/S3 HEAD execution refuses metadata plans that allow writes");
    }
    Ok(())
}

async fn head_check(
    store: &object_store::aws::AmazonS3,
    check: &DxForgeRemoteObjectMetadataCheck,
) -> Result<DxForgeRemoteObjectHeadExecutionCheck> {
    let path = ObjectPath::from(check.object_key.as_str());
    let result = store.head(&path).await;
    let (exists, bytes, etag, last_modified) = match result {
        Ok(meta) => (
            true,
            Some(meta.size),
            meta.e_tag,
            Some(meta.last_modified.to_rfc3339()),
        ),
        Err(ObjectStoreError::NotFound { .. }) => (false, None, None, None),
        Err(error) => return Err(error).context("head object"),
    };

    Ok(DxForgeRemoteObjectHeadExecutionCheck {
        intent: check.intent.clone(),
        metadata_operation: check.metadata_operation.clone(),
        object_key: check.object_key.clone(),
        required: check.required,
        expected_hash: check.expected_hash.clone(),
        expected_bytes: check.expected_bytes,
        approved: true,
        executed: true,
        status: DxForgeRemoteObjectHeadExecutionStatus::Measured,
        measured_exists: Some(exists),
        measured_bytes: bytes,
        measured_etag: etag,
        measured_last_modified: last_modified,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_r2_head_adapter_requires_explicit_network_approval() {
        let plan = DxForgeRemoteObjectMetadataPlan {
            schema_version: "dx.forge.remote_object_metadata_plan".to_string(),
            provider_kind: DxForgeRemoteProviderKind::S3CompatibleObjectStorage,
            package_id: "auth/better-auth".to_string(),
            version: "0.1.0".to_string(),
            network_allowed: false,
            write_allowed: false,
            checks: Vec::new(),
            warnings: Vec::new(),
        };
        let approval = DxForgeRemoteObjectHeadExecutionApproval {
            approved_by: "test".to_string(),
            provider_mode: "r2-head".to_string(),
            network_allowed: false,
        };

        let runtime = tokio::runtime::Runtime::new().expect("tokio runtime");
        let error = runtime
            .block_on(execute_r2_remote_object_head_checks_from_env(
                &plan, approval,
            ))
            .expect_err("network approval should be required before env lookup");

        assert!(
            error
                .to_string()
                .contains("requires explicit network approval"),
            "{error:?}"
        );
    }
}
