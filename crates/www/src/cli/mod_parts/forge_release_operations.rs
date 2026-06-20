fn forge_release_bundle_terminal(report: &DxForgeReleaseBundleReport) -> String {
    let mut output = format!(
        "DX Forge release bundle\nBundle: {}\nPassed: {}\nScore: {}\nRoutes: {}\nArtifacts: {}\nNo node_modules: {}\n",
        report.bundle_dir.display(),
        report.passed,
        report.score,
        report.route_count,
        report.artifact_count,
        report.no_node_modules
    );
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

fn forge_release_bundle_markdown(report: &DxForgeReleaseBundleReport) -> String {
    let mut output = format!(
        "# DX Forge Release Bundle\n\n- Bundle directory: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Routes: `{}`\n- Artifacts: `{}`\n- No node_modules: `{}`\n\n",
        report.bundle_dir.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.route_count,
        report.artifact_count,
        report.no_node_modules
    );

    output.push_str("## Public Route Checks\n\n");
    output.push_str("| Route or check | Artifacts | Status |\n");
    output.push_str("| --- | --- | --- |\n");
    for route in &report.routes {
        let links = route
            .artifacts
            .iter()
            .map(|artifact| markdown_artifact_link(artifact))
            .collect::<Vec<_>>()
            .join(", ");
        output.push_str(&format!(
            "| `{}` | {} | {} |\n",
            route.route, links, route.message
        ));
    }

    output.push_str("\n## Artifact Checks\n\n");
    output.push_str("| Artifact | Link | Bytes | Status |\n");
    output.push_str("| --- | --- | ---: | --- |\n");
    for artifact in &report.artifacts {
        output.push_str(&format!(
            "| `{}` | {} | {} | {} |\n",
            artifact.name,
            markdown_artifact_link(&artifact.name),
            artifact.bytes,
            artifact.message
        ));
    }

    if !report.findings.is_empty() {
        output.push_str("\n## Findings\n\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output
}

fn forge_release_bundle_failure_summary(report: &DxForgeReleaseBundleReport) -> String {
    if report.findings.is_empty() {
        return format!(
            "Forge release bundle verification failed with score {}",
            report.score
        );
    }
    format!(
        "Forge release bundle verification failed: {}",
        report.findings.join("; ")
    )
}

fn generate_forge_publisher_key(
    out_dir: &Path,
    signer: &str,
    force: bool,
) -> anyhow::Result<DxForgePublisherKeyGenerateReport> {
    let signer = validate_forge_publisher_signer(signer)?;
    let private_key_path = out_dir.join(FORGE_PUBLISHER_PRIVATE_KEY_FILE);
    let public_key_path = out_dir.join(FORGE_PUBLISHER_PUBLIC_KEY_FILE);
    if !force {
        for path in [&private_key_path, &public_key_path] {
            if path.exists() {
                anyhow::bail!(
                    "Publisher key file already exists: {}. Pass --force to replace it.",
                    path.display()
                );
            }
        }
    }

    std::fs::create_dir_all(out_dir)?;
    let secret = forge_generate_publisher_secret();
    let signing_key = SigningKey::from_bytes(&secret);
    let public_key_bytes = signing_key.verifying_key().to_bytes();
    let public_key = format!("ed25519:{}", encode_hex(&public_key_bytes));
    let private_key = format!("ed25519-seed:{}", encode_hex(&secret));
    let key_id = forge_release_bundle_publisher_key_id(&public_key_bytes);
    let generated_at = Utc::now().to_rfc3339();

    let private_file = DxForgePublisherPrivateKeyFile {
        version: 1,
        scheme: FORGE_PUBLISHER_KEY_SCHEME.to_string(),
        created_at: generated_at.clone(),
        signer: signer.clone(),
        algorithm: "ed25519".to_string(),
        key_id: key_id.clone(),
        public_key: public_key.clone(),
        private_key,
        message: "Store this private publisher key outside public release artifacts.".to_string(),
    };
    let public_file = DxForgePublisherPublicKeyFile {
        version: 1,
        scheme: FORGE_PUBLISHER_KEY_SCHEME.to_string(),
        created_at: generated_at.clone(),
        signer: signer.clone(),
        algorithm: "ed25519".to_string(),
        key_id: key_id.clone(),
        public_key: public_key.clone(),
        message: "Public publisher key metadata for verifying DX Forge release manifests."
            .to_string(),
    };
    std::fs::write(&private_key_path, serde_json::to_vec_pretty(&private_file)?)?;
    std::fs::write(&public_key_path, serde_json::to_vec_pretty(&public_file)?)?;

    Ok(DxForgePublisherKeyGenerateReport {
        version: 1,
        generated_at,
        passed: true,
        score: 100,
        signer,
        algorithm: "ed25519".to_string(),
        key_id,
        public_key,
        private_key_path: private_key_path.clone(),
        public_key_path: public_key_path.clone(),
        private_key_written: private_key_path.is_file(),
        public_key_written: public_key_path.is_file(),
        findings: Vec::new(),
        next_commands: vec![format!(
            "dx forge publisher-key sign --key {} --manifest <release-bundle>\\{} --format markdown",
            private_key_path.display(),
            FORGE_RELEASE_BUNDLE_MANIFEST_JSON
        )],
    })
}

fn sign_forge_release_manifest_with_publisher_key(
    key_path: &Path,
    manifest_path: &Path,
    manifest_output: Option<&Path>,
) -> anyhow::Result<DxForgePublisherKeySignReport> {
    let key = read_forge_publisher_private_key(key_path)?;
    let secret = decode_prefixed_hex(&key.private_key, "ed25519-seed:", "private_key", 32)
        .map_err(|error| anyhow::anyhow!("{error}"))?;
    let secret: [u8; 32] = secret
        .try_into()
        .map_err(|_| anyhow::anyhow!("publisher private_key must decode to 32 bytes"))?;
    let signing_key = SigningKey::from_bytes(&secret);
    let public_key_bytes = signing_key.verifying_key().to_bytes();
    let public_key = format!("ed25519:{}", encode_hex(&public_key_bytes));
    let key_id = forge_release_bundle_publisher_key_id(&public_key_bytes);
    if key.algorithm != "ed25519" {
        anyhow::bail!("Unsupported publisher key algorithm `{}`", key.algorithm);
    }
    if key.public_key != public_key {
        anyhow::bail!("Publisher key public_key does not match private key seed");
    }
    if key.key_id != key_id {
        anyhow::bail!("Publisher key key_id does not match private key seed");
    }

    let mut manifest: DxForgeReleaseBundleManifest =
        serde_json::from_slice(&std::fs::read(manifest_path)?)?;
    let signed_at = Utc::now().to_rfc3339();
    let payload = forge_release_bundle_manifest_signing_payload(
        &manifest,
        &key.signer,
        &key.key_id,
        &key.public_key,
        &signed_at,
    );
    let signature = format!(
        "ed25519:{}",
        encode_hex(&signing_key.sign(payload.as_bytes()).to_bytes())
    );
    manifest.integrity.signed = true;
    manifest.integrity.signature = Some(signature.clone());
    manifest.integrity.message =
        "Signed release manifest: Ed25519 publisher identity verified against the artifact digest."
            .to_string();
    manifest.publisher_identity.status = FORGE_RELEASE_BUNDLE_SIGNATURE_STATUS_SIGNED.to_string();
    manifest.publisher_identity.signer = Some(key.signer.clone());
    manifest.publisher_identity.key_id = Some(key.key_id.clone());
    manifest.publisher_identity.algorithm = Some("ed25519".to_string());
    manifest.publisher_identity.public_key = Some(key.public_key.clone());
    manifest.publisher_identity.signature = Some(signature.clone());
    manifest.publisher_identity.signed_at = Some(signed_at.clone());
    manifest.publisher_identity.message =
        "Publisher identity is attached with an Ed25519 signature over the release manifest digest."
            .to_string();

    let mut findings = Vec::new();
    let signature_verified = match verify_forge_release_bundle_manifest_signature(&manifest) {
        Ok(()) => true,
        Err(error) => {
            findings.push(error);
            false
        }
    };
    let output_manifest_path = manifest_output
        .map(Path::to_path_buf)
        .unwrap_or_else(|| manifest_path.to_path_buf());
    let mut wrote_manifest = false;
    let mut markdown_path = None;
    if signature_verified {
        if let Some(parent) = output_manifest_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&output_manifest_path, serde_json::to_vec_pretty(&manifest)?)?;
        wrote_manifest = true;
        if output_manifest_path
            .file_name()
            .and_then(|name| name.to_str())
            == Some(FORGE_RELEASE_BUNDLE_MANIFEST_JSON)
        {
            if let Some(parent) = output_manifest_path.parent() {
                let path = parent.join(FORGE_RELEASE_BUNDLE_MANIFEST_MD);
                std::fs::write(&path, forge_release_bundle_manifest_markdown(&manifest))?;
                markdown_path = Some(path);
            }
        }
    }

    let passed = signature_verified && wrote_manifest && findings.is_empty();
    Ok(DxForgePublisherKeySignReport {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score: if passed { 100 } else { 0 },
        signer: key.signer,
        key_id: key.key_id,
        public_key: key.public_key,
        key_path: key_path.to_path_buf(),
        manifest_path: manifest_path.to_path_buf(),
        output_manifest_path,
        markdown_path,
        signed_at,
        wrote_manifest,
        signature,
        signature_verified,
        manifest_digest: manifest.artifact_integrity.digest,
        artifact_count: manifest.artifacts.len(),
        findings,
        next_commands: vec![format!(
            "dx forge release-bundle --verify {} --format markdown --fail-under 90",
            manifest_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .display()
        )],
    })
}

fn read_forge_publisher_private_key(
    key_path: &Path,
) -> anyhow::Result<DxForgePublisherPrivateKeyFile> {
    let key: DxForgePublisherPrivateKeyFile = serde_json::from_slice(&std::fs::read(key_path)?)?;
    if key.version != 1 {
        anyhow::bail!("Unsupported publisher key version {}", key.version);
    }
    if key.scheme != FORGE_PUBLISHER_KEY_SCHEME {
        anyhow::bail!("Unsupported publisher key scheme `{}`", key.scheme);
    }
    validate_forge_publisher_signer(&key.signer)?;
    Ok(key)
}

fn validate_forge_publisher_signer(signer: &str) -> anyhow::Result<String> {
    let signer = signer.trim();
    if signer.is_empty() {
        anyhow::bail!("Publisher signer must not be empty");
    }
    if signer.len() > 128 || signer.chars().any(|ch| ch.is_control()) {
        anyhow::bail!("Publisher signer must be printable text up to 128 bytes");
    }
    Ok(signer.to_string())
}

fn forge_generate_publisher_secret() -> [u8; 32] {
    let left = *uuid::Uuid::new_v4().as_bytes();
    let right = *uuid::Uuid::new_v4().as_bytes();
    let mut secret = [0u8; 32];
    secret[..16].copy_from_slice(&left);
    secret[16..].copy_from_slice(&right);
    secret
}

fn build_forge_release_operations_report(
    project: &Path,
    release_manifest_path: &Path,
    trust_regression_path: &Path,
    release_candidate_path: &Path,
    ci_artifacts_path: &Path,
    public_evidence_path: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeReleaseOperationsReport> {
    let signed_manifest = summarize_forge_release_operations_manifest(release_manifest_path)?;
    let trust_regression = summarize_forge_release_operations_json_evidence(
        trust_regression_path,
        "trust-regression",
        fail_under,
        Some(6),
    );
    let release_candidate = summarize_forge_release_operations_json_evidence(
        release_candidate_path,
        "release-candidate",
        fail_under,
        Some(6),
    );
    let ci_report = verify_forge_ci_artifacts(ci_artifacts_path)?;
    let public_evidence_report = verify_forge_public_evidence_export(public_evidence_path)?;
    let package_gallery = summarize_forge_release_operations_package_gallery(
        release_manifest_path,
        &signed_manifest.artifact_dir,
    );
    let no_node_modules = verify_release_candidate_no_node_modules(&[
        project,
        signed_manifest.artifact_dir.as_path(),
        ci_artifacts_path,
        public_evidence_path,
        trust_regression_path
            .parent()
            .unwrap_or(trust_regression_path),
        release_candidate_path
            .parent()
            .unwrap_or(release_candidate_path),
    ]);

    let checks = DxForgeReleaseOperationsChecks {
        signed_manifest: release_operations_check(
            signed_manifest.exists
                && signed_manifest.signed
                && signed_manifest.signature_verified
                && signed_manifest.manifest_digest_verified
                && signed_manifest.artifact_integrity_verified,
            release_operations_manifest_score(&signed_manifest),
            format!(
                "publisher status `{}` with {} artifact(s), signature verified={}, artifact integrity={}.",
                signed_manifest.publisher_status,
                signed_manifest.artifact_count,
                signed_manifest.signature_verified,
                signed_manifest.artifact_integrity_verified
            ),
            Some(signed_manifest.path.display().to_string()),
        ),
        trust_regression: release_operations_check(
            trust_regression.passed
                && trust_regression.score >= fail_under
                && trust_regression.case_count.unwrap_or_default() >= 6
                && trust_regression.no_node_modules == Some(true),
            trust_regression.score,
            format!(
                "{} score {}, cases {:?}, no_node_modules {:?}.",
                trust_regression.label,
                trust_regression.score,
                trust_regression.case_count,
                trust_regression.no_node_modules
            ),
            Some(trust_regression.path.display().to_string()),
        ),
        release_candidate: release_operations_check(
            release_candidate.passed
                && release_candidate.score >= fail_under
                && release_candidate.check_count.unwrap_or_default() >= 6
                && release_candidate.no_node_modules == Some(true),
            release_candidate.score,
            format!(
                "{} score {}, checks {:?}.",
                release_candidate.label, release_candidate.score, release_candidate.check_count
            ),
            Some(release_candidate.path.display().to_string()),
        ),
        ci_artifacts: release_operations_check(
            ci_report.passed && ci_report.score >= fail_under,
            ci_report.score,
            format!(
                "{} CI artifact(s) and {} route check(s) verified.",
                ci_report.artifacts.len(),
                ci_report.routes.len()
            ),
            Some(ci_artifacts_path.display().to_string()),
        ),
        public_evidence: release_operations_check(
            public_evidence_report.passed && public_evidence_report.score >= fail_under,
            public_evidence_report.score,
            format!(
                "{} public evidence artifact(s) and {} route/link check(s) verified.",
                public_evidence_report.artifacts.len(),
                public_evidence_report.checks.len()
            ),
            Some(public_evidence_path.display().to_string()),
        ),
        package_gallery: release_operations_check(
            package_gallery.passed && package_gallery.score >= fail_under,
            package_gallery.score,
            format!(
                "hosted package-gallery route `{}` has {} artifact(s), {} package(s), and {} migration guide(s).",
                package_gallery.route,
                package_gallery.artifact_count,
                package_gallery.package_count,
                package_gallery.migration_guide_count
            ),
            Some(package_gallery.html_path.display().to_string()),
        ),
        no_node_modules: release_operations_check(
            no_node_modules.passed,
            no_node_modules.score,
            format!(
                "{} shipping boundary path(s) checked.",
                no_node_modules.checked_paths.len()
            ),
            None,
        ),
    };

    let ci_artifacts = DxForgeReleaseOperationsArtifactEvidence {
        path: ci_report.artifact_dir.clone(),
        passed: ci_report.passed,
        score: ci_report.score,
        artifact_count: ci_report.artifacts.len(),
        check_count: ci_report.routes.len(),
        findings: ci_report.findings.clone(),
    };
    let public_evidence = DxForgeReleaseOperationsArtifactEvidence {
        path: public_evidence_report.evidence_dir.clone(),
        passed: public_evidence_report.passed,
        score: public_evidence_report.score,
        artifact_count: public_evidence_report.artifacts.len(),
        check_count: public_evidence_report.checks.len(),
        findings: public_evidence_report.findings.clone(),
    };

    let score = [
        checks.signed_manifest.score,
        checks.trust_regression.score,
        checks.release_candidate.score,
        checks.ci_artifacts.score,
        checks.public_evidence.score,
        checks.package_gallery.score,
        checks.no_node_modules.score,
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let mut findings = Vec::new();
    append_release_operations_check_finding(
        "signed-manifest",
        &checks.signed_manifest,
        &mut findings,
    );
    append_release_operations_check_finding(
        "trust-regression",
        &checks.trust_regression,
        &mut findings,
    );
    append_release_operations_check_finding(
        "release-candidate",
        &checks.release_candidate,
        &mut findings,
    );
    append_release_operations_check_finding("ci-artifacts", &checks.ci_artifacts, &mut findings);
    append_release_operations_check_finding(
        "public-evidence",
        &checks.public_evidence,
        &mut findings,
    );
    append_release_operations_check_finding(
        "package-gallery",
        &checks.package_gallery,
        &mut findings,
    );
    append_release_operations_check_finding(
        "no-node-modules",
        &checks.no_node_modules,
        &mut findings,
    );
    findings.extend(
        signed_manifest
            .findings
            .iter()
            .map(|finding| format!("signed-manifest: {finding}")),
    );
    findings.extend(
        trust_regression
            .findings
            .iter()
            .map(|finding| format!("trust-regression: {finding}")),
    );
    findings.extend(
        release_candidate
            .findings
            .iter()
            .map(|finding| format!("release-candidate: {finding}")),
    );
    findings.extend(
        ci_artifacts
            .findings
            .iter()
            .map(|finding| format!("ci-artifacts: {finding}")),
    );
    findings.extend(
        public_evidence
            .findings
            .iter()
            .map(|finding| format!("public-evidence: {finding}")),
    );
    findings.extend(
        package_gallery
            .findings
            .iter()
            .map(|finding| format!("package-gallery: {finding}")),
    );
    findings.extend(
        no_node_modules
            .findings
            .iter()
            .map(|finding| format!("no-node-modules: {finding}")),
    );

    let passed = findings.is_empty()
        && score >= fail_under
        && checks.signed_manifest.passed
        && checks.trust_regression.passed
        && checks.release_candidate.passed
        && checks.ci_artifacts.passed
        && checks.public_evidence.passed
        && checks.package_gallery.passed
        && checks.no_node_modules.passed;
    let status = if passed {
        "ready-to-ship"
    } else {
        "needs-review"
    }
    .to_string();
    let shipping_gate = forge_release_operations_signoff_items(
        &signed_manifest,
        &trust_regression,
        &release_candidate,
        &ci_artifacts,
        &public_evidence,
        &package_gallery,
        &checks,
    );

    Ok(DxForgeReleaseOperationsReport {
        version: 1,
        project: project.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        status,
        fail_under,
        inputs: DxForgeReleaseOperationsInputs {
            release_manifest: release_manifest_path.to_path_buf(),
            trust_regression: trust_regression_path.to_path_buf(),
            release_candidate: release_candidate_path.to_path_buf(),
            ci_artifacts: ci_artifacts_path.to_path_buf(),
            public_evidence: public_evidence_path.to_path_buf(),
        },
        checks,
        signed_manifest,
        trust_regression,
        release_candidate,
        ci_artifacts,
        public_evidence,
        package_gallery,
        no_node_modules,
        shipping_gate,
        findings,
        next_commands: vec![
            format!(
                "dx forge release-operations --project . --release-manifest {} --trust-regression {} --release-candidate {} --ci-artifacts {} --public-evidence {} --format markdown --fail-under {}",
                release_manifest_path.display(),
                trust_regression_path.display(),
                release_candidate_path.display(),
                ci_artifacts_path.display(),
                public_evidence_path.display(),
                fail_under
            ),
            "dx forge release-bundle --verify <bundle> --format markdown --fail-under 90"
                .to_string(),
            "dx forge public-evidence --verify <public-dir> --format markdown --fail-under 90"
                .to_string(),
        ],
    })
}

fn summarize_forge_release_operations_manifest(
    manifest_path: &Path,
) -> anyhow::Result<DxForgeReleaseOperationsSignedManifest> {
    let artifact_dir = manifest_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let mut findings = Vec::new();
    let raw = match std::fs::read(manifest_path) {
        Ok(raw) => raw,
        Err(error) => {
            findings.push(format!(
                "release manifest is missing or unreadable: {error}"
            ));
            return Ok(DxForgeReleaseOperationsSignedManifest {
                path: manifest_path.to_path_buf(),
                artifact_dir,
                exists: false,
                signed: false,
                publisher_status: "missing".to_string(),
                publisher_signer: None,
                publisher_key_id: None,
                signature_verified: false,
                manifest_digest_verified: false,
                artifact_integrity_verified: false,
                artifact_count: 0,
                digest: None,
                findings,
            });
        }
    };
    let manifest: DxForgeReleaseBundleManifest = serde_json::from_slice(&raw)?;
    let manifest_digest = forge_release_bundle_manifest_digest(&manifest.artifacts)?;
    let manifest_digest_verified = manifest_digest == manifest.integrity.digest
        && manifest.artifact_integrity.digest == manifest.integrity.digest
        && manifest.artifact_integrity.artifact_count == manifest.artifacts.len();
    if !manifest_digest_verified {
        findings.push("release manifest digest does not match listed artifacts".to_string());
    }

    let artifact_integrity_verified = forge_release_operations_artifact_integrity_verified(
        &artifact_dir,
        &manifest,
        &mut findings,
    );
    let signed = manifest.integrity.signed
        && manifest.publisher_identity.status == FORGE_RELEASE_BUNDLE_SIGNATURE_STATUS_SIGNED;
    let signature_verified = if signed {
        match verify_forge_release_bundle_manifest_signature(&manifest) {
            Ok(()) => true,
            Err(error) => {
                findings.push(format!("publisher signature verification failed: {error}"));
                false
            }
        }
    } else {
        findings.push(
            "release-operations requires a signed publisher identity before shipping".to_string(),
        );
        false
    };

    Ok(DxForgeReleaseOperationsSignedManifest {
        path: manifest_path.to_path_buf(),
        artifact_dir,
        exists: true,
        signed,
        publisher_status: manifest.publisher_identity.status,
        publisher_signer: manifest.publisher_identity.signer,
        publisher_key_id: manifest.publisher_identity.key_id,
        signature_verified,
        manifest_digest_verified,
        artifact_integrity_verified,
        artifact_count: manifest.artifacts.len(),
        digest: Some(manifest.artifact_integrity.digest),
        findings,
    })
}

fn summarize_forge_release_operations_package_gallery(
    manifest_path: &Path,
    artifact_dir: &Path,
) -> DxForgeReleaseOperationsPackageGallery {
    let route = "/forge/package-gallery/".to_string();
    let html_path = artifact_dir.join(FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_HTML);
    let json_path = artifact_dir.join(FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_JSON);
    let markdown_path = artifact_dir.join(FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_MD);
    let migration_html_path = artifact_dir.join(FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_HTML);
    let migration_json_path = artifact_dir.join(FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_JSON);
    let migration_markdown_path = artifact_dir.join(FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_MD);
    let mut findings = Vec::new();
    let mut penalty = 0u16;
    let mut artifact_count = 0usize;
    let mut package_count = 0usize;
    let mut migration_guide_count = 0usize;

    match std::fs::read_to_string(&html_path) {
        Ok(html) => {
            artifact_count += 1;
            for expected in [
                "DX Forge Package Gallery",
                "Trust signals",
                "Migration guides",
                "ui/button",
                "auth/better-auth",
                "migration/static-site",
                "not a universal npm replacement",
                "no node_modules",
            ] {
                if !html.contains(expected) {
                    findings.push(format!(
                        "hosted package-gallery HTML is missing `{expected}`"
                    ));
                    penalty = penalty.saturating_add(10);
                }
            }
        }
        Err(error) => {
            findings.push(format!(
                "hosted package-gallery HTML is missing or unreadable: {error}"
            ));
            penalty = penalty.saturating_add(30);
        }
    }

    match std::fs::read(&json_path) {
        Ok(raw) => {
            artifact_count += 1;
            match serde_json::from_slice::<serde_json::Value>(&raw) {
                Ok(value) => {
                    package_count = value
                        .get("package_count")
                        .and_then(|count| count.as_u64())
                        .or_else(|| {
                            value
                                .get("packages")
                                .and_then(|packages| packages.as_array())
                                .map(|packages| packages.len() as u64)
                        })
                        .unwrap_or_default() as usize;
                    migration_guide_count = value
                        .get("migration_guides")
                        .and_then(|guides| guides.as_array())
                        .map(Vec::len)
                        .unwrap_or_default();
                    let json_route = value
                        .get("route")
                        .and_then(|route| route.as_str())
                        .unwrap_or_default();
                    if json_route != route {
                        findings.push(format!(
                            "hosted package-gallery JSON route is `{json_route}`, expected `{route}`"
                        ));
                        penalty = penalty.saturating_add(15);
                    }
                    if value.get("passed").and_then(|passed| passed.as_bool()) != Some(true) {
                        findings.push("hosted package-gallery JSON did not pass".to_string());
                        penalty = penalty.saturating_add(15);
                    }
                    if value
                        .get("no_node_modules")
                        .and_then(|no_node_modules| no_node_modules.as_bool())
                        != Some(true)
                    {
                        findings.push(
                            "hosted package-gallery JSON does not prove no node_modules"
                                .to_string(),
                        );
                        penalty = penalty.saturating_add(15);
                    }
                    if package_count < FORGE_WWW_TEMPLATE_PACKAGE_IDS.len() {
                        findings.push(format!(
                            "hosted package-gallery lists {package_count} package(s), expected at least {}",
                            FORGE_WWW_TEMPLATE_PACKAGE_IDS.len()
                        ));
                        penalty = penalty.saturating_add(15);
                    }
                    if migration_guide_count == 0 {
                        findings.push(
                            "hosted package-gallery JSON has no migration-guide entries"
                                .to_string(),
                        );
                        penalty = penalty.saturating_add(15);
                    }
                }
                Err(error) => {
                    findings.push(format!("hosted package-gallery JSON is invalid: {error}"));
                    penalty = penalty.saturating_add(30);
                }
            }
        }
        Err(error) => {
            findings.push(format!(
                "hosted package-gallery JSON is missing or unreadable: {error}"
            ));
            penalty = penalty.saturating_add(30);
        }
    }

    match std::fs::read_to_string(&markdown_path) {
        Ok(markdown) => {
            artifact_count += 1;
            for expected in [
                "# DX Forge Hosted Package Gallery",
                "## Migration Guides",
                "dx forge migration-guide --package ui/button",
            ] {
                if !markdown.contains(expected) {
                    findings.push(format!(
                        "hosted package-gallery Markdown is missing `{expected}`"
                    ));
                    penalty = penalty.saturating_add(10);
                }
            }
        }
        Err(error) => {
            findings.push(format!(
                "hosted package-gallery Markdown is missing or unreadable: {error}"
            ));
            penalty = penalty.saturating_add(30);
        }
    }

    match std::fs::read_to_string(&migration_html_path) {
        Ok(html) => {
            artifact_count += 1;
            for expected in [
                "DX Forge Migration Gallery",
                "Supported scope",
                "Manual gaps",
                "Package evidence",
                "Payload comparison boundaries",
                "migration/static-site",
                "no node_modules",
            ] {
                if !html.contains(expected) {
                    findings.push(format!(
                        "hosted migration-gallery HTML is missing `{expected}`"
                    ));
                    penalty = penalty.saturating_add(10);
                }
            }
        }
        Err(error) => {
            findings.push(format!(
                "hosted migration-gallery HTML is missing or unreadable: {error}"
            ));
            penalty = penalty.saturating_add(30);
        }
    }

    match std::fs::read(&migration_json_path) {
        Ok(raw) => {
            artifact_count += 1;
            match serde_json::from_slice::<serde_json::Value>(&raw) {
                Ok(value) => {
                    let json_route = value
                        .get("route")
                        .and_then(|route| route.as_str())
                        .unwrap_or_default();
                    if json_route != "/forge/migration-gallery/" {
                        findings.push(format!(
                            "hosted migration-gallery JSON route is `{json_route}`, expected `/forge/migration-gallery/`"
                        ));
                        penalty = penalty.saturating_add(15);
                    }
                    if value.get("passed").and_then(|passed| passed.as_bool()) != Some(true) {
                        findings.push("hosted migration-gallery JSON did not pass".to_string());
                        penalty = penalty.saturating_add(15);
                    }
                    if value
                        .get("no_node_modules")
                        .and_then(|no_node_modules| no_node_modules.as_bool())
                        != Some(true)
                    {
                        findings.push(
                            "hosted migration-gallery JSON does not prove no node_modules"
                                .to_string(),
                        );
                        penalty = penalty.saturating_add(15);
                    }
                    let evidence_count = value
                        .get("package_evidence")
                        .and_then(|evidence| evidence.as_array())
                        .map(Vec::len)
                        .unwrap_or_default();
                    if evidence_count == 0 {
                        findings.push(
                            "hosted migration-gallery JSON has no package evidence".to_string(),
                        );
                        penalty = penalty.saturating_add(15);
                    }
                }
                Err(error) => {
                    findings.push(format!("hosted migration-gallery JSON is invalid: {error}"));
                    penalty = penalty.saturating_add(30);
                }
            }
        }
        Err(error) => {
            findings.push(format!(
                "hosted migration-gallery JSON is missing or unreadable: {error}"
            ));
            penalty = penalty.saturating_add(30);
        }
    }

    match std::fs::read_to_string(&migration_markdown_path) {
        Ok(markdown) => {
            artifact_count += 1;
            for expected in [
                "# DX Forge Migration Gallery",
                "## Payload Comparison Boundaries",
                "migration-static-site-source-files",
            ] {
                if !markdown.contains(expected) {
                    findings.push(format!(
                        "hosted migration-gallery Markdown is missing `{expected}`"
                    ));
                    penalty = penalty.saturating_add(10);
                }
            }
        }
        Err(error) => {
            findings.push(format!(
                "hosted migration-gallery Markdown is missing or unreadable: {error}"
            ));
            penalty = penalty.saturating_add(30);
        }
    }

    let manifest_lists_gallery = std::fs::read(manifest_path)
        .ok()
        .and_then(|raw| serde_json::from_slice::<DxForgeReleaseBundleManifest>(&raw).ok())
        .is_some_and(|manifest| {
            let package_gallery_ready = [
                FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_HTML,
                FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_JSON,
                FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_MD,
            ]
            .iter()
            .all(|expected| {
                manifest.artifacts.iter().any(|artifact| {
                    artifact.path == *expected
                        && artifact.artifact_type == "package-gallery"
                        && artifact.route.as_deref() == Some("/forge/package-gallery/")
                })
            });
            let migration_gallery_ready = [
                FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_HTML,
                FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_JSON,
                FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_MD,
            ]
            .iter()
            .all(|expected| {
                manifest.artifacts.iter().any(|artifact| {
                    artifact.path == *expected
                        && artifact.artifact_type == "migration-gallery"
                        && artifact.route.as_deref() == Some("/forge/migration-gallery/")
                })
            });
            package_gallery_ready && migration_gallery_ready
        });
    if !manifest_lists_gallery {
        findings.push(
            "signed release manifest does not list hosted package-gallery and migration-gallery artifacts".to_string(),
        );
        penalty = penalty.saturating_add(20);
    }

    let no_node_modules = !artifact_dir.join("node_modules").exists();
    if !no_node_modules {
        findings.push("release bundle contains node_modules".to_string());
        penalty = penalty.saturating_add(30);
    }

    let score = 100u8.saturating_sub(penalty.min(100) as u8);
    let passed = findings.is_empty() && score >= 90;

    DxForgeReleaseOperationsPackageGallery {
        route,
        artifact_dir: artifact_dir.to_path_buf(),
        html_path,
        json_path,
        markdown_path,
        passed,
        score,
        artifact_count,
        package_count,
        migration_guide_count,
        no_node_modules,
        findings,
    }
}

fn forge_release_operations_artifact_integrity_verified(
    artifact_dir: &Path,
    manifest: &DxForgeReleaseBundleManifest,
    findings: &mut Vec<String>,
) -> bool {
    let mut verified = true;
    for artifact in &manifest.artifacts {
        let path = artifact_dir.join(&artifact.path);
        match std::fs::read(&path) {
            Ok(raw) => {
                let actual = blake3::hash(&raw).to_hex().to_string();
                if actual != artifact.blake3 {
                    verified = false;
                    findings.push(format!(
                        "artifact hash mismatch for `{}`: expected {}, got {}",
                        artifact.path, artifact.blake3, actual
                    ));
                }
            }
            Err(error) => {
                verified = false;
                findings.push(format!(
                    "release manifest artifact is missing or unreadable: {} ({error})",
                    artifact.path
                ));
            }
        }
    }
    verified && manifest.artifact_integrity.verified_locally
}

fn release_operations_manifest_score(manifest: &DxForgeReleaseOperationsSignedManifest) -> u8 {
    let mut penalty = 0u16;
    if !manifest.exists {
        penalty = penalty.saturating_add(100);
    }
    if !manifest.signed {
        penalty = penalty.saturating_add(20);
    }
    if !manifest.signature_verified {
        penalty = penalty.saturating_add(40);
    }
    if !manifest.manifest_digest_verified {
        penalty = penalty.saturating_add(30);
    }
    if !manifest.artifact_integrity_verified {
        penalty = penalty.saturating_add(30);
    }
    100u8.saturating_sub(penalty.min(100) as u8)
}

fn summarize_forge_release_operations_json_evidence(
    path: &Path,
    label: &str,
    fail_under: u8,
    minimum_cases_or_checks: Option<u64>,
) -> DxForgeReleaseOperationsJsonEvidence {
    let raw = match std::fs::read(path) {
        Ok(raw) => raw,
        Err(error) => {
            return DxForgeReleaseOperationsJsonEvidence {
                path: path.to_path_buf(),
                exists: false,
                passed: false,
                score: 0,
                label: label.to_string(),
                case_count: None,
                check_count: None,
                no_node_modules: None,
                findings: vec![format!("{label} report is missing or unreadable: {error}")],
            };
        }
    };
    let value: serde_json::Value = match serde_json::from_slice(&raw) {
        Ok(value) => value,
        Err(error) => {
            return DxForgeReleaseOperationsJsonEvidence {
                path: path.to_path_buf(),
                exists: true,
                passed: false,
                score: 0,
                label: label.to_string(),
                case_count: None,
                check_count: None,
                no_node_modules: None,
                findings: vec![format!("{label} report is invalid JSON: {error}")],
            };
        }
    };
    let score = json_u8(value.get("score")).unwrap_or(0);
    let passed_signal = value
        .get("passed")
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    let case_count = json_u64(value.get("case_count"));
    let check_count = value
        .get("checks")
        .and_then(|value| value.as_object())
        .map(|checks| checks.len());
    let no_node_modules = value
        .get("no_node_modules")
        .and_then(|value| value.as_bool())
        .or_else(|| {
            value
                .get("no_node_modules")
                .and_then(|value| value.get("passed"))
                .and_then(|value| value.as_bool())
        });
    let mut findings = release_operations_json_findings(&value);
    if !passed_signal {
        findings.push(format!("{label} report did not pass"));
    }
    if score < fail_under {
        findings.push(format!("{label} score {score} is below {fail_under}"));
    }
    if let Some(minimum) = minimum_cases_or_checks {
        let observed = case_count.or_else(|| check_count.map(|count| count as u64));
        if observed.unwrap_or_default() < minimum {
            findings.push(format!(
                "{label} covers {} case/check item(s); expected at least {minimum}",
                observed.unwrap_or_default()
            ));
        }
    }
    if no_node_modules == Some(false) {
        findings.push(format!("{label} report found node_modules"));
    }

    DxForgeReleaseOperationsJsonEvidence {
        path: path.to_path_buf(),
        exists: true,
        passed: passed_signal && findings.is_empty() && score >= fail_under,
        score,
        label: label.to_string(),
        case_count,
        check_count,
        no_node_modules,
        findings,
    }
}

fn release_operations_json_findings(value: &serde_json::Value) -> Vec<String> {
    value
        .get("findings")
        .and_then(|value| value.as_array())
        .map(|findings| {
            findings
                .iter()
                .filter_map(|finding| finding.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn release_operations_check(
    passed: bool,
    score: u8,
    message: impl Into<String>,
    evidence: Option<String>,
) -> DxForgeReleaseOperationsCheck {
    DxForgeReleaseOperationsCheck {
        passed,
        score,
        message: message.into(),
        evidence,
    }
}

fn append_release_operations_check_finding(
    label: &str,
    check: &DxForgeReleaseOperationsCheck,
    findings: &mut Vec<String>,
) {
    if !check.passed {
        findings.push(format!("{label}: {}", check.message));
    }
}

fn forge_release_operations_signoff_items(
    signed_manifest: &DxForgeReleaseOperationsSignedManifest,
    trust_regression: &DxForgeReleaseOperationsJsonEvidence,
    release_candidate: &DxForgeReleaseOperationsJsonEvidence,
    ci_artifacts: &DxForgeReleaseOperationsArtifactEvidence,
    public_evidence: &DxForgeReleaseOperationsArtifactEvidence,
    package_gallery: &DxForgeReleaseOperationsPackageGallery,
    checks: &DxForgeReleaseOperationsChecks,
) -> Vec<DxForgeReleaseOperationsSignoffItem> {
    vec![
        release_operations_signoff_item(
            "Publish signed release manifest",
            signed_manifest.path.display().to_string(),
            &checks.signed_manifest,
            "Publisher identity, manifest digest, and bundle artifact hashes are verified before publish.",
        ),
        release_operations_signoff_item(
            "Preserve trust-regression evidence",
            trust_regression.path.display().to_string(),
            &checks.trust_regression,
            "Green, yellow, and red package trust decisions remain distinguishable.",
        ),
        release_operations_signoff_item(
            "Preserve release-candidate gate",
            release_candidate.path.display().to_string(),
            &checks.release_candidate,
            "CI, Pages, route comparison, source-owned review, competitor evidence, and secret scans stay joined.",
        ),
        release_operations_signoff_item(
            "Attach CI artifacts",
            ci_artifacts.path.display().to_string(),
            &checks.ci_artifacts,
            "The secret-free CI packet is available for review.",
        ),
        release_operations_signoff_item(
            "Publish public evidence export",
            public_evidence.path.display().to_string(),
            &checks.public_evidence,
            "The public evidence surface links the launch routes, badges, claims, and benchmark artifacts.",
        ),
        release_operations_signoff_item(
            "Publish hosted package gallery",
            package_gallery.html_path.display().to_string(),
            &checks.package_gallery,
            "Adopters can inspect source-owned packages, trust signals, advisories, and migration guides without running the CLI first.",
        ),
        release_operations_signoff_item(
            "Confirm no node_modules boundary",
            "project, manifest, package gallery, CI, public evidence, trust, and RC inputs"
                .to_string(),
            &checks.no_node_modules,
            "Shipping evidence does not smuggle npm install output into the release path.",
        ),
    ]
}

fn release_operations_signoff_item(
    label: &str,
    artifact: String,
    check: &DxForgeReleaseOperationsCheck,
    message: &str,
) -> DxForgeReleaseOperationsSignoffItem {
    DxForgeReleaseOperationsSignoffItem {
        label: label.to_string(),
        artifact,
        status: if check.passed {
            "ready"
        } else {
            "needs-review"
        }
        .to_string(),
        required: true,
        message: message.to_string(),
    }
}

fn build_forge_release_review_report(
    project: &Path,
    bundle_dir: &Path,
    dashboard_path: &Path,
    history_path: &Path,
    route_comparison_path: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeReleaseReviewReport> {
    let dashboard_value =
        read_forge_release_review_json(dashboard_path, "Forge release-dashboard JSON")?;
    let dashboard =
        summarize_forge_release_review_dashboard(dashboard_path, &dashboard_value, fail_under);
    let bundle_report = verify_forge_release_bundle(bundle_dir)?;
    let manifest_path = bundle_dir.join(FORGE_RELEASE_BUNDLE_MANIFEST_JSON);
    let manifest: DxForgeReleaseBundleManifest =
        serde_json::from_slice(&std::fs::read(&manifest_path)?)?;
    let manifest_digest = forge_release_bundle_manifest_digest(&manifest.artifacts)?;
    let manifest_digest_verified = manifest_digest == manifest.artifact_integrity.digest
        && manifest.artifact_integrity.digest == manifest.integrity.digest;
    let bundle = DxForgeReleaseReviewBundle {
        bundle_dir: bundle_dir.to_path_buf(),
        passed: bundle_report.passed,
        score: bundle_report.score,
        route_count: bundle_report.route_count,
        artifact_count: bundle_report.artifact_count,
        no_node_modules: bundle_report.no_node_modules,
        manifest_path: manifest_path.clone(),
        manifest_artifacts: manifest.artifacts.len(),
        manifest_digest: manifest.artifact_integrity.digest.clone(),
        manifest_digest_verified,
    };
    let launch_changelog_report =
        build_forge_launch_changelog_report(DxForgeLaunchChangelogInput {
            history_path: history_path.to_path_buf(),
        })?;
    let launch_changelog = DxForgeReleaseReviewLaunchChangelog {
        path: bundle_dir.join(FORGE_RELEASE_BUNDLE_LAUNCH_CHANGELOG_JSON),
        passed: launch_changelog_report.passed,
        score: launch_changelog_report.score,
        status: launch_changelog_report.status.clone(),
        record_count: launch_changelog_report.record_count,
        honest_scope_count: launch_changelog_report.honest_scope.len(),
        finding_count: launch_changelog_report.findings.len(),
    };
    let route_comparison_value = read_forge_release_review_json(
        route_comparison_path,
        "Forge public route comparison JSON",
    )?;
    let route_comparison = summarize_forge_release_review_route_comparison(
        route_comparison_path,
        &route_comparison_value,
    );
    let history: DxForgePublicReleaseHistory =
        serde_json::from_slice(&std::fs::read(history_path)?)?;
    let release_history = summarize_forge_release_review_history(history_path, &history);

    let release_dashboard_check = release_review_check(
        dashboard.passed && dashboard.score >= fail_under,
        dashboard.score,
        format!(
            "release-dashboard score {} with {} check(s) and {} finding(s).",
            dashboard.score, dashboard.check_count, dashboard.finding_count
        ),
    );
    let release_bundle_check = release_review_check(
        bundle.passed && bundle.score >= fail_under,
        bundle.score,
        format!(
            "release bundle verifies {} route(s), {} artifact(s), and no_node_modules={}.",
            bundle.route_count, bundle.artifact_count, bundle.no_node_modules
        ),
    );
    let bundle_manifest_check = release_review_check(
        manifest_digest_verified && manifest.artifact_count == manifest.artifacts.len(),
        if manifest_digest_verified { 100 } else { 0 },
        format!(
            "release bundle manifest lists {} artifact(s) with artifact digest {} and publisher identity `{}`.",
            manifest.artifacts.len(),
            manifest.artifact_integrity.digest,
            manifest.publisher_identity.status
        ),
    );
    let launch_changelog_check = release_review_check(
        launch_changelog.passed
            && launch_changelog.score >= fail_under
            && launch_changelog.honest_scope_count >= 4,
        launch_changelog.score,
        format!(
            "launch changelog status `{}` from {} release-history record(s).",
            launch_changelog.status, launch_changelog.record_count
        ),
    );
    let route_comparison_check = release_review_check(
        route_comparison.passed && route_comparison.score >= fail_under,
        route_comparison.score,
        format!(
            "{} public route(s), {} Brotli bytes total, {} budget failure(s).",
            route_comparison.route_count,
            route_comparison.total_brotli_bytes,
            route_comparison.failing_budget_routes.len()
        ),
    );
    let release_history_check = release_review_check(
        release_history.passed && release_history.score >= fail_under,
        release_history.score,
        format!(
            "{} release-history record(s), latest route count {:?}, latest regression finding(s): {}.",
            release_history.record_count,
            release_history.latest_route_count,
            release_history.latest_regression_findings
        ),
    );
    let checks = DxForgeReleaseReviewChecks {
        release_dashboard: release_dashboard_check,
        release_bundle: release_bundle_check,
        bundle_manifest: bundle_manifest_check,
        launch_changelog: launch_changelog_check,
        route_comparison: route_comparison_check,
        release_history: release_history_check,
    };

    let score = [
        checks.release_dashboard.score,
        checks.release_bundle.score,
        checks.bundle_manifest.score,
        checks.launch_changelog.score,
        checks.route_comparison.score,
        checks.release_history.score,
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let mut findings = Vec::new();
    append_release_review_check_finding(
        "release-dashboard",
        &checks.release_dashboard,
        &mut findings,
    );
    append_release_review_check_finding("release-bundle", &checks.release_bundle, &mut findings);
    append_release_review_check_finding("bundle-manifest", &checks.bundle_manifest, &mut findings);
    append_release_review_check_finding(
        "launch-changelog",
        &checks.launch_changelog,
        &mut findings,
    );
    append_release_review_check_finding(
        "route-comparison",
        &checks.route_comparison,
        &mut findings,
    );
    append_release_review_check_finding("release-history", &checks.release_history, &mut findings);
    findings.extend(
        bundle_report
            .findings
            .iter()
            .map(|finding| format!("release-bundle: {finding}")),
    );
    findings.extend(
        launch_changelog_report
            .findings
            .iter()
            .map(|finding| format!("launch-changelog: {finding}")),
    );

    let passed = findings.is_empty()
        && score >= fail_under
        && checks.release_dashboard.passed
        && checks.release_bundle.passed
        && checks.bundle_manifest.passed
        && checks.launch_changelog.passed
        && checks.route_comparison.passed
        && checks.release_history.passed;
    let status = if passed {
        "ready-for-human-signoff"
    } else {
        "needs-review"
    }
    .to_string();
    let signoff_items = forge_release_review_signoff_items(
        dashboard_path,
        bundle_dir,
        history_path,
        route_comparison_path,
        &checks,
    );

    Ok(DxForgeReleaseReviewReport {
        version: 1,
        project: project.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        status,
        fail_under,
        inputs: DxForgeReleaseReviewInputs {
            dashboard: dashboard_path.to_path_buf(),
            bundle_dir: bundle_dir.to_path_buf(),
            bundle_manifest: manifest_path,
            launch_changelog: bundle_dir.join(FORGE_RELEASE_BUNDLE_LAUNCH_CHANGELOG_JSON),
            route_comparison: route_comparison_path.to_path_buf(),
            release_history: history_path.to_path_buf(),
        },
        checks,
        release_dashboard: dashboard,
        release_bundle: bundle,
        launch_changelog,
        route_comparison,
        release_history,
        signoff_items,
        findings,
    })
}

fn read_forge_release_review_json(path: &Path, label: &str) -> anyhow::Result<serde_json::Value> {
    if !path.is_file() {
        anyhow::bail!("{label} is missing: {}", path.display());
    }
    Ok(serde_json::from_slice(&std::fs::read(path)?)?)
}

fn summarize_forge_release_review_dashboard(
    path: &Path,
    value: &serde_json::Value,
    fail_under: u8,
) -> DxForgeReleaseReviewDashboard {
    let score = json_u8(value.get("score")).unwrap_or(0);
    let passed = value
        .get("passed")
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
        && score >= fail_under;
    let no_node_modules = value
        .get("release_notes")
        .and_then(|value| value.get("no_node_modules"))
        .and_then(|value| value.as_bool())
        .or_else(|| {
            value
                .get("no_node_modules")
                .and_then(|value| value.as_bool())
        })
        .unwrap_or(false);
    let check_count = value
        .get("checks")
        .and_then(|value| value.as_object())
        .map(|checks| checks.len())
        .unwrap_or(0);
    let finding_count = value
        .get("findings")
        .and_then(|value| value.as_array())
        .map(|findings| findings.len())
        .unwrap_or(0);
    DxForgeReleaseReviewDashboard {
        path: path.to_path_buf(),
        passed,
        score,
        no_node_modules,
        check_count,
        finding_count,
    }
}

fn summarize_forge_release_review_route_comparison(
    path: &Path,
    value: &serde_json::Value,
) -> DxForgeReleaseReviewRouteComparison {
    let route_count = json_u64(value.get("route_count")).unwrap_or(0);
    let total_decoded_bytes = json_u64(value.get("total_decoded_bytes")).unwrap_or(0);
    let total_brotli_bytes = json_u64(value.get("total_brotli_bytes")).unwrap_or(0);
    let routes = value
        .get("routes")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();
    let missing_routes = FORGE_REQUIRED_PUBLIC_ROUTES
        .iter()
        .filter(|required| {
            !routes.iter().any(|route| {
                route.get("route").and_then(|value| value.as_str()) == Some(**required)
            })
        })
        .map(|route| (*route).to_string())
        .collect::<Vec<_>>();
    let failing_budget_routes = routes
        .iter()
        .filter(|route| route.get("budget_passed").and_then(|value| value.as_bool()) == Some(false))
        .filter_map(|route| {
            route
                .get("route")
                .and_then(|value| value.as_str())
                .map(str::to_string)
        })
        .collect::<Vec<_>>();
    let mut penalty = 0u16;
    if route_count < FORGE_REQUIRED_PUBLIC_ROUTES.len() as u64 {
        penalty = penalty.saturating_add(25);
    }
    if !missing_routes.is_empty() {
        penalty = penalty.saturating_add((missing_routes.len() as u16).saturating_mul(20));
    }
    if !failing_budget_routes.is_empty() {
        penalty = penalty.saturating_add((failing_budget_routes.len() as u16).saturating_mul(15));
    }
    let score = 100u8.saturating_sub(penalty.min(100) as u8);
    let passed = route_count >= FORGE_REQUIRED_PUBLIC_ROUTES.len() as u64
        && missing_routes.is_empty()
        && failing_budget_routes.is_empty();
    DxForgeReleaseReviewRouteComparison {
        path: path.to_path_buf(),
        passed,
        score,
        route_count,
        total_decoded_bytes,
        total_brotli_bytes,
        missing_routes,
        failing_budget_routes,
    }
}

fn summarize_forge_release_review_history(
    path: &Path,
    history: &DxForgePublicReleaseHistory,
) -> DxForgeReleaseReviewHistory {
    let latest = history.records.first();
    let latest_regression_findings = latest
        .map(|record| record.regression_findings.len())
        .unwrap_or_default();
    let mut penalty = 0u16;
    if history.records.is_empty() {
        penalty = penalty.saturating_add(100);
    }
    if latest.is_some_and(|record| !record.dashboard.passed) {
        penalty = penalty.saturating_add(25);
    }
    if latest_regression_findings > 0 {
        penalty = penalty.saturating_add((latest_regression_findings as u16).saturating_mul(10));
    }
    let score = 100u8.saturating_sub(penalty.min(100) as u8);
    DxForgeReleaseReviewHistory {
        path: path.to_path_buf(),
        passed: !history.records.is_empty() && latest_regression_findings == 0,
        score,
        record_count: history.records.len(),
        latest_dashboard_score: latest.map(|record| record.dashboard.score),
        latest_route_count: latest.map(|record| record.route_comparison.route_count),
        latest_total_brotli_bytes: latest.map(|record| record.route_comparison.total_brotli_bytes),
        latest_regression_findings,
    }
}

fn release_review_check(
    passed: bool,
    score: u8,
    message: impl Into<String>,
) -> DxForgeReleaseReviewCheck {
    DxForgeReleaseReviewCheck {
        passed,
        score,
        message: message.into(),
    }
}

fn append_release_review_check_finding(
    label: &str,
    check: &DxForgeReleaseReviewCheck,
    findings: &mut Vec<String>,
) {
    if !check.passed {
        findings.push(format!("{label}: {}", check.message));
    }
}

fn forge_release_review_signoff_items(
    dashboard_path: &Path,
    bundle_dir: &Path,
    history_path: &Path,
    route_comparison_path: &Path,
    checks: &DxForgeReleaseReviewChecks,
) -> Vec<DxForgeReleaseReviewSignoffItem> {
    vec![
        forge_release_review_signoff_item(
            "Review release-dashboard gate",
            dashboard_path.display().to_string(),
            &checks.release_dashboard,
            "Operator confirms the release-dashboard gate is still the intended threshold.",
        ),
        forge_release_review_signoff_item(
            "Review release bundle manifest digest",
            bundle_dir
                .join(FORGE_RELEASE_BUNDLE_MANIFEST_JSON)
                .display()
                .to_string(),
            &checks.bundle_manifest,
            "Operator confirms the unsigned BLAKE3 manifest matches the publish bundle.",
        ),
        forge_release_review_signoff_item(
            "Review public route comparison",
            route_comparison_path.display().to_string(),
            &checks.route_comparison,
            "Operator confirms route payloads and budget signals are acceptable.",
        ),
        forge_release_review_signoff_item(
            "Review public launch changelog",
            bundle_dir
                .join(FORGE_RELEASE_BUNDLE_LAUNCH_CHANGELOG_MD)
                .display()
                .to_string(),
            &checks.launch_changelog,
            "Operator confirms changelog language is honest and ready for launch notes.",
        ),
        forge_release_review_signoff_item(
            "Review release history",
            history_path.display().to_string(),
            &checks.release_history,
            "Operator confirms release history has no unexplained regressions.",
        ),
        forge_release_review_signoff_item(
            "Review release bundle boundary",
            bundle_dir.display().to_string(),
            &checks.release_bundle,
            "Operator confirms the publish bundle contains no node_modules or secret markers.",
        ),
    ]
}

fn forge_release_review_signoff_item(
    label: &str,
    artifact: String,
    check: &DxForgeReleaseReviewCheck,
    message: &str,
) -> DxForgeReleaseReviewSignoffItem {
    DxForgeReleaseReviewSignoffItem {
        label: label.to_string(),
        artifact,
        status: if check.passed {
            "ready"
        } else {
            "needs-review"
        }
        .to_string(),
        required: true,
        message: message.to_string(),
    }
}

fn json_u8(value: Option<&serde_json::Value>) -> Option<u8> {
    value
        .and_then(|value| value.as_u64())
        .and_then(|value| u8::try_from(value).ok())
}

fn json_u64(value: Option<&serde_json::Value>) -> Option<u64> {
    value.and_then(|value| value.as_u64())
}

fn forge_publisher_key_generate_terminal(report: &DxForgePublisherKeyGenerateReport) -> String {
    let mut output = format!(
        "DX Forge publisher key\nSigner: {}\nKey id: {}\nPublic key: {}\nPrivate key file: {}\nPublic key file: {}\nPassed: {}\n",
        report.signer,
        report.key_id,
        report.public_key,
        report.private_key_path.display(),
        report.public_key_path.display(),
        report.passed
    );
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

fn forge_publisher_key_generate_markdown(report: &DxForgePublisherKeyGenerateReport) -> String {
    let mut output = format!(
        "# DX Forge Publisher Key\n\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Signer: `{}`\n- Algorithm: `{}`\n- Key id: `{}`\n- Public key: `{}`\n- Private key file: `{}`\n- Public key file: `{}`\n\n",
        report.generated_at,
        report.passed,
        report.score,
        report.signer,
        report.algorithm,
        report.key_id,
        report.public_key,
        report.private_key_path.display(),
        report.public_key_path.display()
    );
    output.push_str("## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No publisher-key findings. The private key value is intentionally not included in this report.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }
    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{command}`\n"));
    }
    output
}

fn forge_publisher_key_generate_failure_summary(
    report: &DxForgePublisherKeyGenerateReport,
) -> String {
    if report.findings.is_empty() {
        return "DX Forge publisher-key generate failed".to_string();
    }
    format!(
        "DX Forge publisher-key generate failed: {}",
        report.findings.join("; ")
    )
}

fn forge_publisher_key_sign_terminal(report: &DxForgePublisherKeySignReport) -> String {
    let mut output = format!(
        "DX Forge manifest signing\nSigner: {}\nKey id: {}\nManifest: {}\nOutput manifest: {}\nSignature verified: {}\nWrote manifest: {}\nPassed: {}\n",
        report.signer,
        report.key_id,
        report.manifest_path.display(),
        report.output_manifest_path.display(),
        report.signature_verified,
        report.wrote_manifest,
        report.passed
    );
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

fn forge_publisher_key_sign_markdown(report: &DxForgePublisherKeySignReport) -> String {
    let mut output = format!(
        "# DX Forge Manifest Signing\n\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Signer: `{}`\n- Key id: `{}`\n- Public key: `{}`\n- Key file: `{}`\n- Source manifest: `{}`\n- Output manifest: `{}`\n- Markdown manifest: `{}`\n- Signed at: `{}`\n- Signature verified: `{}`\n- Wrote manifest: `{}`\n- Artifacts covered: `{}`\n- Manifest digest: `{}`\n\n",
        report.generated_at,
        report.passed,
        report.score,
        report.signer,
        report.key_id,
        report.public_key,
        report.key_path.display(),
        report.manifest_path.display(),
        report.output_manifest_path.display(),
        report
            .markdown_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "not written".to_string()),
        report.signed_at,
        report.signature_verified,
        report.wrote_manifest,
        report.artifact_count,
        report.manifest_digest
    );
    output.push_str("## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No signing findings. The private key value is intentionally not included in this report.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }
    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{command}`\n"));
    }
    output
}

fn forge_publisher_key_sign_failure_summary(report: &DxForgePublisherKeySignReport) -> String {
    if report.findings.is_empty() {
        return "DX Forge publisher-key sign failed".to_string();
    }
    format!(
        "DX Forge publisher-key sign failed: {}",
        report.findings.join("; ")
    )
}

fn forge_release_operations_terminal(report: &DxForgeReleaseOperationsReport) -> String {
    let mut output = format!(
        "DX Forge release operations\nProject: {}\nStatus: {} ({} / 100)\nPassed: {}\nSigned manifest: {} / 100\nTrust regression: {} / 100\nRelease candidate: {} / 100\nCI artifacts: {} / 100\nPublic evidence: {} / 100\nPackage gallery: {} / 100\nNo node_modules: {} / 100\n",
        report.project.display(),
        report.status,
        report.score,
        report.passed,
        report.checks.signed_manifest.score,
        report.checks.trust_regression.score,
        report.checks.release_candidate.score,
        report.checks.ci_artifacts.score,
        report.checks.public_evidence.score,
        report.checks.package_gallery.score,
        report.checks.no_node_modules.score
    );
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

fn forge_release_operations_markdown(report: &DxForgeReleaseOperationsReport) -> String {
    let mut output = format!(
        "# DX Forge Release Operations\n\n- Project: `{}`\n- Generated: `{}`\n- Status: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n\n",
        report.project.display(),
        report.generated_at,
        report.status,
        report.passed,
        report.score,
        report.fail_under
    );

    output.push_str("## Shipping Gate\n\n");
    output.push_str("| Gate | Passed | Score | Evidence |\n");
    output.push_str("| --- | --- | ---: | --- |\n");
    for (label, check) in [
        ("signed manifest", &report.checks.signed_manifest),
        ("trust regression", &report.checks.trust_regression),
        ("release candidate", &report.checks.release_candidate),
        ("CI artifacts", &report.checks.ci_artifacts),
        ("public evidence", &report.checks.public_evidence),
        ("package gallery", &report.checks.package_gallery),
        ("no node_modules", &report.checks.no_node_modules),
    ] {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} |\n",
            label,
            check.passed,
            check.score,
            markdown_table_cell(&check.message)
        ));
    }

    output.push_str("\n## Signed Manifest\n\n");
    output.push_str(&format!(
        "- Manifest: `{}`\n- Publisher status: `{}`\n- Signer: `{}`\n- Key id: `{}`\n- Signature verified: `{}`\n- Manifest digest verified: `{}`\n- Artifact integrity verified: `{}`\n- Artifacts: `{}`\n- Digest: `{}`\n\n",
        report.signed_manifest.path.display(),
        report.signed_manifest.publisher_status,
        report
            .signed_manifest
            .publisher_signer
            .as_deref()
            .unwrap_or("not attached"),
        report
            .signed_manifest
            .publisher_key_id
            .as_deref()
            .unwrap_or("not attached"),
        report.signed_manifest.signature_verified,
        report.signed_manifest.manifest_digest_verified,
        report.signed_manifest.artifact_integrity_verified,
        report.signed_manifest.artifact_count,
        report
            .signed_manifest
            .digest
            .as_deref()
            .unwrap_or("missing")
    ));

    output.push_str("## Evidence Inputs\n\n");
    output.push_str(&format!(
        "- Trust regression: `{}` (`{}` / `100`, cases: `{}`)\n- Release candidate: `{}` (`{}` / `100`, checks: `{}`)\n- CI artifacts: `{}` (`{}` files, `{}` route checks)\n- Public evidence: `{}` (`{}` files, `{}` route/link checks)\n- Package gallery: `{}` (`{}` files, `{}` packages, `{}` migration guides)\n- Dependency boundary paths checked: `{}`\n\n",
        report.trust_regression.path.display(),
        report.trust_regression.score,
        report.trust_regression.case_count.unwrap_or_default(),
        report.release_candidate.path.display(),
        report.release_candidate.score,
        report.release_candidate.check_count.unwrap_or_default(),
        report.ci_artifacts.path.display(),
        report.ci_artifacts.artifact_count,
        report.ci_artifacts.check_count,
        report.public_evidence.path.display(),
        report.public_evidence.artifact_count,
        report.public_evidence.check_count,
        report.package_gallery.html_path.display(),
        report.package_gallery.artifact_count,
        report.package_gallery.package_count,
        report.package_gallery.migration_guide_count,
        report.no_node_modules.checked_paths.len()
    ));

    output.push_str("## Required Signoff\n\n");
    output.push_str("| Item | Status | Artifact | Why |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for item in &report.shipping_gate {
        output.push_str(&format!(
            "| {} | `{}` | `{}` | {} |\n",
            markdown_table_cell(&item.label),
            item.status,
            markdown_table_cell(&item.artifact),
            markdown_table_cell(&item.message)
        ));
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No release-operations findings for the configured threshold.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{command}`\n"));
    }

    output
}

fn forge_release_operations_failure_summary(report: &DxForgeReleaseOperationsReport) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge release-operations did not pass: score {} / 100, required {} / 100",
            report.score, report.fail_under
        );
    }

    format!(
        "DX Forge release-operations did not pass: {}",
        report.findings.join("; ")
    )
}

fn build_forge_publish_plan_report(
    project: &Path,
    release_bundle: &Path,
    pages: &Path,
    registry_smoke_path: &Path,
    release_operations_path: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgePublishPlanReport> {
    let release_bundle_report = verify_forge_release_bundle(release_bundle)?;
    let pages_report = verify_forge_pages_bundle(pages)?;
    let registry_smoke = read_forge_publish_plan_json(registry_smoke_path, "registry smoke")?;
    let release_operations =
        read_forge_publish_plan_json(release_operations_path, "release operations")?;
    let manifest_path = release_bundle.join(FORGE_RELEASE_BUNDLE_MANIFEST_JSON);
    let manifest: DxForgeReleaseBundleManifest =
        serde_json::from_slice(&std::fs::read(&manifest_path)?)?;

    let registry_local_path = forge_publish_plan_json_path(&registry_smoke, &["local_registry"]);
    let mut artifact_targets =
        forge_publish_plan_artifact_targets(release_bundle, &manifest, &registry_smoke);
    artifact_targets.extend(forge_publish_plan_local_targets(
        release_bundle,
        registry_smoke_path,
        release_operations_path,
        &manifest_path,
    ));
    let cache_headers = forge_publish_plan_cache_headers();
    let rollback_inputs = forge_publish_plan_rollback_inputs(
        release_bundle,
        registry_smoke_path,
        release_operations_path,
        &release_operations,
        registry_local_path.as_deref(),
    );

    let mut dependency_paths = vec![
        project.to_path_buf(),
        release_bundle.to_path_buf(),
        pages.to_path_buf(),
        registry_smoke_path
            .parent()
            .unwrap_or(registry_smoke_path)
            .to_path_buf(),
        release_operations_path
            .parent()
            .unwrap_or(release_operations_path)
            .to_path_buf(),
    ];
    if let Some(path) = &registry_local_path {
        dependency_paths.push(path.clone());
    }
    let dependency_path_refs = dependency_paths
        .iter()
        .map(PathBuf::as_path)
        .collect::<Vec<_>>();
    let no_node_modules = verify_release_candidate_no_node_modules(&dependency_path_refs);
    let secret_requirements = forge_publish_plan_secret_requirements(
        &registry_smoke,
        &[
            release_bundle.to_path_buf(),
            pages.to_path_buf(),
            registry_smoke_path.to_path_buf(),
            release_operations_path.to_path_buf(),
        ],
    );

    let release_operations_passed = forge_publish_plan_json_bool(&release_operations, &["passed"])
        && forge_publish_plan_json_string(&release_operations, &["status"])
            == Some("ready-to-ship")
        && forge_publish_plan_json_bool(
            &release_operations,
            &["checks", "package_gallery", "passed"],
        );
    let registry_smoke_passed = forge_publish_plan_json_bool(&registry_smoke, &["passed"])
        && !forge_publish_plan_json_bool(&registry_smoke, &["requires_secrets"]);
    let pages_target_count = artifact_targets
        .iter()
        .filter(|target| target.channel == "pages")
        .count();
    let r2_target_count = artifact_targets
        .iter()
        .filter(|target| target.channel == "r2")
        .count();
    let has_package_gallery_target = artifact_targets.iter().any(|target| {
        target.channel == "pages" && target.route.as_deref() == Some("/forge/package-gallery/")
    });
    let public_targets_have_cache = artifact_targets.iter().all(|target| {
        target.channel == "local" || (!target.cache_control.trim().is_empty() && target.passed)
    });
    let rollback_inputs_passed = rollback_inputs
        .iter()
        .filter(|input| input.required)
        .all(|input| input.passed);

    let checks = DxForgePublishPlanChecks {
        local_artifacts: forge_publish_plan_check(
            release_bundle_report.passed && release_operations_passed,
            release_bundle_report
                .score
                .min(if release_operations_passed { 100 } else { 0 }),
            format!(
                "release bundle has {} artifact(s), route_count={}, operations_ready={}.",
                release_bundle_report.artifact_count,
                release_bundle_report.route_count,
                release_operations_passed
            ),
            Some(release_bundle.display().to_string()),
        ),
        pages_artifacts: forge_publish_plan_check(
            pages_report.passed && pages_target_count >= 6 && has_package_gallery_target,
            pages_report.score,
            format!(
                "Pages preview verifies {} artifact(s), {} check(s), and publish plan maps {} Pages target(s).",
                pages_report.artifacts.len(),
                pages_report.checks.len(),
                pages_target_count
            ),
            Some(pages.display().to_string()),
        ),
        r2_artifacts: forge_publish_plan_check(
            registry_smoke_passed && r2_target_count > 0,
            if registry_smoke_passed && r2_target_count > 0 {
                json_u8(registry_smoke.get("score")).unwrap_or(100)
            } else {
                0
            },
            format!(
                "registry smoke ready={}, R2 object target(s)={}.",
                registry_smoke_passed, r2_target_count
            ),
            Some(registry_smoke_path.display().to_string()),
        ),
        cache_headers: forge_publish_plan_check(
            public_targets_have_cache && cache_headers.iter().all(|header| header.passed),
            if public_targets_have_cache { 100 } else { 0 },
            format!(
                "{} cache policy row(s) cover Pages HTML, JSON/Markdown, DXPK packets, and R2 objects.",
                cache_headers.len()
            ),
            None,
        ),
        rollback_inputs: forge_publish_plan_check(
            rollback_inputs_passed,
            if rollback_inputs_passed { 100 } else { 0 },
            format!(
                "{} required rollback input(s) checked.",
                rollback_inputs
                    .iter()
                    .filter(|input| input.required)
                    .count()
            ),
            Some(release_operations_path.display().to_string()),
        ),
        no_secret_requirements: forge_publish_plan_check(
            secret_requirements.passed,
            secret_requirements.score,
            format!(
                "requires_secrets={}, dry_run={}, scanned {} path(s).",
                secret_requirements.requires_secrets,
                secret_requirements.registry_operations_dry_run,
                secret_requirements.scanned_paths.len()
            ),
            Some(registry_smoke_path.display().to_string()),
        ),
        no_node_modules: forge_publish_plan_check(
            no_node_modules.passed,
            no_node_modules.score,
            format!(
                "{} dependency boundary path(s) checked.",
                no_node_modules.checked_paths.len()
            ),
            None,
        ),
    };

    let score = [
        checks.local_artifacts.score,
        checks.pages_artifacts.score,
        checks.r2_artifacts.score,
        checks.cache_headers.score,
        checks.rollback_inputs.score,
        checks.no_secret_requirements.score,
        checks.no_node_modules.score,
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let mut findings = Vec::new();
    append_publish_plan_check_finding("local-artifacts", &checks.local_artifacts, &mut findings);
    append_publish_plan_check_finding("pages-artifacts", &checks.pages_artifacts, &mut findings);
    append_publish_plan_check_finding("r2-artifacts", &checks.r2_artifacts, &mut findings);
    append_publish_plan_check_finding("cache-headers", &checks.cache_headers, &mut findings);
    append_publish_plan_check_finding("rollback-inputs", &checks.rollback_inputs, &mut findings);
    append_publish_plan_check_finding(
        "no-secret-requirements",
        &checks.no_secret_requirements,
        &mut findings,
    );
    append_publish_plan_check_finding("no-node-modules", &checks.no_node_modules, &mut findings);
    findings.extend(
        secret_requirements
            .findings
            .iter()
            .map(|finding| format!("secret-requirements: {finding}")),
    );
    findings.extend(
        no_node_modules
            .findings
            .iter()
            .map(|finding| format!("no-node-modules: {finding}")),
    );
    findings.extend(
        rollback_inputs
            .iter()
            .filter(|input| input.required && !input.passed)
            .map(|input| format!("rollback-inputs: {}", input.message)),
    );

    let passed = findings.is_empty()
        && score >= fail_under
        && checks.local_artifacts.passed
        && checks.pages_artifacts.passed
        && checks.r2_artifacts.passed
        && checks.cache_headers.passed
        && checks.rollback_inputs.passed
        && checks.no_secret_requirements.passed
        && checks.no_node_modules.passed;
    let status = if passed {
        "ready-to-publish-plan"
    } else {
        "needs-review"
    }
    .to_string();

    Ok(DxForgePublishPlanReport {
        version: 1,
        project: project.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        status,
        fail_under,
        inputs: DxForgePublishPlanInputs {
            release_bundle: release_bundle.to_path_buf(),
            pages: pages.to_path_buf(),
            registry_smoke: registry_smoke_path.to_path_buf(),
            release_operations: release_operations_path.to_path_buf(),
        },
        checks,
        artifact_targets,
        cache_headers,
        rollback_inputs,
        secret_requirements,
        no_node_modules,
        findings,
        next_commands: vec![
            "dx forge publish-plan --project . --release-bundle .dx/forge-release-bundle-adoption --pages .dx/forge-pages --registry-smoke .dx/ci/forge-registry-smoke.json --release-operations .dx/ci/forge-release-operations.json --format markdown".to_string(),
            "dx forge release-operations --project . --release-manifest .dx/forge-release-bundle-adoption/forge-release-.dx/build-cache/manifest.json --format markdown".to_string(),
            "dx forge registry smoke --remote r2 --local .dx/forge-registry-smoke --format markdown".to_string(),
        ],
    })
}

fn forge_publish_plan_check(
    passed: bool,
    score: u8,
    message: impl Into<String>,
    evidence: Option<String>,
) -> DxForgePublishPlanCheck {
    DxForgePublishPlanCheck {
        passed,
        score,
        message: message.into(),
        evidence,
    }
}

fn append_publish_plan_check_finding(
    label: &str,
    check: &DxForgePublishPlanCheck,
    findings: &mut Vec<String>,
) {
    if !check.passed {
        findings.push(format!("{label}: {}", check.message));
    }
}

fn read_forge_publish_plan_json(path: &Path, label: &str) -> anyhow::Result<serde_json::Value> {
    let raw = std::fs::read(path).map_err(|error| {
        anyhow::anyhow!("{label} JSON is missing: {} ({error})", path.display())
    })?;
    serde_json::from_slice(&raw)
        .map_err(|error| anyhow::anyhow!("{label} JSON is invalid: {} ({error})", path.display()))
}

fn forge_publish_plan_artifact_targets(
    release_bundle: &Path,
    manifest: &DxForgeReleaseBundleManifest,
    registry_smoke: &serde_json::Value,
) -> Vec<DxForgePublishPlanArtifactTarget> {
    let mut targets = Vec::new();
    for artifact in &manifest.artifacts {
        let route = artifact.route.clone();
        if route.is_some()
            || artifact.artifact_type == "package-gallery"
            || artifact.path == "forge-readiness-badge.json"
        {
            let source = release_bundle.join(&artifact.path);
            let cache_control = forge_publish_plan_cache_control("pages", &artifact.path);
            targets.push(DxForgePublishPlanArtifactTarget {
                channel: "pages".to_string(),
                artifact: artifact.artifact_type.clone(),
                source: source.display().to_string(),
                destination: format!("pages://{}", artifact.path.replace('\\', "/")),
                route,
                cache_control,
                required: true,
                passed: source.is_file(),
                message: "Publish from the verified release bundle to the Pages public surface."
                    .to_string(),
            });
        }
    }

    let mut r2_objects = BTreeMap::<String, String>::new();
    if let Some(operations) = registry_smoke
        .get("operations")
        .and_then(|operations| operations.as_array())
    {
        for operation in operations {
            let action = operation
                .get("action")
                .and_then(|action| action.as_str())
                .unwrap_or("registry-operation");
            if let Some(objects) = operation
                .get("objects")
                .and_then(|objects| objects.as_array())
            {
                for object in objects {
                    if let Some(object) = object.as_str() {
                        r2_objects
                            .entry(object.to_string())
                            .or_insert_with(|| action.to_string());
                    }
                }
            }
        }
    }
    for (object, action) in r2_objects {
        targets.push(DxForgePublishPlanArtifactTarget {
            channel: "r2".to_string(),
            artifact: action,
            source: "registry-smoke".to_string(),
            destination: object.clone(),
            route: None,
            cache_control: forge_publish_plan_cache_control("r2", &object),
            required: true,
            passed: !object.trim().is_empty(),
            message: "Publish package registry manifests and content-addressed files through the reviewed R2 boundary.".to_string(),
        });
    }

    targets
}

fn forge_publish_plan_local_targets(
    release_bundle: &Path,
    registry_smoke_path: &Path,
    release_operations_path: &Path,
    manifest_path: &Path,
) -> Vec<DxForgePublishPlanArtifactTarget> {
    [
        ("release_bundle", release_bundle.to_path_buf()),
        ("signed_release_manifest", manifest_path.to_path_buf()),
        ("registry_smoke", registry_smoke_path.to_path_buf()),
        ("release_operations", release_operations_path.to_path_buf()),
    ]
    .into_iter()
    .map(|(artifact, path)| DxForgePublishPlanArtifactTarget {
        channel: "local".to_string(),
        artifact: artifact.to_string(),
        source: path.display().to_string(),
        destination: path.display().to_string(),
        route: None,
        cache_control: "local review artifact".to_string(),
        required: true,
        passed: path.exists(),
        message: "Keep this local artifact available for rollback, audit, and operator review."
            .to_string(),
    })
    .collect()
}

fn forge_publish_plan_cache_headers() -> Vec<DxForgePublishPlanCacheHeader> {
    vec![
        DxForgePublishPlanCacheHeader {
            channel: "pages".to_string(),
            pattern: "**/*.html".to_string(),
            cache_control: "public, max-age=0, must-revalidate".to_string(),
            required: true,
            passed: true,
            reason: "HTML routes should be quickly replaceable during beta rollback.".to_string(),
        },
        DxForgePublishPlanCacheHeader {
            channel: "pages".to_string(),
            pattern: "**/*.{json,md}".to_string(),
            cache_control: "public, max-age=300, must-revalidate".to_string(),
            required: true,
            passed: true,
            reason: "Evidence and manifest files can be cached briefly but must refresh for operator review.".to_string(),
        },
        DxForgePublishPlanCacheHeader {
            channel: "pages".to_string(),
            pattern: "**/*.dxp".to_string(),
            cache_control: "public, max-age=31536000, immutable".to_string(),
            required: true,
            passed: true,
            reason: "DXPK packet artifacts are hash-verified by the release manifest.".to_string(),
        },
        DxForgePublishPlanCacheHeader {
            channel: "r2".to_string(),
            pattern: "packages/**/files/*".to_string(),
            cache_control: "public, max-age=31536000, immutable".to_string(),
            required: true,
            passed: true,
            reason: "R2 package file objects are content-addressed by registry hashes.".to_string(),
        },
    ]
}

fn forge_publish_plan_cache_control(channel: &str, artifact: &str) -> String {
    match channel {
        "r2" if artifact.contains("/files/") => "public, max-age=31536000, immutable".to_string(),
        "r2" => "public, max-age=300, must-revalidate".to_string(),
        "pages" if artifact.ends_with(".html") => "public, max-age=0, must-revalidate".to_string(),
        "pages" if artifact.ends_with(".dxp") => "public, max-age=31536000, immutable".to_string(),
        "pages" => "public, max-age=300, must-revalidate".to_string(),
        _ => "local review artifact".to_string(),
    }
}

fn forge_publish_plan_rollback_inputs(
    release_bundle: &Path,
    registry_smoke_path: &Path,
    release_operations_path: &Path,
    release_operations: &serde_json::Value,
    registry_local_path: Option<&Path>,
) -> Vec<DxForgePublishPlanRollbackInput> {
    let mut inputs = vec![forge_publish_plan_rollback_input(
        "release_bundle",
        release_bundle.to_path_buf(),
        true,
        "Verified release bundle folder can be promoted or rolled back as one unit.",
    )];
    inputs.push(forge_publish_plan_rollback_input(
        "signed_release_manifest",
        release_bundle.join(FORGE_RELEASE_BUNDLE_MANIFEST_JSON),
        true,
        "Signed manifest pins the publish artifact set and publisher identity.",
    ));
    inputs.push(forge_publish_plan_rollback_input(
        "release_operations",
        release_operations_path.to_path_buf(),
        true,
        "Release operations report records the joined shipping gate.",
    ));
    inputs.push(forge_publish_plan_rollback_input(
        "registry_smoke",
        registry_smoke_path.to_path_buf(),
        true,
        "Registry smoke records the local/R2 object boundary for publish rehearsal.",
    ));
    if let Some(path) =
        forge_publish_plan_json_path(release_operations, &["inputs", "trust_regression"])
    {
        inputs.push(forge_publish_plan_rollback_input(
            "trust_regression",
            path,
            true,
            "Trust-regression evidence is needed before rolling forward after a failed publish.",
        ));
    }
    if let Some(path) =
        forge_publish_plan_json_path(release_operations, &["inputs", "release_candidate"])
    {
        inputs.push(forge_publish_plan_rollback_input(
            "release_candidate",
            path,
            true,
            "Release-candidate evidence explains the exact CI and public-surface state.",
        ));
    }
    if let Some(path) = registry_local_path {
        inputs.push(forge_publish_plan_rollback_input(
            "local_registry",
            path.to_path_buf(),
            true,
            "Local registry manifest mirrors the R2 package object plan.",
        ));
    }
    inputs
}

fn forge_publish_plan_rollback_input(
    name: &str,
    path: PathBuf,
    required: bool,
    message: &str,
) -> DxForgePublishPlanRollbackInput {
    let exists = path.exists();
    DxForgePublishPlanRollbackInput {
        name: name.to_string(),
        path,
        required,
        exists,
        passed: !required || exists,
        message: if exists {
            message.to_string()
        } else {
            format!("{message} Missing input.")
        },
    }
}

fn forge_publish_plan_secret_requirements(
    registry_smoke: &serde_json::Value,
    scan_paths: &[PathBuf],
) -> DxForgePublishPlanSecretRequirements {
    let registry_smoke_requires_secrets =
        forge_publish_plan_json_bool(registry_smoke, &["requires_secrets"]);
    let registry_operations_dry_run = registry_smoke
        .get("operations")
        .and_then(|operations| operations.as_array())
        .is_some_and(|operations| {
            let r2_operations = operations
                .iter()
                .filter(|operation| {
                    let action = operation
                        .get("action")
                        .and_then(|action| action.as_str())
                        .unwrap_or_default();
                    let remote = operation
                        .get("remote")
                        .and_then(|remote| remote.as_str())
                        .unwrap_or_default();
                    remote == "r2" || matches!(action, "registry-publish" | "registry-pull")
                })
                .collect::<Vec<_>>();
            !r2_operations.is_empty()
                && r2_operations.iter().all(|operation| {
                    operation
                        .get("dry_run")
                        .and_then(|dry_run| dry_run.as_bool())
                        == Some(true)
                })
        });
    let scan_refs = scan_paths.iter().map(PathBuf::as_path).collect::<Vec<_>>();
    let secret_markers = verify_release_candidate_secret_markers(&scan_refs);
    let requires_secrets = registry_smoke_requires_secrets || !registry_operations_dry_run;
    let mut findings = Vec::new();
    if registry_smoke_requires_secrets {
        findings.push("registry smoke requires secrets".to_string());
    }
    if !registry_operations_dry_run {
        findings.push("registry operations are not all dry-run".to_string());
    }
    findings.extend(secret_markers.findings.clone());
    let score = [if requires_secrets { 0 } else { 100 }, secret_markers.score]
        .into_iter()
        .min()
        .unwrap_or(0);
    let passed = findings.is_empty() && !requires_secrets && secret_markers.passed;

    DxForgePublishPlanSecretRequirements {
        requires_secrets,
        registry_smoke_requires_secrets,
        registry_operations_dry_run,
        blocked_markers: FORGE_PUBLIC_SECRET_MARKERS
            .iter()
            .map(|marker| (*marker).to_string())
            .collect(),
        scanned_paths: secret_markers.scanned_paths,
        passed,
        score,
        findings,
    }
}

fn forge_publish_plan_json_bool(value: &serde_json::Value, path: &[&str]) -> bool {
    forge_publish_plan_json_value(value, path).and_then(|value| value.as_bool()) == Some(true)
}

fn forge_publish_plan_json_string<'a>(
    value: &'a serde_json::Value,
    path: &[&str],
) -> Option<&'a str> {
    forge_publish_plan_json_value(value, path).and_then(|value| value.as_str())
}

fn forge_publish_plan_json_path(value: &serde_json::Value, path: &[&str]) -> Option<PathBuf> {
    forge_publish_plan_json_string(value, path)
        .filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
}

fn forge_publish_plan_json_value<'a>(
    value: &'a serde_json::Value,
    path: &[&str],
) -> Option<&'a serde_json::Value> {
    let mut current = value;
    for segment in path {
        current = current.get(*segment)?;
    }
    Some(current)
}

fn forge_publish_plan_terminal(report: &DxForgePublishPlanReport) -> String {
    let mut output = format!(
        "DX Forge publish plan\nProject: {}\nStatus: {} ({} / 100)\nPassed: {}\nPages targets: {}\nR2 targets: {}\nRollback inputs: {}\nRequires secrets: {}\nNo node_modules: {}\n",
        report.project.display(),
        report.status,
        report.score,
        report.passed,
        report
            .artifact_targets
            .iter()
            .filter(|target| target.channel == "pages")
            .count(),
        report
            .artifact_targets
            .iter()
            .filter(|target| target.channel == "r2")
            .count(),
        report.rollback_inputs.len(),
        report.secret_requirements.requires_secrets,
        report.no_node_modules.passed
    );
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

fn forge_publish_plan_markdown(report: &DxForgePublishPlanReport) -> String {
    let mut output = format!(
        "# DX Forge Publish Plan\n\n- Project: `{}`\n- Generated: `{}`\n- Status: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Requires secrets: `{}`\n- no `node_modules`: `{}`\n\n",
        report.project.display(),
        report.generated_at,
        report.status,
        report.passed,
        report.score,
        report.fail_under,
        report.secret_requirements.requires_secrets,
        report.no_node_modules.passed
    );

    output.push_str("## Checks\n\n");
    output.push_str("| Check | Passed | Score | Message |\n");
    output.push_str("| --- | --- | ---: | --- |\n");
    for (label, check) in [
        ("local artifacts", &report.checks.local_artifacts),
        ("Pages artifacts", &report.checks.pages_artifacts),
        ("R2 artifacts", &report.checks.r2_artifacts),
        ("cache headers", &report.checks.cache_headers),
        ("rollback inputs", &report.checks.rollback_inputs),
        (
            "no secret requirements",
            &report.checks.no_secret_requirements,
        ),
        ("no node_modules", &report.checks.no_node_modules),
    ] {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | {} |\n",
            label,
            check.passed,
            check.score,
            markdown_table_cell(&check.message)
        ));
    }

    output.push_str("\n## Artifact Targets\n\n");
    output.push_str("| Channel | Artifact | Destination | Route | Cache-Control |\n");
    output.push_str("| --- | --- | --- | --- | --- |\n");
    for target in &report.artifact_targets {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            target.channel,
            markdown_table_cell(&target.artifact),
            markdown_table_cell(&target.destination),
            markdown_table_cell(target.route.as_deref().unwrap_or("global")),
            markdown_table_cell(&target.cache_control)
        ));
    }

    output.push_str("\n## Cache Headers\n\n");
    output.push_str("| Channel | Pattern | Cache-Control | Reason |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for header in &report.cache_headers {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} |\n",
            header.channel,
            markdown_table_cell(&header.pattern),
            markdown_table_cell(&header.cache_control),
            markdown_table_cell(&header.reason)
        ));
    }

    output.push_str("\n## Rollback Inputs\n\n");
    output.push_str("| Input | Exists | Path | Why |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for input in &report.rollback_inputs {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} |\n",
            input.name,
            input.exists,
            markdown_table_cell(&input.path.display().to_string()),
            markdown_table_cell(&input.message)
        ));
    }

    output.push_str("\n## Secret Policy\n\n");
    output.push_str(&format!(
        "- Registry smoke requires secrets: `{}`\n- Registry operations dry-run: `{}`\n- Blocked markers: `{}`\n\n",
        report.secret_requirements.registry_smoke_requires_secrets,
        report.secret_requirements.registry_operations_dry_run,
        report.secret_requirements.blocked_markers.join("`, `")
    ));

    output.push_str("## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No publish-plan findings for the configured threshold.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{command}`\n"));
    }

    output
}

fn forge_publish_plan_failure_summary(report: &DxForgePublishPlanReport) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge publish-plan did not pass: score {} / 100, required {} / 100",
            report.score, report.fail_under
        );
    }
    format!(
        "DX Forge publish-plan did not pass: {}",
        report.findings.join("; ")
    )
}

fn build_forge_beta_artifact_verify_report(
    release_bundle: &Path,
    pages: &Path,
    registry_smoke_path: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeBetaArtifactVerifyReport> {
    let release_bundle_report = verify_forge_release_bundle_with_options(release_bundle, false)?;
    let pages_bundle_report = verify_forge_pages_bundle(pages)?;
    let registry_smoke = read_forge_publish_plan_json(registry_smoke_path, "registry smoke")?;
    let manifest_path = release_bundle.join(FORGE_RELEASE_BUNDLE_MANIFEST_JSON);
    let manifest: DxForgeReleaseBundleManifest =
        serde_json::from_slice(&std::fs::read(&manifest_path)?)?;
    let signed_manifest = summarize_forge_release_operations_manifest(&manifest_path)?;
    let artifact_targets =
        forge_publish_plan_artifact_targets(release_bundle, &manifest, &registry_smoke);
    let cache_headers = forge_publish_plan_cache_headers();
    let registry_local_path = forge_publish_plan_json_path(&registry_smoke, &["local_registry"]);
    let mut rollback_inputs =
        forge_release_bundle_inspect_rollback_inputs(release_bundle, &manifest_path);
    rollback_inputs.push(forge_publish_plan_rollback_input(
        "pages_bundle",
        pages.to_path_buf(),
        true,
        "Pages preview bundle proves hosted public artifacts are already generated.",
    ));
    rollback_inputs.push(forge_publish_plan_rollback_input(
        "registry_smoke",
        registry_smoke_path.to_path_buf(),
        true,
        "Registry smoke evidence proves R2 publish targets without requiring secrets.",
    ));
    if let Some(path) = registry_local_path.as_ref() {
        rollback_inputs.push(forge_publish_plan_rollback_input(
            "local_registry",
            path.clone(),
            true,
            "Local registry mirror preserves package manifests for rollback review.",
        ));
    }

    let mut dependency_paths = vec![
        release_bundle.to_path_buf(),
        pages.to_path_buf(),
        registry_smoke_path
            .parent()
            .unwrap_or(registry_smoke_path)
            .to_path_buf(),
    ];
    if let Some(path) = &registry_local_path {
        dependency_paths.push(path.clone());
    }
    let dependency_path_refs = dependency_paths
        .iter()
        .map(PathBuf::as_path)
        .collect::<Vec<_>>();
    let no_node_modules = verify_release_candidate_no_node_modules(&dependency_path_refs);
    let mut secret_scan_paths = vec![
        release_bundle.to_path_buf(),
        pages.to_path_buf(),
        registry_smoke_path.to_path_buf(),
    ];
    if let Some(path) = &registry_local_path {
        secret_scan_paths.push(path.clone());
    }
    let secret_requirements =
        forge_publish_plan_secret_requirements(&registry_smoke, &secret_scan_paths);

    let registry_smoke_passed = forge_publish_plan_json_bool(&registry_smoke, &["passed"])
        && !forge_publish_plan_json_bool(&registry_smoke, &["requires_secrets"]);
    let pages_target_count = artifact_targets
        .iter()
        .filter(|target| target.channel == "pages")
        .count();
    let r2_target_count = artifact_targets
        .iter()
        .filter(|target| target.channel == "r2")
        .count();
    let has_package_gallery_target = artifact_targets.iter().any(|target| {
        target.channel == "pages" && target.route.as_deref() == Some("/forge/package-gallery/")
    });
    let public_targets_have_cache = artifact_targets
        .iter()
        .all(|target| !target.cache_control.trim().is_empty() && target.passed);
    let rollback_inputs_passed = rollback_inputs
        .iter()
        .filter(|input| input.required)
        .all(|input| input.passed);
    let signed_manifest_passed = signed_manifest.exists
        && signed_manifest.signed
        && signed_manifest.signature_verified
        && signed_manifest.manifest_digest_verified
        && signed_manifest.artifact_integrity_verified;

    let checks = DxForgeBetaArtifactVerifyChecks {
        release_bundle: release_operations_check(
            release_bundle_report.passed
                && release_bundle_report.score >= fail_under
                && signed_manifest_passed,
            release_bundle_report
                .score
                .min(release_operations_manifest_score(&signed_manifest)),
            format!(
                "release bundle has {} artifact(s), {} public route(s), signed_manifest_verified={}.",
                release_bundle_report.artifact_count,
                release_bundle_report.route_count,
                signed_manifest.signature_verified
            ),
            Some(release_bundle.display().to_string()),
        ),
        pages_bundle: release_operations_check(
            pages_bundle_report.passed
                && pages_bundle_report.score >= fail_under
                && pages_target_count >= 6
                && has_package_gallery_target,
            pages_bundle_report.score,
            format!(
                "Pages bundle verifies {} artifact(s), {} route check(s), mapped_targets={}, package_gallery_target={}.",
                pages_bundle_report.artifacts.len(),
                pages_bundle_report.checks.len(),
                pages_target_count,
                has_package_gallery_target
            ),
            Some(pages.display().to_string()),
        ),
        r2_evidence: release_operations_check(
            registry_smoke_passed && r2_target_count > 0,
            if registry_smoke_passed && r2_target_count > 0 {
                json_u8(registry_smoke.get("score")).unwrap_or(100)
            } else {
                0
            },
            format!(
                "registry smoke ready={}, R2 object target(s)={}.",
                registry_smoke_passed, r2_target_count
            ),
            Some(registry_smoke_path.display().to_string()),
        ),
        cache_policy: release_operations_check(
            public_targets_have_cache && cache_headers.iter().all(|header| header.passed),
            if public_targets_have_cache { 100 } else { 0 },
            format!(
                "{} cache policy row(s), target cache coverage={}.",
                cache_headers.len(),
                public_targets_have_cache
            ),
            None,
        ),
        rollback_inputs: release_operations_check(
            rollback_inputs_passed,
            if rollback_inputs_passed { 100 } else { 0 },
            format!(
                "{} required rollback input(s) checked.",
                rollback_inputs
                    .iter()
                    .filter(|input| input.required)
                    .count()
            ),
            Some(registry_smoke_path.display().to_string()),
        ),
        no_secret_requirements: release_operations_check(
            secret_requirements.passed,
            secret_requirements.score,
            format!(
                "requires_secrets={}, dry_run={}, scanned {} path(s).",
                secret_requirements.requires_secrets,
                secret_requirements.registry_operations_dry_run,
                secret_requirements.scanned_paths.len()
            ),
            Some(registry_smoke_path.display().to_string()),
        ),
        no_node_modules: release_operations_check(
            no_node_modules.passed,
            no_node_modules.score,
            format!(
                "{} downloaded artifact boundary path(s) checked.",
                no_node_modules.checked_paths.len()
            ),
            None,
        ),
    };

    let score = [
        checks.release_bundle.score,
        checks.pages_bundle.score,
        checks.r2_evidence.score,
        checks.cache_policy.score,
        checks.rollback_inputs.score,
        checks.no_secret_requirements.score,
        checks.no_node_modules.score,
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let mut findings = Vec::new();
    append_release_operations_check_finding(
        "release-bundle",
        &checks.release_bundle,
        &mut findings,
    );
    append_release_operations_check_finding("pages-bundle", &checks.pages_bundle, &mut findings);
    append_release_operations_check_finding("r2-evidence", &checks.r2_evidence, &mut findings);
    append_release_operations_check_finding("cache-policy", &checks.cache_policy, &mut findings);
    append_release_operations_check_finding(
        "rollback-inputs",
        &checks.rollback_inputs,
        &mut findings,
    );
    append_release_operations_check_finding(
        "no-secret-requirements",
        &checks.no_secret_requirements,
        &mut findings,
    );
    append_release_operations_check_finding(
        "no-node-modules",
        &checks.no_node_modules,
        &mut findings,
    );
    findings.extend(
        release_bundle_report
            .findings
            .iter()
            .map(|finding| format!("release-bundle: {finding}")),
    );
    findings.extend(
        pages_bundle_report
            .findings
            .iter()
            .map(|finding| format!("pages-bundle: {finding}")),
    );
    findings.extend(
        signed_manifest
            .findings
            .iter()
            .map(|finding| format!("signed-manifest: {finding}")),
    );
    findings.extend(
        artifact_targets
            .iter()
            .filter(|target| !target.passed)
            .map(|target| format!("artifact-targets: {}", target.message)),
    );
    findings.extend(
        rollback_inputs
            .iter()
            .filter(|input| input.required && !input.passed)
            .map(|input| format!("rollback-inputs: {}", input.message)),
    );
    findings.extend(
        secret_requirements
            .findings
            .iter()
            .map(|finding| format!("secret-requirements: {finding}")),
    );
    findings.extend(
        no_node_modules
            .findings
            .iter()
            .map(|finding| format!("no-node-modules: {finding}")),
    );

    let passed = findings.is_empty()
        && score >= fail_under
        && checks.release_bundle.passed
        && checks.pages_bundle.passed
        && checks.r2_evidence.passed
        && checks.cache_policy.passed
        && checks.rollback_inputs.passed
        && checks.no_secret_requirements.passed
        && checks.no_node_modules.passed;
    let status = if passed {
        "ready-for-downloaded-beta-install"
    } else {
        "needs-review"
    }
    .to_string();

    Ok(DxForgeBetaArtifactVerifyReport {
        version: 1,
        release_bundle: release_bundle.to_path_buf(),
        pages: pages.to_path_buf(),
        registry_smoke: registry_smoke_path.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        status,
        fail_under,
        requires_rebuild: false,
        checks,
        release_bundle_report,
        pages_bundle_report,
        signed_manifest,
        artifact_targets,
        cache_headers,
        rollback_inputs,
        secret_requirements,
        no_node_modules,
        findings,
        next_commands: vec![
            format!(
                "dx forge beta-artifact-verify --release-bundle {} --pages {} --registry-smoke {} --format markdown --fail-under {}",
                release_bundle.display(),
                pages.display(),
                registry_smoke_path.display(),
                fail_under
            ),
            format!(
                "dx forge beta-install --release-bundle {} --dry-run --format markdown",
                release_bundle.display()
            ),
            "dx forge publish-plan --project . --release-bundle <bundle> --pages <pages> --registry-smoke <json> --release-operations <json> --format markdown".to_string(),
        ],
    })
}

fn forge_beta_artifact_verify_terminal(report: &DxForgeBetaArtifactVerifyReport) -> String {
    let mut output = format!(
        "DX Forge beta artifact verifier\nRelease bundle: {}\nPages bundle: {}\nRegistry smoke: {}\nStatus: {} ({} / 100)\nPassed: {}\nRequires rebuild: {}\nPages targets: {}\nR2 targets: {}\nRollback inputs: {}\nRequires secrets: {}\nNo node_modules: {}\n",
        report.release_bundle.display(),
        report.pages.display(),
        report.registry_smoke.display(),
        report.status,
        report.score,
        report.passed,
        report.requires_rebuild,
        report
            .artifact_targets
            .iter()
            .filter(|target| target.channel == "pages")
            .count(),
        report
            .artifact_targets
            .iter()
            .filter(|target| target.channel == "r2")
            .count(),
        report.rollback_inputs.len(),
        report.secret_requirements.requires_secrets,
        report.no_node_modules.passed
    );
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

fn forge_beta_artifact_verify_markdown(report: &DxForgeBetaArtifactVerifyReport) -> String {
    let mut output = format!(
        "# DX Forge Beta Artifact Verifier\n\n- Release bundle: `{}`\n- Pages bundle: `{}`\n- Registry smoke: `{}`\n- Generated: `{}`\n- Status: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Requires rebuild: `{}`\n- Requires secrets: `{}`\n- no `node_modules`: `{}`\n\n",
        report.release_bundle.display(),
        report.pages.display(),
        report.registry_smoke.display(),
        report.generated_at,
        report.status,
        report.passed,
        report.score,
        report.fail_under,
        report.requires_rebuild,
        report.secret_requirements.requires_secrets,
        report.no_node_modules.passed
    );

    output.push_str("## Checks\n\n");
    output.push_str("| Check | Passed | Score | Message |\n");
    output.push_str("| --- | --- | ---: | --- |\n");
    for (label, check) in [
        ("release bundle", &report.checks.release_bundle),
        ("Pages bundle", &report.checks.pages_bundle),
        ("R2 evidence", &report.checks.r2_evidence),
        ("cache policy", &report.checks.cache_policy),
        ("rollback inputs", &report.checks.rollback_inputs),
        (
            "no secret requirements",
            &report.checks.no_secret_requirements,
        ),
        ("no node_modules", &report.checks.no_node_modules),
    ] {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | {} |\n",
            label,
            check.passed,
            check.score,
            markdown_table_cell(&check.message)
        ));
    }

    output.push_str("\n## Artifact Targets\n\n");
    output.push_str("| Channel | Artifact | Destination | Route | Cache-Control |\n");
    output.push_str("| --- | --- | --- | --- | --- |\n");
    for target in &report.artifact_targets {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            target.channel,
            markdown_table_cell(&target.artifact),
            markdown_table_cell(&target.destination),
            markdown_table_cell(target.route.as_deref().unwrap_or("global")),
            markdown_table_cell(&target.cache_control)
        ));
    }

    output.push_str("\n## Rollback Inputs\n\n");
    output.push_str("| Input | Exists | Path | Why |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for input in &report.rollback_inputs {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} |\n",
            input.name,
            input.exists,
            markdown_table_cell(&input.path.display().to_string()),
            markdown_table_cell(&input.message)
        ));
    }

    output.push_str("\n## Cache Policy\n\n");
    output.push_str("| Channel | Pattern | Cache-Control | Reason |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for header in &report.cache_headers {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} |\n",
            header.channel,
            markdown_table_cell(&header.pattern),
            markdown_table_cell(&header.cache_control),
            markdown_table_cell(&header.reason)
        ));
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No beta artifact verification findings for the configured threshold.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{command}`\n"));
    }

    output
}

fn forge_beta_artifact_verify_failure_summary(report: &DxForgeBetaArtifactVerifyReport) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge beta-artifact-verify did not pass: score {} / 100, required {} / 100",
            report.score, report.fail_under
        );
    }
    format!(
        "DX Forge beta-artifact-verify did not pass: {}",
        report.findings.join("; ")
    )
}

fn forge_release_review_terminal(report: &DxForgeReleaseReviewReport) -> String {
    let mut output = format!(
        "DX Forge release review\nProject: {}\nStatus: {} ({} / 100)\nPassed: {}\nRelease dashboard: {} / 100\nRelease bundle: {} / 100\nLaunch changelog: {} / 100\nRoutes: {} public routes, {} Brotli bytes\nHuman signoff items: {}\n",
        report.project.display(),
        report.status,
        report.score,
        report.passed,
        report.release_dashboard.score,
        report.release_bundle.score,
        report.launch_changelog.score,
        report.route_comparison.route_count,
        report.route_comparison.total_brotli_bytes,
        report.signoff_items.len()
    );
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

fn forge_release_review_markdown(report: &DxForgeReleaseReviewReport) -> String {
    let mut output = format!(
        "# DX Forge Release Review\n\n- Project: `{}`\n- Generated: `{}`\n- Status: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n\n",
        report.project.display(),
        report.generated_at,
        report.status,
        report.passed,
        report.score,
        report.fail_under
    );

    output.push_str("## Evidence Checks\n\n");
    output.push_str("| Evidence | Score | Passed | Message |\n");
    output.push_str("| --- | ---: | --- | --- |\n");
    for (label, check) in [
        ("release-dashboard", &report.checks.release_dashboard),
        ("release bundle", &report.checks.release_bundle),
        ("release bundle manifest", &report.checks.bundle_manifest),
        ("launch changelog", &report.checks.launch_changelog),
        ("public route comparison", &report.checks.route_comparison),
        ("release history", &report.checks.release_history),
    ] {
        output.push_str(&format!(
            "| `{label}` | {} | {} | {} |\n",
            check.score,
            check.passed,
            markdown_table_cell(&check.message)
        ));
    }

    output.push_str("\n## Release Bundle\n\n");
    output.push_str(&format!(
        "- Bundle: `{}`\n- Artifacts: `{}`\n- Routes: `{}`\n- No `node_modules`: `{}`\n- Manifest artifacts: `{}`\n- Manifest digest verified: `{}`\n- Manifest digest: `{}`\n\n",
        report.release_bundle.bundle_dir.display(),
        report.release_bundle.artifact_count,
        report.release_bundle.route_count,
        report.release_bundle.no_node_modules,
        report.release_bundle.manifest_artifacts,
        report.release_bundle.manifest_digest_verified,
        report.release_bundle.manifest_digest
    ));

    output.push_str("## Public Route Evidence\n\n");
    output.push_str(&format!(
        "- Route comparison: `{}`\n- Routes: `{}`\n- Total decoded bytes: `{}` B\n- Total Brotli estimate: `{}` B\n- Missing required routes: `{}`\n- Failing budget routes: `{}`\n\n",
        report.route_comparison.path.display(),
        report.route_comparison.route_count,
        report.route_comparison.total_decoded_bytes,
        report.route_comparison.total_brotli_bytes,
        if report.route_comparison.missing_routes.is_empty() {
            "none".to_string()
        } else {
            report.route_comparison.missing_routes.join(", ")
        },
        if report.route_comparison.failing_budget_routes.is_empty() {
            "none".to_string()
        } else {
            report.route_comparison.failing_budget_routes.join(", ")
        }
    ));

    output.push_str("## Human Signoff\n\n");
    output.push_str("| Required review | Artifact | Status | Why it matters |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for item in &report.signoff_items {
        output.push_str(&format!(
            "| {} | `{}` | `{}` | {} |\n",
            markdown_table_cell(&item.label),
            item.artifact,
            item.status,
            markdown_table_cell(&item.message)
        ));
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No release-review findings.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }

    output
}

fn forge_release_review_failure_summary(report: &DxForgeReleaseReviewReport) -> String {
    if report.findings.is_empty() {
        return format!(
            "Forge release-review failed with score {} / 100",
            report.score
        );
    }
    format!(
        "Forge release-review failed: {}",
        report.findings.join("; ")
    )
}

fn forge_ci_artifact_filename(label: &str) -> &'static str {
    match label {
        "benchmark history" => "forge-benchmark-history.json",
        "release proof" => "forge-evidence.json",
        "package scorecard" => "forge-scorecard.json",
        "launch source" => "forge-page.html",
        "launch html" => "forge.html",
        "release packet" => "forge.dxp",
        "launch runtime" => "forge.dxp.js",
        "launch summary" => "forge-proof.json",
        "launch claims" => "forge.claims.json",
        "launch evidence model" => "forge.evidence.json",
        _ => "forge-artifact",
    }
}

fn forge_smoke_artifact_paths(artifacts: &DxForgeSmokeArtifacts) -> Vec<(&'static str, &Path)> {
    let mut paths = vec![
        (
            "benchmark history",
            artifacts.benchmark_history_path.as_path(),
        ),
        ("release proof", artifacts.evidence_report_path.as_path()),
        (
            "package scorecard",
            artifacts.scorecard_report_path.as_path(),
        ),
        ("launch source", artifacts.launch_source_path.as_path()),
        ("launch html", artifacts.launch_html_path.as_path()),
        ("release packet", artifacts.launch_packet_path.as_path()),
        ("launch summary", artifacts.launch_summary_path.as_path()),
        ("launch claims", artifacts.launch_claims_path.as_path()),
        (
            "launch evidence model",
            artifacts.launch_evidence_model_path.as_path(),
        ),
    ];
    if let Some(path) = &artifacts.launch_runtime_path {
        paths.push(("launch runtime", path.as_path()));
    }
    paths
}

fn forge_ci_summary_json(
    report: &DxForgeSmokeReport,
    out_dir: &Path,
    artifacts: &[PathBuf],
) -> serde_json::Value {
    serde_json::json!({
        "project": report.project,
        "generated_at": report.generated_at,
        "passed": report.passed,
        "score": report.score,
        "no_node_modules": report.no_node_modules,
        "artifacts_dir": out_dir,
        "artifacts": artifacts
            .iter()
            .map(|path| forge_ci_relative_artifact_path(out_dir, path))
            .collect::<Vec<_>>(),
        "routes": [
            {
                "route": "/forge",
                "artifact": "forge.html"
            },
            {
                "route": "/forge claims",
                "artifact": "forge.claims.json"
            },
            {
                "route": "/forge evidence model",
                "artifact": "forge.evidence.json"
            },
            {
                "route": "/forge/ci source model",
                "artifacts": ["forge-smoke.json", "forge-readiness-badge.json"]
            },
            {
                "route": "/forge/adoption",
                "artifacts": [
                    "forge-adoption-report.json",
                    "forge-adoption-report.md",
                    "forge/adoption.html",
                    "forge/adoption.claims.json",
                    "forge/adoption.proof.json",
                    "forge/adoption.dxp"
                ]
            }
        ]
    })
}

fn forge_ci_summary_markdown(
    report: &DxForgeSmokeReport,
    out_dir: &Path,
    artifacts: &[PathBuf],
) -> String {
    let mut output = format!(
        "# DX Forge CI Summary\n\n- Project: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}`\n- No `node_modules`: `{}`\n- Artifact directory: `{}`\n\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.no_node_modules,
        out_dir.display()
    );

    output.push_str("## Artifact Links\n\n");
    output.push_str("| Artifact | Link |\n");
    output.push_str("| --- | --- |\n");
    for artifact in forge_ci_sorted_artifacts(artifacts) {
        let relative = forge_ci_relative_artifact_path(out_dir, artifact);
        output.push_str(&format!(
            "| `{}` | {} |\n",
            relative,
            markdown_artifact_link(&relative)
        ));
    }

    output.push_str("\n## Route And Evidence Map\n\n");
    output.push_str("| Route or evidence | Artifact |\n");
    output.push_str("| --- | --- |\n");
    output.push_str(&format!(
        "| `/forge` | {} |\n",
        markdown_artifact_link("forge.html")
    ));
    output.push_str(&format!(
        "| `/forge` claims | {} |\n",
        markdown_artifact_link("forge.claims.json")
    ));
    output.push_str(&format!(
        "| `/forge` evidence model | {} |\n",
        markdown_artifact_link("forge.evidence.json")
    ));
    output.push_str(&format!(
        "| `/forge` proof packet | {} |\n",
        markdown_artifact_link("forge.dxp")
    ));
    output.push_str(&format!(
        "| `/forge` source page | {} |\n",
        markdown_artifact_link("forge-page.html")
    ));
    output.push_str(&format!(
        "| `/forge/ci` source model | {}, {} |\n",
        markdown_artifact_link("forge-smoke.json"),
        markdown_artifact_link("forge-readiness-badge.json")
    ));
    output.push_str(&format!(
        "| `/forge/adoption` report | {}, {} |\n",
        markdown_artifact_link("forge-adoption-report.json"),
        markdown_artifact_link("forge-adoption-report.md")
    ));
    output.push_str(&format!(
        "| `/forge/adoption` route | {}, {}, {}, {} |\n",
        markdown_artifact_link("forge/adoption.html"),
        markdown_artifact_link("forge/adoption.claims.json"),
        markdown_artifact_link("forge/adoption.proof.json"),
        markdown_artifact_link("forge/adoption.dxp")
    ));

    output.push_str("\n## Review Order\n\n");
    output.push_str(&format!(
        "1. Open {} for the pass/fail score and package list.\n",
        markdown_artifact_link("forge-smoke.json")
    ));
    output.push_str(&format!(
        "2. Open {} for the compact release badge consumed by CI surfaces.\n",
        markdown_artifact_link("forge-readiness-badge.json")
    ));
    output.push_str(&format!(
        "3. Open {} for first-action failure triage before changing thresholds.\n",
        markdown_artifact_link("forge-triage.md")
    ));
    output.push_str(&format!(
        "4. Open {} and {} to review the public route claims and detached evidence model.\n",
        markdown_artifact_link("forge.claims.json"),
        markdown_artifact_link("forge.evidence.json")
    ));
    output.push_str(&format!(
        "5. Open {} and {} to review the source-owned example app adoption evidence.\n",
        markdown_artifact_link("forge-adoption-report.json"),
        markdown_artifact_link("forge/adoption.html")
    ));

    output
}

fn forge_ci_sorted_artifacts(artifacts: &[PathBuf]) -> Vec<&PathBuf> {
    let mut artifacts = artifacts.iter().collect::<Vec<_>>();
    artifacts.sort_by_key(|path| {
        path.file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string())
    });
    artifacts
}

