//! # Route Matcher
//!
//! Optimized route matching using a trie-based structure for fast lookups.

use std::collections::HashMap;

use super::Route;
use super::pattern::{RoutePattern, RouteSegment};

// =============================================================================
// Route Matcher
// =============================================================================

/// Optimized route matcher using a trie structure.
#[derive(Debug, Default)]
pub struct RouteMatcher {
    /// Root node of the trie
    root: TrieNode,
}

/// A node in the route trie.
#[derive(Debug, Default)]
struct TrieNode {
    /// Child nodes for static segments
    static_children: HashMap<String, Box<TrieNode>>,

    /// Child node for dynamic parameter
    param_child: Option<(String, Box<TrieNode>)>,

    /// Child node for catch-all
    catch_all_child: Option<(String, bool, Box<TrieNode>)>,

    /// Route index at this node (if any)
    route_index: Option<usize>,
}

impl RouteMatcher {
    /// Create a new route matcher.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build a matcher from a list of routes.
    pub fn from_routes(routes: &[Route]) -> Self {
        let mut matcher = Self::new();
        for (index, route) in routes.iter().enumerate() {
            matcher.insert(&route.pattern, index);
        }
        matcher
    }

    /// Insert a route pattern into the trie.
    pub fn insert(&mut self, pattern: &RoutePattern, route_index: usize) {
        let mut node = &mut self.root;

        for segment in &pattern.segments {
            node = match segment {
                RouteSegment::Static(s) => {
                    let s_lower = s.to_lowercase();
                    node.static_children.entry(s_lower).or_default()
                }
                RouteSegment::Param(name) => {
                    &mut node
                        .param_child
                        .get_or_insert_with(|| (name.clone(), Box::default()))
                        .1
                }
                RouteSegment::CatchAll(name) => {
                    &mut node
                        .catch_all_child
                        .get_or_insert_with(|| (name.clone(), true, Box::default()))
                        .2
                }
                RouteSegment::RequiredCatchAll(name) => {
                    &mut node
                        .catch_all_child
                        .get_or_insert_with(|| (name.clone(), false, Box::default()))
                        .2
                }
            };
        }

        node.route_index = Some(route_index);
    }

    /// Match a path and return the route index and extracted parameters.
    pub fn match_path(&self, path: &str) -> Option<(usize, HashMap<String, String>)> {
        let segments = normalized_request_path_segments(path);
        let mut params = HashMap::new();

        Self::match_segments(&self.root, &segments, 0, &mut params)
    }

