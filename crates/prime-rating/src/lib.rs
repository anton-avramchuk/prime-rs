//! Rating — порт `p-rating` из PrimeNG 21.1.10.
//!
//! Разметка и классы как в PrimeNG: корень `div.p-rating` (вместо хоста `<p-rating>`), на каждую звезду `div.p-rating-option`
//! со скрытой радиокнопкой (`span.p-hidden-accessible > input[type=radio]`) и иконкой `p-rating-icon`. Звёзды — группа нативных
//! радиокнопок с общим `name`, поэтому стрелки клавиатуры работают без собственного кода.
//!
//! Состояние — двусторонний сигнал `value: RwSignal<Option<u32>>` (номера звёзд с 1); без него компонент ведёт внутреннее
//! состояние, начинающееся с `default_value`. Клик по звезде выбирает её, повторный клик по текущему значению сбрасывает его.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_rating::Rating;
//!
//! let value = RwSignal::new(Some(3));
//! let html = view! { <Rating value=value stars=5 /> }.to_html();
//! assert!(html.contains("p-rating-option-active") && html.contains("p-rating-on-icon") && html.contains("p-rating-off-icon"));
//! ```

mod state;

use leptos::html;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use prime_core::ids::next_unique;
use prime_core::{ClassNames, OptProp, Prop, Pt, use_prime_config};
use prime_icons::{StarFillIcon, StarIcon};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub use state::{RatingState, data_p, next_value, write_classes, write_icon_classes, write_option_classes};

/// Число звёзд по умолчанию.
const DEFAULT_STARS: u32 = 5;

/// Событие выбора: новое значение и исходное событие браузера.
#[derive(Clone, Debug)]
pub struct RatingRate {
    /// Новое значение (`None` — сброшено).
    pub value: Option<u32>,
    /// Исходное событие (`click` по звезде или `change` радиокнопки).
    pub original_event: web_sys::Event,
}

/// Контекст шаблона иконки (как `{ $implicit, class }` в PrimeNG).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RatingIcon {
    /// Номер звезды (с 1).
    pub star: u32,
    /// Классы иконки: `p-rating-icon p-rating-on-icon` или `p-rating-icon p-rating-off-icon`, `p-invalid`.
    pub class: String,
}

/// Pass-through частей Rating.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RatingPt {
    /// Корневой элемент.
    pub root: Pt,
    /// Опция звезды (`div.p-rating-option`).
    pub option: Pt,
    /// Контейнер скрытой радиокнопки (`span.p-hidden-accessible`).
    pub hidden_option_input_container: Pt,
    /// Скрытая радиокнопка.
    pub hidden_option_input: Pt,
    /// Включённая иконка.
    pub on_icon: Pt,
    /// Выключенная иконка.
    pub off_icon: Pt,
}

