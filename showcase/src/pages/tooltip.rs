//! Tooltip page: demos of `position`, `event`, `delay`, `autohide`, `options`, `custom` from `apps/showcase/doc/tooltip` of PrimeNG.
//!
//! In Leptos the analog of the `pTooltip` directive is `use:tooltip=TooltipOptions::…` on any element or component root. The
//! `accessibility` demo (a text page) is not ported; the logo from `custom` is replaced with a text label, and `Tailwind` classes with
//! inline styles.

use super::Demo;
use leptos::prelude::*;
use prime_button::{Button, ButtonSeverity};
use prime_inputtext::InputText;
use prime_tooltip::{TooltipEvent, TooltipOptions, TooltipPosition, tooltip};

/// The `/tooltip` page.
#[component]
pub fn TooltipPage() -> impl IntoView {
    view! {
        <Demo name="position" title="Position">
            <InputText placeholder="Right" use:tooltip=TooltipOptions::text("Enter your username").position(TooltipPosition::Right) />
            <InputText placeholder="Top" use:tooltip=TooltipOptions::text("Enter your username").position(TooltipPosition::Top) />
            <InputText placeholder="Bottom" use:tooltip=TooltipOptions::text("Enter your username").position(TooltipPosition::Bottom) />
            <InputText placeholder="Left" use:tooltip=TooltipOptions::text("Enter your username").position(TooltipPosition::Left) />
        </Demo>
        <Demo name="event" title="Event">
            <InputText placeholder="focus to display tooltip" use:tooltip=TooltipOptions::text("Enter your username").event(TooltipEvent::Focus) />
        </Demo>
        <Demo name="delay" title="Delay">
            <Button label="Save" use:tooltip=TooltipOptions::text("Confirm to proceed").show_delay(1000).hide_delay(300) />
        </Demo>
        <Demo name="autohide" title="Auto Hide">
            <InputText placeholder="autoHide: false" use:tooltip=TooltipOptions::text("Enter your username").auto_hide(false) />
            <InputText placeholder="autoHide: true" use:tooltip=TooltipOptions::text("Enter your username") />
        </Demo>
        <Demo name="options" title="Options">
            <InputText
                placeholder="hover to display tooltip"
                use:tooltip=TooltipOptions::text("Enter your username")
                    .show_delay(150)
                    .auto_hide(false)
                    .event(TooltipEvent::Hover)
                    .position(TooltipPosition::Left)
            />
        </Demo>
        <Demo name="custom" title="Custom">
            <Button
                label="Button"
                severity=ButtonSeverity::Secondary
                use:tooltip=TooltipOptions::view(|| {
                    view! {
                        <div style="display: flex; align-items: center; gap: 0.5rem">
                            <strong style="color: var(--p-primary-color)">"PrimeRS"</strong>
                            <span>"Custom tooltip content"</span>
                        </div>
                    }
                })
                    .position(TooltipPosition::Bottom)
            />
        </Demo>
    }
}
