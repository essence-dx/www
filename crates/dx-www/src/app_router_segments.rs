//! Shared App Router filesystem segment classification.

use std::collections::BTreeSet;

/// A classified Next-familiar App Router path segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AppRouteSegmentKind<'a> {
    /// A static URL segment.
    Static(&'a str),
    /// A dynamic URL segment such as `[id]`.
    Dynamic(&'a str),
    /// A required catch-all URL segment such as `[...slug]`.
    RequiredCatchAll(&'a str),
    /// An optional catch-all URL segment such as `[[...slug]]`.
    OptionalCatchAll(&'a str),
    /// A route group such as `(marketing)`.
    RouteGroup,
    /// A parallel route slot such as `@modal`.
    ParallelSlot,
    /// A private folder such as `_internal`.
    Private,
    /// An intercepting route segment such as `(.)photo`.
    Intercepting,
    /// A filesystem segment that is not a valid App Router segment.
    Malformed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnsupportedAppRouteSegmentReason {
    PrivateFolder,
    InterceptingRoute,
    MalformedSegment,
    DuplicateParamName,
    NonTerminalCatchAll,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UnsupportedAppRouteSegment {
    pub(crate) segment: Option<String>,
    pub(crate) reason: UnsupportedAppRouteSegmentReason,
}

/// Classify a single App Router source-path segment.
pub(crate) fn classify_app_route_segment(segment: &str) -> AppRouteSegmentKind<'_> {
    if is_intercepting_app_route_segment(segment) {
        return AppRouteSegmentKind::Intercepting;
    }
    if is_route_group_segment(segment) {
        return AppRouteSegmentKind::RouteGroup;
    }
    if is_parallel_route_slot_segment(segment) {
        return AppRouteSegmentKind::ParallelSlot;
    }
    if is_private_app_folder_segment(segment) {
        return AppRouteSegmentKind::Private;
    }
    if let Some(name) = optional_catchall_segment_name(segment) {
        return AppRouteSegmentKind::OptionalCatchAll(name);
    }
    if let Some(name) = catchall_segment_name(segment) {
        return AppRouteSegmentKind::RequiredCatchAll(name);
    }
    if let Some(name) = dynamic_segment_name(segment) {
        return AppRouteSegmentKind::Dynamic(name);
    }
    if has_mismatched_route_group_delimiters(segment)
        || segment == "@"
        || segment.starts_with('[')
        || segment.ends_with(']')
    {
        return AppRouteSegmentKind::Malformed;
    }
    AppRouteSegmentKind::Static(segment)
}

/// Return whether a segment is an App Router route group.
pub(crate) fn is_route_group_segment(segment: &str) -> bool {
    if is_intercepting_app_route_segment(segment) {
        return false;
    }
    segment
        .strip_prefix('(')
        .and_then(|segment| segment.strip_suffix(')'))
        .is_some_and(valid_route_group_name)
}

/// Return whether a segment is a private App Router folder.
pub(crate) fn is_private_app_folder_segment(segment: &str) -> bool {
    segment.len() > 1 && segment.starts_with('_')
}

/// Return whether a segment is an App Router intercepting route.
pub(crate) fn is_intercepting_app_route_segment(segment: &str) -> bool {
    segment.starts_with("(.)") || segment.starts_with("(..)") || segment.starts_with("(...")
}

/// Return whether a segment is a parallel route slot.
pub(crate) fn is_parallel_route_slot_segment(segment: &str) -> bool {
    segment.len() > 1 && segment.starts_with('@')
}

fn has_mismatched_route_group_delimiters(segment: &str) -> bool {
    (segment.starts_with('(') || segment.ends_with(')'))
        && !is_intercepting_app_route_segment(segment)
        && !is_route_group_segment(segment)
}

/// Return whether a segment is organizational and not visible in the URL path.
pub(crate) fn is_app_router_non_path_segment(segment: &str) -> bool {
    is_route_group_segment(segment) || is_parallel_route_slot_segment(segment)
}

/// Return whether a bracketed segment is malformed.
pub(crate) fn is_malformed_app_route_parameter_segment(segment: &str) -> bool {
    matches!(
        classify_app_route_segment(segment),
        AppRouteSegmentKind::Malformed
    )
}

/// Return whether a page route contains unsupported App Router filesystem shape.
pub(crate) fn has_unsupported_app_page_route_segments(segments: &[String]) -> bool {
    unsupported_app_page_route_segment(segments).is_some()
}

pub(crate) fn unsupported_app_page_route_segment(
    segments: &[String],
) -> Option<UnsupportedAppRouteSegment> {
    for segment in segments {
        match classify_app_route_segment(segment) {
            AppRouteSegmentKind::Private => {
                return Some(UnsupportedAppRouteSegment {
                    segment: Some(segment.clone()),
                    reason: UnsupportedAppRouteSegmentReason::PrivateFolder,
                });
            }
            AppRouteSegmentKind::Intercepting => {
                return Some(UnsupportedAppRouteSegment {
                    segment: Some(segment.clone()),
                    reason: UnsupportedAppRouteSegmentReason::InterceptingRoute,
                });
            }
            AppRouteSegmentKind::Malformed => {
                return Some(UnsupportedAppRouteSegment {
                    segment: Some(segment.clone()),
                    reason: UnsupportedAppRouteSegmentReason::MalformedSegment,
                });
            }
            AppRouteSegmentKind::Static(_)
            | AppRouteSegmentKind::Dynamic(_)
            | AppRouteSegmentKind::RequiredCatchAll(_)
            | AppRouteSegmentKind::OptionalCatchAll(_)
            | AppRouteSegmentKind::RouteGroup
            | AppRouteSegmentKind::ParallelSlot => {}
        }
    }

    let mut param_names = BTreeSet::new();
    for segment in segments {
        if route_segment_param_name(segment).is_some_and(|name| !param_names.insert(name)) {
            return Some(UnsupportedAppRouteSegment {
                segment: Some(segment.clone()),
                reason: UnsupportedAppRouteSegmentReason::DuplicateParamName,
            });
        }
    }

    let mut catch_all_seen = false;
    for segment in segments {
        match classify_app_route_segment(segment) {
            AppRouteSegmentKind::RouteGroup | AppRouteSegmentKind::ParallelSlot => continue,
            AppRouteSegmentKind::OptionalCatchAll(_) | AppRouteSegmentKind::RequiredCatchAll(_) => {
                if catch_all_seen {
                    return Some(UnsupportedAppRouteSegment {
                        segment: Some(segment.clone()),
                        reason: UnsupportedAppRouteSegmentReason::NonTerminalCatchAll,
                    });
                }
                catch_all_seen = true;
            }
            AppRouteSegmentKind::Static(_) | AppRouteSegmentKind::Dynamic(_) => {
                if catch_all_seen {
                    return Some(UnsupportedAppRouteSegment {
                        segment: Some(segment.clone()),
                        reason: UnsupportedAppRouteSegmentReason::NonTerminalCatchAll,
                    });
                }
            }
            AppRouteSegmentKind::Private
            | AppRouteSegmentKind::Intercepting
            | AppRouteSegmentKind::Malformed => {}
        }
    }

    None
}

/// Return whether a page route repeats a dynamic or catch-all parameter name.
pub(crate) fn route_segments_have_duplicate_param_names(segments: &[String]) -> bool {
    let mut param_names = BTreeSet::new();
    segments.iter().any(|segment| {
        route_segment_param_name(segment).is_some_and(|name| !param_names.insert(name))
    })
}

/// Extract the parameter name from a dynamic, required catch-all, or optional catch-all segment.
pub(crate) fn route_segment_param_name(segment: &str) -> Option<&str> {
    match classify_app_route_segment(segment) {
        AppRouteSegmentKind::OptionalCatchAll(name)
        | AppRouteSegmentKind::RequiredCatchAll(name)
        | AppRouteSegmentKind::Dynamic(name) => Some(name),
        AppRouteSegmentKind::Static(_)
        | AppRouteSegmentKind::RouteGroup
        | AppRouteSegmentKind::ParallelSlot
        | AppRouteSegmentKind::Private
        | AppRouteSegmentKind::Intercepting
        | AppRouteSegmentKind::Malformed => None,
    }
}

/// Return whether a catch-all segment is followed by another public path segment.
pub(crate) fn route_segments_have_nonterminal_catch_all(segments: &[String]) -> bool {
    let mut catch_all_seen = false;
    for segment in segments {
        match classify_app_route_segment(segment) {
            AppRouteSegmentKind::RouteGroup | AppRouteSegmentKind::ParallelSlot => continue,
            AppRouteSegmentKind::OptionalCatchAll(_) | AppRouteSegmentKind::RequiredCatchAll(_) => {
                if catch_all_seen {
                    return true;
                }
                catch_all_seen = true;
            }
            AppRouteSegmentKind::Static(_) | AppRouteSegmentKind::Dynamic(_) => {
                if catch_all_seen {
                    return true;
                }
            }
            AppRouteSegmentKind::Private
            | AppRouteSegmentKind::Intercepting
            | AppRouteSegmentKind::Malformed => {}
        }
    }
    false
}

/// Extract a dynamic parameter name from `[name]`.
pub(crate) fn dynamic_segment_name(segment: &str) -> Option<&str> {
    segment
        .strip_prefix('[')
        .and_then(|segment| segment.strip_suffix(']'))
        .filter(|name| {
            valid_app_route_param_name(name)
                && !name.starts_with("...")
                && !name.starts_with("[...")
        })
}

/// Extract a required catch-all parameter name from `[...name]`.
pub(crate) fn catchall_segment_name(segment: &str) -> Option<&str> {
    segment
        .strip_prefix("[...")
        .and_then(|segment| segment.strip_suffix(']'))
        .filter(|name| valid_app_route_param_name(name))
}

/// Extract an optional catch-all parameter name from `[[...name]]`.
pub(crate) fn optional_catchall_segment_name(segment: &str) -> Option<&str> {
    segment
        .strip_prefix("[[...")
        .and_then(|segment| segment.strip_suffix("]]"))
        .filter(|name| valid_app_route_param_name(name))
}

fn valid_app_route_param_name(name: &str) -> bool {
    !name.is_empty() && !name.contains('[') && !name.contains(']')
}

fn valid_route_group_name(name: &str) -> bool {
    !name.is_empty() && !name.contains('(') && !name.contains(')')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_next_familiar_app_route_segments() {
        assert_eq!(
            classify_app_route_segment("dashboard"),
            AppRouteSegmentKind::Static("dashboard")
        );
        assert_eq!(
            classify_app_route_segment("[team]"),
            AppRouteSegmentKind::Dynamic("team")
        );
        assert_eq!(
            classify_app_route_segment("[...slug]"),
            AppRouteSegmentKind::RequiredCatchAll("slug")
        );
        assert_eq!(
            classify_app_route_segment("[[...slug]]"),
            AppRouteSegmentKind::OptionalCatchAll("slug")
        );
        assert_eq!(
            classify_app_route_segment("(marketing)"),
            AppRouteSegmentKind::RouteGroup
        );
        assert_eq!(
            classify_app_route_segment("@modal"),
            AppRouteSegmentKind::ParallelSlot
        );
        assert_eq!(
            classify_app_route_segment("_internal"),
            AppRouteSegmentKind::Private
        );
        assert_eq!(
            classify_app_route_segment("(.)photo"),
            AppRouteSegmentKind::Intercepting
        );
        assert_eq!(
            classify_app_route_segment("[[id]]"),
            AppRouteSegmentKind::Malformed
        );
        assert_eq!(
            classify_app_route_segment("[...]"),
            AppRouteSegmentKind::Malformed
        );
        assert_eq!(
            classify_app_route_segment("()"),
            AppRouteSegmentKind::Malformed
        );
        assert_eq!(
            classify_app_route_segment("(marketing"),
            AppRouteSegmentKind::Malformed
        );
        assert_eq!(
            classify_app_route_segment("marketing)"),
            AppRouteSegmentKind::Malformed
        );
        assert_eq!(
            classify_app_route_segment("@"),
            AppRouteSegmentKind::Malformed
        );
    }

    #[test]
    fn rejects_unsupported_app_page_route_segment_shapes() {
        let segments = |values: &[&str]| {
            values
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
        };

        assert!(has_unsupported_app_page_route_segments(&segments(&[
            "_private"
        ])));
        assert!(has_unsupported_app_page_route_segments(&segments(&[
            "(.)photo"
        ])));
        assert!(has_unsupported_app_page_route_segments(&segments(&[
            "docs", "[...]"
        ])));
        assert!(has_unsupported_app_page_route_segments(&segments(&[
            "[team]", "[team]"
        ])));
        assert!(has_unsupported_app_page_route_segments(&segments(&[
            "docs",
            "[...slug]",
            "details"
        ])));
        assert!(has_unsupported_app_page_route_segments(&segments(&[
            "files",
            "[[...path]]",
            "preview"
        ])));
        assert!(has_unsupported_app_page_route_segments(&segments(&[
            "[...slug]",
            "[[...rest]]"
        ])));
        assert!(has_unsupported_app_page_route_segments(&segments(&["()"])));
        assert!(has_unsupported_app_page_route_segments(&segments(&[
            "(marketing"
        ])));
        assert!(has_unsupported_app_page_route_segments(&segments(&[
            "marketing)"
        ])));
        assert!(has_unsupported_app_page_route_segments(&segments(&["@"])));

        assert!(!has_unsupported_app_page_route_segments(&segments(&[
            "docs",
            "[...slug]",
            "(guide)"
        ])));
        assert!(!has_unsupported_app_page_route_segments(&segments(&[
            "files",
            "[category]",
            "[[...path]]"
        ])));
        assert!(!has_unsupported_app_page_route_segments(&segments(&[
            "(shop)", "@modal", "products", "[id]"
        ])));
    }

    #[test]
    fn reports_unsupported_app_page_route_segment_reasons() {
        let segments = |values: &[&str]| {
            values
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
        };
        let reason_for = |values: &[&str]| {
            unsupported_app_page_route_segment(&segments(values))
                .expect("unsupported route segment diagnostic")
        };

        let malformed = reason_for(&["()"]);
        assert_eq!(
            malformed.reason,
            UnsupportedAppRouteSegmentReason::MalformedSegment
        );
        assert_eq!(malformed.segment.as_deref(), Some("()"));

        let duplicate = reason_for(&["[team]", "[team]"]);
        assert_eq!(
            duplicate.reason,
            UnsupportedAppRouteSegmentReason::DuplicateParamName
        );
        assert_eq!(duplicate.segment.as_deref(), Some("[team]"));

        let nonterminal = reason_for(&["docs", "[...slug]", "details"]);
        assert_eq!(
            nonterminal.reason,
            UnsupportedAppRouteSegmentReason::NonTerminalCatchAll
        );
        assert_eq!(nonterminal.segment.as_deref(), Some("details"));

        let private = reason_for(&["_private"]);
        assert_eq!(
            private.reason,
            UnsupportedAppRouteSegmentReason::PrivateFolder
        );
        assert_eq!(private.segment.as_deref(), Some("_private"));

        let intercepting = reason_for(&["(.)photo"]);
        assert_eq!(
            intercepting.reason,
            UnsupportedAppRouteSegmentReason::InterceptingRoute
        );
        assert_eq!(intercepting.segment.as_deref(), Some("(.)photo"));

        assert!(
            unsupported_app_page_route_segment(&segments(&[
                "(shop)", "@modal", "products", "[id]"
            ]))
            .is_none()
        );
    }
}
