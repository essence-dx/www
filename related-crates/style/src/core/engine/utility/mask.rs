//! Tailwind-familiar CSS mask utilities.
//!
//! Mask support is browser-sensitive, so generated rules emit both standard
//! and `-webkit-` prefixed properties for the public no-PostCSS path.

pub(super) fn mask_utility(class_name: &str) -> Option<String> {
    if let Some(css) = match class_name {
        "mask-none" => Some("-webkit-mask-image: none; mask-image: none".to_string()),
        "mask-add" => Some(prefixed_mask_composite_property("add", "source-over")),
        "mask-subtract" => Some(prefixed_mask_composite_property("subtract", "source-out")),
        "mask-intersect" => Some(prefixed_mask_composite_property("intersect", "source-in")),
        "mask-exclude" => Some(prefixed_mask_composite_property("exclude", "xor")),
        "mask-alpha" => Some(prefixed_mask_property("mask-mode", "alpha")),
        "mask-luminance" => Some(prefixed_mask_property("mask-mode", "luminance")),
        "mask-match" => Some(prefixed_mask_property("mask-mode", "match-source")),
        "mask-type-alpha" => Some("mask-type: alpha".to_string()),
        "mask-type-luminance" => Some("mask-type: luminance".to_string()),
        "mask-origin-border" => Some(prefixed_mask_property("mask-origin", "border-box")),
        "mask-origin-padding" => Some(prefixed_mask_property("mask-origin", "padding-box")),
        "mask-origin-content" => Some(prefixed_mask_property("mask-origin", "content-box")),
        "mask-origin-fill" => Some(prefixed_mask_property("mask-origin", "fill-box")),
        "mask-origin-stroke" => Some(prefixed_mask_property("mask-origin", "stroke-box")),
        "mask-origin-view" => Some(prefixed_mask_property("mask-origin", "view-box")),
        "mask-clip-border" => Some(prefixed_mask_property("mask-clip", "border-box")),
        "mask-clip-padding" => Some(prefixed_mask_property("mask-clip", "padding-box")),
        "mask-clip-content" => Some(prefixed_mask_property("mask-clip", "content-box")),
        "mask-clip-fill" => Some(prefixed_mask_property("mask-clip", "fill-box")),
        "mask-clip-stroke" => Some(prefixed_mask_property("mask-clip", "stroke-box")),
        "mask-clip-view" => Some(prefixed_mask_property("mask-clip", "view-box")),
        "mask-no-clip" => Some(prefixed_mask_property("mask-clip", "no-clip")),
        "mask-repeat" => Some(prefixed_mask_property("mask-repeat", "repeat")),
        "mask-no-repeat" => Some(prefixed_mask_property("mask-repeat", "no-repeat")),
        "mask-repeat-x" => Some(prefixed_mask_property("mask-repeat", "repeat-x")),
        "mask-repeat-y" => Some(prefixed_mask_property("mask-repeat", "repeat-y")),
        "mask-repeat-space" => Some(prefixed_mask_property("mask-repeat", "space")),
        "mask-repeat-round" => Some(prefixed_mask_property("mask-repeat", "round")),
        "mask-cover" => Some(prefixed_mask_property("mask-size", "cover")),
        "mask-contain" => Some(prefixed_mask_property("mask-size", "contain")),
        "mask-auto" => Some(prefixed_mask_property("mask-size", "auto")),
        "mask-center" => Some(prefixed_mask_property("mask-position", "center")),
        "mask-top" => Some(prefixed_mask_property("mask-position", "top")),
        "mask-bottom" => Some(prefixed_mask_property("mask-position", "bottom")),
        "mask-left" => Some(prefixed_mask_property("mask-position", "left")),
        "mask-right" => Some(prefixed_mask_property("mask-position", "right")),
        "mask-top-left" => Some(prefixed_mask_property("mask-position", "top left")),
        "mask-top-right" => Some(prefixed_mask_property("mask-position", "top right")),
        "mask-bottom-left" => Some(prefixed_mask_property("mask-position", "bottom left")),
        "mask-bottom-right" => Some(prefixed_mask_property("mask-position", "bottom right")),
        "mask-circle" => Some(radial_shape_rule("circle")),
        "mask-ellipse" => Some(radial_shape_rule("ellipse")),
        "mask-radial-closest-side" => Some(radial_size_rule("closest-side".to_string())),
        "mask-radial-closest-corner" => Some(radial_size_rule("closest-corner".to_string())),
        "mask-radial-farthest-side" => Some(radial_size_rule("farthest-side".to_string())),
        "mask-radial-farthest-corner" => Some(radial_size_rule("farthest-corner".to_string())),
        _ => None,
    } {
        return Some(css);
    }

    if let Some(raw_value) = class_name.strip_prefix("mask-radial-from-") {
        let value = mask_radial_stop_or_color(raw_value)?;
        return Some(radial_mask_rule(MaskStop::From, value));
    }

    if let Some(raw_value) = class_name.strip_prefix("mask-radial-to-") {
        let value = mask_radial_stop_or_color(raw_value)?;
        return Some(radial_mask_rule(MaskStop::To, value));
    }

    if let Some(raw_value) = class_name.strip_prefix("mask-radial-at-") {
        let position = mask_position_value(raw_value)?;
        return Some(radial_position_rule(position));
    }

    if let Some(raw_value) = class_name.strip_prefix("mask-radial-") {
        let value = mask_radial_size_value(raw_value)?;
        return Some(radial_size_rule(value));
    }

    if let Some(raw_value) = class_name.strip_prefix("mask-conic-from-") {
        let value = mask_conic_stop_or_color(raw_value)?;
        return Some(conic_mask_rule(value, "from"));
    }

    if let Some(raw_value) = class_name.strip_prefix("mask-conic-to-") {
        let value = mask_conic_stop_or_color(raw_value)?;
        return Some(conic_mask_rule(value, "to"));
    }

    if let Some(raw_value) = class_name.strip_prefix("mask-conic-") {
        let value = mask_angle_value(raw_value)?;
        return Some(format!(
            "--tw-mask-conic-angle: {}; {}",
            value,
            prefixed_mask_property("mask-image", &conic_mask_image())
        ));
    }

    if let Some((edge, stop, raw_value)) = parse_linear_edge_mask_class(class_name) {
        let value = mask_linear_stop_or_color(raw_value)?;
        return Some(linear_edge_mask_rule(edge, stop, value));
    }

    if let Some((axis, stop, raw_value)) = parse_linear_axis_mask_class(class_name) {
        let value = mask_linear_stop_or_color(raw_value)?;
        return Some(linear_axis_mask_rule(axis, stop, value));
    }

    if let Some(raw_value) = class_name.strip_prefix("mask-linear-from-") {
        let value = mask_linear_stop_or_color(raw_value)?;
        return Some(linear_mask_rule(MaskStop::From, value));
    }

    if let Some(raw_value) = class_name.strip_prefix("mask-linear-to-") {
        let value = mask_linear_stop_or_color(raw_value)?;
        return Some(linear_mask_rule(MaskStop::To, value));
    }

    if let Some(raw_value) = class_name.strip_prefix("-mask-linear-") {
        let value = mask_angle_value(raw_value)?;
        return Some(format!(
            "--tw-mask-linear-position: calc({} * -1); {}",
            value,
            prefixed_mask_property("mask-image", &linear_mask_image())
        ));
    }

    if let Some(raw_value) = class_name.strip_prefix("mask-linear-") {
        if let Some(value) = mask_linear_image_value(raw_value) {
            return Some(linear_mask_image_rule(value));
        }

        let value = mask_angle_value(raw_value)?;
        return Some(format!(
            "--tw-mask-linear-position: {}; {}",
            value,
            prefixed_mask_property("mask-image", &linear_mask_image())
        ));
    }

    if let Some(raw_value) = class_name.strip_prefix("mask-position-") {
        let value = mask_position_value(raw_value)?;
        return Some(prefixed_mask_property("mask-position", &value));
    }

    if let Some(raw_value) = class_name.strip_prefix("mask-size-") {
        let value = mask_generic_value(raw_value)?;
        return Some(prefixed_mask_property("mask-size", &value));
    }

    if let Some(value) = class_name.strip_prefix("mask-").and_then(mask_image_value) {
        return Some(prefixed_mask_property("mask-image", &value));
    }

    None
}

