pub fn plan_r2_remote_object_head_execution_receipt(
    metadata_plan: &DxForgeRemoteObjectMetadataPlan,
) -> DxForgeRemoteObjectHeadExecutionReceipt {
    let checks = metadata_plan
        .checks
        .iter()
        .map(|check| DxForgeRemoteObjectHeadExecutionCheck {
            intent: check.intent.clone(),
            metadata_operation: check.metadata_operation.clone(),
            object_key: check.object_key.clone(),
            required: check.required,
            expected_hash: check.expected_hash.clone(),
            expected_bytes: check.expected_bytes,
            approved: false,
            executed: false,
            status: DxForgeRemoteObjectHeadExecutionStatus::RequiresExplicitApproval,
            measured_exists: None,
            measured_bytes: None,
            measured_etag: None,
            measured_last_modified: None,
        })
        .collect();

    let mut warnings = metadata_plan.warnings.clone();
    warnings.push(
        "read-only R2/S3 HEAD execution receipt shape only; no HEAD request was executed"
            .to_string(),
    );
    warnings.push(
        "live object metadata probing requires explicit operator approval and must not enable writes, deletes, blob fetches, or installs".to_string(),
    );

    DxForgeRemoteObjectHeadExecutionReceipt {
        schema_version: "dx.forge.remote_object_head_execution_receipt".to_string(),
        source_plan_schema: metadata_plan.schema_version.clone(),
        provider_kind: metadata_plan.provider_kind,
        package_id: metadata_plan.package_id.clone(),
        version: metadata_plan.version.clone(),
        provider_mode: "planned-only".to_string(),
        approval_required: true,
        approved: false,
        approved_by: None,
        dry_run: true,
        network_allowed: false,
        write_allowed: false,
        checks,
        warnings,
        next_actions: vec![
            "record explicit operator approval before any live HEAD request".to_string(),
            "execute read-only provider metadata checks without fetching package blobs".to_string(),
            "write measured object existence and byte metadata into this receipt shape".to_string(),
        ],
    }
}

/// Execute approved read-only object HEAD checks through an injected provider.
pub fn execute_r2_remote_object_head_checks_with_provider<P>(
    metadata_plan: &DxForgeRemoteObjectMetadataPlan,
    approval: DxForgeRemoteObjectHeadExecutionApproval,
    provider: &P,
) -> Result<DxForgeRemoteObjectHeadExecutionReceipt>
where
    P: DxForgeRemoteObjectHeadProvider,
{
    let approved_by = approval.approved_by.trim();
    if approved_by.is_empty() {
        bail!("read-only R2/S3 HEAD execution requires a non-empty approval operator");
    }
    let provider_mode = approval.provider_mode.trim();
    if provider_mode.is_empty() {
        bail!("read-only R2/S3 HEAD execution requires a provider mode label");
    }

    let checks = metadata_plan
        .checks
        .iter()
        .map(|check| {
            let measurement = provider.head_object(check).with_context(|| {
                format!(
                    "read HEAD metadata for planned Forge object `{}`",
                    check.object_key
                )
            })?;
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
                measured_exists: Some(measurement.exists),
                measured_bytes: measurement.bytes,
                measured_etag: measurement.etag,
                measured_last_modified: measurement.last_modified,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let mut warnings = metadata_plan.warnings.clone();
    if !approval.network_allowed {
        warnings.push(
            "HEAD receipt populated by an approved test provider; no live R2/S3 request was performed"
                .to_string(),
        );
    }
    warnings.push(
        "read-only HEAD execution remains metadata-only: no writes, deletes, blob fetches, installs, or secret values are allowed".to_string(),
    );

    Ok(DxForgeRemoteObjectHeadExecutionReceipt {
        schema_version: "dx.forge.remote_object_head_execution_receipt".to_string(),
        source_plan_schema: metadata_plan.schema_version.clone(),
        provider_kind: metadata_plan.provider_kind,
        package_id: metadata_plan.package_id.clone(),
        version: metadata_plan.version.clone(),
        provider_mode: provider_mode.to_string(),
        approval_required: true,
        approved: true,
        approved_by: Some(approved_by.to_string()),
        dry_run: false,
        network_allowed: approval.network_allowed,
        write_allowed: false,
        checks,
        warnings,
        next_actions: vec![
            "compare measured object existence against required package objects".to_string(),
            "surface missing or byte-mismatched objects in dx-check and Zed before any install"
                .to_string(),
            "keep real R2/S3 execution behind explicit approval and read-only credentials"
                .to_string(),
        ],
    })
}

/// Read-only provider adapter boundary for object metadata planning.
pub trait DxForgeRemoteReadProvider {
    /// Plan object metadata checks from a verified registry package manifest.
    fn object_metadata_plan(
        &self,
        package: &DxForgeRegistryPackage,
    ) -> DxForgeRemoteObjectMetadataPlan;
}

/// R2/S3-compatible provider adapter that only plans metadata checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxForgeR2ReadOnlyProvider {
    prefix: String,
}

impl DxForgeR2ReadOnlyProvider {
    /// Build an R2/S3-compatible metadata planner from redacted env-derived status.
    pub fn from_status(status: &DxForgeR2Status) -> Self {
        Self {
            prefix: status.prefix.clone(),
        }
    }
}

impl DxForgeRemoteReadProvider for DxForgeR2ReadOnlyProvider {
    fn object_metadata_plan(
        &self,
        package: &DxForgeRegistryPackage,
    ) -> DxForgeRemoteObjectMetadataPlan {
        let package_object_path = package.package_id.trim_matches('/');
        let mut checks = vec![
            DxForgeRemoteObjectMetadataCheck {
                intent: "latest-version".to_string(),
                metadata_operation: "head-object".to_string(),
                object_key: format!(
                    "{}/packages/{}/{package_object_path}/latest.json",
                    self.prefix,
                    package.language.as_segment()
                ),
                required: false,
                expected_hash: None,
                expected_bytes: None,
                status: DxForgeRemoteObjectMetadataStatus::PlannedNotChecked,
            },
            DxForgeRemoteObjectMetadataCheck {
                intent: "package-manifest".to_string(),
                metadata_operation: "head-object".to_string(),
                object_key: package_manifest_key(&self.prefix, package),
                required: true,
                expected_hash: None,
                expected_bytes: None,
                status: DxForgeRemoteObjectMetadataStatus::PlannedNotChecked,
            },
        ];
        for file in &package.files {
            checks.push(DxForgeRemoteObjectMetadataCheck {
                intent: "content-blob".to_string(),
                metadata_operation: "head-object".to_string(),
                object_key: package_file_key(&self.prefix, package, &file.hash),
                required: true,
                expected_hash: Some(file.hash.clone()),
                expected_bytes: Some(file.bytes),
                status: DxForgeRemoteObjectMetadataStatus::PlannedNotChecked,
            });
        }

        DxForgeRemoteObjectMetadataPlan {
            schema_version: "dx.forge.remote_object_metadata_plan".to_string(),
            provider_kind: DxForgeRemoteProviderKind::S3CompatibleObjectStorage,
            package_id: package.package_id.clone(),
            version: package.version.clone(),
            network_allowed: false,
            write_allowed: false,
            checks,
            warnings: vec![
                "R2/S3 object metadata checks are planned only; no HEAD or GET request was performed".to_string(),
                "content hashes and byte lengths come from the verified package manifest fixture, not from remote object metadata".to_string(),
            ],
        }
    }
}
