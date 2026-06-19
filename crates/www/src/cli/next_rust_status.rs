use crate::error::DxResult;

use super::forge_error;

pub(super) fn cmd_www_next_rust(args: &[String]) -> DxResult<()> {
    let json = args.iter().any(|arg| arg == "--json");
    let snapshot = crate::next_rust::dx_next_rust_vendor_snapshot();
    if json {
        let capabilities: Vec<_> = snapshot
            .capabilities
            .iter()
            .map(|capability| {
                serde_json::json!({
                    "upstream": capability.upstream,
                    "dx_role": capability.dx_role,
                    "boundary": capability.boundary,
                    "reference_only": capability.reference_only,
                })
            })
            .collect();
        let protected_boundaries: Vec<_> = snapshot
            .protected_boundaries
            .iter()
            .map(|boundary| {
                serde_json::json!({
                    "name": boundary.name,
                    "owner": boundary.owner,
                    "rule": boundary.rule,
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "framework": "DX-WWW",
                "vendor_root": snapshot.vendor_root,
                "license_file": snapshot.license_file,
                "upstream_repo": snapshot.upstream_repo,
                "commit": snapshot.commit,
                "branch": snapshot.branch,
                "imported_on": snapshot.imported_on,
                "source_path_at_import": snapshot.source_path_at_import,
                "runtime_build_adoption": snapshot.runtime_build_adoption,
                "public_architecture": snapshot.public_architecture,
                "capabilities": capabilities,
                "protected_runtime_crates": snapshot.protected_runtime_crates,
                "protected_boundaries": protected_boundaries,
                "excluded_core_foundations": snapshot.excluded_core_foundations,
            }))
            .map_err(forge_error)?
        );
        return Ok(());
    }

    println!("DX-WWW Next/Turbopack Rust vendor snapshot");
    println!("Vendor root: {}", snapshot.vendor_root);
    println!("License: {}", snapshot.license_file);
    println!("Upstream: {} @ {}", snapshot.upstream_repo, snapshot.commit);
    println!(
        "Runtime/build adoption: {}",
        snapshot.runtime_build_adoption
    );
    println!("Public architecture: {}", snapshot.public_architecture);
    println!(
        "Imported from {} on {}",
        snapshot.branch, snapshot.imported_on
    );
    println!();
    println!("DX-owned runtime core remains authoritative:");
    for crate_name in snapshot.protected_runtime_crates {
        println!("  - {crate_name}");
    }
    println!();
    println!("Protected DX boundaries:");
    for boundary in snapshot.protected_boundaries {
        println!("  - {} ({})", boundary.name, boundary.owner);
        println!("    {}", boundary.rule);
    }
    println!();
    println!("Excluded core foundations:");
    for foundation in snapshot.excluded_core_foundations {
        println!("  - {foundation}");
    }
    println!();
    println!("Imported reference capabilities:");
    for capability in snapshot.capabilities {
        let marker = if capability.reference_only {
            "reference-only"
        } else {
            "blocked runtime/build adoption"
        };
        println!(
            "  - {} -> {} ({marker}; {})",
            capability.upstream, capability.dx_role, capability.boundary
        );
    }
    Ok(())
}
