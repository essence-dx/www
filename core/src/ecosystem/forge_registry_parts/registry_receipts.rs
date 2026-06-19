fn write_registry_package_to_local(
    root: &Path,
    package: &DxForgeRegistryPackage,
    index: Option<&DxForgeRegistryIndex>,
) -> Result<Vec<String>> {
    verify_registry_package_integrity(package)?;
    let mut written = Vec::new();
    if let Some(index) = index {
        let index_path = root.join(REGISTRY_INDEX);
        fs::write(&index_path, serde_json::to_vec_pretty(index)?)
            .with_context(|| format!("write `{}`", index_path.display()))?;
        written.push(index_path.display().to_string());
    }

    let manifest_path = root.join(relative_registry_package_manifest(package));
    if let Some(parent) = manifest_path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create `{}`", parent.display()))?;
    }
    fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&package.clone_without_content())?,
    )
    .with_context(|| format!("write `{}`", manifest_path.display()))?;
    written.push(manifest_path.display().to_string());

    for file in &package.files {
        let content = file
            .content
            .as_deref()
            .context("registry package file missing content")?;
        let file_path = root.join(relative_registry_file(package, &file.hash));
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("create `{}`", parent.display()))?;
        }
        fs::write(&file_path, content)
            .with_context(|| format!("write `{}`", file_path.display()))?;
        written.push(file_path.display().to_string());
    }

    Ok(written)
}

fn root_dx_registry_package_from_manifest(
    project: &Path,
    manifest: &DxForgeRootPackageManifest,
) -> Result<DxForgeRegistryPackage> {
    let mut files = Vec::new();
    for file in &manifest.files {
        let source_path = manifest.source_root.join(&file.from);
        let content = fs::read_to_string(&source_path)
            .with_context(|| format!("read root dx package source `{}`", source_path.display()))?;
        files.push(DxSourceFile {
            path: file.to.clone(),
            logical_path: Some(file.from.clone()),
            hash: hash_bytes(content.as_bytes()),
            bytes: content.len() as u64,
            content: Some(content),
        });
    }
    let integrity_hash = package_integrity_hash(&files);

    let package = DxForgeRegistryPackage {
        package_id: manifest.package_id.clone(),
        aliases: Vec::new(),
        language: DxForgeLanguage::Js,
        version: manifest.version.clone(),
        source: DxForgeRegistrySource::RootDx {
            project: project.display().to_string(),
        },
        source_kind: DxSourceKind::Local,
        license: manifest.license.clone(),
        description: manifest.description.clone(),
        provenance: DxForgeProvenanceMetadata {
            source: "root-dx-package-manifest".to_string(),
            upstream_reference: Some(manifest.source_root.display().to_string()),
            verified: false,
            note: "Package files were published from the project's root dx manifest; external provenance verification is not claimed.".to_string(),
        },
        advisory_review: DxForgeAdvisoryMetadata {
            coverage_kind: DxForgeAdvisoryCoverageKind::Missing,
            provider: "none".to_string(),
            live_coverage: false,
            finding_count: 0,
            reviewed_at: None,
            note: "Root dx local registry packages do not have live advisory coverage attached yet."
                .to_string(),
        },
        license_review: DxForgeLicenseReviewMetadata {
            declared_license: manifest.license.clone(),
            reviewed: false,
            reviewed_at: None,
            note: "License is recorded from the root dx package declaration only; no formal DX legal review is claimed.".to_string(),
        },
        exports: manifest
            .exports
            .iter()
            .map(root_manifest_export_to_registry_export)
            .collect(),
        default_exports: manifest.default_exports.clone(),
        allow_selective_imports: manifest.allow_selective_imports,
        files,
        integrity_hash,
    };
    verify_registry_package_integrity(&package)?;
    Ok(package)
}

fn root_manifest_export_to_registry_export(
    export: &DxForgeRootPackageExport,
) -> DxForgeRegistryExport {
    DxForgeRegistryExport {
        name: export.name.clone(),
        files: export.files.clone(),
    }
}

fn local_registry_object_paths(
    root: &Path,
    package: &DxForgeRegistryPackage,
    include_receipt: bool,
) -> Vec<String> {
    let mut objects = vec![
        root.join(REGISTRY_INDEX).display().to_string(),
        root.join(relative_registry_package_manifest(package))
            .display()
            .to_string(),
    ];
    for file in &package.files {
        objects.push(
            root.join(relative_registry_file(package, &file.hash))
                .display()
                .to_string(),
        );
    }
    if include_receipt {
        objects.push(
            root.join("receipts")
                .join(format!(
                    "<timestamp>-{}.json",
                    package.package_id.replace('/', "-")
                ))
                .display()
                .to_string(),
        );
    }
    objects
}

