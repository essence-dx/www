use std::path::{Path, PathBuf};

use chrono::Utc;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

use super::{
    decode_prefixed_hex, encode_hex, forge_release_bundle_publisher_key_id,
    read_forge_publisher_private_key,
};

const BUILD_PROMOTION_FILE: &str = "build-promotion.json";
const BUILD_PROMOTION_SCHEME: &str = "dx-www-build-manifest-promotion-v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DxBuildManifestPromotionReport {
    pub version: u32,
    pub scheme: String,
    pub generated_at: String,
    pub passed: bool,
    pub score: u8,
    pub build_dir: PathBuf,
    pub build_manifest: DxBuildManifestPromotionManifest,
    pub deploy_adapter: DxBuildManifestPromotionDeployAdapter,
    pub publisher_identity: DxBuildManifestPromotionPublisherIdentity,
    pub verification: DxBuildManifestPromotionVerification,
    pub wrote_promotion: bool,
    pub wrote_deploy_adapter: bool,
    pub findings: Vec<String>,
    pub next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DxBuildManifestPromotionManifest {
    pub path: String,
    pub hash: String,
    pub bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DxBuildManifestPromotionDeployAdapter {
    pub path: String,
    pub adapter: String,
    pub no_node_modules_required: bool,
    pub route_count: usize,
    pub immutable_asset_count: usize,
    pub health_check_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DxBuildManifestPromotionPublisherIdentity {
    pub status: String,
    pub signer: String,
    pub key_id: String,
    pub algorithm: String,
    pub public_key: String,
    pub signature: String,
    pub signed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DxBuildManifestPromotionVerification {
    pub manifest_hash_verified: bool,
    pub deploy_adapter_hash_verified: bool,
    pub signature_verified: bool,
    pub ready_for_hosted_release: bool,
}

pub(super) fn promote_build_manifest_with_local_key(
    build_dir: &Path,
    key_path: &Path,
) -> anyhow::Result<DxBuildManifestPromotionReport> {
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

    let manifest_path = build_dir.join("manifest.json");
    let manifest_bytes = std::fs::read(&manifest_path)?;
    let manifest_hash = build_manifest_hash(&manifest_bytes);
    let mut deploy_adapter = read_deploy_adapter(build_dir)?;
    verify_unsigned_deploy_adapter_manifest(&deploy_adapter, &manifest_hash)?;
    let deploy_summary =
        deploy_adapter_summary(&deploy_adapter).map_err(|error| anyhow::anyhow!(error))?;

    let signed_at = Utc::now().to_rfc3339();
    let payload = build_manifest_promotion_signing_payload(
        &manifest_hash,
        "manifest.json",
        &deploy_summary,
        &key.signer,
        &key.key_id,
        &key.public_key,
        &signed_at,
    );
    let signature = format!(
        "ed25519:{}",
        encode_hex(&signing_key.sign(payload.as_bytes()).to_bytes())
    );

    let publisher_identity = DxBuildManifestPromotionPublisherIdentity {
        status: "signed".to_string(),
        signer: key.signer.clone(),
        key_id: key.key_id.clone(),
        algorithm: "ed25519".to_string(),
        public_key: key.public_key.clone(),
        signature: signature.clone(),
        signed_at,
    };
    let verification = verify_build_manifest_signature(
        &manifest_hash,
        "manifest.json",
        &deploy_summary,
        &publisher_identity,
    )
    .map_err(|error| anyhow::anyhow!(error))?;

    attach_signed_build_manifest_to_deploy_adapter(
        &mut deploy_adapter,
        &manifest_hash,
        &publisher_identity,
    );

    let mut report = DxBuildManifestPromotionReport {
        version: 1,
        scheme: BUILD_PROMOTION_SCHEME.to_string(),
        generated_at: Utc::now().to_rfc3339(),
        passed: false,
        score: 0,
        build_dir: build_dir.to_path_buf(),
        build_manifest: DxBuildManifestPromotionManifest {
            path: "manifest.json".to_string(),
            hash: manifest_hash,
            bytes: manifest_bytes.len() as u64,
        },
        deploy_adapter: deploy_summary,
        publisher_identity,
        verification,
        wrote_promotion: false,
        wrote_deploy_adapter: false,
        findings: Vec::new(),
        next_commands: vec![
            "dx preview --production-contract".to_string(),
            "Upload the signed build output only after build-promotion.json verifies.".to_string(),
        ],
    };
    report.passed = report.verification.ready_for_hosted_release;
    report.score = if report.passed { 100 } else { 0 };

    let promotion_path = build_dir.join(BUILD_PROMOTION_FILE);
    std::fs::write(&promotion_path, serde_json::to_vec_pretty(&report)?)?;
    report.wrote_promotion = promotion_path.is_file();
    std::fs::write(
        build_dir.join("deploy-adapter.json"),
        serde_json::to_vec_pretty(&deploy_adapter)?,
    )?;
    report.wrote_deploy_adapter = build_dir.join("deploy-adapter.json").is_file();
    std::fs::write(&promotion_path, serde_json::to_vec_pretty(&report)?)?;

    match verify_build_manifest_promotion(build_dir) {
        Ok(verified) => {
            report.verification = verified;
            report.passed = report.verification.ready_for_hosted_release
                && report.wrote_promotion
                && report.wrote_deploy_adapter;
            report.score = if report.passed { 100 } else { 0 };
        }
        Err(error) => {
            report.findings.push(error);
            report.passed = false;
            report.score = 0;
        }
    }
    std::fs::write(&promotion_path, serde_json::to_vec_pretty(&report)?)?;

    Ok(report)
}

pub(super) fn verify_build_manifest_promotion(
    build_dir: &Path,
) -> Result<DxBuildManifestPromotionVerification, String> {
    let promotion_path = build_dir.join(BUILD_PROMOTION_FILE);
    let promotion: DxBuildManifestPromotionReport =
        serde_json::from_slice(&std::fs::read(&promotion_path).map_err(|error| {
            format!(
                "Failed to read {} before hosted release: {error}",
                promotion_path.display()
            )
        })?)
        .map_err(|error| format!("Failed to parse build-promotion.json: {error}"))?;

    if promotion.version != 1 || promotion.scheme != BUILD_PROMOTION_SCHEME {
        return Err(format!(
            "Unsupported build promotion scheme `{}` version {}",
            promotion.scheme, promotion.version
        ));
    }

    let manifest_bytes = std::fs::read(build_dir.join(&promotion.build_manifest.path))
        .map_err(|error| format!("Failed to read promoted build manifest: {error}"))?;
    let manifest_hash = build_manifest_hash(&manifest_bytes);
    if manifest_hash != promotion.build_manifest.hash {
        return Err(format!(
            "Promoted manifest hash `{}` does not match current manifest hash `{manifest_hash}`",
            promotion.build_manifest.hash
        ));
    }

    let deploy_adapter = read_deploy_adapter(build_dir)
        .map_err(|error| format!("Failed to read deploy adapter: {error}"))?;
    let deploy_summary = deploy_adapter_summary(&deploy_adapter)?;
    if deploy_summary.adapter != promotion.deploy_adapter.adapter
        || deploy_summary.no_node_modules_required
            != promotion.deploy_adapter.no_node_modules_required
        || deploy_summary.route_count != promotion.deploy_adapter.route_count
        || deploy_summary.immutable_asset_count != promotion.deploy_adapter.immutable_asset_count
        || deploy_summary.health_check_count != promotion.deploy_adapter.health_check_count
    {
        return Err(
            "Current deploy-adapter.json no longer matches build-promotion.json".to_string(),
        );
    }

    let build_manifest = &deploy_adapter["build_manifest"];
    if build_manifest["path"].as_str() != Some(promotion.build_manifest.path.as_str()) {
        return Err("deploy-adapter.json build_manifest path does not match promotion".to_string());
    }
    if build_manifest["hash"].as_str() != Some(promotion.build_manifest.hash.as_str()) {
        return Err("deploy-adapter.json build_manifest hash does not match promotion".to_string());
    }
    if build_manifest["signed"].as_bool() != Some(true) {
        return Err("deploy-adapter.json build_manifest is not signed".to_string());
    }
    if build_manifest["signature"].as_str() != Some(promotion.publisher_identity.signature.as_str())
    {
        return Err(
            "deploy-adapter.json build_manifest signature does not match promotion".to_string(),
        );
    }

    verify_build_manifest_signature(
        &manifest_hash,
        &promotion.build_manifest.path,
        &promotion.deploy_adapter,
        &promotion.publisher_identity,
    )
}

pub(super) fn build_manifest_promotion_terminal(report: &DxBuildManifestPromotionReport) -> String {
    format!(
        "DX-WWW build promotion\nBuild: {}\nManifest hash: {}\nSigner: {}\nSignature verified: {}\nReady for hosted release: {}\nPassed: {}\n",
        report.build_dir.display(),
        report.build_manifest.hash,
        report.publisher_identity.signer,
        report.verification.signature_verified,
        report.verification.ready_for_hosted_release,
        report.passed
    )
}

pub(super) fn build_manifest_promotion_markdown(report: &DxBuildManifestPromotionReport) -> String {
    let mut output = format!(
        "# DX-WWW Build Promotion\n\n- Build: `{}`\n- Manifest: `{}`\n- Manifest hash: `{}`\n- Signer: `{}`\n- Key id: `{}`\n- Signed at: `{}`\n- Signature verified: `{}`\n- Ready for hosted release: `{}`\n- Passed: `{}`\n\n",
        report.build_dir.display(),
        report.build_manifest.path,
        report.build_manifest.hash,
        report.publisher_identity.signer,
        report.publisher_identity.key_id,
        report.publisher_identity.signed_at,
        report.verification.signature_verified,
        report.verification.ready_for_hosted_release,
        report.passed
    );
    if report.findings.is_empty() {
        output.push_str("No promotion findings. The private key seed is intentionally omitted.\n");
    } else {
        output.push_str("## Findings\n\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

pub(super) fn build_manifest_promotion_failure_summary(
    report: &DxBuildManifestPromotionReport,
) -> String {
    if report.findings.is_empty() {
        return "DX-WWW build promotion failed".to_string();
    }
    format!(
        "DX-WWW build promotion failed: {}",
        report.findings.join("; ")
    )
}

fn build_manifest_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3::hash(bytes).to_hex())
}

fn read_deploy_adapter(build_dir: &Path) -> anyhow::Result<serde_json::Value> {
    Ok(serde_json::from_slice(&std::fs::read(
        build_dir.join("deploy-adapter.json"),
    )?)?)
}

fn verify_unsigned_deploy_adapter_manifest(
    deploy_adapter: &serde_json::Value,
    manifest_hash: &str,
) -> anyhow::Result<()> {
    let build_manifest = &deploy_adapter["build_manifest"];
    if build_manifest["path"].as_str() != Some("manifest.json") {
        anyhow::bail!("deploy-adapter.json build_manifest.path must be manifest.json");
    }
    if build_manifest["hash"].as_str() != Some(manifest_hash) {
        anyhow::bail!("deploy-adapter.json build_manifest.hash does not match manifest.json");
    }
    if deploy_adapter["no_node_modules_required"].as_bool() != Some(true) {
        anyhow::bail!("signed promotion requires no_node_modules_required=true");
    }
    Ok(())
}

fn deploy_adapter_summary(
    deploy_adapter: &serde_json::Value,
) -> Result<DxBuildManifestPromotionDeployAdapter, String> {
    Ok(DxBuildManifestPromotionDeployAdapter {
        path: "deploy-adapter.json".to_string(),
        adapter: deploy_adapter["adapter"]
            .as_str()
            .ok_or_else(|| "deploy-adapter.json missing adapter".to_string())?
            .to_string(),
        no_node_modules_required: deploy_adapter["no_node_modules_required"].as_bool()
            == Some(true),
        route_count: deploy_adapter["routes"]
            .as_array()
            .map(Vec::len)
            .unwrap_or(0),
        immutable_asset_count: deploy_adapter["immutable_assets"]
            .as_array()
            .map(Vec::len)
            .unwrap_or(0),
        health_check_count: deploy_adapter["health_checks"]
            .as_array()
            .map(Vec::len)
            .unwrap_or(0),
    })
}

fn attach_signed_build_manifest_to_deploy_adapter(
    deploy_adapter: &mut serde_json::Value,
    manifest_hash: &str,
    publisher_identity: &DxBuildManifestPromotionPublisherIdentity,
) {
    deploy_adapter["build_manifest"]["hash"] = serde_json::Value::String(manifest_hash.to_string());
    deploy_adapter["build_manifest"]["signed"] = serde_json::Value::Bool(true);
    deploy_adapter["build_manifest"]["signature"] =
        serde_json::Value::String(publisher_identity.signature.clone());
    deploy_adapter["build_manifest"]["signature_algorithm"] =
        serde_json::Value::String("ed25519".to_string());
    deploy_adapter["build_manifest"]["signed_at"] =
        serde_json::Value::String(publisher_identity.signed_at.clone());
    deploy_adapter["build_manifest"]["publisher"] = serde_json::json!({
        "signer": publisher_identity.signer,
        "key_id": publisher_identity.key_id,
        "algorithm": publisher_identity.algorithm,
        "public_key": publisher_identity.public_key,
    });
    deploy_adapter["build_manifest"]["promotion"] = serde_json::json!({
        "path": BUILD_PROMOTION_FILE,
        "scheme": BUILD_PROMOTION_SCHEME,
        "verified": true,
    });
}

fn verify_build_manifest_signature(
    manifest_hash: &str,
    manifest_path: &str,
    deploy_adapter: &DxBuildManifestPromotionDeployAdapter,
    publisher_identity: &DxBuildManifestPromotionPublisherIdentity,
) -> Result<DxBuildManifestPromotionVerification, String> {
    if publisher_identity.algorithm != "ed25519" {
        return Err(format!(
            "unsupported build promotion signature algorithm `{}`",
            publisher_identity.algorithm
        ));
    }
    let public_key_bytes =
        decode_prefixed_hex(&publisher_identity.public_key, "ed25519:", "public_key", 32)?;
    let expected_key_id = forge_release_bundle_publisher_key_id(&public_key_bytes);
    if publisher_identity.key_id != expected_key_id {
        return Err(format!(
            "publisher key_id `{}` does not match public key fingerprint `{expected_key_id}`",
            publisher_identity.key_id
        ));
    }
    let signature_bytes =
        decode_prefixed_hex(&publisher_identity.signature, "ed25519:", "signature", 64)?;
    let public_key_array: [u8; 32] = public_key_bytes
        .try_into()
        .map_err(|_| "publisher public_key must decode to 32 bytes".to_string())?;
    let signature_array: [u8; 64] = signature_bytes
        .try_into()
        .map_err(|_| "publisher signature must decode to 64 bytes".to_string())?;
    let verifying_key = VerifyingKey::from_bytes(&public_key_array)
        .map_err(|error| format!("publisher public_key is invalid: {error}"))?;
    let signature = Signature::from_bytes(&signature_array);
    let payload = build_manifest_promotion_signing_payload(
        manifest_hash,
        manifest_path,
        deploy_adapter,
        &publisher_identity.signer,
        &publisher_identity.key_id,
        &publisher_identity.public_key,
        &publisher_identity.signed_at,
    );
    verifying_key
        .verify(payload.as_bytes(), &signature)
        .map_err(|error| format!("build manifest signature verification failed: {error}"))?;

    let manifest_hash_verified = manifest_hash.starts_with("blake3:");
    let deploy_adapter_hash_verified = deploy_adapter.no_node_modules_required;
    let signature_verified = true;
    Ok(DxBuildManifestPromotionVerification {
        manifest_hash_verified,
        deploy_adapter_hash_verified,
        signature_verified,
        ready_for_hosted_release: manifest_hash_verified
            && deploy_adapter_hash_verified
            && signature_verified,
    })
}

fn build_manifest_promotion_signing_payload(
    manifest_hash: &str,
    manifest_path: &str,
    deploy_adapter: &DxBuildManifestPromotionDeployAdapter,
    signer: &str,
    key_id: &str,
    public_key: &str,
    signed_at: &str,
) -> String {
    format!(
        "{BUILD_PROMOTION_SCHEME}\nmanifest_path={manifest_path}\nmanifest_hash={manifest_hash}\nadapter={}\nno_node_modules_required={}\nroute_count={}\nimmutable_asset_count={}\nhealth_check_count={}\nsigner={signer}\nkey_id={key_id}\npublic_key={public_key}\nsigned_at={signed_at}\n",
        deploy_adapter.adapter,
        deploy_adapter.no_node_modules_required,
        deploy_adapter.route_count,
        deploy_adapter.immutable_asset_count,
        deploy_adapter.health_check_count
    )
}
