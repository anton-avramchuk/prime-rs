//! Button — порт `p-button` из PrimeNG 21.1.10.
//!
//! Классы, `data-p*` и порядок содержимого (`children`, иконка или индикатор загрузки, метка, бейдж) совпадают с PrimeNG.
//! Вместо хост-элемента `<p-button>` отрисовывается только `<button>`: CSS темы привязан к классам, а не к тегу.
//! Свойства — [`Prop`]/[`OptProp`]: значение не создаёт реактивных примитивов, сигнал подписывает атрибуты.
//! Иконка передаётся как `ViewFn` (например, `<CheckIcon />` из `prime-icons`), а не именем класса шрифта.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_button::{Button, ButtonSeverity};
//!
//! let html = view! { <Button label="Сохранить" severity=ButtonSeverity::Success /> }.to_html();
//! assert!(html.contains("p-button-success") && html.contains("p-button-label"));
//! ```

mod state;

use leptos::prelude::*;
use prime_badge::{Badge, BadgePt, BadgeSeverity};
#[cfg(target_arch = "wasm32")]
use prime_core::use_prime_config;
use prime_core::{ClassNames, OptProp, Prop, Pt, Size};
#[cfg(target_arch = "wasm32")]
use prime_dom::Ripple;
use prime_icons::SpinnerIcon;
use std::borrow::Cow;
use std::sync::Arc;

pub use state::{
    ButtonSeverity, ButtonState, ButtonVariant, IconPos, icon_classes, icon_data_p, label_data_p, root_classes, root_data_p,
    write_root_classes,
};

/// Pass-through частей Button: классы и атрибуты добавляются к собственным, не заменяя их.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ButtonPt {
    /// Корневой `<button>`.
    pub root: Pt,
    /// Метка (`span.p-button-label`).
    pub label: Pt,
    /// Оболочка иконки (`span.p-button-icon`).
    pub icon: Pt,
    /// Индикатор загрузки (`.p-button-loading-icon`).
    pub loading_icon: Pt,
    /// Бейдж (корень `Badge`).
    pub badge: Pt,
}

/// Автофокус и ripple: только в браузере (`wasm32`). На сервере DOM нет, поэтому `Effect` не создаётся вовсе
/// (это также убирает лишние затраты при SSR и не зависит от набора features Leptos).
#[cfg(target_arch = "wasm32")]
fn setup_client(node: NodeRef<leptos::html::Button>, autofocus: bool) {
    let ripple = StoredValue::new_local(None::<Ripple>);
    let ripple_enabled = use_prime_config().ripple;
    Effect::new(move |_| {
        let Some(el) = node.get() else { return };
        if autofocus {
            let _ = el.focus();
        }
        if ripple_enabled && ripple.with_value(Option::is_none) {
            ripple.set_value(Ripple::attach(&el));
        }
    });
    on_cleanup(move || ripple.set_value(None));
}

#[cfg(not(target_arch = "wasm32"))]
fn setup_client(_node: NodeRef<leptos::html::Button>, _autofocus: bool) {}

/// Серьёзность бейджа по умолчанию — `secondary` (как в PrimeNG).
fn badge_severity_or_default(p: &OptProp<BadgeSeverity>) -> OptProp<BadgeSeverity> {
    match p {
        Prop::Static(v) => Prop::Static(Some(v.unwrap_or(BadgeSeverity::Secondary))),
        Prop::Reactive(_) => {
            let p = p.clone();
            Prop::Reactive(Signal::derive(move || Some(p.get().unwrap_or(BadgeSeverity::Secondary))))
        }
    }
}