fn merged_local_registry_index(
    root: &Path,
    package: &DxForgeRegistryPackage,
) -> Result<DxForgeRegistryIndex> {
    let index_path = root.join(REGISTRY_INDEX);
    let mut index = if index_path.exists() {
        let bytes =
            fs::read(&index_path).with_context(|| format!("read `{}`", index_path.display()))?;
        serde_json::from_slice::<DxForgeRegistryIndex>(&bytes)
            .with_context(|| format!("parse `{}`", index_path.display()))?
    } else {
        DxForgeRegistryIndex {
            version: 1,
            generated_at: Utc::now().to_rfc3339(),
            packages: Vec::new(),
            remotes: Vec::new(),
        }
    };
    index.generated_at = Utc::now().to_rfc3339();
    index.packages.retain(|existing| {
        !(existing.package_id == package.package_id && existing.version == package.version)
    });
    index.packages.push(package.clone_without_content());
    Ok(index)
}

fn write_local_registry_publish_receipt(
    root: &Path,
    package: &DxForgeRegistryPackage,
) -> Result<PathBuf> {
    let receipt = registry_publish_receipt(package);
    let receipt_path = root.join("receipts").join(format!(
        "{}-{}.json",
        Utc::now().format("%Y%m%dT%H%M%SZ"),
        package.package_id.replace('/', "-")
    ));
    if let Some(parent) = receipt_path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create `{}`", parent.display()))?;
    }
    fs::write(&receipt_path, serde_json::to_vec_pretty(&receipt)?)
        .with_context(|| format!("write `{}`", receipt_path.display()))?;
    Ok(receipt_path)
}

