//! # Route Pattern Matching
//!
//! This module implements route pattern parsing and matching for dynamic routes.

use std::collections::HashMap;

// =============================================================================
// Route Pattern
// =============================================================================

/// A parsed route pattern that can match URL paths.
#[derive(Debug, Clone)]
pub struct RoutePattern {
    /// The original pattern string
    pub pattern: String,

    /// Parsed segments
    pub segments: Vec<RouteSegment>,

    /// Whether this is a catch-all pattern
    pub is_catch_all: bool,
}

/// A segment of a route pattern.
#[derive(Debug, Clone, PartialEq)]
pub enum RouteSegment {
    /// Static segment (exact match)
    Static(String),

    /// Dynamic parameter segment (:param)
    Param(String),

    /// Catch-all segment (*param)
    CatchAll(String),

    /// Required catch-all segment (+param)
    RequiredCatchAll(String),
}

impl RoutePattern {
    /// Parse a route pattern string.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let pattern = RoutePattern::parse("/user/:id");
    /// let pattern = RoutePattern::parse("/docs/*slug");
    /// ```
    pub fn parse(pattern: &str) -> Self {
        let mut segments = Vec::new();
        let mut is_catch_all = false;

        for part in pattern.split('/').filter(|s| !s.is_empty()) {
            let segment = if let Some(name) = part.strip_prefix(':') {
                RouteSegment::Param(name.to_string())
            } else if let Some(name) = part.strip_prefix('+') {
                is_catch_all = true;
                RouteSegment::RequiredCatchAll(name.to_string())
            } else if let Some(name) = part.strip_prefix('*') {
                is_catch_all = true;
                RouteSegment::CatchAll(name.to_string())
            } else {
                RouteSegment::Static(part.to_string())
            };
            segments.push(segment);
        }

        Self {
            pattern: pattern.to_string(),
            segments,
            is_catch_all,
        }
    }

    /// Match a URL path against this pattern.
    ///
    /// Returns the extracted parameters if the path matches, None otherwise.
    pub fn match_path(&self, path: &str) -> Option<HashMap<String, String>> {
        let path_parts = normalized_request_path_segments(path);
        let mut params = HashMap::new();
        let mut path_idx = 0;

        for segment in &self.segments {
            match segment {
                RouteSegment::Static(expected) => {
                    if path_idx >= path_parts.len() {
                        return None;
                    }
                    if !path_parts[path_idx].eq_ignore_ascii_case(expected) {
                        return None;
                    }
                    path_idx += 1;
                }
                RouteSegment::Param(name) => {
                    if path_idx >= path_parts.len() {
                        return None;
                    }
                    params.insert(name.clone(), path_parts[path_idx].clone());
                    path_idx += 1;
                }
                RouteSegment::CatchAll(name) => {
                    params.insert(name.clone(), path_parts[path_idx..].join("/"));
                    return Some(params);
                }
                RouteSegment::RequiredCatchAll(name) => {
                    if path_idx >= path_parts.len() {
                        return None;
                    }
                    params.insert(name.clone(), path_parts[path_idx..].join("/"));
                    return Some(params);
                }
            }
        }

        // Check if we consumed all path parts (unless catch-all)
        if !self.is_catch_all && path_idx != path_parts.len() {
            return None;
        }

        Some(params)
    }

    /// Check if this pattern is static (no dynamic segments).
    pub fn is_static(&self) -> bool {
        self.segments
            .iter()
            .all(|s| matches!(s, RouteSegment::Static(_)))
    }

    /// Get the parameter names in this pattern.
    pub fn param_names(&self) -> Vec<&str> {
        self.segments
            .iter()
            .filter_map(|s| match s {
                RouteSegment::Param(name)
                | RouteSegment::CatchAll(name)
                | RouteSegment::RequiredCatchAll(name) => Some(name.as_str()),
                _ => None,
            })
            .collect()
    }

