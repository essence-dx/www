mod htip_generator;

use axum::{Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};
use dx_compiler::delivery::{DxOptimizedHtml, optimize_generated_html};
use std::{fmt::Write, net::SocketAddr, sync::OnceLock};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let app = Router::new()
        .route("/", get(index))
        .route("/healthz", get(healthz))
        .route("/robots.txt", get(robots_txt))
        .route("/benchmarks/fair-counter.json", get(fair_counter_report))
        .route("/fair-counter", get(fair_counter))
        .route("/medium-docs", get(medium_docs))
        .route("/medium-cards", get(medium_cards))
        .route("/big-dashboard", get(big_dashboard))
        .route("/todo", get(todo_app))
        .route("/demo/:name", get(demo_page))
        .route("/htip/:name", get(serve_htip))
        .route("/site.css", get(site_css))
        .route("/dx-style.generated.css", get(dx_style_generated_css))
        .route("/site.js", get(site_js))
        .route("/dx-style.json", get(dx_style_manifest))
        .route("/dx-style.dxbd", get(dx_style_binary))
        .route("/dx_www_client.wasm", get(serve_wasm))
        .route("/styles.binary", get(serve_binary_css))
        .nest_service("/static", ServeDir::new("demo"));

    println!("DX-WWW Demo Server");
    println!("Listening on http://{}", addr);
    println!();
    println!("Routes:");
    println!("  http://{addr}/              - DX-WWW workbench website");
    println!("  http://{addr}/healthz       - health check");
    println!("  http://{addr}/benchmarks/fair-counter.json - fair benchmark report");
    println!("  http://{addr}/fair-counter  - tiny micro-JS benchmark route");
    println!("  http://{addr}/medium-docs   - real medium docs benchmark route");
    println!("  http://{addr}/medium-cards  - real medium card grid benchmark route");
    println!("  http://{addr}/big-dashboard - real big dashboard benchmark route");
    println!("  http://{addr}/todo          - legacy full todo demo");
    println!("  http://{addr}/dx-style.generated.css - default generated CSS");
    println!("  http://{addr}/dx-style.dxbd - dx-style binary sidecar");
    println!();
    println!("Optimized for 100/100/100/100 Lighthouse scores");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn index() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            ("content-type", "text/html; charset=utf-8"),
            ("cache-control", "no-cache"),
            ("x-content-type-options", "nosniff"),
            ("x-frame-options", "DENY"),
            ("x-xss-protection", "1; mode=block"),
            ("referrer-policy", "strict-origin-when-cross-origin"),
            (
                "permissions-policy",
                "camera=(), microphone=(), geolocation=()",
            ),
            (
                "content-security-policy",
                "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self'; connect-src 'self'; base-uri 'none'; form-action 'self'; frame-ancestors 'none'",
            ),
        ],
        include_str!("site/index.html"),
    )
}

async fn healthz() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            ("content-type", "application/json; charset=utf-8"),
            ("cache-control", "no-cache"),
            ("x-content-type-options", "nosniff"),
        ],
        r#"{"status":"ok","service":"dx-www-demo","style_delivery":"generated-css"}"#,
    )
}

async fn robots_txt() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            ("content-type", "text/plain; charset=utf-8"),
            ("cache-control", "public, max-age=3600"),
            ("x-content-type-options", "nosniff"),
        ],
        "User-agent: *\nAllow: /\n",
    )
}

async fn fair_counter_report() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            ("content-type", "application/json; charset=utf-8"),
            ("cache-control", "no-cache"),
            ("x-content-type-options", "nosniff"),
        ],
        include_str!("../benchmarks/reports/fair-counter-comparison.json"),
    )
}

async fn todo_app() -> impl IntoResponse {
    let html = include_str!("todo.html");

    (
        StatusCode::OK,
        [
            ("content-type", "text/html; charset=utf-8"),
            ("cache-control", "public, max-age=3600"),
            ("x-content-type-options", "nosniff"),
            ("x-frame-options", "DENY"),
            ("x-xss-protection", "1; mode=block"),
            ("referrer-policy", "strict-origin-when-cross-origin"),
            (
                "permissions-policy",
                "camera=(), microphone=(), geolocation=()",
            ),
            (
                "content-security-policy",
                "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self'; connect-src 'self'; base-uri 'none'; form-action 'self'; frame-ancestors 'none'",
            ),
        ],
        html,
    )
}