fn source_package_from_registry_selected_exports(
    package: &DxForgeRegistryPackage,
    selected_exports: &[String],
    project: &Path,
) -> Result<DxSourcePackage> {
    let selected_files = selected_registry_files(package, selected_exports)?;
    let config = DxForgeProjectConfig::load(project)?;
    let files = selected_files
        .into_iter()
        .map(|file| {
            let materialized_path = registry_materialized_path(&config, &file.path)?;
            let content = file
                .content
                .as_deref()
                .context("local registry package file is missing hydrated content")?;
            let content = if file.path.starts_with("js/") {
                materialized_file_content(
                    &file.path,
                    &materialized_path,
                    content,
                    &config,
                    "default",
                )?
            } else {
                content.to_string()
            };
            let hash = hash_bytes(content.as_bytes());
            Ok(DxSourceFile {
                path: materialized_path,
                logical_path: Some(file.path.clone()),
                hash,
                bytes: content.len() as u64,
                content: Some(content),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let integrity_hash = package_integrity_hash(&files);

    Ok(DxSourcePackage {
        package_id: package.package_id.clone(),
        upstream_name: package.description.clone(),
        version: package.version.clone(),
        generator: "dx-forge/local-registry".to_string(),
        variant: selected_registry_variant(selected_exports),
        last_accepted_update: None,
        rollback_receipt: None,
        source_kind: package.source_kind.clone(),
        integrity_hash,
        license: package.license.clone(),
        provenance: package.provenance.clone(),
        advisory_review: package.advisory_review.clone(),
        license_review: package.license_review.clone(),
        files,
    })
}

fn selected_registry_files<'a>(
    package: &'a DxForgeRegistryPackage,
    selected_exports: &[String],
) -> Result<Vec<&'a DxSourceFile>> {
    let mut selected = normalize_export_names(selected_exports);
    if selected.is_empty() {
        selected = normalize_export_names(&package.default_exports);
    }
    if selected.is_empty() {
        return Ok(package.files.iter().collect());
    }
    if !package.allow_selective_imports && !selected_exports.is_empty() {
        bail!(
            "registry package `{}` does not allow selective imports",
            package.package_id
        );
    }

    let mut destinations = BTreeSet::new();
    for export_name in &selected {
        let Some(export) = package
            .exports
            .iter()
            .find(|export| export.name == *export_name)
        else {
            bail!(
                "registry package `{}` does not define export `{}`",
                package.package_id,
                export_name
            );
        };
        for file in &export.files {
            destinations.insert(file.clone());
        }
    }

    let files = package
        .files
        .iter()
        .filter(|file| destinations.contains(&file.path))
        .collect::<Vec<_>>();
    if files.is_empty() {
        bail!(
            "registry package `{}` selected exports `{}` did not match any files",
            package.package_id,
            selected.join(",")
        );
    }
    Ok(files)
}

fn registry_materialized_path(config: &DxForgeProjectConfig, path: &str) -> Result<String> {
    if path.starts_with("js/") {
        config.materialize_path(path)
    } else {
        validate_project_relative_path(path)?;
        Ok(path.to_string())
    }
}

fn remote_manifest_install_preview(
    package: &DxForgeRegistryPackage,
    selected_exports: &[String],
    manifest_path: &Path,
    project: &Path,
) -> Result<DxForgeRemoteManifestInstallPreview> {
    let files = selected_registry_files(package, selected_exports)?;
    let config = DxForgeProjectConfig::load(project)?;
    let mut file_plans = Vec::new();
    let mut missing_file_count = 0u64;
    let mut matching_file_count = 0u64;
    let mut conflicting_file_count = 0u64;

    for file in files {
        let materialized_path = registry_materialized_path(&config, &file.path)?;
        let target_path = project.join(&materialized_path);
        let existing_hash = if target_path.exists() {
            let bytes = fs::read(&target_path).with_context(|| {
                format!("read existing Forge target `{}`", target_path.display())
            })?;
            Some(hash_bytes(&bytes))
        } else {
            None
        };
        let status = match existing_hash.as_deref() {
            None => {
                missing_file_count += 1;
                DxForgeRemoteManifestFileStatus::Missing
            }
            Some(hash) if hash == file.hash => {
                matching_file_count += 1;
                DxForgeRemoteManifestFileStatus::Matching
            }
            Some(_) => {
                conflicting_file_count += 1;
                DxForgeRemoteManifestFileStatus::ConflictingLocalFile
            }
        };

        file_plans.push(DxForgeRemoteManifestFilePlan {
            logical_path: file
                .logical_path
                .clone()
                .unwrap_or_else(|| file.path.clone()),
            materialized_path,
            manifest_hash: file.hash.clone(),
            bytes: file.bytes,
            existing_hash,
            status,
        });
    }

    let mut normalized_exports = normalize_export_names(selected_exports);
    if normalized_exports.is_empty() {
        normalized_exports = normalize_export_names(&package.default_exports);
    }
    let mut warnings = vec![
        "remote manifest fixture was parsed from local disk; no R2/S3 object was fetched"
            .to_string(),
        "content blobs were not fetched; preview compares declared manifest hashes against existing project files".to_string(),
    ];
    if package
        .files
        .iter()
        .any(|file| file.path.starts_with("js/"))
    {
        warnings.push(
            "source-rewrite transforms are not applied because remote blobs were not fetched; use live local registry install for write validation".to_string(),
        );
    }

    Ok(DxForgeRemoteManifestInstallPreview {
        schema_version: "dx.forge.remote_manifest_install_preview".to_string(),
        manifest_path: manifest_path.to_path_buf(),
        package_id: package.package_id.clone(),
        version: package.version.clone(),
        selected_exports: normalized_exports,
        network_allowed: false,
        write_allowed: false,
        selected_file_count: file_plans.len() as u64,
        missing_file_count,
        matching_file_count,
        conflicting_file_count,
        file_plans,
        warnings,
    })
}

fn normalize_export_names(values: &[String]) -> Vec<String> {
    let mut names = Vec::new();
    for value in values {
        let value = value.trim();
        if !value.is_empty() && !names.iter().any(|existing| existing == value) {
            names.push(value.to_string());
        }
    }
    names
}

fn selected_registry_variant(selected_exports: &[String]) -> String {
    let selected = normalize_export_names(selected_exports);
    if selected.is_empty() {
        "default".to_string()
    } else {
        let joined = selected.join("-");
        let sanitized = joined
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                    ch
                } else {
                    '-'
                }
            })
            .collect::<String>();
        let sanitized = sanitized.trim_matches('-');
        if sanitized.is_empty() {
            "export-selected".to_string()
        } else {
            format!("export-{sanitized}")
        }
    }
}