    /// Calculate the specificity of this pattern (more specific = higher score).
    ///
    /// Used for ordering routes when multiple could match.
    /// Static segments are more specific than dynamic ones, and we also
    /// penalize longer paths to prefer shorter exact matches.
    pub fn specificity(&self) -> u32 {
        let mut static_count = 0u32;
        let mut param_count = 0u32;
        let mut has_catch_all = false;

        for segment in &self.segments {
            match segment {
                RouteSegment::Static(_) => static_count += 1,
                RouteSegment::Param(_) => param_count += 1,
                RouteSegment::CatchAll(_) | RouteSegment::RequiredCatchAll(_) => {
                    has_catch_all = true
                }
            }
        }

        // Higher score = more specific
        // - Catch-all routes are least specific (score starts at 0)
        // - Dynamic segments reduce specificity
        // - Static segments increase specificity
        // Formula: prioritize static-only routes, then penalize dynamic segments
        if has_catch_all {
            // Catch-all is least specific
            1
        } else if param_count > 0 {
            // Routes with params are less specific than pure static
            // But more static segments = more specific within dynamic routes
            100 + static_count * 10 - param_count
        } else {
            // Pure static routes are most specific
            // Shorter static paths are actually more specific for exact matching
            1000 + static_count * 10
        }
    }
}

fn normalized_request_path_segments(path: &str) -> Vec<String> {
    strip_request_suffix(path)
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(decode_path_segment)
        .collect()
}

fn strip_request_suffix(path: &str) -> &str {
    let query_index = path.find('?');
    let fragment_index = path.find('#');
    let end = match (query_index, fragment_index) {
        (Some(query), Some(fragment)) => query.min(fragment),
        (Some(query), None) => query,
        (None, Some(fragment)) => fragment,
        (None, None) => path.len(),
    };
    &path[..end]
}

fn decode_path_segment(part: &str) -> String {
    let bytes = part.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            b'%' if index + 2 < bytes.len() => {
                if let Some(byte) = decode_hex_pair(bytes[index + 1], bytes[index + 2]) {
                    decoded.push(byte);
                    index += 3;
                    continue;
                }
                decoded.push(bytes[index]);
                index += 1;
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }

    String::from_utf8_lossy(&decoded).into_owned()
}

