use std::path::Path;
use reqwest::blocking::multipart;
use base64::{Engine as _, engine::general_purpose};
use std::io::Read;

use crate::error::{DxError, DxResult};

pub fn generate_sandbox_html(output_dir: &Path) -> DxResult<()> {
    let index_html_path = output_dir.join("index.html");
    if !index_html_path.exists() {
        return Err(DxError::InternalError {
            message: "index.html not found in output directory".to_string(),
        });
    }

    let mut html = std::fs::read_to_string(&index_html_path).map_err(|e| DxError::InternalError {
        message: format!("Failed to read index.html: {}", e),
    })?;

    println!("Generating sandbox HTML...");

    // Inline CSS
    html = inline_css(&html, output_dir)?;

    // Inline JS
    html = inline_js(&html, output_dir)?;

    // Upload and replace images/icons
    html = upload_and_replace_media(&html, output_dir)?;

    // Write www.html
    let www_html_path = output_dir.join("www.html");
    std::fs::write(&www_html_path, html).map_err(|e| DxError::InternalError {
        message: format!("Failed to write www.html: {}", e),
    })?;

    println!("Successfully generated sandbox HTML at {}", www_html_path.display());

    Ok(())
}

fn inline_css(html: &str, output_dir: &Path) -> DxResult<String> {
    let mut result = html.to_string();
    let css_regex = regex::Regex::new(r#"<link[^>]*rel="stylesheet"[^>]*href="([^"]+)"[^>]*>"#).unwrap();
    
    for caps in css_regex.captures_iter(html) {
        let full_match = caps.get(0).unwrap().as_str();
        let href = caps.get(1).unwrap().as_str();
        
        if href.starts_with("http") || href.starts_with("//") {
            continue;
        }

        let file_path = output_dir.join(href.trim_start_matches('/'));
        if let Ok(css_content) = std::fs::read_to_string(&file_path) {
            let inline_style = format!("<style>{}</style>", css_content);
            result = result.replace(full_match, &inline_style);
        } else {
            println!("Warning: Could not inline CSS file: {}", file_path.display());
        }
    }
    Ok(result)
}

fn inline_js(html: &str, output_dir: &Path) -> DxResult<String> {
    let mut result = html.to_string();
    let js_regex = regex::Regex::new(r#"<script[^>]*src="([^"]+)"[^>]*>[\s\S]*?</script>"#).unwrap();
    
    for caps in js_regex.captures_iter(html) {
        let full_match = caps.get(0).unwrap().as_str();
        let src = caps.get(1).unwrap().as_str();
        
        if src.starts_with("http") || src.starts_with("//") {
            continue;
        }

        let file_path = output_dir.join(src.trim_start_matches('/'));
        if let Ok(js_content) = std::fs::read_to_string(&file_path) {
            let inline_script = format!("<script>{}</script>", js_content);
            result = result.replace(full_match, &inline_script);
        } else {
            println!("Warning: Could not inline JS file: {}", file_path.display());
        }
    }
    Ok(result)
}

fn upload_and_replace_media(html: &str, output_dir: &Path) -> DxResult<String> {
    let mut result = html.to_string();
    
    let img_regex = regex::Regex::new(r#"<img[^>]*src="([^"]+)""#).unwrap();
    for caps in img_regex.captures_iter(html) {
        let full_match = caps.get(0).unwrap().as_str();
        let src = caps.get(1).unwrap().as_str();
        
        if src.starts_with("http") || src.starts_with("//") || src.starts_with("data:") {
            continue;
        }

        let file_path = output_dir.join(src.trim_start_matches('/'));
        if let Ok(url) = upload_file(&file_path) {
            let new_match = full_match.replace(src, &url);
            result = result.replace(full_match, &new_match);
        }
    }

    let icon_regex = regex::Regex::new(r#"<link[^>]*rel="icon"[^>]*href="([^"]+)"[^>]*>"#).unwrap();
    for caps in icon_regex.captures_iter(html) {
        let full_match = caps.get(0).unwrap().as_str();
        let href = caps.get(1).unwrap().as_str();
        
        if href.starts_with("http") || href.starts_with("//") || href.starts_with("data:") {
            continue;
        }

        let file_path = output_dir.join(href.trim_start_matches('/'));
        if let Ok(url) = upload_file(&file_path) {
            let new_match = full_match.replace(href, &url);
            result = result.replace(full_match, &new_match);
        }
    }

    Ok(result)
}

fn upload_file(path: &Path) -> Result<String, String> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    
    let imgbb_extensions = ["jpg", "jpeg", "png", "bmp", "gif", "tif", "webp", "heic", "pdf"];
    if imgbb_extensions.contains(&ext.as_str()) {
        upload_to_imgbb(path)
    } else {
        upload_to_catbox(path)
    }
}

fn upload_to_imgbb(path: &Path) -> Result<String, String> {
    println!("Uploading {} to ImgBB...", path.display());
    let mut file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
    
    let b64 = general_purpose::STANDARD.encode(&buffer);
    
    let client = reqwest::blocking::Client::new();
    let res = client.post("https://api.imgbb.com/1/upload?key=2c8180634cecec9ced202676d6f3c3f2")
        .form(&[("image", b64)])
        .send()
        .map_err(|e| e.to_string())?;
        
    let json: serde_json::Value = res.json().map_err(|e| e.to_string())?;
    
    if let Some(url) = json["data"]["url"].as_str() {
        Ok(url.to_string())
    } else {
        Err("Failed to parse ImgBB response".to_string())
    }
}

fn upload_to_catbox(path: &Path) -> Result<String, String> {
    println!("Uploading {} to Catbox...", path.display());
    
    let form = multipart::Form::new()
        .text("reqtype", "fileupload")
        .file("fileToUpload", path)
        .map_err(|e| e.to_string())?;
        
    let client = reqwest::blocking::Client::new();
    let res = client.post("https://catbox.moe/user/api.php")
        .multipart(form)
        .send()
        .map_err(|e| e.to_string())?;
        
    let url = res.text().map_err(|e| e.to_string())?;
    if url.starts_with("http") {
        Ok(url)
    } else {
        Err(format!("Failed to upload to catbox: {}", url))
    }
}
