//! Страница SelectButton: демонстрации `basic`, `disabled`, `fluid`, `invalid`, `multiple`, `sizes`, `template` из showcase PrimeNG
//! (`apps/showcase/doc/selectbutton`). Демонстрации с Angular Forms не переносятся; в `template` вместо `pi-align-*` (таких иконок в
//! `prime-icons` нет) шаблон выводит метку в `<strong>`.

use super::Demo;
use leptos::prelude::*;
use prime_core::Size;
use prime_selectbutton::{SelectButton, SelectButtonOption};

fn state_options() -> Vec<SelectButtonOption> {
    vec![SelectButtonOption::new("One-Way", "one-way"), SelectButtonOption::new("Return", "return")]
}

fn level_options() -> Vec<SelectButtonOption> {
    ["Beginner", "Expert"].into_iter().map(|l| SelectButtonOption::new(l, l)).collect()
}

/// Страница `/selectbutton`.
#[component]
pub fn SelectButtonPage() -> impl IntoView {
    let basic = RwSignal::new(Some("one-way".to_owned()));
    let invalid = RwSignal::new(None::<String>);
    let invalid_flag = Signal::derive(move || invalid.get().is_none());
    let payment = RwSignal::new(Vec::<String>::new());
    let justify = RwSignal::new(None::<String>);
    let justify_options: Vec<_> = ["Left", "Right", "Center", "Justify"].into_iter().map(|j| SelectButtonOption::new(j, j)).collect();
    let item = Callback::new(|(option, _): (SelectButtonOption, usize)| view! { <strong>{option.label}</strong> }.into_any());
    let payment_options = (1..=3).map(|n| SelectButtonOption::new(format!("Option {n}"), n.to_string())).collect::<Vec<_>>();
    view! {
        <h2>"SelectButton"</h2>
        <Demo name="basic" title="Basic">
            <SelectButton options=state_options() value=basic aria_labelledby="basic" />
            <span data-testid="echo">{move || basic.get().unwrap_or_default()}</span>
        </Demo>
        <Demo name="multiple" title="Multiple">
            <SelectButton options=payment_options multiple=true values=payment />
            <span data-testid="echo-multiple">{move || payment.get().join(",")}</span>
        </Demo>
        <Demo name="template" title="Template">
            <SelectButton options=justify_options value=justify item_template=item />
        </Demo>
        <Demo name="sizes" title="Sizes">
            <div style="display: flex; flex-direction: column; align-items: center; gap: 1rem">
                <SelectButton options=level_options() size=Size::Small />
                <SelectButton options=level_options() default_value="Beginner" />
                <SelectButton options=level_options() default_value="Expert" size=Size::Large />
            </div>
        </Demo>
        <Demo name="invalid" title="Invalid">
            <SelectButton options=state_options() value=invalid invalid=invalid_flag />
        </Demo>
        <Demo name="disabled" title="Disabled">
            <div style="display: flex; flex-wrap: wrap; justify-content: center; gap: 1rem">
                <SelectButton
                    options=vec![SelectButtonOption::new("Off", "off"), SelectButtonOption::new("On", "on")]
                    default_value="off"
                    disabled=true
                />
                <SelectButton
                    options=vec![SelectButtonOption::new("Option 1", "Option 1"), SelectButtonOption::new("Option 2", "Option 2").disabled(true)]
                    default_value="Option 1"
                />
            </div>
        </Demo>
        <Demo name="fluid" title="Fluid">
            <SelectButton options=state_options() default_value="one-way" fluid=true />
        </Demo>
    }
}
