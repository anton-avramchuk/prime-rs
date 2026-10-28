//! Страница RadioButton: демонстрации `group`, `filled`, `sizes`, `invalid`, `disabled` из showcase PrimeNG (`apps/showcase/doc/radiobutton`).

use super::Demo;
use leptos::prelude::*;
use prime_core::{Size, Variant};
use prime_radiobutton::RadioButton;

/// Страница `/radiobutton`.
#[component]
pub fn RadioButtonPage() -> impl IntoView {
    let ingredient = RwSignal::new(None::<String>);
    let filled = RwSignal::new(None::<String>);
    let sizes = RwSignal::new(Some("M".to_owned()));
    let option = move |value: &'static str, id: &'static str| {
        view! {
            <div style="display: flex; align-items: center; gap: 0.5rem">
                <RadioButton input_id=id name="pizza" value=value model=ingredient />
                <label for=id>{value}</label>
            </div>
        }
    };
    view! {
        <h2>"RadioButton"</h2>
        <Demo name="group" title="Group">
            <div style="display: flex; flex-wrap: wrap; gap: 1rem">
                {option("Cheese", "ingredient1")} {option("Mushroom", "ingredient2")} {option("Pepper", "ingredient3")}
                {option("Onion", "ingredient4")}
            </div>
            <span data-testid="selected">{move || ingredient.get().unwrap_or_default()}</span>
        </Demo>
        <Demo name="filled" title="Filled">
            <RadioButton variant=Variant::Filled value="a" model=filled />
        </Demo>
        <Demo name="sizes" title="Sizes">
            <RadioButton size=Size::Small value="S" model=sizes name="size" />
            <RadioButton value="M" model=sizes name="size" />
            <RadioButton size=Size::Large value="L" model=sizes name="size" />
        </Demo>
        <Demo name="invalid" title="Invalid">
            <RadioButton invalid=true value="a" />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <RadioButton disabled=true value="a" />
            <RadioButton disabled=true default_checked=true value="b" />
        </Demo>
    }
}
