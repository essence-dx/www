use std::collections::BTreeMap;

use super::graph::{SourceBuildRoute, hash_bytes};

pub(super) fn source_route_output_slugs(routes: &[SourceBuildRoute]) -> BTreeMap<String, String> {
    let mut base_counts = BTreeMap::new();
    for route in routes {
        *base_counts
            .entry(source_route_slug(&route.route))
            .or_insert(0usize) += 1;
    }

    routes
        .iter()
        .map(|route| {
            let base = source_route_slug(&route.route);
            let slug = if base_counts.get(&base).copied().unwrap_or(0) > 1 {
                format!("{base}--{}", hash_bytes(route.path.as_bytes()))
            } else {
                base
            };
            (source_route_key(route), slug)
        })
        .collect()
}

pub(super) fn source_route_key(route: &SourceBuildRoute) -> String {
    format!("{}\0{}", route.route, route.path)
}

pub(super) fn source_route_slug(route: &str) -> String {
    let slug = route
        .trim_matches('/')
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();

    if slug.is_empty() {
        "root".to_string()
    } else {
        slug
    }
}
