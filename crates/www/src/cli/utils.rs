//! Utility functions

use console::style;
use std::path::Path;
use std::time::Duration;

pub fn format_time(duration: Duration) -> String {
    let micros = duration.as_micros();
    if micros < 1000 {
        format!("{}μs", micros)
    } else if micros < 1_000_000 {
        format!("{:.2}ms", micros as f64 / 1000.0)
    } else {
        format!("{:.2}s", duration.as_secs_f64())
    }
}

pub fn format_size(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{} bytes", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

pub fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub fn rainbow_text(text: &str) -> String {
    let mut out = String::new();
    for (i, c) in text.chars().enumerate() {
        let hue = (i as f32 * 15.0) % 360.0;
        let c_num = hue / 60.0;
        let x = (1.0 - (c_num % 2.0 - 1.0).abs());
        let (r, g, b) = if c_num < 1.0 {
            (1.0, x, 0.0)
        } else if c_num < 2.0 {
            (x, 1.0, 0.0)
        } else if c_num < 3.0 {
            (0.0, 1.0, x)
        } else if c_num < 4.0 {
            (0.0, x, 1.0)
        } else if c_num < 5.0 {
            (x, 0.0, 1.0)
        } else {
            (1.0, 0.0, x)
        };
        let r = (r * 255.0) as u8;
        let g = (g * 255.0) as u8;
        let b = (b * 255.0) as u8;
        out.push_str(&format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, c));
    }
    out
}

pub fn get_dir_size(path: &Path) -> usize {
    let mut size = 0;
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    let file_name = entry.file_name();
                    let name = file_name.to_string_lossy();
                    if name != ".dx" && name != "node_modules" && name != "target" {
                        size += get_dir_size(&entry.path());
                    }
                } else {
                    size += metadata.len() as usize;
                }
            }
        }
    }
    size
}

pub fn ascii_table_string(rows: &[(&str, &str)]) -> String {
    use console::strip_ansi_codes;
    let mut w1 = 0;
    let mut w2 = 0;
    for (k, v) in rows {
        w1 = w1.max(strip_ansi_codes(k).len());
        w2 = w2.max(strip_ansi_codes(v).len());
    }
    
    let border_line = format!("+{}+{}+", "-".repeat(w1 + 2), "-".repeat(w2 + 2));
    let border_dim = style(&border_line).dim().to_string();

    let mut out = String::new();
    out.push_str(&border_dim);
    out.push('\n');
    
    for (k, v) in rows {
        let pad_k = " ".repeat(w1.saturating_sub(strip_ansi_codes(k).len()));
        let pad_v = " ".repeat(w2.saturating_sub(strip_ansi_codes(v).len()));
        out.push_str(&format!("| {}{} | {}{} |\n", k, pad_k, v, pad_v));
        out.push_str(&border_dim);
        out.push('\n');
    }
    out.trim_end().to_string()
}

pub fn print_ascii_table(rows: &[(&str, &str)]) {
    println!("{}", ascii_table_string(rows));
}

pub fn print_build_summary(
    config: &crate::config::DxConfig,
    app_routes_compiled: usize,
    client_islands_compiled: usize,
    generated_style_assets_compiled: usize,
    _total_size: usize,
    total_time: std::time::Duration,
    output_dir: &Path,
    project_cwd: &Path,
) {
    let project_size = get_dir_size(project_cwd);
    let full_path = project_cwd.join(output_dir).join("index.html");
    
    println!();
    println!("{}", rainbow_text(&format!("◆ Www {} (Dx)", env!("CARGO_PKG_VERSION"))));
    
    let name_val = config.project.name.clone();
    let output_val = style(full_path.display().to_string()).cyan().to_string();
    let size_val = format_size(project_size);
    
    print_ascii_table(
        &[
            ("Name", &name_val),
            ("Output", &output_val),
            ("Size", &size_val),
        ]
    );

    println!();
    println!("{}", style("Creating an optimized production build ...").dim());
    println!("{} Compiled {} routes in {}", style("✓").green(), app_routes_compiled, format_time(total_time));
    if client_islands_compiled > 0 {
        println!("{} Processed {} client islands", style("✓").green(), client_islands_compiled);
    }
    if generated_style_assets_compiled > 0 {
        println!("{} Generated style assets successfully", style("✓").green());
    }
    println!("{} Finalizing page optimization", style("✓").green());
    println!();
}
