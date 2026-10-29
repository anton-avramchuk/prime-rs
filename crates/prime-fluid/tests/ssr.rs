//! Структура DOM, классы, `style`, `pt`, паритет ветвей Fluid и наследование `fluid` потомками (SSR).
#![cfg(feature = "ssr")]
#![recursion_limit = "512"]
use leptos::prelude::*;
use prime_core::Pt;
use prime_fluid::{Fluid, FluidPt};

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

fn norm(s: &str) -> String {
    s.replace("<!>", "").replace("<!---->", "")
}

#[test]
fn root_wraps_children_in_order() {
    let html = view! { <Fluid><input class="first" /><label class="second" /></Fluid> }.to_html();
    assert!(html.starts_with("<div"), "{html}");
    has(&html, &[r#"class="p-fluid""#]);
    assert!(pos(&html, "first") < pos(&html, "second"), "{html}");
}

#[test]
fn empty_renders_root_only() {
    let html = view! { <Fluid /> }.to_html();
    has(&html, &[r#"class="p-fluid""#]);
    lacks(&html, &["style="]);
}

#[test]
fn style_and_class() {
    let html = view! { <Fluid style="width: 20rem" style_class="mine" /> }.to_html();
    has(&html, &[r#"class="p-fluid mine""#, r#"style="width: 20rem;""#]);
}

#[test]
fn pt_root_adds_without_replacing() {
    let pt = FluidPt { root: Pt::new().class("custom").attr("data-x", "1").style("margin: 0") };
    let html = view! { <Fluid style_class="mine" pt=pt /> }.to_html();
    has(&html, &[r#"class="p-fluid mine custom""#, r#"data-x="1""#, "margin: 0"]);
}

#[test]
fn static_and_reactive_branches_render_the_same() {
    let fixed = view! { <Fluid style="color: red" style_class="m">"x"</Fluid> }.to_html();
    let live = view! {
        <Fluid style=RwSignal::new(Some("color: red".to_owned())) style_class=RwSignal::new(Some("m".to_owned()))>"x"</Fluid>
    }
    .to_html();
    assert_eq!(norm(&fixed), norm(&live), "{fixed}\n{live}");
}

#[test]
fn all_consumers_inherit() {
    Owner::new().with(|| {
        use prime_button::Button;
        use prime_inputnumber::InputNumber;
        use prime_inputtext::InputText;
        use prime_select::{Select, SelectOption};
        use prime_textarea::Textarea;

        let options = vec![SelectOption::new("a", "A")];
        let html = view! {
            <Fluid>
                <Button label="b" />
                <InputText />
                <Textarea />
                <InputNumber />
                <Select options=options />
            </Fluid>
        }
        .to_html();
        has(&html, &["p-button-fluid", "p-inputtext-fluid", "p-textarea-fluid", "p-inputnumber-fluid", "p-select-fluid"]);
    });
}

#[test]
fn explicit_false_wins_and_nesting_works() {
    Owner::new().with(|| {
        use prime_inputtext::InputText;

        let html = view! { <Fluid><div><section><InputText fluid=false /><InputText /></section></div></Fluid> }.to_html();
        assert_eq!(html.matches("p-inputtext-fluid").count(), 1, "{html}");
        let outside = view! { <div><InputText /></div> }.to_html();
        lacks(&outside, &["p-inputtext-fluid"]);
    });
}

#[test]
fn reactive_fluid_inside_fluid() {
    Owner::new().with(|| {
        use prime_inputtext::InputText;

        let live = RwSignal::new(Some(false));
        let html = view! { <Fluid><InputText fluid=live /></Fluid> }.to_html();
        lacks(&html, &["p-inputtext-fluid"]);
        live.set(None);
        let html = view! { <Fluid><InputText fluid=live /></Fluid> }.to_html();
        has(&html, &["p-inputtext-fluid"]);
    });
}

#[test]
fn sibling_outside_fluid_is_not_affected() {
    Owner::new().with(|| {
        use prime_inputtext::InputText;

        let html = view! { <div><Fluid><InputText /></Fluid><InputText /></div> }.to_html();
        assert_eq!(html.matches("p-inputtext-fluid").count(), 1, "{html}");
    });
}
