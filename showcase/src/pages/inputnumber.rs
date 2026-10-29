//! Страница InputNumber: демонстрации `locale`, `currency`, `numerals`, `prefixsuffix`, `buttons`, `vertical`, `clearicon`, `sizes`,
//! `filled`, `invalid`, `disabled`, `fluid` из showcase PrimeNG (`apps/showcase/doc/inputnumber`). Демонстрации с Angular Forms,
//! FloatLabel и IftaLabel не переносятся; «jp-JP» из исходника заменён корректным `ja-JP`.

use super::Demo;
use leptos::prelude::*;
use prime_core::{Size, Variant};
use prime_icons::{MinusIcon, PlusIcon};
use prime_inputnumber::{ButtonLayout, CurrencyDisplay, InputNumber, InputNumberMode};

const ROW: &str = "display: flex; flex-wrap: wrap; gap: 1rem; align-items: flex-end";
const CELL: &str = "display: flex; flex-direction: column; gap: 0.5rem; width: 12rem";

fn plus() -> ViewFn {
    ViewFn::from(|| view! { <PlusIcon /> })
}

fn minus() -> ViewFn {
    ViewFn::from(|| view! { <MinusIcon /> })
}

/// Страница `/inputnumber`.
#[component]
pub fn InputNumberPage() -> impl IntoView {
    let (user, us, de, india) =
        (RwSignal::new(Some(1234.5)), RwSignal::new(Some(1234.5)), RwSignal::new(Some(1234.5)), RwSignal::new(Some(1234.5)));
    let (cur_us, cur_de, cur_in, cur_jp) =
        (RwSignal::new(Some(1234.5)), RwSignal::new(Some(1234.5)), RwSignal::new(Some(1234.5)), RwSignal::new(Some(1234.5)));
    let (integer, nogroup, fraction, bounded) =
        (RwSignal::new(Some(42723.0)), RwSignal::new(Some(58151.0)), RwSignal::new(Some(2.0)), RwSignal::new(Some(50.0)));
    let (mile, percent, expiry, temperature) =
        (RwSignal::new(Some(50.0)), RwSignal::new(Some(20.0)), RwSignal::new(Some(10.0)), RwSignal::new(Some(20.0)));
    let (stacked, minmax, horizontal) = (RwSignal::new(Some(58151.0)), RwSignal::new(Some(50.0)), RwSignal::new(Some(2.5)));
    let vertical = RwSignal::new(Some(50.0));
    let clearable = RwSignal::new(Some(3000.0));
    let (small, normal, large) = (RwSignal::new(None::<f64>), RwSignal::new(None::<f64>), RwSignal::new(None::<f64>));
    let filled = RwSignal::new(Some(50.0));
    let (invalid, invalid_filled) = (RwSignal::new(None::<f64>), RwSignal::new(None::<f64>));
    let (invalid_flag, invalid_filled_flag) =
        (Signal::derive(move || invalid.get().is_none()), Signal::derive(move || invalid_filled.get().is_none()));
    let disabled = RwSignal::new(Some(50.0));
    let fluid = RwSignal::new(Some(50.0));
    view! {
        <h2>"InputNumber"</h2>
        <Demo name="locale" title="Locale">
            <div style={ROW}>
                <div style={CELL}>
                    <label for="locale-user">"User Locale"</label>
                    <InputNumber value=user input_id="locale-user" min_fraction_digits=2 fluid=true />
                </div>
                <div style={CELL}>
                    <label for="locale-us">"United States Locale"</label>
                    <InputNumber value=us input_id="locale-us" locale="en-US" min_fraction_digits=2 fluid=true />
                    <span data-testid="echo-us">{move || us.get().map(|v| v.to_string()).unwrap_or_default()}</span>
                </div>
                <div style={CELL}>
                    <label for="locale-german">"German Locale"</label>
                    <InputNumber value=de input_id="locale-german" locale="de-DE" min_fraction_digits=2 fluid=true />
                </div>
                <div style={CELL}>
                    <label for="locale-indian">"Indian Locale"</label>
                    <InputNumber value=india input_id="locale-indian" locale="en-IN" min_fraction_digits=2 fluid=true />
                </div>
            </div>
        </Demo>
        <Demo name="currency" title="Currency">
            <div style={ROW}>
                <div style={CELL}>
                    <label for="currency-us">"United States"</label>
                    <InputNumber value=cur_us input_id="currency-us" mode=InputNumberMode::Currency currency="USD" locale="en-US" fluid=true />
                </div>
                <div style={CELL}>
                    <label for="currency-germany">"Germany"</label>
                    <InputNumber value=cur_de input_id="currency-germany" mode=InputNumberMode::Currency currency="EUR" locale="de-DE" fluid=true />
                </div>
                <div style={CELL}>
                    <label for="currency-india">"India"</label>
                    <InputNumber
                        value=cur_in
                        input_id="currency-india"
                        mode=InputNumberMode::Currency
                        currency="INR"
                        currency_display=CurrencyDisplay::Code
                        locale="en-IN"
                        fluid=true
                    />
                </div>
                <div style={CELL}>
                    <label for="currency-japan">"Japan"</label>
                    <InputNumber value=cur_jp input_id="currency-japan" mode=InputNumberMode::Currency currency="JPY" locale="ja-JP" fluid=true />
                </div>
            </div>
        </Demo>
        <Demo name="numerals" title="Numerals">
            <div style={ROW}>
                <div style={CELL}>
                    <label for="integeronly">"Integer Only"</label>
                    <InputNumber value=integer input_id="integeronly" fluid=true />
                </div>
                <div style={CELL}>
                    <label for="withoutgrouping">"Without Grouping"</label>
                    <InputNumber value=nogroup input_id="withoutgrouping" use_grouping=false fluid=true />
                </div>
                <div style={CELL}>
                    <label for="minmaxfraction">"Min-Max Fraction Digits"</label>
                    <InputNumber value=fraction input_id="minmaxfraction" min_fraction_digits=2 max_fraction_digits=5 fluid=true />
                </div>
                <div style={CELL}>
                    <label for="minmax">"Min-Max Boundaries"</label>
                    <InputNumber value=bounded input_id="minmax" min=0.0 max=100.0 fluid=true />
                    <span data-testid="echo-minmax">{move || bounded.get().map(|v| v.to_string()).unwrap_or_default()}</span>
                </div>
            </div>
        </Demo>
        <Demo name="prefixsuffix" title="Prefix & Suffix">
            <div style={ROW}>
                <div style={CELL}>
                    <label for="mile">"Mile"</label>
                    <InputNumber value=mile input_id="mile" suffix=" mi" fluid=true />
                </div>
                <div style={CELL}>
                    <label for="percent">"Percent"</label>
                    <InputNumber value=percent input_id="percent" prefix="%" fluid=true />
                </div>
                <div style={CELL}>
                    <label for="expiry">"Expiry"</label>
                    <InputNumber value=expiry input_id="expiry" prefix="Expires in " suffix=" days" fluid=true />
                </div>
                <div style={CELL}>
                    <label for="temperature">"Temperature"</label>
                    <InputNumber value=temperature input_id="temperature" prefix="↑ " suffix="℃" min=0.0 max=40.0 fluid=true />
                </div>
            </div>
        </Demo>
        <Demo name="buttons" title="Buttons">
            <div style={ROW}>
                <div style={CELL}>
                    <label for="stacked">"Stacked"</label>
                    <InputNumber value=stacked input_id="stacked" show_buttons=true mode=InputNumberMode::Currency currency="USD" fluid=true />
                    <span data-testid="echo-stacked">{move || stacked.get().map(|v| v.to_string()).unwrap_or_default()}</span>
                </div>
                <div style={CELL}>
                    <label for="minmax-buttons">"Min-Max Boundaries"</label>
                    <InputNumber value=minmax input_id="minmax-buttons" show_buttons=true min=0.0 max=100.0 fluid=true />
                </div>
                <div style={CELL}>
                    <label for="horizontal">"Horizontal with Step"</label>
                    <InputNumber
                        value=horizontal
                        input_id="horizontal"
                        show_buttons=true
                        button_layout=ButtonLayout::Horizontal
                        step=0.25
                        mode=InputNumberMode::Currency
                        currency="EUR"
                        increment_button_icon=plus()
                        decrement_button_icon=minus()
                        fluid=true
                    />
                </div>
            </div>
        </Demo>
        <Demo name="vertical" title="Vertical">
            <InputNumber
                value=vertical
                input_id="vertical-input"
                show_buttons=true
                button_layout=ButtonLayout::Vertical
                input_style="width: 3rem"
                increment_button_icon=plus()
                decrement_button_icon=minus()
            />
        </Demo>
        <Demo name="clearicon" title="Clear Icon">
            <InputNumber
                value=clearable
                input_id="price_input"
                mode=InputNumberMode::Currency
                currency="USD"
                locale="en-US"
                show_clear=true
                input_style_class="w-56"
                input_style="width: 14rem"
            />
        </Demo>
        <Demo name="sizes" title="Sizes">
            <div style="display: flex; flex-direction: column; align-items: center; gap: 1rem">
                <InputNumber value=small size=Size::Small placeholder="Small" mode=InputNumberMode::Currency currency="USD" locale="en-US" />
                <InputNumber value=normal placeholder="Normal" mode=InputNumberMode::Currency currency="USD" locale="en-US" />
                <InputNumber value=large size=Size::Large placeholder="Large" mode=InputNumberMode::Currency currency="USD" locale="en-US" />
            </div>
        </Demo>
        <Demo name="filled" title="Filled">
            <InputNumber value=filled variant=Variant::Filled />
        </Demo>
        <Demo name="invalid" title="Invalid">
            <div style={ROW}>
                <InputNumber value=invalid invalid=invalid_flag min_fraction_digits=2 placeholder="Amount" />
                <InputNumber value=invalid_filled invalid=invalid_filled_flag min_fraction_digits=2 variant=Variant::Filled placeholder="Amount" />
            </div>
        </Demo>
        <Demo name="disabled" title="Disabled">
            <InputNumber value=disabled disabled=true />
        </Demo>
        <Demo name="fluid" title="Fluid">
            <InputNumber value=fluid fluid=true />
        </Demo>
    }
}
