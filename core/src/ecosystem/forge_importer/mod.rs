//! Policy-first Forge import firewall.
//!
//! These types describe how Forge reviews package ecosystems before any source
//! can become app-importable. Package-manager installs and lifecycle scripts
//! stay forbidden; live registry acquisition is enabled only for ecosystems
//! with an explicit non-executing fetcher and receipt model.

pub mod acquire;
pub mod analyze;
pub mod capabilities;
pub mod cargo;
pub mod composer;
pub mod cran;
pub mod disposition;
pub mod gem;
pub mod go;
pub mod hex;
pub mod jsr;
pub mod maven;
pub mod npm;
pub mod nuget;
pub mod pip;
pub mod pub_package;
pub mod quarantine;
pub mod receipts;
pub mod scoring;
pub mod slice;
pub mod swift;
pub mod types;

pub use acquire::*;
pub use analyze::*;
pub use capabilities::*;
pub use cargo::*;
pub use composer::*;
pub use cran::*;
pub use disposition::*;
pub use gem::*;
pub use go::*;
pub use hex::*;
pub use jsr::*;
pub use maven::*;
pub use npm::*;
pub use nuget::*;
pub use pip::*;
pub use pub_package::*;
pub use quarantine::*;
pub use receipts::*;
pub use scoring::*;
pub use slice::*;
pub use swift::*;
pub use types::*;

#[cfg(test)]
mod tests;
