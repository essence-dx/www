use super::dev_options::DxDevServerBinding;
use super::forge_public_evidence::{
    build_forge_public_evidence_report, forge_public_evidence_markdown,
};
use super::forge_public_status::forge_public_status_report;
use super::forge_release_history::{
    DxForgePublicReleaseHistoryInput, record_forge_public_release_history,
};
use super::server_action_runtime::execute_production_contract_server_action;
use super::studio_command::attach_local_www_routes;
use super::studio_manifest::build_www_routes_report;
use super::*;
use crate::config::DxConfig;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use tempfile::tempdir;

use dx_compiler::ecosystem::build_forge_trust_policy_report;

include!("tests/part_01.rs");
include!("tests/part_02.rs");
include!("tests/part_03.rs");
include!("tests/part_04.rs");
include!("tests/part_05.rs");
