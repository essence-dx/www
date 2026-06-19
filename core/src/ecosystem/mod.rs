//! Ecosystem Integration Module for dx-www
//!
//! This module provides unified access to the surrounding dx ecosystem tools:
//! serializer, markdown/content compilation, icons, fonts, media, and codegen helpers.

use anyhow::{Context, Result};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::schema_parser;
use crate::schema_parser::QueryDefinition;
use crate::www_config::{self, DxWwwConfig};

pub use dx_icon;

mod config;
mod content;
mod dx_check_receipt;
mod dx_style_receipts;
mod features;
mod fonts;
mod forge_drizzle;
mod forge_file_transaction;
mod forge_fumadocs;
mod forge_importer;
mod forge_instantdb;
mod forge_motion;
mod forge_n8n_automations;
mod forge_next_intl;
mod forge_package_status_machine;
mod forge_r2_head;
mod forge_react_hook_form;
mod forge_react_markdown;
mod forge_reactive_store;
mod forge_registry;
mod forge_remote_health;
mod forge_root_manifest;
mod forge_scorecard;
mod forge_security;
mod forge_stripe_js;
mod forge_supabase;
mod forge_tanstack_query;
mod forge_three_scene;
mod forge_trpc;
mod forge_trust_policy;
mod forge_vercel_ai;
mod forge_wasm_bindgen;
mod forge_zod;
mod forge_zustand;
mod icons;
mod json_receipt_machine;
mod media;
mod project_check;

pub use config::*;
pub use content::*;
pub use dx_check_receipt::*;
pub use features::*;
pub use fonts::*;
pub use forge_file_transaction::*;
pub use forge_importer::*;
pub use forge_package_status_machine::{
    write_forge_package_status_machine_cache,
    write_forge_package_status_machine_cache_with_performance_receipt,
};
pub use forge_r2_head::*;
pub use forge_registry::*;
pub use forge_remote_health::*;
pub use forge_scorecard::*;
pub use forge_security::*;
pub use forge_trust_policy::*;
pub use icons::*;
pub use json_receipt_machine::write_json_receipt_machine_alias;
pub use media::*;
pub use project_check::*;

#[cfg(test)]
mod property_tests;
#[cfg(test)]
mod unit_tests;
