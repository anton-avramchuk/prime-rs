//! Структура DOM, классы, ARIA, слот иконки, `pt` и начальная скрытость Accordion (SSR).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_accordion::{Accordion, AccordionContent, AccordionHeader, AccordionPanel, AccordionPt};
use prime_core::Pt;

fn has(html: &str, wants: &[&str]) {
    for want in wants {
        assert!(html.contains(want), "{want}: {html}");
    }
}

fn lacks(html: &str, unwanted: &[&str]) {
    for u in unwanted {
        assert!(!html.contains(u), "{u}: {html}");
    }
}

fn pos(html: &str, needle: &str) -> usize {
    html.find(needle).unwrap_or_else(|| panic!("нет {needle}: {html}"))
}

macro_rules! three {
    ($($k:ident = $v:expr),* $(,)?) => {
        Owner::new().with(|| view! {
            <Accordion id="a" $($k=$v)*>
                <AccordionPanel value="0">
                    <AccordionHeader>"Header I"</AccordionHeader>
                    <AccordionContent><p>"Content I"</p></AccordionContent>
                </AccordionPanel>
                <AccordionPanel value="1">
                    <AccordionHeader>"Header II"</AccordionHeader>
                    <AccordionContent><p>"Content II"</p></AccordionContent>
                </AccordionPanel>
                <AccordionPanel value="2" disabled=true>
                    <AccordionHeader>"Header III"</AccordionHeader>
                    <AccordionContent><p>"Content III"</p></AccordionContent>
                </AccordionPanel>
            </Accordion>
        }
        .to_html())
    };
}