/// Кнопка.
///
/// Каждое свойство принимает значение или сигнал; значение не создаёт реактивных примитивов. Если заданы `children`,
/// метка и бейдж не выводятся, а `children` идут первыми, как проецируемое содержимое в PrimeNG.
///
/// ```
/// use leptos::prelude::*;
/// use prime_button::{Button, ButtonPt, IconPos};
/// use prime_core::Pt;
/// use prime_icons::CheckIcon;
///
/// // иконка справа, бейдж и pass-through метки
/// let pt = ButtonPt { label: Pt::new().class("my-label"), ..Default::default() };
/// let html = view! {
///     <Button label="Готово" icon=ViewFn::from(|| view! { <CheckIcon /> }) icon_pos=IconPos::Right badge="2" pt=pt />
/// }
/// .to_html();
/// assert!(html.contains("p-button-icon-right") && html.contains("my-label") && html.contains("p-badge"));
///
/// // состояние загрузки меняется сигналом
/// let loading = RwSignal::new(true);
/// let html = view! { <Button label="Отправка" loading=loading /> }.to_html();
/// assert!(html.contains("p-button-loading") && html.contains("disabled"));
/// ```
#[component]
pub fn Button(
    /// Текст метки.
    #[prop(optional, into)]
    label: OptProp<String>,
    /// Иконка; выводится в `span.p-button-icon`.
    #[prop(optional, into)]
    icon: Option<ViewFn>,
    /// Положение иконки.
    #[prop(optional, into)]
    icon_pos: Prop<IconPos>,
    /// Идёт загрузка: кнопка недоступна, вместо иконки индикатор.
    #[prop(optional, into)]
    loading: Prop<bool>,
    /// Свой индикатор загрузки (по умолчанию `SpinnerIcon`).
    #[prop(optional, into)]
    loading_icon: Option<ViewFn>,
    /// Серьёзность.
    #[prop(optional, into)]
    severity: OptProp<ButtonSeverity>,
    /// Вид: с рамкой или текстовая.
    #[prop(optional, into)]
    variant: OptProp<ButtonVariant>,
    /// Текстовая кнопка.
    #[prop(optional, into)]
    text: Prop<bool>,
    /// Кнопка с рамкой.
    #[prop(optional, into)]
    outlined: Prop<bool>,
    /// Простой текст.
    #[prop(optional, into)]
    plain: Prop<bool>,
    /// Ссылка.
    #[prop(optional, into)]
    link: Prop<bool>,
    /// С тенью.
    #[prop(optional, into)]
    raised: Prop<bool>,
    /// Скруглённая.
    #[prop(optional, into)]
    rounded: Prop<bool>,
    /// Размер.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// На всю ширину контейнера.
    #[prop(optional, into)]
    fluid: Prop<bool>,
    /// Значение бейджа.
    #[prop(optional, into)]
    badge: OptProp<String>,
    /// Серьёзность бейджа (по умолчанию `secondary`).
    #[prop(optional, into)]
    badge_severity: OptProp<BadgeSeverity>,
    /// Недоступна.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Атрибут `type` (по умолчанию `button`).
    #[prop(optional, into)]
    button_type: OptProp<String>,
    /// Атрибут `aria-label`.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Атрибут `tabindex`.
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Получить фокус после монтирования в браузере (на сервере не выставляется).
    #[prop(optional)]
    autofocus: bool,
    /// Вызывается при клике на доступную кнопку.
    #[prop(optional, into)]
    on_click: Option<Callback<web_sys::MouseEvent>>,
    /// Вызывается при получении фокуса.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается при потере фокуса.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<ButtonPt>,
    /// Содержимое вместо метки и бейджа.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let has_children = children.is_some();
    let content = children.map(|c| c());
    let has_icon = icon.is_some() || loading_icon.is_some();
    let badge_severity = badge_severity_or_default(&badge_severity);

    // Все свойства в одном `Arc`: замыкания атрибутов клонируют счётчик ссылок, а не строки значений.
    let props = Arc::new(Props {
        label,
        badge,
        loading,
        severity,
        variant,
        size,
        icon_pos,
        text,
        outlined,
        plain,
        link,
        raised,
        rounded,
        fluid,
        disabled,
        button_type,
        aria_label,
        tabindex,
        style,
        style_class,
        has_icon,
    });

    let class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            props.with_state(|s| write_root_classes(&mut cn, s));
            cn.add("p-ripple");
            props.style_class.with(|c| cn.add_opt(c.as_deref()));
            cn.add_opt(pt.root.extra_class());
            cn.finish()
        }
    };
    let data_attr = {
        let props = props.clone();
        move || props.with_state(root_data_p)
    };
    let root_style = {
        let (props, pt) = (props.clone(), pt.clone());
        move || props.style.with(|st| (st.is_some() || pt.root.has_style()).then(|| pt.root.merge_style(st.as_deref().unwrap_or(""))))
    };
    let type_attr = {
        let props = props.clone();
        move || -> Cow<'static, str> {
            props.button_type.with(|t| t.as_deref().map_or(Cow::Borrowed("button"), |s| Cow::Owned(s.to_owned())))
        }
    };
    let aria_attr = {
        let props = props.clone();
        move || props.aria_label.get()
    };
    let tabindex_attr = {
        let props = props.clone();
        move || props.tabindex.get()
    };
    let disabled_attr = {
        let props = props.clone();
        move || props.is_disabled()
    };
    let data_disabled = {
        let props = props.clone();
        move || props.is_disabled().then_some("true")
    };
    let severity_attr = {
        let props = props.clone();
        move || props.severity.get().map(ButtonSeverity::as_str)
    };

    let node = NodeRef::<leptos::html::Button>::new();
    setup_client(node, autofocus);

    let indicator = {
        let (props, pt) = (props.clone(), pt.clone());
        move || -> AnyView {
            if props.loading.get() {
                match loading_icon.clone() {
                    Some(custom) => view! {
                        <span class=pt.loading_icon.merge_class("p-button-loading-icon") aria-hidden="true" {..pt.loading_icon.attrs()}>
                            {custom.run()}
                        </span>
                    }
                    .into_any(),
                    None => view! {
                        <SpinnerIcon spin=true class=pt.loading_icon.merge_class("p-button-loading-icon") {..pt.loading_icon.attrs()} />
                    }
                    .into_any(),
                }
            } else if let Some(icon) = icon.clone() {
                let (class, data) = props.with_state(|s| (icon_classes(s), icon_data_p(s)));
                view! {
                    <span class=pt.icon.merge_class(&class) data-p={data} {..pt.icon.attrs()}>
                        {icon.run()}
                    </span>
                }
                .into_any()
            } else {
                ().into_any()
            }
        }
    };

    let label_view = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            if has_children {
                return None;
            }
            let data = props.with_state(|s| s.has_label().then(|| label_data_p(s)))?;
            Some(view! {
                <span class=pt.label.merge_class("p-button-label") data-p={data} {..pt.label.attrs()}>
                    {props.label.get()}
                </span>
            })
        }
    };

    let badge_view = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let present = !has_children && props.with_state(|s| s.has_badge());
            present.then(|| {
                view! { <Badge value=props.badge.clone() severity=badge_severity.clone() pt=BadgePt { root: pt.badge.clone() } /> }
            })
        }
    };

    let click_props = props.clone();
    let root_attrs = pt.root.attrs();

    view! {
        <button
            type=type_attr
            class=class
            style=root_style
            aria-label=aria_attr
            tabindex=tabindex_attr
            disabled=disabled_attr
            node_ref=node
            on:click=move |e| {
                if let (Some(cb), false) = (on_click, click_props.is_disabled()) {
                    cb.run(e);
                }
            }
            on:focus=move |e| {
                if let Some(cb) = on_focus {
                    cb.run(e);
                }
            }
            on:blur=move |e| {
                if let Some(cb) = on_blur {
                    cb.run(e);
                }
            }
            data-p={data_attr}
            data-p-disabled=data_disabled
            data-p-severity={severity_attr}
            {..root_attrs}
        >
            {content}
            {indicator}
            {label_view}
            {badge_view}
        </button>
    }
}

