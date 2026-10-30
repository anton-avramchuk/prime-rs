//! Сценарии бенчмарков: каждый рендерит компонент(ы) в HTML-строку через SSR.
//!
//! Новый компонент добавляет сюда свои сценарии (один экземпляр и 1000 экземпляров) — они автоматически попадают
//! в `render` (divan, время), `alloc` (dhat, аллокации) и `time` (медиана в JSON для сравнения с базовой линией).

use leptos::prelude::*;
use prime_accordion::{Accordion, AccordionContent, AccordionHeader, AccordionPanel};
use prime_avatar::{Avatar, AvatarShape, AvatarSize};
use prime_avatargroup::AvatarGroup;
use prime_badge::{Badge, BadgeSeverity};
use prime_breadcrumb::{Breadcrumb, BreadcrumbItem};
use prime_button::{Button, ButtonSeverity};
use prime_buttongroup::ButtonGroup;
use prime_card::Card;
use prime_checkbox::Checkbox;
use prime_chip::Chip;
use prime_divider::{Divider, DividerAlign};
use prime_fieldset::Fieldset;
use prime_floatlabel::{FloatLabel, FloatLabelVariant};
use prime_fluid::Fluid;
use prime_iconfield::{IconField, InputIcon};
use prime_icons::{AllIcons, CheckIcon, SpinnerIcon};
use prime_iftalabel::IftaLabel;
use prime_inputgroup::{InputGroup, InputGroupAddon};
use prime_inputnumber::{InputNumber, InputNumberMode};
use prime_inputotp::InputOtp;
use prime_inputtext::InputText;
use prime_knob::Knob;
use prime_message::{Message, MessageSeverity};
use prime_metergroup::{MeterGroup, MeterItem};
use prime_overlaybadge::OverlayBadge;
use prime_panel::Panel;
use prime_progressbar::ProgressBar;
use prime_progressspinner::ProgressSpinner;
use prime_radiobutton::RadioButton;
use prime_rating::Rating;
use prime_scrollpanel::ScrollPanel;
use prime_select::{Select, SelectOption};
use prime_selectbutton::{SelectButton, SelectButtonOption};
use prime_skeleton::{Skeleton, SkeletonShape};
use prime_slider::Slider;
use prime_tag::{Tag, TagSeverity};
use prime_textarea::Textarea;
use prime_timeline::Timeline;
use prime_togglebutton::ToggleButton;
use prime_toggleswitch::ToggleSwitch;
use prime_toolbar::Toolbar;

/// Сценарий: имя и функция, возвращающая HTML.
pub struct Scenario {
    /// Имя вида `<компонент>-<число экземпляров>`.
    pub name: &'static str,
    /// Рендер в строку.
    pub run: fn() -> String,
}

fn render(view: impl IntoView + 'static) -> String {
    Owner::new().with(|| view.into_view().to_html())
}

fn check_n(n: usize) -> String {
    render((0..n).map(|_| view! { <CheckIcon /> }).collect_view())
}

fn spinner_n(n: usize) -> String {
    render((0..n).map(|_| view! { <SpinnerIcon spin=true /> }).collect_view())
}

fn badge_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Badge value="5" severity=BadgeSeverity::Danger /> }).collect_view())
}

fn button_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Button label="Сохранить" severity=ButtonSeverity::Success /> }).collect_view())
}

fn button_full_n(n: usize) -> String {
    let icon = ViewFn::from(|| view! { <CheckIcon /> });
    render((0..n).map(|_| view! { <Button label="Сохранить" icon=icon.clone() badge="2" /> }).collect_view())
}

fn tag_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Tag value="New" severity=TagSeverity::Success rounded=true /> }).collect_view())
}

fn chip_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Chip label="Action" /> }).collect_view())
}

fn chip_removable_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Chip label="Thriller" removable=true /> }).collect_view())
}

fn avatar_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Avatar label="P" shape=AvatarShape::Circle size=AvatarSize::Large /> }).collect_view())
}

