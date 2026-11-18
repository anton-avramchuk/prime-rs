//! Home: what prime-rs is and why, a component index, links, icons and a scenario for memory measurements.

use crate::app::route;
use crate::doc::MENU;
use leptos::prelude::*;
use leptos_router::components::A;
use prime_badge::{Badge, BadgeSeverity};
use prime_button::{Button, ButtonSeverity};
use prime_icons::{
    AllIcons, ArrowRightIcon, CheckIcon, EyeIcon, InfoCircleIcon, PencilIcon, RefreshIcon, SearchIcon, StarIcon, ThLargeIcon,
};

const REPO: &str = "https://github.com/anton-avramchuk/prime-rs";
const PRIMENG: &str = "https://primeng.org";
const LEPTOS: &str = "https://leptos.dev";
const CRATES_IO: &str = "https://crates.io/crates/prime-rs-core";

/// Scenario for in-browser memory measurements: mounts and removes the given number of components.
#[component]
fn Bench() -> impl IntoView {
    let count = RwSignal::new(0usize);
    let buttons = RwSignal::new(0usize);
    let buttons_live = RwSignal::new(0usize);
    let live_loading = RwSignal::new(false);
    let badges = RwSignal::new(0usize);
    let badges_live = RwSignal::new(0usize);
    // the badge value is passed as a signal: the reactive path
    let live_value = RwSignal::new(Some("5".to_owned()));
    view! {
        <section>
            <h3>"Measurements"</h3>
            <p>"Scenarios for in-browser memory and speed measurements (used by the e2e benchmarks): each button mounts or removes 1000 components."</p>
            <button type="button" class="p-button p-component" data-testid="bench-mount" on:click=move |_| count.set(1000)>
                "Show 1000 icons"
            </button>
            <button type="button" class="p-button p-component" data-testid="bench-clear" on:click=move |_| count.set(0)>
                "Remove"
            </button>
            <div data-testid="bench-area" data-count=move || count.get().to_string()>
                {move || (0..count.get()).map(|_| view! { <prime_icons::CheckIcon /> }).collect_view()}
            </div>
            <button type="button" class="p-button p-component" data-testid="bench-buttons-mount" on:click=move |_| buttons.set(1000)>
                "Show 1000 buttons"
            </button>
            <button type="button" class="p-button p-component" data-testid="bench-buttons-clear" on:click=move |_| buttons.set(0)>
                "Remove buttons"
            </button>
            <div data-testid="bench-buttons-area" data-count=move || buttons.get().to_string()>
                {move || {
                    (0..buttons.get()).map(|_| view! { <Button label="Save" severity=ButtonSeverity::Success /> }).collect_view()
                }}
            </div>
            <button type="button" class="p-button p-component" data-testid="bench-buttons-live-mount" on:click=move |_| buttons_live.set(1000)>
                "Show 1000 buttons (signal)"
            </button>
            <button type="button" class="p-button p-component" data-testid="bench-buttons-live-clear" on:click=move |_| buttons_live.set(0)>
                "Remove buttons (signal)"
            </button>
            <div data-testid="bench-buttons-live-area" data-count=move || buttons_live.get().to_string()>
                {move || {
                    (0..buttons_live.get())
                        .map(|_| view! { <Button label="Save" severity=ButtonSeverity::Success loading=live_loading /> })
                        .collect_view()
                }}
            </div>
            <button type="button" class="p-button p-component" data-testid="bench-badges-mount" on:click=move |_| badges.set(1000)>
                "Show 1000 badges"
            </button>
            <button type="button" class="p-button p-component" data-testid="bench-badges-clear" on:click=move |_| badges.set(0)>
                "Remove badges"
            </button>
            <div data-testid="bench-badges-area" data-count=move || badges.get().to_string()>
                {move || (0..badges.get()).map(|_| view! { <Badge value="5" severity=BadgeSeverity::Danger /> }).collect_view()}
            </div>
            <button type="button" class="p-button p-component" data-testid="bench-badges-live-mount" on:click=move |_| badges_live.set(1000)>
                "Show 1000 badges (signal)"
            </button>
            <button type="button" class="p-button p-component" data-testid="bench-badges-live-clear" on:click=move |_| badges_live.set(0)>
                "Remove badges (signal)"
            </button>
            <div data-testid="bench-badges-live-area" data-count=move || badges_live.get().to_string()>
                {move || {
                    (0..badges_live.get())
                        .map(|_| view! { <Badge value=live_value severity=BadgeSeverity::Danger /> })
                        .collect_view()
                }}
            </div>
        </section>
    }
}