    /// Recursively match path segments against the trie.
    fn match_segments(
        node: &TrieNode,
        segments: &[String],
        index: usize,
        params: &mut HashMap<String, String>,
    ) -> Option<(usize, HashMap<String, String>)> {
        // If we've consumed all segments, check if this node has a route.
        if index >= segments.len() {
            if let Some(route_index) = node.route_index {
                return Some((route_index, params.clone()));
            }
            if let Some((name, allow_empty, child)) = &node.catch_all_child
                && *allow_empty
            {
                params.insert(name.clone(), String::new());
                let result = child.route_index.map(|idx| (idx, params.clone()));
                params.remove(name);
                if result.is_some() {
                    return result;
                }
            }
            return None;
        }

        let segment = &segments[index];

        // Try static match first (most specific)
        if let Some(child) = node.static_children.get(&segment.to_lowercase()) {
            if let Some(result) = Self::match_segments(child, segments, index + 1, params) {
                return Some(result);
            }
        }

        // Try parameter match
        if let Some((name, child)) = &node.param_child {
            params.insert(name.clone(), segment.clone());
            if let Some(result) = Self::match_segments(child, segments, index + 1, params) {
                return Some(result);
            }
            params.remove(name);
        }

        // Try catch-all match
        if let Some((name, _allow_empty, child)) = &node.catch_all_child {
            params.insert(name.clone(), segments[index..].join("/"));
            if let Some(route_index) = child.route_index {
                return Some((route_index, params.clone()));
            }
            params.remove(name);
        }

        None
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
    fn test_static_matching() {
        let mut matcher = RouteMatcher::new();
        matcher.insert(&RoutePattern::parse("/about"), 0);
        matcher.insert(&RoutePattern::parse("/contact"), 1);

        let result = matcher.match_path("/about");
        assert!(result.is_some());
        assert_eq!(result.unwrap().0, 0);

        let result = matcher.match_path("/contact");
        assert!(result.is_some());
        assert_eq!(result.unwrap().0, 1);

        let result = matcher.match_path("/other");
        assert!(result.is_none());
    }

    #[test]
    fn test_dynamic_matching() {
        let mut matcher = RouteMatcher::new();
        matcher.insert(&RoutePattern::parse("/user/:id"), 0);

        let result = matcher.match_path("/user/123");
        assert!(result.is_some());
        let (index, params) = result.unwrap();
        assert_eq!(index, 0);
        assert_eq!(params.get("id"), Some(&"123".to_string()));
    }

    #[test]
    fn test_catch_all_matching() {
        let mut matcher = RouteMatcher::new();
        matcher.insert(&RoutePattern::parse("/docs/*slug"), 0);

        let result = matcher.match_path("/docs/getting-started/intro");
        assert!(result.is_some());
        let (index, params) = result.unwrap();
        assert_eq!(index, 0);
        assert_eq!(
            params.get("slug"),
            Some(&"getting-started/intro".to_string())
        );
    }

    #[test]
    fn matcher_distinguishes_required_and_optional_catch_all() {
        let mut optional = RouteMatcher::new();
        optional.insert(&RoutePattern::parse("/docs/*slug"), 0);
        let result = optional.match_path("/docs");
        assert!(result.is_some());
        let (_index, params) = result.unwrap();
        assert_eq!(params.get("slug"), Some(&"".to_string()));

        let mut required = RouteMatcher::new();
        required.insert(&RoutePattern::parse("/docs/+slug"), 1);
        assert!(required.match_path("/docs").is_none());

        let result = required.match_path("/docs/getting-started/intro");
        assert!(result.is_some());
        let (index, params) = result.unwrap();
        assert_eq!(index, 1);
        assert_eq!(
            params.get("slug"),
            Some(&"getting-started/intro".to_string())
        );
    }

    #[test]
    fn matcher_ignores_query_and_decodes_params() {
        let mut matcher = RouteMatcher::new();
        matcher.insert(&RoutePattern::parse("/blog/:slug"), 0);
        matcher.insert(&RoutePattern::parse("/docs/*slug"), 1);

        let (index, params) = matcher
            .match_path("/blog/launch%20notes?preview=true#comments")
            .expect("dynamic match");
        assert_eq!(index, 0);
        assert_eq!(params.get("slug"), Some(&"launch notes".to_string()));

        let (index, params) = matcher
            .match_path("/docs/guide%20one/api%2Fintro?tab=read")
            .expect("catch-all match");
        assert_eq!(index, 1);
        assert_eq!(params.get("slug"), Some(&"guide one/api/intro".to_string()));
    }

    #[test]
    fn test_priority_static_over_dynamic() {
        let mut matcher = RouteMatcher::new();
        matcher.insert(&RoutePattern::parse("/user/:id"), 0);
        matcher.insert(&RoutePattern::parse("/user/me"), 1);

        // Static should match over dynamic
        let result = matcher.match_path("/user/me");
        assert!(result.is_some());
        assert_eq!(result.unwrap().0, 1);

        // Dynamic should still work for other values
        let result = matcher.match_path("/user/123");
        assert!(result.is_some());
        assert_eq!(result.unwrap().0, 0);
    }

    #[test]
    fn test_nested_routes() {
        let mut matcher = RouteMatcher::new();
        matcher.insert(&RoutePattern::parse("/api/users"), 0);
        matcher.insert(&RoutePattern::parse("/api/users/:id"), 1);
        matcher.insert(&RoutePattern::parse("/api/users/:id/posts"), 2);

        let result = matcher.match_path("/api/users");
        assert_eq!(result.unwrap().0, 0);

        let result = matcher.match_path("/api/users/123");
        assert_eq!(result.unwrap().0, 1);

        let result = matcher.match_path("/api/users/123/posts");
        assert_eq!(result.unwrap().0, 2);
    }
}
