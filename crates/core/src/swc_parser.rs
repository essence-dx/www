//! Legacy Oxc-backed TSX parser entrypoint.
//!
//! Older call sites used this module name when the parser bridge was still
//! called "SWC". Keep the module as a thin compatibility shim over the current
//! parser pipeline so the `oxc` feature has one maintained implementation path.

use std::path::Path;

use anyhow::{Result, anyhow};

use crate::linker::SymbolTable;
use crate::parser::{ParsedModule, parse_entry};

/// Parse a TSX/JSX file through the production parser pipeline.
pub fn parse_tsx_file(path: &Path, verbose: bool) -> Result<ParsedModule> {
    let symbol_table = SymbolTable::new();
    let mut modules = parse_entry(path, &symbol_table, verbose)?;
    modules
        .pop()
        .ok_or_else(|| anyhow!("No parseable TSX module found in {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_component() {
        let code = r#"
function HelloWorld() {
    return <div>Hello World</div>;
}
        "#;

        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_component.tsx");
        std::fs::write(&temp_file, code).unwrap();

        let result = parse_tsx_file(&temp_file, false);
        assert!(result.is_ok());

        let module = result.unwrap();
        assert_eq!(module.components.len(), 1);
        assert_eq!(module.components[0].name, "HelloWorld");
    }
}