async fn fair_counter() -> impl IntoResponse {
    tiny_benchmark_html(fair_counter_html())
}

async fn medium_docs() -> impl IntoResponse {
    benchmark_html(real_docs_html())
}

async fn medium_cards() -> impl IntoResponse {
    benchmark_html(real_cards_html())
}

async fn big_dashboard() -> impl IntoResponse {
    benchmark_html(real_dashboard_html())
}

fn benchmark_html(route: &'static DxOptimizedHtml) -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            ("content-type", "text/html; charset=utf-8"),
            ("cache-control", "no-cache"),
            ("x-content-type-options", "nosniff"),
            ("x-frame-options", "DENY"),
            ("x-xss-protection", "1; mode=block"),
            ("referrer-policy", "strict-origin-when-cross-origin"),
            (
                "permissions-policy",
                "camera=(), microphone=(), geolocation=()",
            ),
            (
                "content-security-policy",
                "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self'; connect-src 'self'; base-uri 'none'; form-action 'self'; frame-ancestors 'none'",
            ),
            ("x-dx-delivery-mode", route.profile().delivery_mode.as_str()),
        ],
        route.html(),
    )
}

fn tiny_benchmark_html(route: &'static DxOptimizedHtml) -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            ("content-type", "text/html; charset=utf-8"),
            ("cache-control", "no-cache"),
            ("x-dx-delivery-mode", route.profile().delivery_mode.as_str()),
        ],
        route.html(),
    )
}

static FAIR_COUNTER_HTML: OnceLock<DxOptimizedHtml> = OnceLock::new();
static REAL_DOCS_HTML: OnceLock<String> = OnceLock::new();
static REAL_CARDS_HTML: OnceLock<String> = OnceLock::new();
static REAL_DASHBOARD_HTML: OnceLock<String> = OnceLock::new();
static REAL_DOCS_ROUTE: OnceLock<DxOptimizedHtml> = OnceLock::new();
static REAL_CARDS_ROUTE: OnceLock<DxOptimizedHtml> = OnceLock::new();
static REAL_DASHBOARD_ROUTE: OnceLock<DxOptimizedHtml> = OnceLock::new();

fn fair_counter_html() -> &'static DxOptimizedHtml {
    FAIR_COUNTER_HTML.get_or_init(|| optimize_generated_html(include_str!("index.html")))
}

fn real_docs_html() -> &'static DxOptimizedHtml {
    REAL_DOCS_ROUTE.get_or_init(|| {
        optimize_generated_html(
            REAL_DOCS_HTML
                .get_or_init(|| render_docs_page("DX-WWW", "Rust route + generated HTML")),
        )
    })
}

fn real_cards_html() -> &'static DxOptimizedHtml {
    REAL_CARDS_ROUTE.get_or_init(|| {
        optimize_generated_html(
            REAL_CARDS_HTML.get_or_init(|| render_cards_page("DX-WWW", "Rust route + micro JS")),
        )
    })
}

fn real_dashboard_html() -> &'static DxOptimizedHtml {
    REAL_DASHBOARD_ROUTE.get_or_init(|| {
        optimize_generated_html(
            REAL_DASHBOARD_HTML
                .get_or_init(|| render_dashboard_page("DX-WWW", "Rust route + micro JS", 1200)),
        )
    })
}

fn benchmark_head(title: &str) -> String {
    format!(
        r#"<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><meta name="description" content="Real DX-WWW framework benchmark route"><title>{title}</title><style>{}</style></head><body>"#,
        benchmark_css()
    )
}

