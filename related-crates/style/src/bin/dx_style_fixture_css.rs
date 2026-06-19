use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use style::core::StyleEngine;

#[derive(Debug, Deserialize)]
struct FixtureMatrix {
    classes: Vec<FixtureClass>,
}

#[derive(Debug, Deserialize)]
struct FixtureClass {
    #[serde(rename = "className")]
    class_name: String,
}

#[derive(Debug, Serialize)]
struct FixtureCssReceipt {
    schema: &'static str,
    schema_version: u8,
    generated_by: &'static str,
    class_count: usize,
    classes: Vec<FixtureCssEntry>,
}

#[derive(Debug, Serialize)]
struct FixtureCssEntry {
    class_name: String,
    supported: bool,
    generated_css: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matrix_path = matrix_path_from_args()?;
    let matrix_source = fs::read_to_string(&matrix_path)?;
    let matrix: FixtureMatrix = serde_json::from_str(&matrix_source)?;
    let engine = StyleEngine::empty();

    let classes = matrix
        .classes
        .into_iter()
        .map(|fixture| {
            let generated_css = engine.css_for_class(&fixture.class_name);
            FixtureCssEntry {
                supported: generated_css.is_some(),
                class_name: fixture.class_name,
                generated_css,
            }
        })
        .collect::<Vec<_>>();

    let receipt = FixtureCssReceipt {
        schema: "dx.style.fixtureCssReceipt",
        schema_version: 1,
        generated_by: "dx_style_fixture_css",
        class_count: classes.len(),
        classes,
    };

    println!("{}", serde_json::to_string_pretty(&receipt)?);
    Ok(())
}

fn matrix_path_from_args() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--matrix" {
            if let Some(path) = args.next() {
                return Ok(PathBuf::from(path));
            }
            return Err("--matrix requires a path".into());
        }
    }

    Err("usage: dx_style_fixture_css --matrix <path>".into())
}
