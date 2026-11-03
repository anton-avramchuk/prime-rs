//! SplitButton — порт `p-splitbutton` из PrimeNG 21.1.10.
//!
//! Кнопка с действием по умолчанию (`p-splitbutton-button`, это `Button`), кнопка-стрелка (`p-splitbutton-dropdown`) и всплывающее
//! меню команд (`Menu` в режиме `popup`), привязанное ко всему компоненту. В PrimeNG меню — `TieredMenu` с подменю; здесь пока
//! плоский `Menu` с группами, вложенные подменю появятся вместе с `TieredMenu`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_menu::MenuItem;
//! use prime_splitbutton::SplitButton;
//!
//! let model = vec![MenuItem::new().label("Update"), MenuItem::new().label("Delete")];
//! let html = Owner::new().with(|| view! { <SplitButton label="Save" model=model /> }.to_html());
//! assert!(html.contains("p-splitbutton-button") && html.contains("p-splitbutton-dropdown") && html.contains("aria-expanded=\"false\""));
//! ```

mod state;

use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::attr::custom::custom_attribute;
use leptos::html;
use leptos::prelude::*;
use prime_button::{Button, ButtonSeverity, ButtonState, IconPos, root_classes as button_classes, root_data_p as button_data_p};
use prime_core::{ClassNames, OptProp, Prop, Pt, Size};
use prime_icons::ChevronDownIcon;
use prime_menu::{Menu, MenuItem, MenuRef};
use std::sync::Arc;

pub use state::{is_toggle_key, root_classes};

/// Pass-through частей SplitButton.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SplitButtonPt {
    /// Корень.
    pub root: Pt,
    /// Основная кнопка (классы и атрибуты добавляются к `Button`).
    pub button: Pt,
    /// Кнопка раскрытия.
    pub dropdown: Pt,
}