fn decode_hex_pair(high: u8, low: u8) -> Option<u8> {
    Some(hex_value(high)? << 4 | hex_value(low)?)
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_static_pattern() {
        let pattern = RoutePattern::parse("/about");
        assert_eq!(pattern.segments.len(), 1);
        assert!(matches!(&pattern.segments[0], RouteSegment::Static(s) if s == "about"));
        assert!(pattern.is_static());
    }

    #[test]
    fn test_parse_dynamic_pattern() {
        let pattern = RoutePattern::parse("/user/:id");
        assert_eq!(pattern.segments.len(), 2);
        assert!(matches!(&pattern.segments[0], RouteSegment::Static(s) if s == "user"));
        assert!(matches!(&pattern.segments[1], RouteSegment::Param(s) if s == "id"));
        assert!(!pattern.is_static());
    }

    #[test]
    fn test_parse_catch_all_pattern() {
        let pattern = RoutePattern::parse("/docs/*slug");
        assert_eq!(pattern.segments.len(), 2);
        assert!(matches!(&pattern.segments[0], RouteSegment::Static(s) if s == "docs"));
        assert!(matches!(&pattern.segments[1], RouteSegment::CatchAll(s) if s == "slug"));
        assert!(pattern.is_catch_all);
    }

    #[test]
    fn test_parse_required_catch_all_pattern() {
        let pattern = RoutePattern::parse("/docs/+slug");
        assert_eq!(pattern.segments.len(), 2);
        assert!(matches!(&pattern.segments[0], RouteSegment::Static(s) if s == "docs"));
        assert!(matches!(&pattern.segments[1], RouteSegment::RequiredCatchAll(s) if s == "slug"));
        assert!(pattern.is_catch_all);
    }

    #[test]
    fn test_match_static_path() {
        let pattern = RoutePattern::parse("/about");
        assert!(pattern.match_path("/about").is_some());
        assert!(pattern.match_path("/other").is_none());
    }

    #[test]
    fn test_match_dynamic_path() {
        let pattern = RoutePattern::parse("/user/:id");

        let result = pattern.match_path("/user/123");
        assert!(result.is_some());
        let params = result.unwrap();
        assert_eq!(params.get("id"), Some(&"123".to_string()));

        assert!(pattern.match_path("/user").is_none());
        assert!(pattern.match_path("/user/123/extra").is_none());
    }

    #[test]
    fn test_match_catch_all_path() {
        let pattern = RoutePattern::parse("/docs/*slug");

        let result = pattern.match_path("/docs/getting-started/intro");
        assert!(result.is_some());
        let params = result.unwrap();
        assert_eq!(
            params.get("slug"),
            Some(&"getting-started/intro".to_string())
        );

        let result = pattern.match_path("/docs");
        assert!(result.is_some());
        let params = result.unwrap();
        assert_eq!(params.get("slug"), Some(&"".to_string()));
    }

    #[test]
    fn match_path_ignores_query_and_fragment_suffixes() {
        let pattern = RoutePattern::parse("/blog/:slug");

        let result = pattern.match_path("/blog/launch?preview=true#comments");
        assert!(result.is_some());
        assert_eq!(result.unwrap().get("slug"), Some(&"launch".to_string()));
    }

    #[test]
    fn match_path_decodes_dynamic_and_catch_all_params() {
        let dynamic = RoutePattern::parse("/users/:id");
        let result = dynamic.match_path("/users/alice%20ng");
        assert!(result.is_some());
        assert_eq!(result.unwrap().get("id"), Some(&"alice ng".to_string()));

        let catch_all = RoutePattern::parse("/docs/*slug");
        let result = catch_all.match_path("/docs/guide%20one/api%2Fintro?tab=read");
        assert!(result.is_some());
        assert_eq!(
            result.unwrap().get("slug"),
            Some(&"guide one/api/intro".to_string())
        );
    }

    #[test]
    fn match_path_distinguishes_required_and_optional_catch_all() {
        let optional = RoutePattern::parse("/docs/*slug");
        let result = optional.match_path("/docs");
        assert!(result.is_some());
        assert_eq!(result.unwrap().get("slug"), Some(&"".to_string()));

        let required = RoutePattern::parse("/docs/+slug");
        assert!(required.match_path("/docs").is_none());

        let result = required.match_path("/docs/getting-started/intro");
        assert!(result.is_some());
        assert_eq!(
            result.unwrap().get("slug"),
            Some(&"getting-started/intro".to_string())
        );
    }

    #[test]
    fn test_multiple_params() {
        let pattern = RoutePattern::parse("/user/:userId/post/:postId");

        let result = pattern.match_path("/user/42/post/123");
        assert!(result.is_some());
        let params = result.unwrap();
        assert_eq!(params.get("userId"), Some(&"42".to_string()));
        assert_eq!(params.get("postId"), Some(&"123".to_string()));
    }

    #[test]
    fn test_param_names() {
        let pattern = RoutePattern::parse("/user/:id/post/:postId");
        let names = pattern.param_names();
        assert_eq!(names, vec!["id", "postId"]);
    }

    #[test]
    fn test_specificity() {
        let static_pattern = RoutePattern::parse("/about");
        let dynamic_pattern = RoutePattern::parse("/user/:id");
        let catch_all_pattern = RoutePattern::parse("/docs/*slug");

        assert!(static_pattern.specificity() > dynamic_pattern.specificity());
        assert!(dynamic_pattern.specificity() > catch_all_pattern.specificity());
    }
}