fn prefixed_mask_property(property: &str, value: &str) -> String {
    format!("-webkit-{property}: {value}; {property}: {value}")
}

fn prefixed_mask_composite_property(value: &str, webkit_value: &str) -> String {
    format!("-webkit-mask-composite: {webkit_value}; mask-composite: {value}")
}

fn mask_radial_stop_or_color(raw_value: &str) -> Option<MaskGradientValue> {
    if let Some(value) = mask_radial_stop(raw_value) {
        return Some(MaskGradientValue::Stop(value));
    }

    super::color_value(raw_value).map(MaskGradientValue::Color)
}

fn mask_radial_stop(raw_value: &str) -> Option<String> {
    if let Some(variable) = mask_variable_value(raw_value) {
        return Some(variable);
    }

    if let Some(value) = safe_arbitrary_value(raw_value) {
        return Some(value);
    }

    if let Some(numeric) = raw_value.strip_suffix('%') {
        let value = numeric.parse::<f32>().ok()?;
        if (0.0..=100.0).contains(&value) {
            return Some(format!("{}%", trim_float(value)));
        }
        return None;
    }

    let value = raw_value.parse::<f32>().ok()?;
    Some(format!("calc(var(--spacing) * {})", trim_float(value)))
}

fn mask_radial_size_value(raw_value: &str) -> Option<String> {
    if let Some(variable) = mask_variable_value(raw_value) {
        return Some(variable);
    }

    let value = safe_arbitrary_value(raw_value)?;
    if value.trim().is_empty() || value.contains(',') {
        return None;
    }
    Some(value)
}

