//! # DX Format Parsers
//!
//! Parsers for DX-WWW file formats (.html, .tsx, .lyt, dx config)
//!
//! ## File Formats
//!
//! - `.html` - Page files (route handlers with template and logic)
//! - `.tsx` - Component files (reusable UI components)
//! - `.lyt` - Layout files (page wrappers with slots)
//! - `dx` - Configuration file (no extension)
//!
//! ## Example
//!
//! ```rust,ignore
//! use dx_core::dx_parser::{parse_dx_file, BlockType};
//!
//! let source = std::fs::read_to_string("pages/index.html")?;
//! let ast = parse_dx_file(&source, Some(BlockType::Page))?;
//!
//! for class in &ast.css_classes {
//!     println!("CSS class: {}", class);
//! }
//! ```

pub mod dx_format;

pub use dx_format::*;
