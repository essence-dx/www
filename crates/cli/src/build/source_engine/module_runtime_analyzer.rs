pub fn has_runtime_imports(source: &str) -> bool {
    source.lines().any(|line| {
        let line = line.trim_start();
        line.starts_with("import ") || line.starts_with("import{")
    })
}

pub fn looks_like_jsx(source: &str) -> bool {
    source.contains("return <") || source.contains("= <") || source.contains("(<")
}

pub fn exported_function_names(source: &str) -> Vec<String> {
    let mut names = Vec::new();
    for (index, marker) in source.match_indices("export function ") {
        let after = &source[index + marker.len()..];
        let name = read_identifier(after);
        if !name.is_empty() && !names.contains(&name) {
            names.push(name);
        }
    }
    names
}

fn read_identifier(source: &str) -> String {
    source
        .chars()
        .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
        .collect()
}
