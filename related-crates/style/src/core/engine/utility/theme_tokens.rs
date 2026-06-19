pub(super) fn theme_color_token_name(raw_value: &str) -> Option<&'static str> {
    Some(match raw_value {
        "background" => "background",
        "foreground" => "foreground",
        "surface" => "surface",
        "card" => "card",
        "card-foreground" => "card-foreground",
        "popover" => "popover",
        "popover-foreground" => "popover-foreground",
        "primary" => "primary",
        "primary-foreground" => "primary-foreground",
        "secondary" => "secondary",
        "secondary-foreground" => "secondary-foreground",
        "muted" => "muted",
        "muted-foreground" => "muted-foreground",
        "accent" => "accent",
        "accent-foreground" => "accent-foreground",
        "destructive" => "destructive",
        "destructive-foreground" => "destructive-foreground",
        "border" => "border",
        "input" => "input",
        "ring" => "ring",
        "chart-1" => "chart-1",
        "chart-2" => "chart-2",
        "chart-3" => "chart-3",
        "chart-4" => "chart-4",
        "chart-5" => "chart-5",
        "success" => "success",
        "success-foreground" => "success-foreground",
        "warning" => "warning",
        "warning-foreground" => "warning-foreground",
        "sidebar" => "sidebar",
        "sidebar-foreground" => "sidebar-foreground",
        "sidebar-primary" => "sidebar-primary",
        "sidebar-primary-foreground" => "sidebar-primary-foreground",
        "sidebar-accent" => "sidebar-accent",
        "sidebar-accent-foreground" => "sidebar-accent-foreground",
        "sidebar-border" => "sidebar-border",
        "sidebar-ring" => "sidebar-ring",
        _ => return None,
    })
}

pub(super) fn is_color_like_custom_property_reference(value: &str) -> bool {
    let Some(variable) = value
        .trim()
        .strip_prefix("var(")
        .and_then(|value| value.strip_suffix(')'))
        .and_then(|value| value.split(',').next())
        .map(str::trim)
    else {
        return false;
    };

    if !is_custom_property_name(variable) {
        return false;
    }

    let mut color_like = false;
    for part in variable.trim_start_matches("--").split(['-', '_']) {
        let part = part.to_ascii_lowercase();
        match part.as_str() {
            "width" | "height" | "size" | "length" | "offset" | "radius" | "spacing" | "gap"
            | "padding" | "margin" | "inset" => return false,
            "color" | "bg" | "background" | "foreground" | "fg" | "text" | "border" | "ring"
            | "outline" | "accent" | "primary" | "secondary" | "muted" | "destructive"
            | "success" | "warning" | "surface" | "card" | "popover" | "focus" => color_like = true,
            _ => {}
        }
    }

    color_like
}

fn is_custom_property_name(value: &str) -> bool {
    value
        .strip_prefix("--")
        .is_some_and(|name| !name.is_empty() && name.chars().all(is_custom_property_char))
}

fn is_custom_property_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'
}