fn forge_ci_relative_artifact_path(out_dir: &Path, artifact: &Path) -> String {
    artifact
        .strip_prefix(out_dir)
        .unwrap_or(artifact)
        .to_string_lossy()
        .replace('\\', "/")
}

fn markdown_artifact_link(relative: &str) -> String {
    let href = relative.replace(' ', "%20");
    format!("[`{relative}`]({href})")
}

#[cfg(test)]
fn forge_ci_artifact_paths_from_dir(out_dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    fn collect(dir: &Path, artifacts: &mut Vec<PathBuf>) -> anyhow::Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                collect(&path, artifacts)?;
            } else {
                artifacts.push(path);
            }
        }
        Ok(())
    }

    let mut artifacts = Vec::new();
    collect(out_dir, &mut artifacts)?;
    artifacts.sort_by_key(|path| forge_ci_relative_artifact_path(out_dir, path));
    Ok(artifacts)
}

fn forge_smoke_json_bool(path: &Path, key: &str) -> anyhow::Result<bool> {
    let value: serde_json::Value = serde_json::from_slice(&std::fs::read(path)?)?;
    Ok(value
        .get(key)
        .and_then(|value| value.as_bool())
        .unwrap_or(false))
}

fn default_forge_smoke_project() -> PathBuf {
    let timestamp = Utc::now()
        .to_rfc3339()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();
    std::env::temp_dir().join(format!(
        "dx-forge-smoke-{}-{}",
        std::process::id(),
        timestamp
    ))
}

fn default_forge_adoption_smoke_project() -> PathBuf {
    let timestamp = Utc::now()
        .to_rfc3339()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();
    std::env::temp_dir().join(format!(
        "dx-forge-adoption-smoke-{}-{}",
        std::process::id(),
        timestamp
    ))
}