fn radial_mask_rule(stop: MaskStop, value: MaskGradientValue) -> String {
    format!(
        "{} {}",
        linear_stop_declaration("radial", stop, value),
        prefixed_mask_property("mask-image", &radial_mask_image())
    )
}

fn radial_position_rule(position: String) -> String {
    format!("--tw-mask-radial-position: {}", position)
}

fn radial_shape_rule(shape: &str) -> String {
    format!(
        "--tw-mask-radial-shape: {}; {}",
        shape,
        prefixed_mask_property("mask-image", &radial_mask_image())
    )
}

fn radial_size_rule(value: String) -> String {
    format!(
        "--tw-mask-radial-size: {}; {}",
        value,
        prefixed_mask_property("mask-image", &radial_mask_image())
    )
}

fn radial_mask_image() -> String {
    "radial-gradient(var(--tw-mask-radial-shape, ellipse) var(--tw-mask-radial-size, farthest-corner) at var(--tw-mask-radial-position, center), var(--tw-mask-radial-from-color, #000) var(--tw-mask-radial-from, 0%), var(--tw-mask-radial-to-color, transparent) var(--tw-mask-radial-to, 100%))".to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MaskGradientValue {
    Stop(String),
    Color(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MaskStop {
    From,
    To,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MaskEdge {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MaskAxis {
    X,
    Y,
}

fn mask_conic_stop_or_color(raw_value: &str) -> Option<MaskGradientValue> {
    if let Some(value) = mask_conic_stop(raw_value) {
        return Some(MaskGradientValue::Stop(value));
    }

    super::color_value(raw_value).map(MaskGradientValue::Color)
}

fn mask_conic_stop(raw_value: &str) -> Option<String> {
    if let Some(variable) = mask_variable_value(raw_value) {
        return Some(variable);
    }

    if let Some(value) = safe_arbitrary_value(raw_value) {
        return mask_conic_literal_stop(&value);
    }

    mask_conic_literal_stop(raw_value)
}

fn mask_conic_literal_stop(value: &str) -> Option<String> {
    let numeric = value.strip_suffix('%').unwrap_or(value);
    if numeric
        .parse::<f32>()
        .ok()
        .is_some_and(|n| (0.0..=100.0).contains(&n))
    {
        return Some(format!("{numeric}%"));
    }

    if is_angle_value(value) {
        return Some(value.to_string());
    }

    None
}

fn mask_angle_value(raw_value: &str) -> Option<String> {
    if let Some(variable) = mask_variable_value(raw_value) {
        return Some(variable);
    }

    let value = safe_arbitrary_value(raw_value).unwrap_or_else(|| raw_value.to_string());
    if is_angle_value(&value) {
        return Some(value);
    }

    let angle = value.parse::<f32>().ok()?;
    Some(format!("{}deg", trim_float(angle)))
}

fn is_angle_value(value: &str) -> bool {
    let number = value
        .strip_suffix("deg")
        .or_else(|| value.strip_suffix("rad"))
        .or_else(|| value.strip_suffix("grad"))
        .or_else(|| value.strip_suffix("turn"));
    number.is_some_and(|raw| raw.parse::<f32>().is_ok())
}

fn conic_mask_rule(value: MaskGradientValue, stop: &str) -> String {
    let declaration = match (stop, value) {
        ("from", MaskGradientValue::Stop(value)) => format!("--tw-mask-conic-from: {value};"),
        ("from", MaskGradientValue::Color(value)) => {
            format!("--tw-mask-conic-from-color: {value};")
        }
        ("to", MaskGradientValue::Stop(value)) => format!("--tw-mask-conic-to: {value};"),
        ("to", MaskGradientValue::Color(value)) => format!("--tw-mask-conic-to-color: {value};"),
        _ => return String::new(),
    };

    format!(
        "{} {}",
        declaration,
        prefixed_mask_property("mask-image", &conic_mask_image())
    )
}

fn conic_mask_image() -> String {
    "conic-gradient(from var(--tw-mask-conic-angle, 0deg), var(--tw-mask-conic-from-color, #000) var(--tw-mask-conic-from, 0%), var(--tw-mask-conic-to-color, transparent) var(--tw-mask-conic-to, 100%))".to_string()
}

fn parse_linear_edge_mask_class(class_name: &str) -> Option<(MaskEdge, MaskStop, &str)> {
    for (prefix, edge, stop) in [
        ("mask-t-from-", MaskEdge::Top, MaskStop::From),
        ("mask-t-to-", MaskEdge::Top, MaskStop::To),
        ("mask-r-from-", MaskEdge::Right, MaskStop::From),
        ("mask-r-to-", MaskEdge::Right, MaskStop::To),
        ("mask-b-from-", MaskEdge::Bottom, MaskStop::From),
        ("mask-b-to-", MaskEdge::Bottom, MaskStop::To),
        ("mask-l-from-", MaskEdge::Left, MaskStop::From),
        ("mask-l-to-", MaskEdge::Left, MaskStop::To),
    ] {
        if let Some(raw_value) = class_name.strip_prefix(prefix) {
            return Some((edge, stop, raw_value));
        }
    }
    None
}

fn parse_linear_axis_mask_class(class_name: &str) -> Option<(MaskAxis, MaskStop, &str)> {
    for (prefix, axis, stop) in [
        ("mask-x-from-", MaskAxis::X, MaskStop::From),
        ("mask-x-to-", MaskAxis::X, MaskStop::To),
        ("mask-y-from-", MaskAxis::Y, MaskStop::From),
        ("mask-y-to-", MaskAxis::Y, MaskStop::To),
    ] {
        if let Some(raw_value) = class_name.strip_prefix(prefix) {
            return Some((axis, stop, raw_value));
        }
    }
    None
}

fn mask_linear_stop_or_color(raw_value: &str) -> Option<MaskGradientValue> {
    if let Some(value) = mask_linear_stop(raw_value) {
        return Some(MaskGradientValue::Stop(value));
    }

    super::color_value(raw_value).map(MaskGradientValue::Color)
}

fn mask_linear_stop(raw_value: &str) -> Option<String> {
    if let Some(variable) = mask_variable_value(raw_value) {
        return Some(variable);
    }

    if let Some(value) = safe_arbitrary_value(raw_value) {
        return Some(value);
    }

    if let Some(numeric) = raw_value.strip_suffix('%') {
        let value = numeric.parse::<f32>().ok()?;
        if (0.0..=100.0).contains(&value) {
            return Some(format!("{}%", trim_float(value)));
        }
        return None;
    }

    let value = raw_value.parse::<f32>().ok()?;
    Some(format!("calc(var(--spacing) * {})", trim_float(value)))
}

fn linear_edge_mask_rule(edge: MaskEdge, stop: MaskStop, value: MaskGradientValue) -> String {
    format!(
        "{} {}",
        linear_stop_declaration(edge.variable_stem(), stop, value),
        prefixed_mask_property("mask-image", &edge.linear_mask_image())
    )
}

fn linear_axis_mask_rule(axis: MaskAxis, stop: MaskStop, value: MaskGradientValue) -> String {
    let (first, second) = axis.edges();
    format!(
        "{} {} {} {}",
        linear_stop_declaration(first.variable_stem(), stop, value.clone()),
        linear_stop_declaration(second.variable_stem(), stop, value),
        prefixed_mask_property("mask-image", &axis.linear_mask_image()),
        prefixed_mask_composite_property("intersect", "source-in")
    )
}

fn linear_mask_rule(stop: MaskStop, value: MaskGradientValue) -> String {
    format!(
        "{} {}",
        linear_stop_declaration("linear", stop, value),
        prefixed_mask_property("mask-image", &linear_mask_image())
    )
}

fn linear_mask_image_rule(value: String) -> String {
    if value.starts_with("var(") {
        return prefixed_mask_property("mask-image", &value);
    }

    prefixed_mask_property("mask-image", &format!("linear-gradient({value})"))
}

fn linear_stop_declaration(stem: &str, stop: MaskStop, value: MaskGradientValue) -> String {
    match (stop, value) {
        (MaskStop::From, MaskGradientValue::Stop(value)) => {
            format!("--tw-mask-{stem}-from: {value};")
        }
        (MaskStop::From, MaskGradientValue::Color(value)) => {
            format!("--tw-mask-{stem}-from-color: {value};")
        }
        (MaskStop::To, MaskGradientValue::Stop(value)) => {
            format!("--tw-mask-{stem}-to: {value};")
        }
        (MaskStop::To, MaskGradientValue::Color(value)) => {
            format!("--tw-mask-{stem}-to-color: {value};")
        }
    }
}

impl MaskEdge {
    fn variable_stem(self) -> &'static str {
        match self {
            MaskEdge::Top => "top",
            MaskEdge::Right => "right",
            MaskEdge::Bottom => "bottom",
            MaskEdge::Left => "left",
        }
    }

    fn direction(self) -> &'static str {
        match self {
            MaskEdge::Top => "to top",
            MaskEdge::Right => "to right",
            MaskEdge::Bottom => "to bottom",
            MaskEdge::Left => "to left",
        }
    }

    fn linear_mask_image(self) -> String {
        let stem = self.variable_stem();
        format!(
            "linear-gradient({}, var(--tw-mask-{stem}-from-color, #000) var(--tw-mask-{stem}-from, 0%), var(--tw-mask-{stem}-to-color, transparent) var(--tw-mask-{stem}-to, 100%))",
            self.direction()
        )
    }
}

impl MaskAxis {
    fn edges(self) -> (MaskEdge, MaskEdge) {
        match self {
            MaskAxis::X => (MaskEdge::Left, MaskEdge::Right),
            MaskAxis::Y => (MaskEdge::Top, MaskEdge::Bottom),
        }
    }

    fn linear_mask_image(self) -> String {
        let (first, second) = self.edges();
        format!(
            "{}, {}",
            first.linear_mask_image(),
            second.linear_mask_image()
        )
    }
}

fn linear_mask_image() -> String {
    "linear-gradient(var(--tw-mask-linear-position, 180deg), var(--tw-mask-linear-from-color, #000) var(--tw-mask-linear-from, 0%), var(--tw-mask-linear-to-color, transparent) var(--tw-mask-linear-to, 100%))".to_string()
}

fn mask_linear_image_value(raw_value: &str) -> Option<String> {
    if let Some(value) = mask_variable_value(raw_value) {
        return Some(value);
    }

    let value = safe_arbitrary_value(raw_value)?;
    if value.trim().is_empty() || !value.contains(',') {
        return None;
    }
    Some(value)
}

fn mask_image_value(raw_value: &str) -> Option<String> {
    mask_variable_value(raw_value).or_else(|| safe_arbitrary_value(raw_value))
}

fn mask_position_value(raw_value: &str) -> Option<String> {
    if let Some(value) = safe_arbitrary_value(raw_value) {
        return Some(value);
    }

    Some(
        match raw_value {
            "center" => "center",
            "top" => "top",
            "bottom" => "bottom",
            "left" => "left",
            "right" => "right",
            "top-left" => "top left",
            "top-right" => "top right",
            "bottom-left" => "bottom left",
            "bottom-right" => "bottom right",
            "left-top" => "left top",
            "left-bottom" => "left bottom",
            "right-top" => "right top",
            "right-bottom" => "right bottom",
            _ => return None,
        }
        .to_string(),
    )
}

fn mask_generic_value(raw_value: &str) -> Option<String> {
    mask_variable_value(raw_value).or_else(|| safe_arbitrary_value(raw_value))
}

fn mask_variable_value(raw_value: &str) -> Option<String> {
    let variable = raw_value
        .strip_prefix('(')
        .and_then(|raw| raw.strip_suffix(')'))?;
    if variable.starts_with("--")
        && variable
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'))
    {
        return Some(format!("var({variable})"));
    }
    None
}

fn safe_arbitrary_value(raw_value: &str) -> Option<String> {
    let value = raw_value.strip_prefix('[')?.strip_suffix(']')?;
    if value.chars().any(|ch| matches!(ch, '{' | '}' | ';')) {
        return None;
    }
    Some(value.replace('_', " "))
}

fn trim_float(value: f32) -> String {
    let mut out = format!("{value:.4}");
    while out.contains('.') && out.ends_with('0') {
        out.pop();
    }
    if out.ends_with('.') {
        out.pop();
    }
    out
}
