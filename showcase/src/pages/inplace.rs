//! Страница Inplace: демонстрации `basic`, `input`, `image`, `lazy` по `apps/showcase/doc/inplace` PrimeNG.
//!
//! В `lazy` вместо `Table` обычная `<table>`; в `input` нет `pAutoFocus`, а в `image` вместо значка `pi-image` — `EyeIcon` (в `prime-icons` нет `image`).

use super::Demo;
use leptos::prelude::*;
use prime_button::{Button, ButtonSeverity};
use prime_icons::{EyeIcon, TimesIcon};
use prime_inplace::Inplace;
use prime_inputtext::InputText;

const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

/// Страница `/inplace`.
#[component]
pub fn InplacePage() -> impl IntoView {
    let products = RwSignal::new(Vec::<(&'static str, &'static str, &'static str, u32)>::new());
    let load = Callback::new(move |()| {
        products.set(vec![
            ("f230fh0g3", "Bamboo Watch", "Accessories", 24),
            ("nvklal433", "Black Watch", "Accessories", 61),
            ("zz21cz3c1", "Blue Band", "Fitness", 2),
        ]);
    });
    view! {
        <h2>"Inplace"</h2>
        <Demo name="basic" title="Basic">
            <Inplace
                display=ViewFn::from(|| view! { <span>"View Content"</span> })
                content=Callback::new(|_close| view! { <p style="margin: 0">{LOREM}</p> }.into_any())
            />
        </Demo>
        <Demo name="input" title="Input">
            <Inplace
                display=ViewFn::from(|| view! { <span>"Click to Edit"</span> })
                content=Callback::new(|close: Callback<()>| {
                    view! {
                        <span style="display: inline-flex; gap: 0.5rem">
                            <InputText />
                            <Button
                                button_type="button"
                                text=true
                                severity=ButtonSeverity::Danger
                                icon=ViewFn::from(|| view! { <TimesIcon /> })
                                on_click=Callback::new(move |_| close.run(()))
                            />
                        </span>
                    }
                    .into_any()
                })
            />
        </Demo>
        <Demo name="image" title="Image">
            <Inplace
                display=ViewFn::from(|| {
                    view! {
                        <span style="display: inline-flex; align-items: center; gap: 0.5rem">
                            <EyeIcon />
                            <span>"View Photo"</span>
                        </span>
                    }
                })
                content=Callback::new(|_close| {
                    view! {
                        <img
                            style="width: 100%; max-width: 20rem; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.2)"
                            src="https://primefaces.org/cdn/primeng/images/demo/galleria/galleria5.jpg"
                            alt="Nature"
                        />
                    }
                    .into_any()
                })
            />
        </Demo>
        <Demo name="lazy" title="Lazy">
            <Inplace
                on_activate=load
                display=ViewFn::from(|| view! { <span>"View Data"</span> })
                content=Callback::new(move |_close| {
                    view! {
                        <table style="width: 100%; text-align: left">
                            <thead>
                                <tr>
                                    <th>"Code"</th>
                                    <th>"Name"</th>
                                    <th>"Category"</th>
                                    <th>"Quantity"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <For each=move || products.get() key=|row| row.0 let:row>
                                    <tr>
                                        <td>{row.0}</td>
                                        <td>{row.1}</td>
                                        <td>{row.2}</td>
                                        <td>{row.3}</td>
                                    </tr>
                                </For>
                            </tbody>
                        </table>
                    }
                    .into_any()
                })
            />
        </Demo>
    }
}
