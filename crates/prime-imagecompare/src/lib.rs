//! ImageCompare — порт `p-imagecompare` из PrimeNG 21.1.10.
//!
//! Разметка и классы как в PrimeNG: корень `div.p-imagecompare` с двумя представлениями (`left`, `right`, обычно `<img>`) и
//! ползунком `input[type=range].p-imagecompare-slider`. Ввод в ползунок обрезает `clip-path` элемент, стоящий перед ним
//! (правое изображение); правило темы `img + img` задаёт обрезку по умолчанию (50 %).
//!
//! ```
//! use leptos::prelude::*;
//! use prime_imagecompare::ImageCompare;
//!
//! let html = Owner::new()
//!     .with(|| view! { <ImageCompare left=|| view! { <img src="a.jpg" /> } right=|| view! { <img src="b.jpg" /> } /> }.to_html());
//! assert!(html.contains("p-imagecompare-slider") && html.contains("type=\"range\"") && html.contains("a.jpg"));
//! ```

mod state;

use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use prime_core::{OptProp, Pt};
use std::sync::Arc;

pub use state::clip_path;

/// Pass-through частей ImageCompare.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ImageComparePt {
    /// Корневой элемент.
    pub root: Pt,
    /// Ползунок.
    pub slider: Pt,
}

/// Сравнение двух изображений ползунком.
///
/// `left` и `right` выводятся без обёрток; для правила темы `img + img` оба должны быть `<img>`.
#[component]
pub fn ImageCompare(
    /// Левое представление (нижнее изображение).
    #[prop(into)]
    left: ViewFn,
    /// Правое представление (верхнее изображение; его обрезает ползунок).
    #[prop(into)]
    right: ViewFn,
    /// Атрибут `tabindex` корня.
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// Атрибут `aria-label` корня.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Атрибут `aria-labelledby` корня.
    #[prop(optional, into)]
    aria_labelledby: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<ImageComparePt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let class = {
        let pt = pt.clone();
        move || {
            let mut cn = prime_core::ClassNames::new();
            cn.add("p-imagecompare");
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
    let slider_class = pt.slider.merge_class("p-imagecompare-slider");
    let (root_attrs, slider_attrs) = (pt.root.attrs(), pt.slider.attrs());
    let on_input = move |e: leptos::web_sys::Event| slide(&e);

    view! {
        <div class={class} style={root_style} tabindex={move || tabindex.get()} aria-label={move || aria_label.get()} aria-labelledby={move || aria_labelledby.get()} {..root_attrs}>
            {left.run()}
            {right.run()}
            <input type="range" min="0" max="100" value="50" class={slider_class} on:input={on_input} {..slider_attrs} />
        </div>
    }
}

/// Обработчик ввода (`onSlide`): обрезает элемент перед ползунком; направление берётся из ближайшего `dir="rtl"`.
fn slide(event: &leptos::web_sys::Event) {
    let Some(input) = event.target().and_then(|t| t.dyn_into::<leptos::web_sys::HtmlInputElement>().ok()) else { return };
    let Ok(value) = input.value().parse::<f64>() else { return };
    let Some(image) = input.previous_element_sibling().and_then(|p| p.dyn_into::<leptos::web_sys::HtmlElement>().ok()) else { return };
    let rtl = input.closest("[dir=\"rtl\"]").ok().flatten().is_some();
    let _ = image.style().set_property("clip-path", &clip_path(value, rtl));
}
