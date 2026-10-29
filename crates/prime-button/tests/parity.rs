//! Паритет статической и реактивной ветвей Button: одни и те же значения, переданные значением и сигналом, дают одинаковый HTML.
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use prime_badge::BadgeSeverity;
use prime_button::{Button, ButtonPt, ButtonSeverity, ButtonVariant, IconPos};
use prime_core::{OptProp, Prop, Pt, Size};
use prime_icons::CheckIcon;

fn icon() -> ViewFn {
    ViewFn::from(|| view! { <CheckIcon /> })
}

/// Значения всех свойств одного сценария.
#[derive(Clone)]
struct Case {
    label: Option<&'static str>,
    badge: Option<&'static str>,
    badge_severity: Option<BadgeSeverity>,
    severity: Option<ButtonSeverity>,
    variant: Option<ButtonVariant>,
    size: Option<Size>,
    icon_pos: IconPos,
    loading: bool,
    disabled: bool,
    flags: [bool; 6], // text, outlined, plain, link, raised, rounded
    fluid: bool,
    with_icon: bool,
    aria: Option<&'static str>,
    tabindex: Option<i32>,
    ty: Option<&'static str>,
    style: Option<&'static str>,
    class: Option<&'static str>,
}

impl Case {
    fn base() -> Self {
        Self {
            label: None,
            badge: None,
            badge_severity: None,
            severity: None,
            variant: None,
            size: None,
            icon_pos: IconPos::Left,
            loading: false,
            disabled: false,
            flags: [false; 6],
            fluid: false,
            with_icon: false,
            aria: None,
            tabindex: None,
            ty: None,
            style: None,
            class: None,
        }
    }
}

fn cases() -> Vec<Case> {
    let b = Case::base;
    vec![
        b(),
        Case { label: Some("Сохранить"), ..b() },
        Case {
            label: Some("x"),
            severity: Some(ButtonSeverity::Danger),
            variant: Some(ButtonVariant::Outlined),
            size: Some(Size::Small),
            ..b()
        },
        Case { with_icon: true, ..b() },
        Case { label: Some("x"), with_icon: true, icon_pos: IconPos::Top, ..b() },
        Case { label: Some("x"), with_icon: true, icon_pos: IconPos::Right, badge: Some("3"), ..b() },
        Case { label: Some("x"), badge: Some("9"), badge_severity: Some(BadgeSeverity::Danger), ..b() },
        Case { label: Some("x"), with_icon: true, loading: true, ..b() },
        Case { label: Some("x"), disabled: true, aria: Some("Закрыть"), tabindex: Some(2), ty: Some("submit"), ..b() },
        Case { label: Some("x"), flags: [true, true, true, true, true, true], fluid: true, size: Some(Size::Large), ..b() },
        Case { label: Some("x"), style: Some("margin: 4px"), class: Some("custom extra"), ..b() },
    ]
}

macro_rules! button_with {
    ($c:expr, $mk:expr, $($icon:ident)?) => {{
        let c: &Case = $c;
        let o = $mk;
        let s = |v: Option<&str>| v.map(str::to_owned);
        view! {
            <Button
                label=o.opt_s(s(c.label))
                badge=o.opt_s(s(c.badge))
                badge_severity=o.v(c.badge_severity)
                severity=o.v(c.severity)
                variant=o.v(c.variant)
                size=o.v(c.size)
                icon_pos=o.p(c.icon_pos)
                loading=o.p(c.loading)
                disabled=o.p(c.disabled)
                text=o.p(c.flags[0])
                outlined=o.p(c.flags[1])
                plain=o.p(c.flags[2])
                link=o.p(c.flags[3])
                raised=o.p(c.flags[4])
                rounded=o.p(c.flags[5])
                fluid=o.v(Some(c.fluid))
                aria_label=o.opt_s(s(c.aria))
                tabindex=o.v(c.tabindex)
                button_type=o.opt_s(s(c.ty))
                style=o.opt_s(s(c.style))
                style_class=o.opt_s(s(c.class))
                pt=ButtonPt { label: Pt::new().class("pl"), ..Default::default() }
                $(icon=$icon())?
            />
        }
        .to_html()
    }};
}

/// Источник значений: готовые значения (статический путь).
struct Fixed;
impl Fixed {
    fn p<T: Send + Sync + 'static>(&self, v: T) -> Prop<T> {
        Prop::Static(v)
    }
    fn v<T: Send + Sync + 'static>(&self, v: Option<T>) -> OptProp<T> {
        Prop::Static(v)
    }
    fn opt_s(&self, v: Option<String>) -> OptProp<String> {
        Prop::Static(v)
    }
}

/// Источник значений: сигналы (реактивный путь).
struct Live;
impl Live {
    fn p<T: Send + Sync + 'static>(&self, v: T) -> Prop<T> {
        Prop::Reactive(RwSignal::new(v).into())
    }
    fn v<T: Send + Sync + 'static>(&self, v: Option<T>) -> OptProp<T> {
        Prop::Reactive(RwSignal::new(v).into())
    }
    fn opt_s(&self, v: Option<String>) -> OptProp<String> {
        Prop::Reactive(RwSignal::new(v).into())
    }
}

fn fixed(c: &Case) -> String {
    if c.with_icon { button_with!(c, Fixed, icon) } else { button_with!(c, Fixed,) }
}

fn live(c: &Case) -> String {
    if c.with_icon { button_with!(c, Live, icon) } else { button_with!(c, Live,) }
}

#[test]
fn every_case_renders_identically_static_and_reactive() {
    let owner = Owner::new();
    owner.with(|| {
        for (i, c) in cases().iter().enumerate() {
            assert_eq!(fixed(c), live(c), "сценарий #{i}");
        }
    });
}

#[test]
fn one_signal_is_enough_to_switch_to_the_reactive_path_without_changing_output() {
    let owner = Owner::new();
    owner.with(|| {
        let static_html = view! { <Button label="x" severity=ButtonSeverity::Success badge="1" /> }.to_html();
        let one_signal = view! { <Button label="x" severity=ButtonSeverity::Success badge="1" loading=RwSignal::new(false) /> }.to_html();
        assert_eq!(static_html, one_signal);
    });
}
