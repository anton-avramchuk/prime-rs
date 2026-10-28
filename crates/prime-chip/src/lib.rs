//! Chip — порт `p-chip` из PrimeNG 21.1.10.
//!
//! Разметка, классы и `data-p` совпадают с PrimeNG; вместо хост-элемента `<p-chip>` отрисовывается `<div class="p-chip …">`.
//! Удаляемый Chip скрывается (`display: none`) по клику на элемент удаления и по клавишам Enter и Backspace и вызывает `on_remove`.
//!
//! Свойства — [`Prop`]/[`OptProp`]; если все они статические и Chip не удаляемый, компонент рисует готовые значения
//! без замыканий и `RenderEffect` (удаляемому нужно внутреннее состояние видимости, поэтому он идёт реактивным путём).
//!
//! ```
//! use leptos::prelude::*;
//! use prime_chip::Chip;
//!
//! let html = view! { <Chip label="Amy Elsner" removable=true /> }.to_html();
//! assert!(html.contains("p-chip-label") && html.contains("p-chip-remove-icon") && html.contains(r#"role="button""#));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, all_static, use_prime_config};
use prime_icons::TimesCircleIcon;
use std::sync::Arc;

pub use state::{ChipState, data_p, is_remove_key, write_classes};

/// Pass-through частей Chip.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChipPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Изображение (`img.p-chip-image`).
    pub image: Pt,
    /// Иконка (`span.p-chip-icon`).
    pub icon: Pt,
    /// Метка (`div.p-chip-label`).
    pub label: Pt,
    /// Элемент удаления (`span.p-chip-remove-icon`).
    pub remove_icon: Pt,
}

/// Chip: метка, иконка или изображение, удаление.
///
/// Каждое свойство принимает значение или сигнал. `children` выводятся первыми, как проецируемое содержимое в PrimeNG.
#[component]
pub fn Chip(
    /// Текст метки.
    #[prop(optional, into)]
    label: OptProp<String>,
    /// Иконка; выводится в `span.p-chip-icon`, если нет `image`.
    #[prop(optional, into)]
    icon: Option<ViewFn>,
    /// URL изображения; заменяет иконку.
    #[prop(optional, into)]
    image: OptProp<String>,
    /// Альтернативный текст изображения.
    #[prop(optional, into)]
    alt: OptProp<String>,
    /// Недоступен (`p-disabled`, `tabindex=-1` у элемента удаления).
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Показывать элемент удаления.
    #[prop(optional, into)]
    removable: Prop<bool>,
    /// Своя иконка удаления (по умолчанию `TimesCircleIcon`).
    #[prop(optional, into)]
    remove_icon: Option<ViewFn>,
    /// Встроенный стиль.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Вызывается при удалении (клик, Enter, Backspace) с исходным событием.
    #[prop(optional, into)]
    on_remove: Option<Callback<web_sys::Event>>,
    /// Вызывается при ошибке загрузки изображения.
    #[prop(optional, into)]
    on_image_error: Option<Callback<web_sys::Event>>,
    /// Pass-through.
    #[prop(optional)]
    pt: Option<ChipPt>,
    /// Содержимое перед изображением, меткой и элементом удаления.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let is_static = all_static!(label, image, alt, disabled, removable, style, style_class) && !removable.get_untracked();
    let props = Arc::new(Props { label, image, alt, disabled, removable, style, style_class, icon, remove_icon });
    let content = children.map(|c| c());
    let attrs = pt.root.attrs();

    if is_static {
        // Статический путь: значения и блоки считаются один раз, замыканий и `RenderEffect` нет.
        let (class, style, data) = (class_of(&props, &pt), style_of(&props, &pt, true), data_of(&props));
        let (media, label) = (media_of(&props, &pt, on_image_error), label_of(&props, &pt));
        let remove: Option<AnyView> = None;
        Either::Left(chip_markup!(class, style, data, attrs, content, media, label, remove))
    } else {
        let visible = RwSignal::new(true);
        let class = {
            let (props, pt) = (props.clone(), pt.clone());
            move || class_of(&props, &pt)
        };
        let style = {
            let (props, pt) = (props.clone(), pt.clone());
            move || style_of(&props, &pt, visible.get())
        };
        let data = {
            let props = props.clone();
            move || data_of(&props)
        };
        let media = {
            let (props, pt) = (props.clone(), pt.clone());
            move || media_of(&props, &pt, on_image_error)
        };
        let label = {
            let (props, pt) = (props.clone(), pt.clone());
            move || label_of(&props, &pt)
        };
        let remove = move || props.removable.get().then(|| remove_of(&props, &pt, visible, on_remove));
        Either::Right(chip_markup!(class, style, data, attrs, content, media, label, remove))
    }
}

