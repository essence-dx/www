pub(super) fn first_dx_add_subject<'a>(args: &'a [&str]) -> Option<&'a str> {
    let mut index = 0usize;
    while index < args.len() {
        match args[index] {
            "--project" => index += 2,
            "--write" | "--dry-run" => index += 1,
            value if value.starts_with('-') => index += 1,
            value => return Some(value),
        }
    }
    None
}

pub(super) fn is_source_owned_add_candidate(
    value: &str,
    canonical_value: &str,
    forge_launch_package_ids: &[&str],
) -> bool {
    value == "icon" || value.contains('/') || forge_launch_package_ids.contains(&canonical_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_dx_add_subject_skips_project_and_write_flags() {
        assert_eq!(
            first_dx_add_subject(&["--project", "demo", "--write", "ui/button"]),
            Some("ui/button")
        );
        assert_eq!(
            first_dx_add_subject(&["--dry-run", "--project", "demo", "forms"]),
            Some("forms")
        );
    }

    #[test]
    fn first_dx_add_subject_ignores_flag_only_args() {
        assert_eq!(
            first_dx_add_subject(&["--project", "demo", "--dry-run"]),
            None
        );
        assert_eq!(first_dx_add_subject(&["--write"]), None);
    }

    #[test]
    fn source_owned_add_candidate_accepts_icon_slash_and_known_canonical_ids() {
        let package_ids = ["forms/react-hook-form", "shadcn/ui/button"];

        assert!(is_source_owned_add_candidate("icon", "icon", &package_ids));
        assert!(is_source_owned_add_candidate(
            "ui/button",
            "shadcn/ui/button",
            &package_ids
        ));
        assert!(is_source_owned_add_candidate(
            "forms",
            "forms/react-hook-form",
            &package_ids
        ));
    }

    #[test]
    fn source_owned_add_candidate_rejects_plain_unknown_components() {
        let package_ids = ["forms/react-hook-form", "shadcn/ui/button"];

        assert!(!is_source_owned_add_candidate(
            "button",
            "button",
            &package_ids
        ));
    }
}
