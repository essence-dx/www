use super::*;

/// Process ecosystem features from source code
pub fn process_ecosystem_features(source: &str) -> Result<Vec<EcosystemFeature>> {
    let mut features = Vec::new();

    // Parse form schemas
    let form_schemas = schema_parser::parse_form_schema(source);
    for schema in form_schemas {
        features.push(EcosystemFeature::FormSchema(schema));
    }

    // Parse queries
    let queries = schema_parser::parse_query_definitions(source);
    for query in queries {
        features.push(EcosystemFeature::QueryDefinition(query));
    }

    // Parse DB schemas
    let db_schemas = schema_parser::parse_db_schema(source);
    for schema in db_schemas {
        features.push(EcosystemFeature::DatabaseSchema(schema));
    }

    // Parse state definitions
    let state_defs = schema_parser::parse_state_definitions(source);
    for def in state_defs {
        features.push(EcosystemFeature::StateDefinition(def));
    }

    Ok(features)
}

/// Ecosystem feature types
#[derive(Debug)]
pub enum EcosystemFeature {
    FormSchema(schema_parser::FormSchema),
    QueryDefinition(schema_parser::QueryDefinition),
    DatabaseSchema(schema_parser::TableSchema),
    StateDefinition(schema_parser::StateDefinition),
}

/// Generate Rust code for ecosystem features
pub fn generate_code(features: &[EcosystemFeature]) -> String {
    let mut code = String::new();

    code.push_str("// Generated ecosystem code\n\n");

    for feature in features {
        match feature {
            EcosystemFeature::FormSchema(schema) => {
                code.push_str(&generate_form_validator(schema));
            }
            EcosystemFeature::QueryDefinition(query) => {
                code.push_str(&generate_query_function(query));
            }
            EcosystemFeature::DatabaseSchema(schema) => {
                code.push_str(&generate_db_struct(schema));
            }
            EcosystemFeature::StateDefinition(def) => {
                code.push_str(&generate_state_struct(def));
            }
        }
    }

    code
}

/// Generate form validator code
fn generate_form_validator(schema: &schema_parser::FormSchema) -> String {
    let mut code = format!("// Form validator for {}\n", schema.name);
    code.push_str(&format!("pub struct {} {{\n", schema.name));

    for field in &schema.fields {
        code.push_str(&format!("    pub {}: {},\n", field.name, field.field_type));
    }

    code.push_str("}\n\n");
    code
}

/// Generate query function
fn generate_query_function(query: &QueryDefinition) -> String {
    format!(
        "// Query function for {}\npub async fn {}({}) -> Result<Response> {{\n    compile_error!(\"dx-www query binding required: {} {}\");\n}}\n\n",
        query.name,
        query.name,
        query.params.join(", "),
        query.method,
        query.endpoint
    )
}

/// Generate database struct
fn generate_db_struct(schema: &schema_parser::TableSchema) -> String {
    let mut code = format!(
        "// Database struct for {}\n#[repr(C)]\npub struct {} {{\n",
        schema.name, schema.name
    );

    for col in &schema.columns {
        let col_type = if col.nullable {
            format!("Option<{}>", col.column_type)
        } else {
            col.column_type.clone()
        };
        code.push_str(&format!("    pub {}: {},\n", col.name, col_type));
    }

    code.push_str("}\n\n");
    code
}

/// Generate state struct
fn generate_state_struct(def: &schema_parser::StateDefinition) -> String {
    let mut code = format!(
        "// State struct for {}\n#[repr(C)]\npub struct {} {{\n",
        def.name, def.name
    );

    for (field, field_type) in &def.fields {
        code.push_str(&format!("    pub {}: {},\n", field, field_type));
    }

    code.push_str("}\n\n");
    code
}

// ============================================================================
// Tests
// ============================================================================