fn benchmark_css() -> &'static str {
    "*{box-sizing:border-box}body{margin:0;background:#f7f8fb;color:#14171f;font-family:system-ui,-apple-system,BlinkMacSystemFont,\"Segoe UI\",sans-serif;line-height:1.5}.shell{max-width:1180px;margin:0 auto;padding:24px}.hero{display:flex;justify-content:space-between;gap:16px;align-items:flex-end;border-bottom:1px solid #dfe3ea;margin-bottom:20px;padding-bottom:16px}.eyebrow{color:#5b6472;font-size:13px;font-weight:700;letter-spacing:.08em;text-transform:uppercase}h1{font-size:32px;line-height:1.1;margin:6px 0 0}.metric-row{display:flex;gap:8px;flex-wrap:wrap}.metric{background:#fff;border:1px solid #dfe3ea;border-radius:8px;padding:8px 10px;min-width:110px}.metric b{display:block;font-size:20px}.doc-section,.card,.panel{background:#fff;border:1px solid #dfe3ea;border-radius:8px;margin-bottom:10px;padding:14px}.doc-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(260px,1fr));gap:10px}.card-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(220px,1fr));gap:10px}.toolbar{display:flex;gap:8px;margin:0 0 14px}.toolbar input{border:1px solid #cad0da;border-radius:6px;flex:1;font:inherit;padding:10px 12px}.tag{background:#edf2ff;border-radius:999px;color:#2c55c7;display:inline-block;font-size:12px;font-weight:700;margin-top:10px;padding:3px 8px}table{border-collapse:collapse;width:100%;font-size:13px}th,td{border-bottom:1px solid #e6e9ef;padding:8px;text-align:left}th{background:#f0f3f8;color:#4b5565;font-size:12px;text-transform:uppercase}.status-ok{color:#087443;font-weight:700}.status-risk{color:#a15c00;font-weight:700}@media(max-width:700px){.shell{padding:16px}.hero{display:block}h1{font-size:26px}table{font-size:12px}}"
}

fn render_docs_page(framework: &str, _runtime: &str) -> String {
    let mut html = benchmark_head("Medium Docs Benchmark");
    html.push_str("<main class=\"shell\"><header class=\"hero\"><div><div class=\"eyebrow\">Medium route</div><h1>Framework documentation system</h1></div><div class=\"metric-row\">");
    write_metric(&mut html, "Sections", "160");
    write_metric(&mut html, "Runtime", framework);
    html.push_str("</div></header><section class=\"doc-grid\">");
    for index in 1..=160 {
        let principle = match index % 4 {
            0 => "routing",
            1 => "serialization",
            2 => "styling",
            _ => "deployment",
        };
        let _ = write!(
            html,
            "<article class=\"doc-section\"><h2>Binary web principle {index}</h2><p>This block covers cache behavior, payload shape, routing boundaries, and production maintenance for the same generated content.</p><span class=\"tag\">{principle}</span></article>"
        );
    }
    html.push_str("</section></main></body></html>");
    html
}

fn render_cards_page(framework: &str, _runtime: &str) -> String {
    let mut html = benchmark_head("Medium Cards Benchmark");
    html.push_str("<main class=\"shell\"><header class=\"hero\"><div><div class=\"eyebrow\">Medium interactive route</div><h1>Component registry catalog</h1></div><div class=\"metric-row\">");
    write_metric(&mut html, "Cards", "180");
    write_metric(&mut html, "Runtime", framework);
    html.push_str("</div></header><div class=\"toolbar\"><input id=\"card-filter\" type=\"search\" placeholder=\"Filter registry cards\" aria-label=\"Filter registry cards\"></div><section id=\"card-grid\" class=\"card-grid\">");
    for index in 1..=180 {
        let category = match index % 5 {
            0 => "auth",
            1 => "dashboard",
            2 => "commerce",
            3 => "editor",
            _ => "analytics",
        };
        let _ = write!(
            html,
            "<article class=\"card\" data-search=\"{category} component {index}\"><h2>{category} component {index}</h2><p>Editable source-owned component packaging with predictable upgrade metadata.</p><span class=\"tag\">{category}</span></article>"
        );
    }
    html.push_str(
        "</section></main><script>const cardInput=document.getElementById('card-filter');const cards=[...document.querySelectorAll('[data-search]')];cardInput.addEventListener('input',()=>{const q=cardInput.value.toLowerCase();for(const card of cards){card.hidden=!card.dataset.search.includes(q)}});</script></body></html>",
    );
    html
}