/// Разметка Chip: одна для статической и реактивной ветвей (меняются только выражения атрибутов и блоков).
macro_rules! chip_markup {
    ($class:expr, $style:expr, $data:expr, $attrs:expr, $content:expr, $media:expr, $label:expr, $remove:expr) => {
        view! {
            <div class={$class} style={$style} data-p={$data} {..$attrs}>
                {$content}
                {$media}
                {$label}
                {$remove}
            </div>
        }
    };
}
use chip_markup;

fn class_of(props: &Props, pt: &ChipPt) -> String {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, &ChipState { disabled: props.disabled.get(), removable: props.removable.get() });
    props.style_class.with(|c| cn.add_opt(c.as_deref()));
    cn.add_opt(pt.root.extra_class());
    cn.finish()
}

fn data_of(props: &Props) -> String {
    data_p(&ChipState { disabled: props.disabled.get(), removable: props.removable.get() })
}

/// Встроенный стиль корня: `display: none` после удаления, затем `style` и стиль из `pt`.
fn style_of(props: &Props, pt: &ChipPt, visible: bool) -> Option<String> {
    props.style.with(|st| {
        let user = st.as_deref().unwrap_or("");
        let base = Pt::new().style(user).merge_style(if visible { "" } else { "display: none" });
        let merged = pt.root.merge_style(&base);
        (!merged.is_empty()).then_some(merged)
    })
}

/// Изображение, а при его отсутствии иконка.
fn media_of(props: &Props, pt: &ChipPt, on_image_error: Option<Callback<web_sys::Event>>) -> Option<AnyView> {
    if let Some(src) = props.image.get().filter(|s| !s.is_empty()) {
        let alt = props.alt.get();
        Some(
            view! {
                <img
                    class=pt.image.merge_class("p-chip-image")
                    src=src
                    alt=alt
                    on:error=move |e| {
                        if let Some(cb) = on_image_error {
                            cb.run(e.into());
                        }
                    }
                    {..pt.image.attrs()}
                />
            }
            .into_any(),
        )
    } else {
        props
            .icon
            .clone()
            .map(|icon| view! { <span class=pt.icon.merge_class("p-chip-icon") {..pt.icon.attrs()}>{icon.run()}</span> }.into_any())
    }
}

/// Метка (`None`, если пуста).
fn label_of(props: &Props, pt: &ChipPt) -> Option<AnyView> {
    props
        .label
        .get()
        .filter(|s| !s.is_empty())
        .map(|label| view! { <div class=pt.label.merge_class("p-chip-label") {..pt.label.attrs()}>{label}</div> }.into_any())
}

/// Элемент удаления: `role="button"`, подпись из локали, клик и клавиши Enter и Backspace.
fn remove_of(props: &Props, pt: &ChipPt, visible: RwSignal<bool>, on_remove: Option<Callback<web_sys::Event>>) -> AnyView {
    let aria = use_prime_config().locale.text("aria.removeLabel").to_owned();
    let tabindex = if props.disabled.get() { -1 } else { 0 };
    let close = move |e: web_sys::Event| {
        visible.set(false);
        if let Some(cb) = on_remove {
            cb.run(e);
        }
    };
    let click = move |e: web_sys::MouseEvent| close(e.into());
    let keydown = move |e: web_sys::KeyboardEvent| {
        if is_remove_key(&e.key()) {
            close(e.into());
        }
    };
    let inner = match props.remove_icon.clone() {
        Some(icon) => icon.run(),
        None => view! { <TimesCircleIcon /> }.into_any(),
    };
    view! {
        <span
            class=pt.remove_icon.merge_class("p-chip-remove-icon")
            role="button"
            tabindex=tabindex
            aria-label=aria
            on:click={click}
            on:keydown={keydown}
            {..pt.remove_icon.attrs()}
        >
            {inner}
        </span>
    }
    .into_any()
}

/// Свойства экземпляра.
struct Props {
    label: OptProp<String>,
    image: OptProp<String>,
    alt: OptProp<String>,
    disabled: Prop<bool>,
    removable: Prop<bool>,
    style: OptProp<String>,
    style_class: OptProp<String>,
    icon: Option<ViewFn>,
    remove_icon: Option<ViewFn>,
}