#[test]
fn structure_classes_and_order() {
    let html = three!(default_value = vec!["0".to_owned()]);
    assert!(html.starts_with("<div"), "{html}");
    has(
        &html,
        &[
            "p-accordion p-component",
            "p-accordionpanel",
            "p-accordionheader",
            "p-accordionheader-toggle-icon",
            "p-accordioncontent",
            "p-motion",
            "p-accordioncontent-wrapper",
            "p-accordioncontent-content",
            r#"role="button""#,
            r#"role="region""#,
            "user-select: none",
        ],
    );
    let (panel, header, content, motion, wrapper, inner) = (
        pos(&html, "p-accordionpanel"),
        pos(&html, "p-accordionheader\""),
        pos(&html, "p-accordioncontent\""),
        pos(&html, "p-motion"),
        pos(&html, "p-accordioncontent-wrapper"),
        pos(&html, "p-accordioncontent-content"),
    );
    assert!(panel < header && header < content && content < motion && motion < wrapper && wrapper < inner, "{html}");
    assert_eq!(html.matches("p-accordionpanel-active").count(), 1, "активна одна панель: {html}");
    assert_eq!(html.matches(r#"class="p-accordionpanel p-disabled""#).count(), 1, "отключена одна панель: {html}");
}

#[test]
fn aria_ids_and_state() {
    let html = three!(default_value = vec!["0".to_owned()]);
    has(
        &html,
        &[
            r#"id="a_accordionheader_0""#,
            r#"id="a_accordioncontent_0""#,
            r#"aria-controls="a_accordioncontent_0""#,
            r#"aria-labelledby="a_accordionheader_0""#,
            r#"aria-expanded="true""#,
            r#"aria-expanded="false""#,
            r#"aria-disabled="true""#,
            r#"aria-disabled="false""#,
            r#"tabindex="0""#,
            r#"tabindex="-1""#,
            r#"data-p-active="true""#,
            r#"data-p-disabled="true""#,
            r#"data-p="active""#,
        ],
    );
    assert_eq!(html.matches(r#"data-p="active""#).count(), 1, "{html}");
}

#[test]
fn only_active_content_is_visible_from_the_first_render() {
    let html = three!(default_value = vec!["1".to_owned()]);
    assert_eq!(html.matches("display: none").count(), 2, "скрыты содержимое неактивных панелей: {html}");
    let hidden_first = html.find("display: none").unwrap();
    assert!(hidden_first < pos(&html, "Content I<"), "панель 0 скрыта до своего содержимого: {html}");
    let all_open = three!(multiple = true, default_value = vec!["0".to_owned(), "1".to_owned(), "2".to_owned()]);
    lacks(&all_open, &["display: none"]);
}

#[test]
fn chevron_follows_state() {
    let open = three!(default_value = vec!["0".to_owned()]);
    let closed = three!();
    let icon = |s: &str| s.split("<svg").nth(1).and_then(|r| r.split("</svg>").next()).map(str::to_owned);
    assert!(icon(&open).is_some() && icon(&closed).is_some());
    assert_ne!(icon(&open), icon(&closed), "шеврон вверх у активной и вниз у неактивной");
    assert_eq!(open.matches("p-accordionheader-toggle-icon").count(), 3, "иконка у каждого заголовка: {open}");
}

#[test]
fn external_signal_sets_state() {
    let value = RwSignal::new(vec!["1".to_owned()]);
    let html = three!(value = value);
    let (active, header2) = (pos(&html, "p-accordionpanel p-accordionpanel-active"), pos(&html, "Header II"));
    assert!(pos(&html, "Header I<") < active || active < header2, "{html}");
    assert_eq!(html.matches("p-accordionpanel-active").count(), 1);
}

#[test]
fn toggle_icon_template_replaces_chevron() {
    let tpl = Callback::new(|active: bool| view! { <u class="own">{if active { "open" } else { "closed" }}</u> }.into_any());
    let html = Owner::new().with(|| {
        view! {
            <Accordion default_value=vec!["0".to_owned()]>
                <AccordionPanel value="0">
                    <AccordionHeader toggle_icon_template=tpl>"H"</AccordionHeader>
                    <AccordionContent>"C"</AccordionContent>
                </AccordionPanel>
            </Accordion>
        }
        .to_html()
    });
    has(&html, &["class=\"own\"", "open"]);
    lacks(&html, &["<svg"]);
}

#[test]
fn style_class_is_merged_on_every_part() {
    let html = Owner::new().with(|| {
        view! {
            <Accordion style_class="r" default_value=vec!["0".to_owned()]>
                <AccordionPanel value="0" style_class="p">
                    <AccordionHeader style_class="h">"H"</AccordionHeader>
                    <AccordionContent style_class="c">"C"</AccordionContent>
                </AccordionPanel>
            </Accordion>
        }
        .to_html()
    });
    has(&html, &["p-accordion p-component r", "p-accordionpanel p-accordionpanel-active p", "p-accordionheader h", "p-accordioncontent c"]);
}

#[test]
fn pt_parts() {
    let pt = AccordionPt {
        root: Pt::new().class("r").attr("data-testid", "root"),
        panel: Pt::new().class("p"),
        header: Pt::new().class("h"),
        toggle_icon: Pt::new().class("i"),
        content_container: Pt::new().class("cc"),
        content_wrapper: Pt::new().class("cw"),
        content: Pt::new().class("c"),
    };
    let html = Owner::new().with(|| {
        view! {
            <Accordion pt=pt>
                <AccordionPanel value="0">
                    <AccordionHeader>"H"</AccordionHeader>
                    <AccordionContent>"C"</AccordionContent>
                </AccordionPanel>
            </Accordion>
        }
        .to_html()
    });
    has(
        &html,
        &[
            "p-accordion p-component r",
            r#"data-testid="root""#,
            "p-accordionpanel p",
            "p-accordionheader h",
            "p-accordionheader-toggle-icon i",
            "p-accordioncontent cc",
            "p-accordioncontent-wrapper cw",
            "p-accordioncontent-content c",
        ],
    );
}

#[test]
fn unique_ids_without_explicit_id() {
    let one = Owner::new().with(|| {
        view! {
            <Accordion>
                <AccordionPanel value="0">
                    <AccordionHeader>"H"</AccordionHeader>
                    <AccordionContent>"C"</AccordionContent>
                </AccordionPanel>
            </Accordion>
        }
        .to_html()
    });
    let id = |s: &str| s.split("id=\"").nth(1).and_then(|r| r.split('"').next()).map(str::to_owned).unwrap();
    assert!(id(&one).starts_with("pn_id_") && id(&one).ends_with("_accordionheader_0"), "{one}");
}
