//! Showcase server (SSR).

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use showcase::{App, shell};

    let conf = get_configuration(None).expect("Leptos configuration");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    println!("showcase: http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("port is in use");
    axum::serve(listener, app.into_make_service()).await.expect("server stopped");
}

#[cfg(not(feature = "ssr"))]
fn main() {}
