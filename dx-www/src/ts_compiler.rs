use regex::Regex;

/// A simple, fast TypeScript to JavaScript transpiler using Regex.
/// For complex TS features, an AST-based parser like oxc or swc is recommended, 
/// but for basic static assets like whiteboard-runtime.ts, this safely strips types.
pub fn transpile_ts_to_js(source_text: &str, _file_path: &str) -> Result<String, String> {
    let mut js = source_text.to_string();

    // Remove interface definitions
    let re_interface = Regex::new(r"(?ms)^interface\s+\w+\s*\{.*?\n\}").unwrap();
    js = re_interface.replace_all(&js, "").to_string();

    // Remove type aliases
    let re_type = Regex::new(r"(?ms)^type\s+\w+\s*=.*?;").unwrap();
    js = re_type.replace_all(&js, "").to_string();

    // Remove basic type annotations: variable: Type
    let re_var_type = Regex::new(r"(let|const|var)\s+(\w+)\s*:\s*[A-Z][a-zA-Z0-9_]*(\[\])?").unwrap();
    js = re_var_type.replace_all(&js, "$1 $2").to_string();
    
    // Remove primitive type annotations
    let re_var_prim = Regex::new(r"(let|const|var)\s+(\w+)\s*:\s*(string|number|boolean|any|void)").unwrap();
    js = re_var_prim.replace_all(&js, "$1 $2").to_string();

    // Remove function argument types: (arg: Type) -> (arg)
    let re_arg_type = Regex::new(r"(\w+)\s*:\s*[A-Za-z0-9_\[\]]+(?=[,)])").unwrap();
    js = re_arg_type.replace_all(&js, "$1").to_string();

    // Remove return types: function foo(): Type { -> function foo() {
    let re_return_type = Regex::new(r"(\([^)]*\))\s*:\s*[A-Za-z0-9_\[\]]+\s*\{").unwrap();
    js = re_return_type.replace_all(&js, "$1 {").to_string();

    // Remove "as Type" assertions
    let re_as = Regex::new(r"\s+as\s+[A-Za-z0-9_\[\]]+").unwrap();
    js = re_as.replace_all(&js, "").to_string();

    Ok(js)
}