/// Свойства экземпляра.
struct Props {
    label: OptProp<String>,
    badge: OptProp<String>,
    loading: Prop<bool>,
    severity: OptProp<ButtonSeverity>,
    variant: OptProp<ButtonVariant>,
    size: OptProp<Size>,
    icon_pos: Prop<IconPos>,
    text: Prop<bool>,
    outlined: Prop<bool>,
    plain: Prop<bool>,
    link: Prop<bool>,
    raised: Prop<bool>,
    rounded: Prop<bool>,
    fluid: Prop<bool>,
    disabled: Prop<bool>,
    button_type: OptProp<String>,
    aria_label: OptProp<String>,
    tabindex: OptProp<i32>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    has_icon: bool,
}

impl Props {
    /// Недоступна: `disabled` или идёт загрузка.
    fn is_disabled(&self) -> bool {
        self.disabled.get() || self.loading.get()
    }

    /// Вызывает `f` с заимствованным снимком состояния (без копирования строк).
    fn with_state<R>(&self, f: impl FnOnce(&ButtonState) -> R) -> R {
        self.label.with(|l| {
            self.badge.with(|b| {
                f(&ButtonState {
                    label: l.as_deref(),
                    has_icon: self.has_icon,
                    badge: b.as_deref(),
                    loading: self.loading.get(),
                    severity: self.severity.get(),
                    variant: self.variant.get(),
                    text: self.text.get(),
                    outlined: self.outlined.get(),
                    plain: self.plain.get(),
                    link: self.link.get(),
                    raised: self.raised.get(),
                    rounded: self.rounded.get(),
                    size: self.size.get(),
                    fluid: self.fluid.get(),
                    icon_pos: self.icon_pos.get(),
                })
            })
        })
    }
}
