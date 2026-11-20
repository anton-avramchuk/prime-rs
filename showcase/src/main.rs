//! Showcase server (SSR).
//!
//! `showcase --export <dir>` renders every route once and writes the pages as static HTML (`index.html`,
//! `<route>.html`, `404.html`) for the GitHub Pages build (`cargo xtask pages`); the server is not started.
// Same limit as the library: monomorphizing the page views here goes as deep.
#![recursion_limit = "512"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use showcase_app::{App, shell};

    let conf = get_configuration(None).expect("Leptos configuration");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    let paths: Vec<String> = routes.iter().map(|r| r.path().to_string()).collect();

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let args: Vec<String> = std::env::args().collect();
    if let Some(dir) = args.iter().position(|a| a == "--export").and_then(|i| args.get(i + 1)) {
        export(app, &paths, std::path::Path::new(dir)).await;
        return;
    }

    println!("showcase: http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("port is in use");
    axum::serve(listener, app.into_make_service()).await.expect("server stopped");
}

/// Renders each route path (and a missing one for `404.html`) through the router and writes the HTML into `dir`.
#[cfg(feature = "ssr")]
async fn export(app: axum::Router, paths: &[String], dir: &std::path::Path) {
    use axum::body::{Body, to_bytes};
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    let base = option_env!("SHOWCASE_BASE").unwrap_or("");
    std::fs::create_dir_all(dir).expect("create the export directory");
    let mut pages: Vec<(String, String, StatusCode)> = paths
        .iter()
        .map(|path| {
            let route = path.strip_prefix(base).unwrap_or(path).trim_matches('/');
            let file = if route.is_empty() { "index.html".to_string() } else { format!("{route}.html") };
            // The listed path is exactly what the router serves (the root under a base is `/base`, without a slash).
            (path.clone(), file, StatusCode::OK)
        })
        .collect();
    pages.push((format!("{base}/__missing__"), "404.html".to_string(), StatusCode::NOT_FOUND));

    for (url, file, expected) in pages {
        let request = Request::get(&url).body(Body::empty()).expect("request");
        let response = app.clone().oneshot(request).await.expect("render");
        assert_eq!(response.status(), expected, "unexpected status for {url}");
        let html = to_bytes(response.into_body(), usize::MAX).await.expect("response body");
        let target = dir.join(&file);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).expect("create the page directory");
        }
        std::fs::write(&target, html).expect("write the page");
    }
    println!("showcase: exported {} pages to {}", paths.len() + 1, dir.display());
}

#[cfg(not(feature = "ssr"))]
fn main() {}