/// Home page.
#[component]
pub fn HomePage() -> impl IntoView {
    let components: usize = MENU.iter().skip(1).map(|(_, items)| items.len()).sum();
    let categories = MENU
        .iter()
        .skip(1)
        .map(|(title, items)| {
            let links = items.iter().map(|(path, label)| view! { <A href=route(&format!("/{path}"))>{*label}</A> }).collect_view();
            view! {
                <div class="site-cat">
                    <h3>{*title} <small>{items.len()}</small></h3>
                    <div class="site-pills">{links}</div>
                </div>
            }
        })
        .collect_view();
    view! {
        <div class="site">
            <section class="site-hero">
                <span class="site-eyebrow">"PrimeNG 21.1.10 \u{b7} Leptos 0.8 \u{b7} MIT"</span>
                <h1>"PrimeNG components for " <span>"Leptos"</span></h1>
                <p class="site-lead">
                    "prime-rs brings the PrimeNG component library to Rust. The markup, CSS classes, keyboard behaviour and theme are the
                    same as in PrimeNG, so you get a complete, accessible UI kit for your Leptos app without writing the components yourself."
                </p>
                <div class="site-cta">
                    <A href=route("/install") attr:class="p-button p-component">
                        <span class="p-button-label">"Get started"</span>
                        <ArrowRightIcon size=14 />
                    </A>
                    <A href=route("/button") attr:class="p-button p-component p-button-outlined">
                        <ThLargeIcon size=14 />
                        <span class="p-button-label">"Browse components"</span>
                    </A>
                    <a class="p-button p-component p-button-text" href=REPO target="_blank" rel="noopener noreferrer">
                        <StarIcon size=14 />
                        <span class="p-button-label">"GitHub"</span>
                    </a>
                </div>
                <div class="site-stats">
                    <div>
                        <b>{components}</b>
                        <span>"components"</span>
                    </div>
                    <div>
                        <b>"54"</b>
                        <span>"icons"</span>
                    </div>
                    <div>
                        <b>"SSR + CSR"</b>
                        <span>"server rendering and hydration"</span>
                    </div>
                    <div>
                        <b>"0.1"</b>
                        <span>"first crates.io release"</span>
                    </div>
                </div>
            </section>

            <h2>"What it is, and why"</h2>
            <div class="site-split">
                <div>
                    <h3>
                        <InfoCircleIcon size=16 />
                        "What"
                    </h3>
                    <p>
                        "A port of the " <a href=PRIMENG target="_blank" rel="noopener noreferrer">"PrimeNG"</a>
                        " component library to " <a href=LEPTOS target="_blank" rel="noopener noreferrer">"Leptos"</a>
                        ". One component is one crate (" <code>"prime-button"</code> ", " <code>"prime-dialog"</code> ", "
                        <code>"prime-table"</code> ", ...) with shared crates for configuration, DOM helpers, theme CSS and icons. The port
                        is synced with a fixed PrimeNG version and every difference is written down on the component page."
                    </p>
                </div>
                <div>
                    <h3>
                        <CheckIcon size=16 />
                        "Why"
                    </h3>
                    <p>
                        "Rich widgets - tables, dialogs, date pickers, trees, menus - are expensive to build and easy to get wrong: focus
                        handling, keyboard navigation, overlay positioning, accessibility. PrimeNG has solved these and has a mature
                        design system. Porting it keeps its behaviour, documentation and themes, and lets you write the whole app in Rust."
                    </p>
                </div>
            </div>

            <h2>"Highlights"</h2>
            <div class="site-grid">
                <article class="site-card">
                    <span class="site-icon"><CheckIcon size=20 /></span>
                    <h3>"A faithful port"</h3>
                    <p>
                        "Same DOM structure, " <code>"p-*"</code> " classes, ARIA attributes and keyboard handling as PrimeNG, checked
                        against the original in browser and e2e tests."
                    </p>
                </article>
                <article class="site-card">
                    <span class="site-icon"><ThLargeIcon size=20 /></span>
                    <h3>"One crate per component"</h3>
                    <p>"Depend only on what you use. Theme CSS and icons are behind features, so unused styles never reach your bundle."</p>
                </article>
                <article class="site-card">
                    <span class="site-icon"><RefreshIcon size=20 /></span>
                    <h3>"SSR and hydration"</h3>
                    <p>
                        "Components render on the server and hydrate in the browser (" <code>"ssr"</code> " and " <code>"hydrate"</code>
                        " features), and work in client-only apps as well."
                    </p>
                </article>
                <article class="site-card">
                    <span class="site-icon"><EyeIcon size=20 /></span>
                    <h3>"Themeable"</h3>
                    <p>"The Aura theme ships as static CSS built on design tokens (CSS variables), with a light and a dark scheme."</p>
                </article>
                <article class="site-card">
                    <span class="site-icon"><PencilIcon size=20 /></span>
                    <h3>"Pass-through"</h3>
                    <p>
                        "Customise any inner element of a component through " <code>"pt"</code>
                        " options without forking it, exactly as in PrimeNG."
                    </p>
                </article>
                <article class="site-card">
                    <span class="site-icon"><SearchIcon size=20 /></span>
                    <h3>"Measured"</h3>
                    <p>"Rendering speed, allocations and wasm size are benchmarked, so the cost of each component is known."</p>
                </article>
            </div>

            <h2>"Components"</h2>
            <p>"Every page has live examples with Rust code, an API reference, styling classes and notes about the port."</p>
            <div class="site-cats">{categories}</div>

            <h2>"Links"</h2>
            <div class="site-links">
                <A href=route("/install") attr:class="p-button p-component">
                    <span class="p-button-label">"Install"</span>
                    <ArrowRightIcon size=14 />
                </A>
                <a class="p-button p-component p-button-outlined" href=REPO target="_blank" rel="noopener noreferrer">
                    <span class="p-button-label">"Source on GitHub"</span>
                </a>
                <a class="p-button p-component p-button-outlined" href=CRATES_IO target="_blank" rel="noopener noreferrer">
                    <span class="p-button-label">"crates.io"</span>
                </a>
                <a class="p-button p-component p-button-outlined" href=PRIMENG target="_blank" rel="noopener noreferrer">
                    <span class="p-button-label">"PrimeNG"</span>
                </a>
            </div>

            <h2>"Icons"</h2>
            <p>
                "54 PrimeNG icons in " <code>"prime-icons"</code>
                ". Each one is a component behind its own feature flag, sized and coloured with CSS."
            </p>
            <div data-testid="icons" style="display: flex; flex-wrap: wrap; gap: 1rem; font-size: 1.5rem">
                <AllIcons />
            </div>

            <h2>"For contributors"</h2>
            <div class="site-bench">
                <Bench />
            </div>
        </div>
    }
}
