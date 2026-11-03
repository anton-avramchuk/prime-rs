//! Разметка SpeedDial (SSR): корень по типу и направлению, кнопка, пункты, маска, шаблоны, `pt`.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_core::Pt;
use prime_icons::PlusIcon;
use prime_speeddial::{SpeedDial, SpeedDialDirection, SpeedDialItem, SpeedDialPt, SpeedDialType};
use prime_tooltip::TooltipOptions;

fn render(f: impl FnOnce() -> String) -> String {
    Owner::new().with(f)
}

fn items() -> Vec<SpeedDialItem> {
    vec![
        SpeedDialItem::new().label("Add").icon(ViewFn::from(|| view! { <PlusIcon /> })),
        SpeedDialItem::new().label("Delete").disabled(true),
        SpeedDialItem::new().label("Hidden").visible(false).style_class("extra"),
    ]
}

#[test]
fn markup_has_root_button_list_and_items() {
    let html = render(|| view! { <SpeedDial id="sd" model=items() /> }.to_html());
    assert!(html.contains("p-speeddial p-component p-speeddial-linear p-speeddial-direction-up"), "{html}");
    assert!(html.contains("align-items: center") && html.contains("flex-direction: column-reverse"), "{html}");
    assert!(html.contains("p-speeddial-button") && html.contains("p-button-icon-only") && html.contains("p-button-rounded"), "{html}");
    assert!(html.contains("p-speeddial-rotate"), "вращение по умолчанию: {html}");
    assert!(
        html.contains("aria-expanded=\"false\"") && html.contains("aria-haspopup=\"true\"") && html.contains("aria-controls=\"sd_list\""),
        "{html}"
    );
    assert!(
        html.contains("<ul")
            && html.contains("id=\"sd_list\"")
            && html.contains("class=\"p-speeddial-list\"")
            && html.contains("tabindex=\"-1\"")
            && html.contains("role=\"menu\""),
        "{html}"
    );
    assert_eq!(html.matches("role=\"menuitem\"").count(), 6, "li и кнопка действия у каждого из трёх пунктов: {html}");
    assert!(html.contains("id=\"sd_0\"") && html.contains("id=\"sd_2\""), "{html}");
    assert!(!html.contains("aria-activedescendant") && !html.contains("p-speeddial-mask") && !html.contains("p-speeddial-open"), "{html}");
}

#[test]
fn items_carry_state_and_action_buttons() {
    let html = render(|| view! { <SpeedDial id="sd" model=items() /> }.to_html());
    assert_eq!(html.matches("p-speeddial-action").count(), 3, "{html}");
    assert!(html.contains("p-hidden") && html.contains("extra"), "скрытый пункт: {html}");
    assert!(html.contains("aria-label=\"Add\"") && html.contains("aria-label=\"Delete\""), "{html}");
    assert_eq!(html.matches("disabled=\"\"").count(), 1, "отключён один пункт: {html}");
    assert!(html.contains("p-button-secondary") && html.contains("p-button-sm"), "{html}");
    assert!(html.contains("transition-delay: 60ms") || html.contains("transition-delay: 0ms"), "{html}");
    assert!(!html.contains("tabindex=\"0\""), "список скрыт: у действий нет tabindex");
}

