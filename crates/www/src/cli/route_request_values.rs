use std::collections::BTreeMap;

pub(super) fn decode_path_segment(value: &str) -> String {
    percent_decode_component(value, false)
}

pub(super) fn decode_path_segments(segments: &[&str]) -> String {
    segments
        .iter()
        .map(|segment| decode_path_segment(segment))
        .collect::<Vec<_>>()
        .join("/")
}

pub(super) fn parse_search_params(path: &str) -> BTreeMap<String, String> {
    let mut params = BTreeMap::new();
    let Some((_, query)) = path.split_once('?') else {
        return params;
    };
    let query = query.split_once('#').map_or(query, |(query, _)| query);
    for pair in query.split('&').filter(|pair| !pair.is_empty()) {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        let key = percent_decode_component(key, true);
        if !key.is_empty() {
            params.insert(key, percent_decode_component(value, true));
        }
    }
    params
}

fn percent_decode_component(value: &str, plus_as_space: bool) -> String {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            b'+' if plus_as_space => {
                decoded.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                if let Some(byte) = decode_hex_pair(bytes[index + 1], bytes[index + 2]) {
                    decoded.push(byte);
                    index += 3;
                } else {
                    decoded.push(bytes[index]);
                    index += 1;
                }
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