fn divider_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Divider align=DividerAlign::Center>"OR"</Divider> }).collect_view())
}

fn inputtext_n(n: usize) -> String {
    render((0..n).map(|_| view! { <InputText placeholder="Name" /> }).collect_view())
}

fn checkbox_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Checkbox binary=true default_checked=true name="agree" /> }).collect_view())
}

fn radiobutton_n(n: usize) -> String {
    render((0..n).map(|_| view! { <RadioButton default_checked=true name="pizza" value="Cheese" /> }).collect_view())
}

fn rating_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Rating default_value=3 name="grade" /> }).collect_view())
}

fn inputnumber_n(n: usize) -> String {
    render(
        (0..n)
            .map(|_| view! { <InputNumber default_value=1234.5 mode=InputNumberMode::Currency currency="USD" locale="en-US" show_buttons=true /> })
            .collect_view(),
    )
}

fn select_n(n: usize) -> String {
    render(
        (0..n)
            .map(|_| {
                let options = ["New York", "Rome", "London", "Istanbul", "Paris"].map(|c| SelectOption::new(c, c)).to_vec();
                view! { <Select options=options default_value="London" placeholder="Select a City" /> }
            })
            .collect_view(),
    )
}

fn knob_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Knob default_value=60.0 /> }).collect_view())
}

fn slider_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Slider default_value=30.0 /> }).collect_view())
}

fn selectbutton_n(n: usize) -> String {
    render(
        (0..n)
            .map(|_| {
                let options = vec![SelectButtonOption::new("One-Way", "one-way"), SelectButtonOption::new("Return", "return")];
                view! { <SelectButton options=options default_value="one-way" /> }
            })
            .collect_view(),
    )
}

fn togglebutton_n(n: usize) -> String {
    render((0..n).map(|_| view! { <ToggleButton default_checked=true on_label="On" off_label="Off" /> }).collect_view())
}

fn toggleswitch_n(n: usize) -> String {
    render((0..n).map(|_| view! { <ToggleSwitch default_checked=true name="notify" /> }).collect_view())
}

fn message_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Message severity=MessageSeverity::Success text="Saved" /> }).collect_view())
}

fn progressbar_n(n: usize) -> String {
    render((0..n).map(|_| view! { <ProgressBar value=50.0 /> }).collect_view())
}

fn avatargroup_n(n: usize) -> String {
    render((0..n).map(|_| view! { <AvatarGroup><span>"a"</span></AvatarGroup> }).collect_view())
}

fn skeleton_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Skeleton shape=SkeletonShape::Circle size="4rem" /> }).collect_view())
}

fn textarea_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Textarea rows=3u32 placeholder="Text" /> }).collect_view())
}

fn panel_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Panel header="Header" toggleable=true><p>"Content"</p></Panel> }).collect_view())
}

fn panel_plain_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Panel header="Header"><p>"Content"</p></Panel> }).collect_view())
}

fn fieldset_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Fieldset legend="Header" toggleable=true><p>"Content"</p></Fieldset> }).collect_view())
}

fn accordion_n(n: usize) -> String {
    // Accordion передаёт состояние через контекст, поэтому части создаются внутри владельца.
    Owner::new().with(|| {
        render(
            (0..n)
                .map(|_| {
                    view! {
                        <Accordion default_value=vec!["0".to_owned()]>
                            <AccordionPanel value="0">
                                <AccordionHeader>"Header I"</AccordionHeader>
                                <AccordionContent><p>"Content I"</p></AccordionContent>
                            </AccordionPanel>
                            <AccordionPanel value="1">
                                <AccordionHeader>"Header II"</AccordionHeader>
                                <AccordionContent><p>"Content II"</p></AccordionContent>
                            </AccordionPanel>
                            <AccordionPanel value="2">
                                <AccordionHeader>"Header III"</AccordionHeader>
                                <AccordionContent><p>"Content III"</p></AccordionContent>
                            </AccordionPanel>
                        </Accordion>
                    }
                })
                .collect_view(),
        )
    })
}

