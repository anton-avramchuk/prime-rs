//! Install page: how to add the library to a Leptos project.

use crate::app::route;
use leptos::prelude::*;
use leptos_router::components::A;
use prime_icons::{ArrowRightIcon, InfoCircleIcon};

const CARGO_TOML: &str = r#"[dependencies]
leptos = "0.8"
prime-button = "0.1"
# CSS of a component is included only when its feature is enabled.
prime-theme = { version = "0.1", features = ["button"] }"#;

const APP_RS: &str = r#"use leptos::prelude::*;
use prime_button::{Button, ButtonSeverity};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <style>{prime_theme::stylesheet(&["button"])}</style>
        <Button label="Save" severity=ButtonSeverity::Success />
    }
}"#;

const CLIENT_RS: &str = r#"fn main() {
    leptos::mount::mount_to_body(app::App);
}"#;

/// A code block with a file name caption.
#[component]
fn Code(file: &'static str, code: &'static str) -> impl IntoView {
    view! {
        <div class="site-code">
            <div>{file}</div>
            <pre><code>{code}</code></pre>
        </div>
    }
}

/// Install page: what the library is, the crates it consists of, dependencies, theme CSS and a minimal example.
#[component]
pub fn InstallPage() -> impl IntoView {
    view! {
        <article class="site">
            <span class="site-eyebrow">"Getting started"</span>
            <h2 style="margin-top:.75rem">"What is prime-rs"</h2>
            <p class="site-lead">
                "prime-rs is a port of the PrimeNG 21.1.10 component library to " <code>"Leptos 0.8"</code>
                ". Each component keeps the PrimeNG markup, CSS classes, keyboard behaviour and pass-through (" <code>"pt"</code>
                ") options, so the PrimeNG theme styles apply unchanged. Components render on the server and hydrate in the browser."
            </p>
            <p>"Use it when you build a Leptos app and need PrimeNG-style components without writing them yourself."</p>

            <h2>"Install"</h2>
            <p>
                "The library is published on crates.io as a set of crates: one crate per component (" <code>"prime-button"</code>
                ", " <code>"prime-dialog"</code> ", ...) plus the shared crates below. Add the components you use; the shared crates
                come in as dependencies."
            </p>
            <table class="doc-table">
                <thead>
                    <tr>
                        <th>"Crate"</th>
                        <th>"What it provides"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"prime-rs-core"</td>
                        <td>"Configuration, class name helpers, z-index management and the pass-through (pt) mechanism."</td>
                    </tr>
                    <tr>
                        <td>"prime-dom"</td>
                        <td>"Overlay positioning, focus trap, portal and ripple."</td>
                    </tr>
                    <tr>
                        <td>"prime-theme"</td>
                        <td>"The static Aura theme CSS, one feature per component."</td>
                    </tr>
                    <tr>
                        <td>"prime-icons"</td>
                        <td>"PrimeNG SVG icons, one feature per icon."</td>
                    </tr>
                </tbody>
            </table>

            <div class="site-note">
                <InfoCircleIcon size=16 />
                <p>
                    <b>"Versions. "</b> "All crates share one version. The first release is " <code>"0.1.0"</code> ", so use "
                    <code>"0.1"</code> " in " <code>"Cargo.toml"</code> " to take any compatible " <code>"0.1.x"</code> " release."
                </p>
            </div>

            <ol class="site-steps">
                <li>
                    <h3>"Add the dependencies"</h3>
                    <Code file="Cargo.toml" code=CARGO_TOML />
                    <p>
                        "Theme CSS is per component. Enable the component's feature on " <code>"prime-theme"</code> " (or "
                        <code>"all-components"</code> ") so that " <code>"prime_theme::stylesheet"</code> " includes its styles."
                    </p>
                </li>
                <li>
                    <h3>"Use the component"</h3>
                    <Code file="src/app.rs" code=APP_RS />
                    <p>"This is the component and the styles."</p>
                </li>
                <li>
                    <h3>"Mount the app"</h3>
                    <p>"A client-only app (for example, built with " <code>"trunk"</code> ") mounts it like this:"</p>
                    <Code file="src/main.rs" code=CLIENT_RS />
                    <p>
                        "For server rendering (" <code>"cargo leptos"</code> ") enable the " <code>"ssr"</code>
                        " feature on the components in the server build and " <code>"hydrate"</code>
                        " in the browser build, as in the showcase."
                    </p>
                </li>
            </ol>

            <div class="site-links">
                <A href=route("/") attr:class="p-button p-component p-button-outlined">
                    <span class="p-button-label">"Back to overview"</span>
                </A>
                <A href=route("/button") attr:class="p-button p-component">
                    <span class="p-button-label">"Browse the components"</span>
                    <ArrowRightIcon size=14 />
                </A>
            </div>
        </article>
    }
}
