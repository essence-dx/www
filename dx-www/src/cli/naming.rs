use std::path::Path;

pub(super) fn to_pascal_case(value: &str) -> String {
    value
        .split(|ch: char| ch == '-' || ch == '_' || ch.is_whitespace())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect()
}

pub(super) fn dx_new_project_name(project_dir: &Path) -> String {
    project_dir
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or("dx-www-app")
        .to_string()
}

pub(super) fn toml_basic_string_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            ch => escaped.push(ch),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn pascal_case_normalizes_generator_names() {
        assert_eq!(to_pascal_case("my-component"), "MyComponent");
        assert_eq!(to_pascal_case("my_component"), "MyComponent");
        assert_eq!(to_pascal_case("mycomponent"), "Mycomponent");
        assert_eq!(to_pascal_case("my component"), "MyComponent");
    }

    #[test]
    fn dx_new_project_name_uses_safe_folder_name() {
        assert_eq!(
            dx_new_project_name(&PathBuf::from("launch-app")),
            "launch-app"
        );
        assert_eq!(dx_new_project_name(Path::new("   ")), "dx-www-app");
        assert_eq!(dx_new_project_name(Path::new("")), "dx-www-app");
    }

    #[test]
    fn toml_basic_string_escape_keeps_generated_dx_config_valid() {
        assert_eq!(
            toml_basic_string_escape("app \"one\"\npath\\tab\tend\r"),
            "app \\\"one\\\"\\npath\\\\tab\\tend\\r"
        );
    }
}