/// Выбор оценки звёздами.
///
/// Каждое свойство принимает значение или сигнал; `value` — двусторонний сигнал.
#[component]
pub fn Rating(
    /// Двусторонний сигнал значения.
    #[prop(optional)]
    value: Option<RwSignal<Option<u32>>>,
    /// Начальное значение, если `value` не передан.
    #[prop(optional, into)]
    default_value: OptProp<u32>,
    /// Число звёзд (по умолчанию 5).
    #[prop(optional, into)]
    stars: OptProp<u32>,
    /// Только чтение.
    #[prop(optional, into)]
    readonly: Prop<bool>,
    /// Недоступен.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Недопустимое значение.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Обязателен.
    #[prop(optional, into)]
    required: Prop<bool>,
    /// Атрибут `name` радиокнопок (по умолчанию уникальный).
    #[prop(optional, into)]
    name: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Иконка включённой звезды вместо звезды по умолчанию (в `span` с классами иконки).
    #[prop(optional, into)]
    on_icon: Option<ViewFn>,
    /// Иконка выключенной звезды вместо звезды по умолчанию (в `span` с классами иконки).
    #[prop(optional, into)]
    off_icon: Option<ViewFn>,
    /// Заменяет иконку включённой звезды целиком.
    #[prop(optional, into)]
    on_icon_template: Option<Callback<RatingIcon, AnyView>>,
    /// Заменяет иконку выключенной звезды целиком.
    #[prop(optional, into)]
    off_icon_template: Option<Callback<RatingIcon, AnyView>>,
    /// Вызывается после выбора или сброса пользователем.
    #[prop(optional, into)]
    on_rate: Option<Callback<RatingRate>>,
    /// Вызывается при получении фокуса звездой.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается при потере фокуса звездой.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<RatingPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let locale = use_prime_config().locale;
    let props = Arc::new(Props {
        value: value.unwrap_or_else(|| RwSignal::new(default_value.get_untracked())),
        visible_focus: RwSignal::new(None),
        pointer_focus: AtomicBool::new(false),
        stars,
        readonly,
        disabled,
        invalid,
        required,
        name,
        default_name: format!("pn_id_{}_name", next_unique()),
        style,
        style_class,
        on_icon,
        off_icon,
        on_icon_template,
        off_icon_template,
        on_rate,
        on_focus,
        on_blur,
        label_one: locale.text("aria.star").to_owned(),
        label_many: locale.text("aria.stars").to_owned(),
    });

    let root_class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            write_classes(&mut cn, &state_of(&props));
            props.style_class.with(|c| cn.add_user(c.as_deref()));
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let root_style = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let style = props.style.with(|st| pt.root.merge_style(st.as_deref().unwrap_or_default()));
            (!style.is_empty()).then_some(style)
        }
    };
    let data = {
        let props = props.clone();
        move || data_p(&state_of(&props))
    };
    // Замыкание списка читает только `stars`: чтение значения здесь пересоздавало бы звёзды и роняло фокус.
    let options = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let count = props.stars.get().unwrap_or(DEFAULT_STARS);
            (1..=count).map(|star| star_view(&props, &pt, star)).collect_view()
        }
    };
    let root_attrs = pt.root.attrs();

    view! {
        <div class={root_class} style={root_style} data-p={data} {..root_attrs}>
            {options}
        </div>
    }
}

/// Одна звезда: опция со скрытой радиокнопкой и иконкой.
fn star_view(props: &Arc<Props>, pt: &Arc<RatingPt>, star: u32) -> impl IntoView + use<> {
    let input_ref = NodeRef::<html::Input>::new();
    let option_class = {
        let (props, pt) = (props.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            let active = props.value.get().is_some_and(|v| star <= v);
            write_option_classes(&mut cn, active, props.visible_focus.get() == Some(star));
            cn.add_user(pt.option.extra_class());
            cn.finish_cow()
        }
    };
    let option_style = {
        let pt = pt.clone();
        move || pt.option.has_style().then(|| pt.option.merge_style(""))
    };
    let click = {
        let props = props.clone();
        move |e: web_sys::MouseEvent| {
            // Клик по самой радиокнопке (стрелки, программный `click()`) приходит вместе с `change`, его обрабатывает `change`.
            if e.target().is_some_and(|t| t.is_instance_of::<web_sys::HtmlInputElement>()) {
                return;
            }
            props.choose(star, e.into(), true);
            props.visible_focus.set(None);
            if let Some(input) = input_ref.get() {
                // фокус после клика не считается видимым (`isFocusVisibleItem = false` в `onOptionClick`)
                props.pointer_focus.store(true, Ordering::Relaxed);
                let _ = input.focus();
                props.pointer_focus.store(false, Ordering::Relaxed);
            }
        }
    };
    let change = {
        let props = props.clone();
        move |e: web_sys::Event| props.choose(star, e, false)
    };
    let focus = {
        let props = props.clone();
        move |e: web_sys::FocusEvent| {
            if props.readonly.get_untracked() || props.disabled.get_untracked() {
                return;
            }
            let visible = !props.pointer_focus.load(Ordering::Relaxed)
                && input_ref.get().is_some_and(|input| input.matches(":focus-visible").unwrap_or(false));
            props.visible_focus.set(visible.then_some(star));
            if let Some(cb) = props.on_focus {
                cb.run(e);
            }
        }
    };
    let blur = {
        let props = props.clone();
        move |e: web_sys::FocusEvent| {
            props.visible_focus.set(None);
            if let Some(cb) = props.on_blur {
                cb.run(e);
            }
        }
    };
    let input_name = {
        let props = props.clone();
        move || props.name.get().unwrap_or_else(|| props.default_name.clone())
    };
    let checked = {
        let props = props.clone();
        move || props.value.get() == Some(star)
    };
    let required = {
        let props = props.clone();
        move || props.required.get()
    };
    let readonly = {
        let props = props.clone();
        move || props.readonly.get().then_some("")
    };
    let disabled = {
        let props = props.clone();
        move || props.disabled.get()
    };
    let aria_label = props.star_label(star);
    let icon = {
        let (props, pt) = (props.clone(), pt.clone());
        move || icon_of(&props, &pt, star)
    };
    let container_class = pt.hidden_option_input_container.merge_class("p-hidden-accessible");
    // У радиокнопки нет собственного класса: `class` выводится только из `pt` (атрибутом, иначе получился бы `class=""`).
    let input_pt = match pt.hidden_option_input.extra_class() {
        Some(class) => pt.hidden_option_input.clone().attr("class", class),
        None => pt.hidden_option_input.clone(),
    };
    let (option_attrs, container_attrs, input_attrs) = (pt.option.attrs(), pt.hidden_option_input_container.attrs(), input_pt.attrs());

    view! {
        <div class={option_class} style={option_style} on:click={click} {..option_attrs}>
            <span class={container_class} data-p-hidden-accessible="true" {..container_attrs}>
                <input
                    node_ref={input_ref}
                    type="radio"
                    name={input_name}
                    value={star}
                    checked={checked.clone()}
                    prop:checked={checked}
                    required={required}
                    readonly={readonly}
                    disabled={disabled}
                    aria-label={aria_label}
                    on:focus={focus}
                    on:blur={blur}
                    on:change={change}
                    {..input_attrs}
                />
            </span>
            {icon}
        </div>
    }
}

