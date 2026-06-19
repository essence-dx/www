use std::collections::BTreeMap;

const REDACTED_PRESENT_VALUE: &str = "redacted-present";

const SENSITIVE_PRESENCE_ENV: &[&str] = &[
    "DX_DATABASE_URL",
    "DX_SQLITE_DATABASE_PATH",
    "BETTER_AUTH_SECRET",
    "GOOGLE_CLIENT_SECRET",
    "STRIPE_SECRET_KEY",
    "STRIPE_WEBHOOK_SECRET",
    "STRIPE_PRICE_ID",
    "STRIPE_PRICE_ID_STARTER",
    "STRIPE_PRICE_ID_TEAM",
    "STRIPE_PRICE_ID_SCALE",
    "SLACK_BOT_TOKEN",
    "SLACK_CLIENT_ID",
    "SLACK_CLIENT_SECRET",
    "NOTION_API_KEY",
    "NOTION_CLIENT_ID",
    "NOTION_CLIENT_SECRET",
    "AI_PROVIDER_API_KEY",
    "AI_GATEWAY_API_KEY",
];

const PUBLIC_VALUE_ENV: &[&str] = &[
    "BETTER_AUTH_URL",
    "NEXT_PUBLIC_BETTER_AUTH_URL",
    "GOOGLE_CLIENT_ID",
    "DX_DATABASE_MIGRATIONS_REVIEWED",
    "DX_DATABASE_AUTHORIZATION_REVIEWED",
    "NEXT_PUBLIC_SUPABASE_URL",
    "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY",
    "NEXT_PUBLIC_INSTANT_APP_ID",
    "NEXT_PUBLIC_INSTANT_API_URI",
    "NEXT_PUBLIC_INSTANT_FIRST_PARTY_PATH",
    "NEXT_PUBLIC_INSTANT_WEBSOCKET_URI",
    "NEXT_PUBLIC_INSTANT_DEVTOOL",
    "NEXT_PUBLIC_INSTANT_DISABLE_VALIDATION",
    "NEXT_PUBLIC_INSTANT_QUERY_CACHE_LIMIT",
    "NEXT_PUBLIC_INSTANT_VERBOSE",
    "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY",
    "DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS",
    "DX_HTTP_JSON_ALLOWED_ORIGINS",
    "DX_ENABLE_EXTENDED_AI_ROUTES",
];

pub(super) fn route_handler_runtime_env() -> BTreeMap<String, String> {
    let mut env = BTreeMap::new();

    for name in SENSITIVE_PRESENCE_ENV {
        if has_runtime_env_value(name) {
            env.insert((*name).to_string(), REDACTED_PRESENT_VALUE.to_string());
        }
    }

    for name in PUBLIC_VALUE_ENV {
        if let Some(value) = runtime_env_value(name) {
            env.insert((*name).to_string(), value);
        }
    }

    env
}

fn has_runtime_env_value(name: &str) -> bool {
    runtime_env_value(name).is_some()
}

fn runtime_env_value(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
