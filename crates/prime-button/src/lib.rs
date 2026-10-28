//! Button — порт `p-button` из PrimeNG 21.1.10.
//!
//! Классы, `data-p*` и порядок содержимого (`children`, иконка или индикатор загрузки, метка, бейдж) совпадают с PrimeNG.
//! Вместо хост-элемента `<p-button>` отрисовывается только `<button>`: CSS темы привязан к классам, а не к тегу.
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
use prime_core::{Pt, Size, class_memo};
use prime_icons::SpinnerIcon;

pub use state::{ButtonSeverity, ButtonState, ButtonVariant, IconPos, icon_classes, icon_data_p, label_data_p, root_classes, root_data_p};

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

/// Кнопка.
///
/// Свойства, влияющие на вид и состояние, реактивны (принимают значение или сигнал). Если заданы `children`,
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
    label: MaybeProp<String>,
    /// Иконка; выводится в `span.p-button-icon`.
    #[prop(optional, into)]
    icon: Option<ViewFn>,
    /// Положение иконки.
    #[prop(optional, into)]
    icon_pos: Signal<IconPos>,
    /// Идёт загрузка: кнопка недоступна, вместо иконки индикатор.
    #[prop(optional, into)]
    loading: Signal<bool>,
    /// Свой индикатор загрузки (по умолчанию `SpinnerIcon`).
    #[prop(optional, into)]
    loading_icon: Option<ViewFn>,
    /// Серьёзность.
    #[prop(optional, into)]
    severity: MaybeProp<ButtonSeverity>,
    /// Вид: с рамкой или текстовая.
    #[prop(optional, into)]
    variant: MaybeProp<ButtonVariant>,
    /// Текстовая кнопка.
    #[prop(optional, into)]
    text: Signal<bool>,
    /// Кнопка с рамкой.
    #[prop(optional, into)]
    outlined: Signal<bool>,
    /// Простой текст.
    #[prop(optional, into)]
    plain: Signal<bool>,
    /// Ссылка.
    #[prop(optional, into)]
    link: Signal<bool>,
    /// С тенью.
    #[prop(optional, into)]
    raised: Signal<bool>,
    /// Скруглённая.
    #[prop(optional, into)]
    rounded: Signal<bool>,
    /// Размер.
    #[prop(optional, into)]
    size: MaybeProp<Size>,
    /// На всю ширину контейнера.
    #[prop(optional, into)]
    fluid: Signal<bool>,
    /// Значение бейджа.
    #[prop(optional, into)]
    badge: MaybeProp<String>,
    /// Серьёзность бейджа (по умолчанию `secondary`).
    #[prop(optional, into)]
    badge_severity: MaybeProp<BadgeSeverity>,
    /// Недоступна.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Атрибут `type` (по умолчанию `button`).
    #[prop(optional, into)]
    button_type: MaybeProp<String>,
    /// Атрибут `aria-label`.
    #[prop(optional, into)]
    aria_label: MaybeProp<String>,
    /// Атрибут `tabindex`.
    #[prop(optional, into)]
    tabindex: MaybeProp<i32>,
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: MaybeProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: MaybeProp<String>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<ButtonPt>,
    /// Содержимое вместо метки и бейджа.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = pt.unwrap_or_default();
    let has_children = children.is_some();
    let content = children.map(|c| c());
    let has_icon = icon.is_some() || loading_icon.is_some();

    let state = Memo::new(move |_| ButtonState {
        label: label.get(),
        has_icon,
        badge: badge.get(),
        loading: loading.get(),
        severity: severity.get(),
        variant: variant.get(),
        text: text.get(),
        outlined: outlined.get(),
        plain: plain.get(),
        link: link.get(),
        raised: raised.get(),
        rounded: rounded.get(),
        size: size.get(),
        fluid: fluid.get(),
        icon_pos: icon_pos.get(),
    });

    let class = class_memo({
        let pt = pt.root.clone();
        move |cn| {
            cn.add(&root_classes(&state.read())).add("p-ripple").add_opt(style_class.get().as_deref());
            cn.add(&pt.merge_class(""));
        }
    });
    let root_style = {
        let pt = pt.root.clone();
        move || {
            let s = pt.merge_style(style.get().as_deref().unwrap_or(""));
            (!s.is_empty()).then_some(s)
        }
    };
    let is_disabled = move || disabled.get() || loading.get();

    let indicator = {
        let pt = pt.clone();
        move || -> AnyView {
            if loading.get() {
                let pt = pt.loading_icon.clone();
                match loading_icon.clone() {
                    Some(custom) => view! {
                        <span class=pt.merge_class("p-button-loading-icon") aria-hidden="true" {..pt.attrs()}>
                            {custom.run()}
                        </span>
                    }
                    .into_any(),
                    None => view! { <SpinnerIcon spin=true class=pt.merge_class("p-button-loading-icon") {..pt.attrs()} /> }.into_any(),
                }
            } else if let Some(icon) = icon.clone() {
                let pt = pt.icon.clone();
                let class = icon_classes(&state.read());
                view! {
                    <span class=pt.merge_class(&class) data-p=move || icon_data_p(&state.read()) {..pt.attrs()}>
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
        let pt = pt.label.clone();
        move || {
            let s = state.read();
            (!has_children && s.has_label()).then(|| {
                view! {
                    <span class=pt.merge_class("p-button-label") data-p=label_data_p(&s) {..pt.attrs()}>
                        {s.label.clone()}
                    </span>
                }
            })
        }
    };

    let badge_view = {
        let pt = pt.badge.clone();
        move || {
            (!has_children && state.read().has_badge()).then(|| {
                let severity = Signal::derive(move || Some(badge_severity.get().unwrap_or(BadgeSeverity::Secondary)));
                view! { <Badge value=badge severity=severity pt=BadgePt { root: pt.clone() } /> }
            })
        }
    };

    view! {
        <button
            type=move || button_type.get().unwrap_or_else(|| "button".to_owned())
            class=class
            style=root_style
            aria-label=move || aria_label.get()
            tabindex=move || tabindex.get()
            disabled=is_disabled
            data-p=move || root_data_p(&state.read())
            data-p-disabled=move || is_disabled().then_some("true")
            data-p-severity=move || severity.get().map(ButtonSeverity::as_str)
            {..pt.root.attrs()}
        >
            {content}
            {indicator}
            {label_view}
            {badge_view}
        </button>
    }
}