fn toolbar_n(n: usize) -> String {
    render(
        (0..n)
            .map(|_| {
                view! {
                    <Toolbar
                        start_template=ViewFn::from(|| view! { "Start" })
                        center_template=ViewFn::from(|| view! { "Center" })
                        end_template=ViewFn::from(|| view! { "End" })
                    />
                }
            })
            .collect_view(),
    )
}

fn inputgroup_n(n: usize) -> String {
    render(
        (0..n)
            .map(|_| {
                view! {
                    <InputGroup>
                        <InputGroupAddon>"$"</InputGroupAddon>
                        <input class="p-inputtext p-component" placeholder="Price" />
                        <InputGroupAddon>".00"</InputGroupAddon>
                    </InputGroup>
                }
            })
            .collect_view(),
    )
}

fn buttongroup_n(n: usize) -> String {
    render(
        (0..n)
            .map(|_| {
                view! {
                    <ButtonGroup>
                        <Button label="Save" />
                        <Button label="Delete" />
                        <Button label="Cancel" />
                    </ButtonGroup>
                }
            })
            .collect_view(),
    )
}

fn fluid_n(n: usize) -> String {
    render(
        (0..n)
            .map(|_| {
                view! {
                    <Fluid>
                        <InputText placeholder="Input" />
                        <Button label="Button" />
                    </Fluid>
                }
            })
            .collect_view(),
    )
}

fn iconfield_n(n: usize) -> String {
    render(
        (0..n)
            .map(|_| {
                view! {
                    <IconField>
                        <InputIcon>
                            <CheckIcon />
                        </InputIcon>
                        <input class="p-inputtext p-component" placeholder="Search" />
                    </IconField>
                }
            })
            .collect_view(),
    )
}

fn floatlabel_n(n: usize) -> String {
    render(
        (0..n)
            .map(|_| {
                view! {
                    <FloatLabel variant=FloatLabelVariant::On>
                        <input id="u" class="p-inputtext p-component" />
                        <label for="u">"Username"</label>
                    </FloatLabel>
                }
            })
            .collect_view(),
    )
}

fn iftalabel_n(n: usize) -> String {
    render(
        (0..n)
            .map(|_| {
                view! {
                    <IftaLabel>
                        <input id="u" class="p-inputtext p-component" />
                        <label for="u">"Username"</label>
                    </IftaLabel>
                }
            })
            .collect_view(),
    )
}

fn overlaybadge_n(n: usize) -> String {
    render((0..n).map(|_| view! { <OverlayBadge value="2" severity=BadgeSeverity::Danger><CheckIcon /></OverlayBadge> }).collect_view())
}