/// Иконка звезды: шаблон, затем `on_icon`/`off_icon`, затем звезда по умолчанию.
fn icon_of(props: &Props, pt: &RatingPt, star: u32) -> AnyView {
    let on = props.value.get().is_some_and(|v| star <= v);
    let mut cn = ClassNames::new();
    write_icon_classes(&mut cn, on, props.invalid.get());
    let (template, custom, icon_pt) =
        if on { (props.on_icon_template, &props.on_icon, &pt.on_icon) } else { (props.off_icon_template, &props.off_icon, &pt.off_icon) };
    let class = icon_pt.merge_class(&cn.finish());
    if let Some(template) = template {
        return template.run(RatingIcon { star, class: class.into_owned() });
    }
    match (custom, on) {
        (Some(icon), _) => view! { <span class={class} {..icon_pt.attrs()}>{icon.run()}</span> }.into_any(),
        (None, true) => view! { <StarFillIcon class=class.into_owned() {..icon_pt.attrs()} /> }.into_any(),
        (None, false) => view! { <StarIcon class=class.into_owned() {..icon_pt.attrs()} /> }.into_any(),
    }
}

fn state_of(props: &Props) -> RatingState {
    RatingState { readonly: props.readonly.get(), disabled: props.disabled.get() }
}

/// Свойства экземпляра.
struct Props {
    value: RwSignal<Option<u32>>,
    /// Звезда с клавиатурным (видимым) фокусом.
    visible_focus: RwSignal<Option<u32>>,
    /// Идёт программный фокус после клика по звезде.
    pointer_focus: AtomicBool,
    stars: OptProp<u32>,
    readonly: Prop<bool>,
    disabled: Prop<bool>,
    invalid: Prop<bool>,
    required: Prop<bool>,
    name: OptProp<String>,
    default_name: String,
    style: OptProp<String>,
    style_class: OptProp<String>,
    on_icon: Option<ViewFn>,
    off_icon: Option<ViewFn>,
    on_icon_template: Option<Callback<RatingIcon, AnyView>>,
    off_icon_template: Option<Callback<RatingIcon, AnyView>>,
    on_rate: Option<Callback<RatingRate>>,
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    label_one: String,
    label_many: String,
}

impl Props {
    /// `aria-label` звезды: `aria.star` для первой, иначе `aria.stars` с подстановкой `{star}`.
    fn star_label(&self, star: u32) -> String {
        if star == 1 { self.label_one.clone() } else { self.label_many.replace("{star}", &star.to_string()) }
    }

    /// Выбор звезды. `toggle` — сбрасывать ли повторный выбор текущего значения (клик; `change` всегда приносит новое значение).
    fn choose(&self, star: u32, original_event: web_sys::Event, toggle: bool) {
        if self.readonly.get_untracked() || self.disabled.get_untracked() {
            return;
        }
        let current = self.value.get_untracked();
        let next = if toggle { next_value(current, star) } else { Some(star) };
        self.value.set(next);
        if let Some(cb) = self.on_rate {
            cb.run(RatingRate { value: next, original_event });
        }
    }
}