fn hydrate_local_registry_package(
    root: &Path,
    mut package: DxForgeRegistryPackage,
) -> Result<DxForgeRegistryPackage> {
    for index in 0..package.files.len() {
        let hash = package.files[index].hash.clone();
        let file_path = root.join(relative_registry_file(&package, &hash));
        let bytes = fs::read(&file_path)
            .with_context(|| format!("read local registry blob `{}`", file_path.display()))?;
        verify_registry_file_blob(&package, index, &bytes)
            .with_context(|| format!("verify local registry blob `{}`", file_path.display()))?;
        package.files[index].content =
            Some(String::from_utf8(bytes).with_context(|| {
                format!("decode local registry blob `{}`", file_path.display())
            })?);
    }
    Ok(package)
}

async fn hydrate_r2_registry_package(
    store: &object_store::aws::AmazonS3,
    prefix: &str,
    mut package: DxForgeRegistryPackage,
) -> Result<DxForgeRegistryPackage> {
    for index in 0..package.files.len() {
        let hash = package.files[index].hash.clone();
        let file_key = package_file_key(prefix, &package, &hash);
        let bytes = store
            .get(&ObjectPath::from(file_key.as_str()))
            .await
            .with_context(|| format!("pull registry blob `{file_key}`"))?
            .bytes()
            .await
            .with_context(|| format!("read registry blob `{file_key}`"))?;
        verify_registry_file_blob(&package, index, &bytes)
            .with_context(|| format!("verify registry blob `{file_key}`"))?;
        package.files[index].content = Some(
            String::from_utf8(bytes.to_vec())
                .with_context(|| format!("decode registry blob `{file_key}` as UTF-8 source"))?,
        );
    }
    Ok(package)
}

fn verify_registry_file_blob(
    package: &DxForgeRegistryPackage,
    index: usize,
    bytes: &[u8],
) -> Result<()> {
    let file = package
        .files
        .get(index)
        .with_context(|| format!("missing file index {index} in registry package"))?;
    let actual_hash = hash_bytes(bytes);
    if actual_hash != file.hash {
        bail!(
            "registry package `{}` file `{}` blob hash mismatch",
            package.package_id,
            file.path
        );
    }
    let actual_bytes = bytes.len() as u64;
    if actual_bytes != file.bytes {
        bail!(
            "registry package `{}` file `{}` blob byte count mismatch",
            package.package_id,
            file.path
        );
    }
    Ok(())
}

fn registry_object_keys(
    prefix: &str,
    package: &DxForgeRegistryPackage,
    include_receipt_prefix: bool,
) -> Vec<String> {
    let mut keys = vec![
        format!("{prefix}/{REGISTRY_INDEX}"),
        package_manifest_key(prefix, package),
    ];
    for file in &package.files {
        keys.push(package_file_key(prefix, package, &file.hash));
    }
    if include_receipt_prefix {
        keys.push(format!(
            "{prefix}/receipts/<timestamp>-{}.json",
            package.package_id.replace('/', "-")
        ));
    }
    keys
}

fn relative_registry_package_manifest(package: &DxForgeRegistryPackage) -> PathBuf {
    PathBuf::from(format!(
        "packages/{}/{}/{}/manifest.json",
        package.language.as_segment(),
        package.package_id,
        package.version
    ))
}

fn relative_registry_file(package: &DxForgeRegistryPackage, hash: &str) -> PathBuf {
    PathBuf::from(format!(
        "packages/{}/{}/{}/files/{hash}",
        package.language.as_segment(),
        package.package_id,
        package.version
    ))
}

fn package_manifest_key(prefix: &str, package: &DxForgeRegistryPackage) -> String {
    format!(
        "{prefix}/packages/{}/{}/{}/manifest.json",
        package.language.as_segment(),
        package.package_id,
        package.version
    )
}

fn package_file_key(prefix: &str, package: &DxForgeRegistryPackage, hash: &str) -> String {
    format!(
        "{prefix}/packages/{}/{}/{}/files/{hash}",
        package.language.as_segment(),
        package.package_id,
        package.version
    )
}

