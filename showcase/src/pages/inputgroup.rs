//! Страница InputGroup: демонстрации `basic`, `button`, `checkbox`, `multiple` из showcase PrimeNG (`apps/showcase/doc/inputgroup`).
//!
//! Нет InputNumber, Select, Menu, FloatLabel и IftaLabel — вместо них `InputText`; иконок `user`, `map`, `clock` в `prime-icons` тоже нет,
//! поэтому взяты `search`, `star-fill`, `check`, `times`.

use super::Demo;
use leptos::prelude::*;
use prime_button::{Button, ButtonSeverity};
use prime_checkbox::Checkbox;
use prime_icons::{CheckIcon, SearchIcon, StarFillIcon, TimesIcon};
use prime_inputgroup::{InputGroup, InputGroupAddon};
use prime_inputtext::InputText;
use prime_radiobutton::RadioButton;

/// Страница `/inputgroup`.
#[component]
pub fn InputGroupPage() -> impl IntoView {
    let checked = RwSignal::new(false);
    let radio = RwSignal::new(None::<String>);
    view! {
        <h2>"InputGroup"</h2>
        <Demo name="basic" title="Basic">
            <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 1rem; width: 100%">
                <InputGroup>
                    <InputGroupAddon>
                        <SearchIcon />
                    </InputGroupAddon>
                    <InputText placeholder="Username" />
                </InputGroup>
                <InputGroup>
                    <InputGroupAddon>"$"</InputGroupAddon>
                    <InputText placeholder="Price" />
                    <InputGroupAddon>".00"</InputGroupAddon>
                </InputGroup>
                <InputGroup>
                    <InputGroupAddon>"www"</InputGroupAddon>
                    <InputText placeholder="Website" />
                </InputGroup>
            </div>
        </Demo>
        <Demo name="button" title="Button">
            <div style="display: flex; flex-wrap: wrap; gap: 1rem; width: 100%">
                <InputGroup>
                    <Button label="Search" />
                    <InputText placeholder="Keyword" />
                </InputGroup>
                <InputGroup>
                    <InputText placeholder="Keyword" />
                    <InputGroupAddon>
                        <Button icon=ViewFn::from(|| view! { <SearchIcon /> }) severity=ButtonSeverity::Secondary text=true aria_label="Search" />
                    </InputGroupAddon>
                </InputGroup>
                <InputGroup>
                    <InputGroupAddon>
                        <Button icon=ViewFn::from(|| view! { <CheckIcon /> }) severity=ButtonSeverity::Secondary aria_label="Yes" />
                    </InputGroupAddon>
                    <InputText placeholder="Vote" />
                    <InputGroupAddon>
                        <Button icon=ViewFn::from(|| view! { <TimesIcon /> }) severity=ButtonSeverity::Secondary aria_label="No" />
                    </InputGroupAddon>
                </InputGroup>
            </div>
        </Demo>
        <Demo name="checkbox" title="Checkbox">
            <div style="display: flex; flex-wrap: wrap; gap: 1rem; width: 100%">
                <InputGroup>
                    <InputText placeholder="Price" />
                    <InputGroupAddon>
                        <RadioButton name="rb1" value="rb1" model=radio />
                    </InputGroupAddon>
                </InputGroup>
                <InputGroup>
                    <InputGroupAddon>
                        <Checkbox binary=true checked=checked />
                    </InputGroupAddon>
                    <InputText placeholder="Username" />
                </InputGroup>
            </div>
            <span data-testid="echo">{move || checked.get().to_string()}</span>
        </Demo>
        <Demo name="multiple" title="Multiple">
            <InputGroup style="width: 30rem; max-width: 100%">
                <InputGroupAddon>
                    <StarFillIcon />
                </InputGroupAddon>
                <InputGroupAddon>
                    <StarFillIcon />
                </InputGroupAddon>
                <InputText placeholder="Price" />
                <InputGroupAddon>"$"</InputGroupAddon>
                <InputGroupAddon>".00"</InputGroupAddon>
            </InputGroup>
        </Demo>
    }
}
