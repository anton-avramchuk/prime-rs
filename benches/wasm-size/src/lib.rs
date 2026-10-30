//! Приложение для измерения размера wasm: набор отрисованного определяется feature-флагом `case-*`.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

// Случай выбирается по приоритету: при нескольких включённых features (например, `--all-features`) побеждает первый,
// поэтому крейт собирается при любой комбинации, а в обычной сборке с одним `case-*` типы и размер не меняются.
cfg_if::cfg_if! {
    if #[cfg(feature = "case-baseline")] {
        fn case() -> impl IntoView {
            view! { <div>"baseline"</div> }
        }
    } else if #[cfg(feature = "case-one-icon")] {
        fn case() -> impl IntoView {
            view! { <div><prime_icons::CheckIcon /></div> }
        }
    } else if #[cfg(feature = "case-all-icons")] {
        fn case() -> impl IntoView {
            view! { <div><prime_icons::AllIcons /></div> }
        }
    } else if #[cfg(feature = "case-badge")] {
        fn case() -> impl IntoView {
            view! { <prime_badge::Badge value="5" severity=prime_badge::BadgeSeverity::Danger /> }
        }
    } else if #[cfg(feature = "case-tag")] {
        fn case() -> impl IntoView {
            view! { <prime_tag::Tag value="New" severity=prime_tag::TagSeverity::Success rounded=true /> }
        }
    } else if #[cfg(feature = "case-chip")] {
        fn case() -> impl IntoView {
            view! { <prime_chip::Chip label="Thriller" removable=true /> }
        }
    } else if #[cfg(feature = "case-avatar")] {
        fn case() -> impl IntoView {
            view! { <prime_avatar::Avatar label="P" shape=prime_avatar::AvatarShape::Circle size=prime_avatar::AvatarSize::Large /> }
        }
    } else if #[cfg(feature = "case-divider")] {
        fn case() -> impl IntoView {
            view! { <prime_divider::Divider align=prime_divider::DividerAlign::Center>"OR"</prime_divider::Divider> }
        }
    } else if #[cfg(feature = "case-card")] {
        fn case() -> impl IntoView {
            view! { <prime_card::Card header="Simple Card" subheader="Subtitle"><p>"Content"</p></prime_card::Card> }
        }
    } else if #[cfg(feature = "case-inputtext")] {
        fn case() -> impl IntoView {
            view! { <prime_inputtext::InputText placeholder="Name" /> }
        }
    } else if #[cfg(feature = "case-checkbox")] {
        fn case() -> impl IntoView {
            view! { <prime_checkbox::Checkbox binary=true default_checked=true name="agree" /> }
        }
    } else if #[cfg(feature = "case-radiobutton")] {
        fn case() -> impl IntoView {
            view! { <prime_radiobutton::RadioButton default_checked=true name="pizza" value="Cheese" /> }
        }
    } else if #[cfg(feature = "case-rating")] {
        fn case() -> impl IntoView {
            view! { <prime_rating::Rating default_value=3 name="grade" /> }
        }
    } else if #[cfg(feature = "case-inputnumber")] {
        fn case() -> impl IntoView {
            view! {
        <prime_inputnumber::InputNumber
            default_value=1234.5
            mode=prime_inputnumber::InputNumberMode::Currency
            currency="USD"
            locale="en-US"
            show_buttons=true
        />
    }
        }
    } else if #[cfg(feature = "case-select")] {
        fn case() -> impl IntoView {
            view! {
        <prime_select::Select
            options=["New York", "Rome", "London", "Istanbul", "Paris"].map(|c| prime_select::SelectOption::new(c, c)).to_vec()
            default_value="London"
            placeholder="Select a City"
        />
    }
        }
    } else if #[cfg(feature = "case-knob")] {
        fn case() -> impl IntoView {
            view! { <prime_knob::Knob default_value=60.0 /> }
        }
    } else if #[cfg(feature = "case-slider")] {
        fn case() -> impl IntoView {
            view! { <prime_slider::Slider default_value=30.0 /> }
        }
    } else if #[cfg(feature = "case-selectbutton")] {
        fn case() -> impl IntoView {
            view! {
        <prime_selectbutton::SelectButton
            options=vec![
                prime_selectbutton::SelectButtonOption::new("One-Way", "one-way"),
                prime_selectbutton::SelectButtonOption::new("Return", "return"),
            ]
            default_value="one-way"
        />
    }
        }
    } else if #[cfg(feature = "case-togglebutton")] {
        fn case() -> impl IntoView {
            view! { <prime_togglebutton::ToggleButton default_checked=true on_label="On" off_label="Off" /> }
        }
    } else if #[cfg(feature = "case-toggleswitch")] {
        fn case() -> impl IntoView {
            view! { <prime_toggleswitch::ToggleSwitch default_checked=true name="notify" /> }
        }
    } else if #[cfg(feature = "case-message")] {
        fn case() -> impl IntoView {
            view! { <prime_message::Message severity=prime_message::MessageSeverity::Success text="Saved" /> }
        }
    } else if #[cfg(feature = "case-progressbar")] {
        fn case() -> impl IntoView {
            view! { <prime_progressbar::ProgressBar value=50.0 /> }
        }
    } else if #[cfg(feature = "case-avatargroup")] {
        fn case() -> impl IntoView {
            view! { <prime_avatargroup::AvatarGroup><span>"a"</span></prime_avatargroup::AvatarGroup> }
        }
    } else if #[cfg(feature = "case-skeleton")] {
        fn case() -> impl IntoView {
            view! { <prime_skeleton::Skeleton shape=prime_skeleton::SkeletonShape::Circle size="4rem" /> }
        }
    } else if #[cfg(feature = "case-textarea")] {
        fn case() -> impl IntoView {
            view! { <prime_textarea::Textarea rows=3u32 placeholder="Text" /> }
        }
    } else if #[cfg(feature = "case-toolbar")] {
        fn case() -> impl IntoView {
            view! {
        <prime_toolbar::Toolbar
            start_template=ViewFn::from(|| view! { "Start" })
            center_template=ViewFn::from(|| view! { "Center" })
            end_template=ViewFn::from(|| view! { "End" })
        />
    }
        }
    } else if #[cfg(feature = "case-inputgroup")] {
        fn case() -> impl IntoView {
            view! {
        <prime_inputgroup::InputGroup>
            <prime_inputgroup::InputGroupAddon>"$"</prime_inputgroup::InputGroupAddon>
            <input class="p-inputtext p-component" placeholder="Price" />
            <prime_inputgroup::InputGroupAddon>".00"</prime_inputgroup::InputGroupAddon>
        </prime_inputgroup::InputGroup>
    }
        }
    } else if #[cfg(feature = "case-buttongroup")] {
        fn case() -> impl IntoView {
            view! { <prime_buttongroup::ButtonGroup><button class="p-button">"Save"</button><button class="p-button">"Delete"</button><button class="p-button">"Cancel"</button></prime_buttongroup::ButtonGroup> }
        }
    } else if #[cfg(feature = "case-fluid")] {
        fn case() -> impl IntoView {
            view! { <prime_fluid::Fluid><input class="p-inputtext p-component" placeholder="Input" /></prime_fluid::Fluid> }
        }
    } else if #[cfg(feature = "case-iconfield")] {
        fn case() -> impl IntoView {
            view! { <prime_iconfield::IconField><prime_iconfield::InputIcon><svg /></prime_iconfield::InputIcon><input class="p-inputtext p-component" placeholder="Search" /></prime_iconfield::IconField> }
        }
    } else if #[cfg(feature = "case-floatlabel")] {
        fn case() -> impl IntoView {
            view! { <prime_floatlabel::FloatLabel variant=prime_floatlabel::FloatLabelVariant::On><input id="u" class="p-inputtext p-component" /><label for="u">"Username"</label></prime_floatlabel::FloatLabel> }
        }
    } else if #[cfg(feature = "case-iftalabel")] {
        fn case() -> impl IntoView {
            view! { <prime_iftalabel::IftaLabel><input id="u" class="p-inputtext p-component" /><label for="u">"Username"</label></prime_iftalabel::IftaLabel> }
        }
    } else if #[cfg(feature = "case-overlaybadge")] {
        fn case() -> impl IntoView {
            view! { <prime_overlaybadge::OverlayBadge value="2"><span>"x"</span></prime_overlaybadge::OverlayBadge> }
        }
    } else if #[cfg(feature = "case-timeline")] {
        fn case() -> impl IntoView {
            let content = Callback::new(|e: &'static str| view! { <b>{e}</b> }.into_any());
            view! { <prime_timeline::Timeline value={vec!["a", "b", "c"]} content=content /> }
        }
    } else if #[cfg(feature = "case-metergroup")] {
        fn case() -> impl IntoView {
            view! { <prime_metergroup::MeterGroup value={vec![prime_metergroup::MeterItem::new("Apps", "#34d399", 16.0)]} /> }
        }
    } else if #[cfg(feature = "case-breadcrumb")] {
        fn case() -> impl IntoView {
            let model = vec![prime_breadcrumb::BreadcrumbItem::new().label("Electronics"), prime_breadcrumb::BreadcrumbItem::new().label("Computer")];
            view! { <prime_breadcrumb::Breadcrumb model=model home=prime_breadcrumb::BreadcrumbItem::new() /> }
        }
    } else if #[cfg(feature = "case-inputotp")] {
        fn case() -> impl IntoView {
            view! { <prime_inputotp::InputOtp length=6 /> }
        }
    } else if #[cfg(feature = "case-tabs")] {
        fn case() -> impl IntoView {
            view! {
                <prime_tabs::Tabs default_value="0">
                    <prime_tabs::TabList>
                        <prime_tabs::Tab value="0">"Header I"</prime_tabs::Tab>
                        <prime_tabs::Tab value="1">"Header II"</prime_tabs::Tab>
                    </prime_tabs::TabList>
                    <prime_tabs::TabPanels>
                        <prime_tabs::TabPanel value="0"><p>"One"</p></prime_tabs::TabPanel>
                        <prime_tabs::TabPanel value="1"><p>"Two"</p></prime_tabs::TabPanel>
                    </prime_tabs::TabPanels>
                </prime_tabs::Tabs>
            }
        }
    } else if #[cfg(feature = "case-popover")] {
        fn case() -> impl IntoView {
            let op = prime_popover::PopoverRef::new();
            view! {
                <button on:click=move |e| op.toggle(&e)>"Show"</button>
                <prime_popover::Popover popover_ref=op><p>"Content"</p></prime_popover::Popover>
            }
        }
    } else if #[cfg(feature = "case-drawer")] {
        fn case() -> impl IntoView {
            let visible = RwSignal::new(false);
            view! {
                <button on:click=move |_| visible.set(true)>"Show"</button>
                <prime_drawer::Drawer visible=visible header="Drawer"><p>"Content"</p></prime_drawer::Drawer>
            }
        }
    } else if #[cfg(feature = "case-splitter")] {
        fn case() -> impl IntoView {
            view! {
                <prime_splitter::Splitter
                    panels=vec![ViewFn::from(|| view! { <p>"One"</p> }), ViewFn::from(|| view! { <p>"Two"</p> })]
                />
            }
        }
    } else if #[cfg(feature = "case-inplace")] {
        fn case() -> impl IntoView {
            view! {
                <prime_inplace::Inplace
                    display=ViewFn::from(|| view! { <span>"View Content"</span> })
                    content=Callback::new(|_close| view! { <p>"Content"</p> }.into_any())
                />
            }
        }
    } else if #[cfg(feature = "case-password")] {
        fn case() -> impl IntoView {
            view! { <prime_password::Password toggle_mask=true /> }
        }
    } else if #[cfg(feature = "case-scrolltop")] {
        fn case() -> impl IntoView {
            view! { <prime_scrolltop::ScrollTop /> }
        }
    } else if #[cfg(feature = "case-progressspinner")] {
        fn case() -> impl IntoView {
            view! { <prime_progressspinner::ProgressSpinner aria_label="loading" /> }
        }
    } else if #[cfg(feature = "case-scrollpanel")] {
        fn case() -> impl IntoView {
            view! { <prime_scrollpanel::ScrollPanel style="width: 100%; height: 150px"><p>"Content"</p></prime_scrollpanel::ScrollPanel> }
        }
    } else if #[cfg(feature = "case-accordion")] {
        fn case() -> impl IntoView {
            view! {
        <prime_accordion::Accordion default_value=vec!["0".to_owned()]>
            <prime_accordion::AccordionPanel value="0">
                <prime_accordion::AccordionHeader>"Header"</prime_accordion::AccordionHeader>
                <prime_accordion::AccordionContent><p>"Content"</p></prime_accordion::AccordionContent>
            </prime_accordion::AccordionPanel>
        </prime_accordion::Accordion>
    }
        }
    } else if #[cfg(feature = "case-fieldset")] {
        fn case() -> impl IntoView {
            view! { <prime_fieldset::Fieldset legend="Header" toggleable=true><p>"Content"</p></prime_fieldset::Fieldset> }
        }
    } else if #[cfg(feature = "case-panel")] {
        fn case() -> impl IntoView {
            view! { <prime_panel::Panel header="Header" toggleable=true><p>"Content"</p></prime_panel::Panel> }
        }
    } else if #[cfg(feature = "case-button")] {
        fn case() -> impl IntoView {
            view! { <prime_button::Button label="Сохранить" severity=prime_button::ButtonSeverity::Success /> }
        }
    } else {
        compile_error!("wasm-size: включите ровно один feature `case-*`");
    }
}

#[component]
fn App() -> impl IntoView {
    case()
}

/// Точка входа wasm.
#[wasm_bindgen(start)]
pub fn start() {
    leptos::mount::mount_to_body(App);
}