fn registry_publish_receipt(package: &DxForgeRegistryPackage) -> DxForgeReceipt {
    let source_package = if matches!(package.source, DxForgeRegistrySource::Curated) {
        default_source_package(&package.package_id)
            .unwrap_or_else(|_| registry_package_to_source_package(package))
    } else {
        registry_package_to_source_package(package)
    };
    let file_map = source_package
        .files
        .iter()
        .map(|file| DxForgeFileMap {
            logical_path: file
                .logical_path
                .clone()
                .unwrap_or_else(|| file.path.clone()),
            materialized_path: file.path.clone(),
            hash: file.hash.clone(),
            bytes: file.bytes,
        })
        .collect();

    DxForgeReceipt {
        action: DxForgeAction::Audit,
        package: source_package,
        selected_exports: Vec::new(),
        files_written: package
            .files
            .iter()
            .map(|file| DxSourceFile {
                path: file.path.clone(),
                logical_path: file.logical_path.clone(),
                hash: file.hash.clone(),
                bytes: file.bytes,
                content: None,
            })
            .collect(),
        file_map,
        policy_decisions: vec![DxPolicyDecision {
            policy: "registry-publish".to_string(),
            traffic: DxUpdateTraffic::Green,
            message: "Published source-owned package metadata and content blobs.".to_string(),
        }],
        update_decisions: Vec::new(),
        risk_score: 100,
        timestamp: Utc::now().to_rfc3339(),
        signature: None,
    }
}

fn registry_package_to_source_package(package: &DxForgeRegistryPackage) -> DxSourcePackage {
    DxSourcePackage {
        package_id: package.package_id.clone(),
        upstream_name: registry_upstream_name(&package.package_id).to_string(),
        version: package.version.clone(),
        generator: registry_generator_name(&package.package_id).to_string(),
        variant: "default".to_string(),
        last_accepted_update: None,
        rollback_receipt: None,
        source_kind: package.source_kind.clone(),
        integrity_hash: package.integrity_hash.clone(),
        license: package.license.clone(),
        provenance: package.provenance.clone(),
        advisory_review: package.advisory_review.clone(),
        license_review: package.license_review.clone(),
        files: package
            .files
            .iter()
            .map(|file| DxSourceFile {
                path: file.path.clone(),
                logical_path: file.logical_path.clone(),
                hash: file.hash.clone(),
                bytes: file.bytes,
                content: None,
            })
            .collect(),
    }
}

async fn put_json<T: Serialize>(
    store: &object_store::aws::AmazonS3,
    key: &str,
    value: &T,
) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(value)?;
    put_bytes(store, key, &bytes).await
}

async fn put_bytes(store: &object_store::aws::AmazonS3, key: &str, bytes: &[u8]) -> Result<()> {
    store
        .put(&ObjectPath::from(key), Bytes::copy_from_slice(bytes).into())
        .await
        .with_context(|| format!("put R2 object `{key}`"))?;
    Ok(())
}

fn package_integrity_hash(files: &[DxSourceFile]) -> String {
    let mut hasher = blake3::Hasher::new();
    for file in files {
        hasher.update(file.path.as_bytes());
        hasher.update(file.hash.as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

fn hash_bytes(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

fn ensure_requested_package(
    package: &DxForgeRegistryPackage,
    package_id: &str,
    version: &str,
) -> Result<()> {
    if package.package_id != package_id {
        bail!(
            "registry manifest package id mismatch: expected `{package_id}`, found `{}`",
            package.package_id
        );
    }
    if package.version != version {
        bail!(
            "registry manifest version mismatch for `{package_id}`: expected `{version}`, found `{}`",
            package.version
        );
    }
    Ok(())
}

fn is_blake3_hex(value: &str) -> bool {
    value.len() == 64 && value.as_bytes().iter().all(|byte| byte.is_ascii_hexdigit())
}

fn validate_project_relative_path(path: &str) -> Result<()> {
    if path.trim().is_empty() {
        bail!("path cannot be empty");
    }
    if path.contains('\\') {
        bail!("path must use `/` separators");
    }
    let path = Path::new(path);
    if path.is_absolute() {
        bail!("path must be project-relative");
    }
    for component in path.components() {
        if matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        ) {
            bail!("path cannot escape the project root");
        }
    }
    Ok(())
}

fn url_escape_segment(segment: &str) -> String {
    segment
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                vec![byte as char]
            }
            other => format!("%{other:02X}").chars().collect(),
        })
        .collect()
}

fn object_url_from_status(status: &DxForgeR2Status, key: &str) -> String {
    let key = if status.public_base_url_set {
        key.split('/')
            .map(url_escape_segment)
            .collect::<Vec<_>>()
            .join("/")
    } else {
        key.to_string()
    };
    let bucket = if status.bucket_set {
        "<configured-bucket>"
    } else {
        "<unconfigured-bucket>"
    };
    format!("r2://{bucket}/{key}")
}