fn render_dashboard_page(framework: &str, _runtime: &str, rows: usize) -> String {
    let mut html = benchmark_head("Big Dashboard Benchmark");
    html.push_str("<main class=\"shell\"><header class=\"hero\"><div><div class=\"eyebrow\">Big interactive route</div><h1>Revenue operations dashboard</h1></div><div class=\"metric-row\">");
    write_metric(&mut html, "Rows", &rows.to_string());
    write_metric(&mut html, "Runtime", framework);
    html.push_str("</div></header><div class=\"toolbar\"><input id=\"row-filter\" type=\"search\" placeholder=\"Filter customers or status\" aria-label=\"Filter dashboard rows\"></div><section class=\"panel\"><table><thead><tr><th>Account</th><th>Plan</th><th>Region</th><th>Status</th><th>MRR</th><th>Risk</th></tr></thead><tbody>");
    for index in 1..=rows {
        let plan = match index % 4 {
            0 => "Enterprise",
            1 => "Pro",
            2 => "Team",
            _ => "Starter",
        };
        let region = match index % 5 {
            0 => "APAC",
            1 => "EU",
            2 => "NA",
            3 => "LATAM",
            _ => "MEA",
        };
        let status = if index % 9 == 0 { "Review" } else { "Healthy" };
        let risk_class = if status == "Review" {
            "status-risk"
        } else {
            "status-ok"
        };
        let mrr = 400 + (index % 37) * 91;
        let _ = write!(
            html,
            "<tr data-search=\"account {index} {plan} {region} {status}\"><td>Account {index}</td><td>{plan}</td><td>{region}</td><td class=\"{risk_class}\">{status}</td><td>${mrr}</td><td>{}</td></tr>",
            (index * 7) % 100
        );
    }
    html.push_str(
        "</tbody></table></section></main><script>const rowInput=document.getElementById('row-filter');const rows=[...document.querySelectorAll('tbody tr')];rowInput.addEventListener('input',()=>{const q=rowInput.value.toLowerCase();for(const row of rows){row.hidden=!row.dataset.search.includes(q)}});</script></body></html>",
    );
    html
}

fn write_metric(html: &mut String, label: &str, value: &str) {
    let _ = write!(
        html,
        "<div class=\"metric\"><span>{label}</span><b>{value}</b></div>"
    );
}

async fn demo_page(Path(name): Path<String>) -> impl IntoResponse {
    let html = match name.as_str() {
        "counter" | "todo" | "dashboard" => include_str!("demo_full.html"),
        _ => {
            return (
                StatusCode::NOT_FOUND,
                [("content-type", "text/html")],
                "Demo not found",
            );
        }
    };

    (
        StatusCode::OK,
        [("content-type", "text/html; charset=utf-8")],
        html,
    )
}

async fn site_css() -> impl IntoResponse {
    dx_style_generated_css().await
}

async fn dx_style_generated_css() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            ("content-type", "text/css; charset=utf-8"),
            ("cache-control", "no-cache"),
            ("x-content-type-options", "nosniff"),
        ],
        include_str!("site/site.css"),
    )
}

async fn site_js() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            ("content-type", "application/javascript; charset=utf-8"),
            ("cache-control", "no-cache"),
            ("x-content-type-options", "nosniff"),
        ],
        include_str!("site/site.js"),
    )
}

async fn dx_style_manifest() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            ("content-type", "application/json; charset=utf-8"),
            ("cache-control", "no-cache"),
            ("x-content-type-options", "nosniff"),
        ],
        include_str!("site/dx-style.json"),
    )
}

async fn dx_style_binary() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            ("content-type", "application/vnd.dx-style.dxbd"),
            ("cache-control", "public, max-age=31536000, immutable"),
            ("x-content-type-options", "nosniff"),
        ],
        include_bytes!("styles.dxbd").as_slice(),
    )
}

async fn serve_htip(Path(name): Path<String>) -> impl IntoResponse {
    let htip_data = match name.as_str() {
        "counter" => htip_generator::generate_counter_htip(),
        "todo" => htip_generator::generate_todo_htip(),
        "dashboard" => htip_generator::generate_dashboard_htip(),
        _ => {
            return (
                StatusCode::NOT_FOUND,
                [
                    ("content-type", "text/plain"),
                    ("cache-control", "no-cache"),
                    ("x-content-type-options", "nosniff"),
                ],
                vec![],
            );
        }
    };

    (
        StatusCode::OK,
        [
            ("content-type", "application/octet-stream"),
            ("cache-control", "public, max-age=31536000, immutable"),
            ("x-content-type-options", "nosniff"),
        ],
        htip_data,
    )
}

async fn serve_wasm() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            ("content-type", "application/wasm"),
            ("cache-control", "public, max-age=31536000, immutable"),
            ("x-content-type-options", "nosniff"),
        ],
        include_bytes!("dx_www_client.wasm").as_slice(),
    )
}

async fn serve_binary_css() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            ("content-type", "application/octet-stream"),
            ("cache-control", "no-cache, no-store, must-revalidate"),
            ("x-content-type-options", "nosniff"),
        ],
        include_bytes!("styles.binary").as_slice(),
    )
}
