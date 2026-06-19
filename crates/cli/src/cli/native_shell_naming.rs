pub(super) fn dx_project_name(dx: &str) -> Option<String> {
    if let Some(project) = dx.find("project(") {
        let after_project = &dx[project..];
        let name = after_project.find("name=\"")? + "name=\"".len();
        let after_name = &after_project[name..];
        let end = after_name.find('"')?;
        return Some(after_name[..end].to_string());
    }

    let setting = dx.find("project.name=\"")? + "project.name=\"".len();
    let after_name = &dx[setting..];
    let end = after_name.find('"')?;
    Some(after_name[..end].to_string())
}

pub(super) fn title_case_project_name(name: &str) -> String {
    name.split(['-', '_', ' '])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn identifier_suffix(name: &str) -> String {
    kebab_case(name).replace('-', ".")
}

pub(super) fn native_lib_name(project_name: &str) -> String {
    format!("{}_native_lib", kebab_case(project_name).replace('-', "_"))
}

pub(super) fn kebab_case(value: &str) -> String {
    let mut output = String::new();
    let mut previous_dash = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            output.push(character.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash {
            output.push('-');
            previous_dash = true;
        }
    }
    output.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::{dx_project_name, identifier_suffix, native_lib_name, title_case_project_name};

    #[test]
    fn parses_project_name_from_function_and_setting_forms() {
        assert_eq!(
            dx_project_name(r#"project(name="dx-mobile-app")"#).as_deref(),
            Some("dx-mobile-app")
        );
        assert_eq!(
            dx_project_name(r#"project.name="dx-mobile-app""#).as_deref(),
            Some("dx-mobile-app")
        );
    }

    #[test]
    fn derives_native_names_from_project_name() {
        assert_eq!(title_case_project_name("dx-mobile-app"), "Dx Mobile App");
        assert_eq!(identifier_suffix("dx-mobile-app"), "dx.mobile.app");
        assert_eq!(native_lib_name("dx-mobile-app"), "dx_mobile_app_native_lib");
    }
}