#[test]
fn types_and_directions_set_classes_and_layout() {
    let circle = render(|| view! { <SpeedDial model=items() r#type=SpeedDialType::Circle radius=80.0 /> }.to_html());
    assert!(circle.contains("p-speeddial-circle") && !circle.contains("p-speeddial-direction-"), "{circle}");
    assert!(circle.contains("left: calc(80px + var(--item-diff-x, 0px))"), "{circle}");
    for (kind, direction, class, side) in [
        (SpeedDialType::SemiCircle, SpeedDialDirection::Up, "p-speeddial-semi-circle p-speeddial-direction-up", "bottom: calc("),
        (SpeedDialType::SemiCircle, SpeedDialDirection::Right, "p-speeddial-direction-right", "top: calc("),
        (
            SpeedDialType::QuarterCircle,
            SpeedDialDirection::DownLeft,
            "p-speeddial-quarter-circle p-speeddial-direction-down-left",
            "right: calc(",
        ),
    ] {
        let html = render(|| view! { <SpeedDial model=items() r#type=kind direction=direction /> }.to_html());
        assert!(html.contains(class) && html.contains(side), "{class}: {html}");
    }
    let left = render(|| view! { <SpeedDial model=items() direction=SpeedDialDirection::Left /> }.to_html());
    assert!(left.contains("justify-content: center") && left.contains("flex-direction: row-reverse"), "{left}");
}

#[test]
fn open_disabled_and_mask_follow_props() {
    let html = render(|| {
        let visible = RwSignal::new(true);
        view! { <SpeedDial model=items() visible=visible mask=true disabled=true mask_class="my-mask" /> }.to_html()
    });
    assert!(html.contains("p-speeddial-open") && html.contains("p-disabled"), "{html}");
    assert!(html.contains("aria-expanded=\"true\"") && html.contains("p-speeddial-mask") && html.contains("my-mask"), "{html}");
    assert!(html.contains("tabindex=\"0\""), "список показан: у доступных действий tabindex 0: {html}");
}

#[test]
fn icons_labels_severity_and_classes() {
    let html = render(|| {
        let visible = RwSignal::new(true);
        view! {
            <SpeedDial
                model=items()
                visible=visible
                show_icon=ViewFn::from(|| view! { <i class="show-ico" /> })
                hide_icon=ViewFn::from(|| view! { <i class="hide-ico" /> })
                aria_label="Actions"
                aria_labelledby="lbl"
                button_class="btn-extra"
                style_class="mine"
                style="position: absolute"
            />
        }
        .to_html()
    });
    assert!(html.contains("hide-ico") && !html.contains("show-ico"), "открыт: значок скрытия: {html}");
    assert!(!html.contains("p-speeddial-rotate"), "с hide_icon вращения нет: {html}");
    assert!(html.contains("aria-label=\"Actions\"") && html.contains("aria-labelledby=\"lbl\""), "{html}");
    assert!(html.contains("btn-extra") && html.contains("mine") && html.contains("position: absolute"), "{html}");
    let closed = render(|| view! { <SpeedDial model=items() show_icon=ViewFn::from(|| view! { <i class="show-ico" /> }) /> }.to_html());
    assert!(
        closed.contains("show-ico") && closed.matches("<svg").count() == 1,
        "значок кнопки заменён, остался только значок пункта: {closed}"
    );
}

#[test]
fn templates_replace_button_item_and_icon() {
    let html = render(|| {
        view! {
            <SpeedDial
                model=items()
                button_content=Callback::new(|_toggle| view! { <span class="my-toggler">"T"</span> }.into_any())
                item_content=Callback::new(|ctx: prime_speeddial::SpeedDialItemContext| {
                    view! { <b class="my-item">{ctx.item.label.unwrap_or_default()}</b> }.into_any()
                })
            />
        }
        .to_html()
    });
    assert!(html.contains("my-toggler") && !html.contains("p-speeddial-button"), "{html}");
    assert_eq!(html.matches("class=\"my-item\"").count(), 3, "{html}");
    assert!(!html.contains("p-speeddial-action"), "{html}");
    let icon = render(|| view! { <SpeedDial model=items() icon_content=ViewFn::from(|| view! { <i class="own-ico" /> }) /> }.to_html());
    assert!(icon.contains("own-ico") && icon.matches("<svg").count() == 1, "{icon}");
}

#[test]
fn tooltip_template_does_not_change_the_markup() {
    let plain = render(|| view! { <SpeedDial id="a" model=items() /> }.to_html());
    let tipped = render(|| view! { <SpeedDial id="a" model=items() tooltip_options=TooltipOptions::text("") /> }.to_html());
    assert_eq!(plain.replace("pn_id_", ""), tipped.replace("pn_id_", ""), "директива работает только в браузере");
}

#[test]
fn pt_adds_classes_and_attributes() {
    let pt = SpeedDialPt {
        root: Pt::new().class("r").attr("data-x", "1"),
        button: Pt::new().class("b"),
        list: Pt::new().class("l"),
        item: Pt::new().class("i"),
        action: Pt::new().class("a"),
        ..SpeedDialPt::default()
    };
    let html = render(|| view! { <SpeedDial model=items() pt=pt /> }.to_html());
    assert!(html.contains("data-x=\"1\"") && html.contains("p-speeddial-list l") && html.contains("p-speeddial-item i"), "{html}");
    assert!(html.contains("p-speeddial-action a") && html.contains("p-speeddial-rotate b"), "{html}");
}

#[test]
fn empty_model_renders_an_empty_list() {
    let html = render(|| view! { <SpeedDial /> }.to_html());
    assert!(html.contains("p-speeddial-list") && !html.contains("role=\"menuitem\""), "{html}");
}