fn timeline_n(n: usize) -> String {
    render(
        (0..n)
            .map(|_| {
                let content = Callback::new(|e: &'static str| view! { <b>{e}</b> }.into_any());
                view! { <Timeline value={vec!["Ordered", "Processing", "Shipped", "Delivered"]} content=content /> }
            })
            .collect_view(),
    )
}

fn metergroup_n(n: usize) -> String {
    render(
        (0..n)
            .map(|_| {
                let items = vec![
                    MeterItem::new("Apps", "#34d399", 16.0),
                    MeterItem::new("Messages", "#fbbf24", 8.0),
                    MeterItem::new("Media", "#60a5fa", 24.0),
                ];
                view! { <MeterGroup value={items} /> }
            })
            .collect_view(),
    )
}

fn breadcrumb_n(n: usize) -> String {
    render(
        (0..n)
            .map(|_| {
                let model = ["Electronics", "Computer", "Accessories", "Keyboard"]
                    .into_iter()
                    .map(|l| BreadcrumbItem::new().label(l))
                    .collect::<Vec<_>>();
                view! { <Breadcrumb model=model home=BreadcrumbItem::new() /> }
            })
            .collect_view(),
    )
}

fn inputotp_n(n: usize) -> String {
    render((0..n).map(|_| view! { <InputOtp length=6 /> }).collect_view())
}

fn scrollpanel_n(n: usize) -> String {
    render((0..n).map(|_| view! { <ScrollPanel style="width: 100%; height: 150px"><p>"Content"</p></ScrollPanel> }).collect_view())
}

fn progressspinner_n(n: usize) -> String {
    render((0..n).map(|_| view! { <ProgressSpinner aria_label="loading" /> }).collect_view())
}

fn card_n(n: usize) -> String {
    render((0..n).map(|_| view! { <Card header="Simple Card" subheader="Subtitle"><p>"Content"</p></Card> }).collect_view())
}

/// Все сценарии.
pub const SCENARIOS: &[Scenario] = &[
    Scenario { name: "check-icon-1", run: || check_n(1) },
    Scenario { name: "check-icon-1000", run: || check_n(1000) },
    Scenario { name: "spinner-icon-1000", run: || spinner_n(1000) },
    Scenario { name: "badge-1", run: || badge_n(1) },
    Scenario { name: "badge-1000", run: || badge_n(1000) },
    Scenario { name: "button-1", run: || button_n(1) },
    Scenario { name: "button-1000", run: || button_n(1000) },
    Scenario { name: "button-full-1000", run: || button_full_n(1000) },
    Scenario { name: "tag-1", run: || tag_n(1) },
    Scenario { name: "tag-1000", run: || tag_n(1000) },
    Scenario { name: "chip-1", run: || chip_n(1) },
    Scenario { name: "chip-1000", run: || chip_n(1000) },
    Scenario { name: "chip-removable-1000", run: || chip_removable_n(1000) },
    Scenario { name: "avatar-1", run: || avatar_n(1) },
    Scenario { name: "avatar-1000", run: || avatar_n(1000) },
    Scenario { name: "divider-1", run: || divider_n(1) },
    Scenario { name: "divider-1000", run: || divider_n(1000) },
    Scenario { name: "card-1", run: || card_n(1) },
    Scenario { name: "card-1000", run: || card_n(1000) },
    Scenario { name: "inputtext-1", run: || inputtext_n(1) },
    Scenario { name: "inputtext-1000", run: || inputtext_n(1000) },
    Scenario { name: "checkbox-1", run: || checkbox_n(1) },
    Scenario { name: "checkbox-1000", run: || checkbox_n(1000) },
    Scenario { name: "radiobutton-1", run: || radiobutton_n(1) },
    Scenario { name: "radiobutton-1000", run: || radiobutton_n(1000) },
    Scenario { name: "rating-1", run: || rating_n(1) },
    Scenario { name: "rating-1000", run: || rating_n(1000) },
    Scenario { name: "inputnumber-1", run: || inputnumber_n(1) },
    Scenario { name: "inputnumber-1000", run: || inputnumber_n(1000) },
    Scenario { name: "select-1", run: || select_n(1) },
    Scenario { name: "select-1000", run: || select_n(1000) },
    Scenario { name: "knob-1", run: || knob_n(1) },
    Scenario { name: "knob-1000", run: || knob_n(1000) },
    Scenario { name: "slider-1", run: || slider_n(1) },
    Scenario { name: "slider-1000", run: || slider_n(1000) },
    Scenario { name: "selectbutton-1", run: || selectbutton_n(1) },
    Scenario { name: "selectbutton-1000", run: || selectbutton_n(1000) },
    Scenario { name: "togglebutton-1", run: || togglebutton_n(1) },
    Scenario { name: "togglebutton-1000", run: || togglebutton_n(1000) },
    Scenario { name: "toggleswitch-1", run: || toggleswitch_n(1) },
    Scenario { name: "toggleswitch-1000", run: || toggleswitch_n(1000) },
    Scenario { name: "message-1", run: || message_n(1) },
    Scenario { name: "message-1000", run: || message_n(1000) },
    Scenario { name: "progressbar-1", run: || progressbar_n(1) },
    Scenario { name: "progressbar-1000", run: || progressbar_n(1000) },
    Scenario { name: "avatargroup-1", run: || avatargroup_n(1) },
    Scenario { name: "avatargroup-1000", run: || avatargroup_n(1000) },
    Scenario { name: "skeleton-1", run: || skeleton_n(1) },
    Scenario { name: "skeleton-1000", run: || skeleton_n(1000) },
    Scenario { name: "textarea-1", run: || textarea_n(1) },
    Scenario { name: "textarea-1000", run: || textarea_n(1000) },
    Scenario { name: "panel-1", run: || panel_n(1) },
    Scenario { name: "panel-1000", run: || panel_n(1000) },
    Scenario { name: "panel-plain-1000", run: || panel_plain_n(1000) },
    Scenario { name: "fieldset-1", run: || fieldset_n(1) },
    Scenario { name: "fieldset-1000", run: || fieldset_n(1000) },
    Scenario { name: "accordion-1", run: || accordion_n(1) },
    Scenario { name: "accordion-1000", run: || accordion_n(1000) },
    Scenario { name: "toolbar-1", run: || toolbar_n(1) },
    Scenario { name: "toolbar-1000", run: || toolbar_n(1000) },
    Scenario { name: "inputgroup-1", run: || inputgroup_n(1) },
    Scenario { name: "inputgroup-1000", run: || inputgroup_n(1000) },
    Scenario { name: "buttongroup-1", run: || buttongroup_n(1) },
    Scenario { name: "buttongroup-1000", run: || buttongroup_n(1000) },
    Scenario { name: "fluid-1", run: || fluid_n(1) },
    Scenario { name: "fluid-1000", run: || fluid_n(1000) },
    Scenario { name: "iconfield-1", run: || iconfield_n(1) },
    Scenario { name: "iconfield-1000", run: || iconfield_n(1000) },
    Scenario { name: "floatlabel-1", run: || floatlabel_n(1) },
    Scenario { name: "floatlabel-1000", run: || floatlabel_n(1000) },
    Scenario { name: "iftalabel-1", run: || iftalabel_n(1) },
    Scenario { name: "iftalabel-1000", run: || iftalabel_n(1000) },
    Scenario { name: "overlaybadge-1", run: || overlaybadge_n(1) },
    Scenario { name: "overlaybadge-1000", run: || overlaybadge_n(1000) },
    Scenario { name: "timeline-1", run: || timeline_n(1) },
    Scenario { name: "timeline-1000", run: || timeline_n(1000) },
    Scenario { name: "metergroup-1", run: || metergroup_n(1) },
    Scenario { name: "metergroup-1000", run: || metergroup_n(1000) },
    Scenario { name: "breadcrumb-1", run: || breadcrumb_n(1) },
    Scenario { name: "breadcrumb-1000", run: || breadcrumb_n(1000) },
    Scenario { name: "inputotp-1", run: || inputotp_n(1) },
    Scenario { name: "inputotp-1000", run: || inputotp_n(1000) },
    Scenario { name: "scrollpanel-1", run: || scrollpanel_n(1) },
    Scenario { name: "scrollpanel-1000", run: || scrollpanel_n(1000) },
    Scenario { name: "progressspinner-1", run: || progressspinner_n(1) },
    Scenario { name: "progressspinner-1000", run: || progressspinner_n(1000) },
    Scenario { name: "all-icons-1", run: || render(view! { <AllIcons /> }) },
    Scenario { name: "all-icons-20", run: || render((0..20).map(|_| view! { <AllIcons /> }).collect_view()) },
];

/// Каталог для JSON-отчётов (`target/bench`).
pub fn out_dir() -> std::path::PathBuf {
    let target = std::env::var_os("CARGO_TARGET_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target"));
    let dir = target.join("bench");
    std::fs::create_dir_all(&dir).expect("не удалось создать target/bench");
    dir
}