/// Кнопка с действием по умолчанию и раскрывающимся меню команд.
///
/// Каждое свойство принимает значение или сигнал.
#[component]
pub fn SplitButton(
    /// Команды меню.
    #[prop(optional, into)]
    model: Prop<Vec<MenuItem>>,
    /// Серьёзность обеих кнопок.
    #[prop(optional, into)]
    severity: OptProp<ButtonSeverity>,
    /// С тенью (класс корня).
    #[prop(optional, into)]
    raised: Prop<bool>,
    /// Скруглённые края (класс корня).
    #[prop(optional, into)]
    rounded: Prop<bool>,
    /// Текстовые кнопки.
    #[prop(optional, into)]
    text: Prop<bool>,
    /// Кнопки с рамкой.
    #[prop(optional, into)]
    outlined: Prop<bool>,
    /// Простой текст.
    #[prop(optional, into)]
    plain: Prop<bool>,
    /// Размер.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Значок основной кнопки.
    #[prop(optional, into)]
    icon: Option<ViewFn>,
    /// Положение значка основной кнопки.
    #[prop(optional, into)]
    icon_pos: Prop<IconPos>,
    /// Метка основной кнопки.
    #[prop(optional, into)]
    label: OptProp<String>,
    /// Атрибут `aria-label` основной кнопки.
    #[prop(optional, into)]
    button_aria_label: OptProp<String>,
    /// Атрибут `aria-label` кнопки раскрытия.
    #[prop(optional, into)]
    expand_aria_label: OptProp<String>,
    /// Значок кнопки раскрытия вместо стрелки.
    #[prop(optional, into)]
    dropdown_icon: Option<ViewFn>,
    /// Содержимое основной кнопки вместо метки и значка.
    #[prop(optional, into)]
    content: Option<ViewFn>,
    /// Обе кнопки недоступны.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Основная кнопка недоступна.
    #[prop(optional, into)]
    button_disabled: Prop<bool>,
    /// Кнопка раскрытия недоступна.
    #[prop(optional, into)]
    menu_button_disabled: Prop<bool>,
    /// Атрибут `tabindex` основной кнопки.
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// Фокус на основную кнопку при показе.
    #[prop(optional)]
    autofocus: bool,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Встроенный стиль меню.
    #[prop(optional, into)]
    menu_style: OptProp<String>,
    /// Дополнительный класс меню.
    #[prop(optional, into)]
    menu_style_class: OptProp<String>,
    /// Клик по основной кнопке.
    #[prop(optional, into)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Клик по кнопке раскрытия.
    #[prop(optional, into)]
    on_dropdown_click: Option<Callback<web_sys::MouseEvent>>,
    /// Меню показано.
    #[prop(optional, into)]
    on_menu_show: Option<Callback<()>>,
    /// Меню скрыто.
    #[prop(optional, into)]
    on_menu_hide: Option<Callback<()>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<SplitButtonPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let menu_id = format!("pn_id_{}", prime_core::ids::next_unique());
    let menu_ref = MenuRef::new();
    let expanded = RwSignal::new(false);
    let root = NodeRef::<html::Div>::new();

    let toggle = move || {
        if menu_ref.visible_untracked() {
            menu_ref.hide();
        } else if let Some(el) = root.get_untracked() {
            menu_ref.show_at(&el);
        }
    };
    let main_click = Callback::new(move |e: web_sys::MouseEvent| {
        if let Some(cb) = on_click {
            cb.run(e);
        }
        menu_ref.hide();
    });
    let dropdown_click = move |e: web_sys::MouseEvent| {
        if let Some(cb) = on_dropdown_click {
            cb.run(e);
        }
        toggle();
    };
    let dropdown_key = move |e: web_sys::KeyboardEvent| {
        if is_toggle_key(&e.code()) {
            toggle();
            e.prevent_default();
        }
    };
    let on_show = Callback::new(move |()| {
        expanded.set(true);
        if let Some(cb) = on_menu_show {
            cb.run(());
        }
    });
    let on_hide = Callback::new(move |()| {
        expanded.set(false);
        if let Some(cb) = on_menu_hide {
            cb.run(());
        }
    });

    let class = {
        let (pt, outlined, text, size) = (pt.clone(), outlined.clone(), text.clone(), size.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add(&root_classes(raised.get(), rounded.get(), outlined.get(), text.get(), size.get()));
            style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let root_style = {
        let pt = pt.clone();
        move || {
            let merged = style.with(|s| pt.root.merge_style(s.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then_some(merged)
        }
    };
    let data_severity = {
        let severity = severity.clone();
        move || severity.get().map(ButtonSeverity::as_str)
    };
    let main_disabled = {
        let disabled = disabled.clone();
        Signal::derive(move || disabled.get() || button_disabled.get())
    };
    let menu_disabled = Signal::derive(move || disabled.get() || menu_button_disabled.get());
    let main_class = pt.button.merge_class("p-splitbutton-button").into_owned();
    let root_attrs = pt.root.attrs();

    let main = main_button(
        MainParts {
            label: label.clone(),
            severity: severity.clone(),
            text: text.clone(),
            outlined: outlined.clone(),
            plain: plain.clone(),
            size: size.clone(),
            icon_pos: icon_pos.clone(),
            tabindex,
            button_aria_label: button_aria_label.clone(),
            disabled: main_disabled,
            autofocus,
            class: main_class,
            on_click: main_click,
        },
        icon.clone(),
        content.clone(),
    );
    let dropdown = dropdown_button(
        DropdownParts {
            severity,
            text,
            outlined,
            plain,
            size,
            expand_aria_label,
            expanded,
            menu_id: menu_id.clone(),
            disabled: menu_disabled,
            pt: pt.clone(),
            dropdown_icon,
        },
        dropdown_click,
        dropdown_key,
    );

    view! {
        <div node_ref=root class={class} style={root_style} data-p-severity={data_severity} {..root_attrs}>
            {main}
            {dropdown}
            <Menu popup=true menu_ref=menu_ref model=model id=menu_id style=menu_style style_class=menu_style_class on_show=on_show on_hide=on_hide />
        </div>
    }
}

/// Свойства основной кнопки.
struct MainParts {
    label: OptProp<String>,
    severity: OptProp<ButtonSeverity>,
    text: Prop<bool>,
    outlined: Prop<bool>,
    plain: Prop<bool>,
    size: OptProp<Size>,
    icon_pos: Prop<IconPos>,
    tabindex: OptProp<i32>,
    button_aria_label: OptProp<String>,
    disabled: Signal<bool>,
    autofocus: bool,
    class: String,
    on_click: Callback<web_sys::MouseEvent>,
}

/// Основная кнопка: `Button` с учётом необязательных `icon` и `content` (четыре варианта разметки).
fn main_button(p: MainParts, icon: Option<ViewFn>, content: Option<ViewFn>) -> AnyView {
    macro_rules! button {
        ($($extra:tt)*) => {
            view! {
                <Button
                    label=p.label.clone()
                    severity=p.severity.clone()
                    text=p.text.clone()
                    outlined=p.outlined.clone()
                    plain=p.plain.clone()
                    size=p.size.clone()
                    icon_pos=p.icon_pos.clone()
                    tabindex=p.tabindex.clone()
                    aria_label=p.button_aria_label.clone()
                    disabled=p.disabled
                    autofocus=p.autofocus
                    style_class=p.class.clone()
                    on_click=p.on_click
                    $($extra)*
                />
            }
            .into_any()
        };
    }
    match (icon, content) {
        (Some(icon), Some(content)) => view! {
            <Button
                severity=p.severity.clone()
                text=p.text.clone()
                outlined=p.outlined.clone()
                plain=p.plain.clone()
                size=p.size.clone()
                icon_pos=p.icon_pos.clone()
                tabindex=p.tabindex.clone()
                aria_label=p.button_aria_label.clone()
                disabled=p.disabled
                autofocus=p.autofocus
                style_class=p.class.clone()
                on_click=p.on_click
                icon=icon
            >
                {content.run()}
            </Button>
        }
        .into_any(),
        (None, Some(content)) => view! {
            <Button
                severity=p.severity.clone()
                text=p.text.clone()
                outlined=p.outlined.clone()
                plain=p.plain.clone()
                size=p.size.clone()
                icon_pos=p.icon_pos.clone()
                tabindex=p.tabindex.clone()
                aria_label=p.button_aria_label.clone()
                disabled=p.disabled
                autofocus=p.autofocus
                style_class=p.class.clone()
                on_click=p.on_click
            >
                {content.run()}
            </Button>
        }
        .into_any(),
        (Some(icon), None) => button!(icon = icon),
        (None, None) => button!(),
    }
}

/// Свойства кнопки раскрытия.
struct DropdownParts {
    severity: OptProp<ButtonSeverity>,
    text: Prop<bool>,
    outlined: Prop<bool>,
    plain: Prop<bool>,
    size: OptProp<Size>,
    expand_aria_label: OptProp<String>,
    expanded: RwSignal<bool>,
    menu_id: String,
    disabled: Signal<bool>,
    pt: Arc<SplitButtonPt>,
    dropdown_icon: Option<ViewFn>,
}

/// Кнопка раскрытия со своей разметкой: `Button` не принимает реактивные `aria-expanded` и `aria-controls`.
fn dropdown_button(
    p: DropdownParts,
    click: impl Fn(web_sys::MouseEvent) + Send + Sync + 'static,
    key: impl Fn(web_sys::KeyboardEvent) + Send + Sync + 'static,
) -> impl IntoView {
    let state = {
        let (severity, text, outlined, plain, size) =
            (p.severity.clone(), p.text.clone(), p.outlined.clone(), p.plain.clone(), p.size.clone());
        move || {
            // у кнопки-стрелки нет метки и бейджа: только значок
            ButtonState {
                has_icon: true,
                severity: severity.get(),
                text: text.get(),
                outlined: outlined.get(),
                plain: plain.get(),
                size: size.get(),
                ..ButtonState::default()
            }
        }
    };
    let class = {
        let (state, pt) = (state.clone(), p.pt.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add(&button_classes(&state())).add("p-splitbutton-dropdown p-ripple").add_user(pt.dropdown.extra_class());
            cn.finish_cow()
        }
    };
    let data = {
        let state = state.clone();
        move || button_data_p(&state())
    };
    let (disabled, expanded) = (p.disabled, p.expanded);
    let attrs: Vec<AnyAttribute> = {
        let expand_label = p.expand_aria_label.clone();
        let menu_id = p.menu_id.clone();
        vec![
            custom_attribute("aria-label", move || expand_label.get()).into_any_attr(),
            custom_attribute("aria-expanded", move || Some(expanded.get().to_string())).into_any_attr(),
            custom_attribute("aria-controls", move || Some(menu_id.clone())).into_any_attr(),
        ]
    };
    let pt_attrs = p.pt.dropdown.attrs();
    let data_severity = {
        let severity = p.severity.clone();
        move || severity.get().map(ButtonSeverity::as_str)
    };
    let icon = match p.dropdown_icon {
        Some(icon) => icon.run().into_any(),
        None => view! { <ChevronDownIcon /> }.into_any(),
    };
    // `NodeRef` нужен только клиенту (ripple): при SSR его нет, а `()` — пустой атрибут вместо `node_ref`.
    #[cfg(target_arch = "wasm32")]
    let node = {
        let node = NodeRef::<html::Button>::new();
        if prime_core::use_prime_config().ripple {
            let ripple = StoredValue::new_local(None::<prime_dom::ripple::Ripple>);
            Effect::new(move |_| {
                if let Some(el) = node.get()
                    && ripple.with_value(Option::is_none)
                {
                    ripple.set_value(prime_dom::ripple::Ripple::attach(&el));
                }
            });
            on_cleanup(move || ripple.set_value(None));
        }
        leptos::tachys::html::node_ref::node_ref::<html::Button, _>(node)
    };
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::let_unit_value)]
    let node = {};
    view! {
        <button
            {..node}
            type="button"
            class={class}
            data-p={data}
            data-p-severity={data_severity}
            data-p-disabled={move || disabled.get().then_some("true")}
            aria-haspopup="true"
            disabled={move || disabled.get().then_some("")}
            on:click={click}
            on:keydown={key}
            {..attrs}
            {..pt_attrs}
        >
            {icon}
        </button>
    }
}
