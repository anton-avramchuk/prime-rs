//! Fluid page: the `basic` demo from `apps/showcase/doc/fluid` in PrimeNG.
//!
//! The same set of fields is shown next to a non-`Fluid` one, so the width difference is visible; `AutoComplete`, `DatePicker` and `Password`
//! (they are not in the project) are replaced by `InputNumber`, `Select` and `Textarea`.

use super::Demo;
use leptos::prelude::*;
use prime_button::Button;
use prime_fluid::Fluid;
use prime_inputnumber::InputNumber;
use prime_inputtext::InputText;
use prime_select::{Select, SelectOption};
use prime_textarea::Textarea;

fn cities() -> Vec<SelectOption> {
    [("New York", "NY"), ("Rome", "RM"), ("London", "LDN")].into_iter().map(|(label, value)| SelectOption::new(label, value)).collect()
}

/// `/fluid` page.
#[component]
pub fn FluidPage() -> impl IntoView {
    view! {
        <Demo name="basic" title="Basic">
            <div style="width: 100%; display: flex; flex-direction: column; gap: 1rem">
                <Fluid>
                    <div data-testid="fluid" style="display: flex; flex-direction: column; align-items: flex-start; gap: 1rem">
                        <InputText placeholder="Input" />
                        <Textarea rows=2u32 placeholder="Textarea" />
                        <InputNumber placeholder="InputNumber" />
                        <Select options=cities() placeholder="Select" />
                        <Button label="Button" />
                        <InputText placeholder="Opted out" fluid=false />
                    </div>
                </Fluid>
                <div data-testid="plain" style="display: flex; flex-direction: column; align-items: flex-start; gap: 1rem">
                    <InputText placeholder="Input" />
                    <Button label="Button" />
                </div>
            </div>
        </Demo>
    }
}
