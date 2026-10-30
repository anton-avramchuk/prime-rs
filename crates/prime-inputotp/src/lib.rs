//! InputOtp — порт компонента `p-inputotp` из PrimeNG 21.1.10.
//!
//! Одноразовый код вводится по одному символу в отдельные поля `<input>` (классы `p-inputtext p-component p-inputotp-input`) с
//! автопереходом, переходом стрелками, вставкой всего кода и маскированием.
//!
//! Значение — необязательное двустороннее `value: RwSignal<String>`: символы строки по порядку заполняют ячейки. Без `value`
//! токены лежат внутри компонента, а результат приходит через `on_change`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_inputotp::InputOtp;
//!
//! let html = view! { <InputOtp length=6 value=RwSignal::new("12".to_owned()) mask=true /> }.to_html();
//! assert_eq!(html.matches("p-inputotp-input").count(), 6);
//! assert!(html.contains(r#"type="password""#));
//! ```

mod state;

use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, Size, Variant, use_prime_config};
use prime_inputtext::{InputTextState, data_p, write_classes};
use std::sync::Arc;
use web_sys::wasm_bindgen::JsCast;

pub use state::{KeyAction, Move, input_move, join, key_decision, paste_value, tokens_of};

/// Число ячеек по умолчанию.
const DEFAULT_LENGTH: usize = 4;

/// Pass-through частей InputOtp.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InputOtpPt {
    /// Корневой `div`.
    pub root: Pt,
    /// Каждая ячейка.
    pub input: Pt,
}

/// Обработчики событий ячейки; шаблон `input` подключает их к своему полю.
#[derive(Clone, Copy)]
pub struct InputOtpEvents {
    /// `input`.
    pub on_input: Callback<web_sys::Event>,
    /// `keydown`.
    pub on_keydown: Callback<web_sys::KeyboardEvent>,
    /// `focus`.
    pub on_focus: Callback<web_sys::FocusEvent>,
    /// `blur`.
    pub on_blur: Callback<web_sys::FocusEvent>,
    /// `paste`.
    pub on_paste: Callback<web_sys::ClipboardEvent>,
}

/// Данные ячейки для шаблона `input`.
#[derive(Clone)]
pub struct InputOtpSlot {
    /// Символ ячейки (пустая строка, если не заполнена).
    pub token: String,
    /// Номер ячейки с нуля.
    pub index: usize,
    /// Обработчики событий.
    pub events: InputOtpEvents,
}

/// Поле ввода одноразового кода.
///
/// Каждое свойство принимает значение или сигнал. `value` — двусторонняя привязка (`RwSignal<String>`).
#[component]
pub fn InputOtp(
    /// Двусторонне связанное значение; без него токены хранятся внутри компонента.
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    /// Число ячеек (по умолчанию 4).
    #[prop(optional, into)]
    length: OptProp<usize>,
    /// Скрывать символы (`type="password"`).
    #[prop(optional, into)]
    mask: Prop<bool>,
    /// Только цифры.
    #[prop(optional, into)]
    integer_only: Prop<bool>,
    /// Атрибут `name` каждой ячейки.
    #[prop(optional, into)]
    name: OptProp<String>,
    /// Атрибут `tabindex` каждой ячейки.
    #[prop(optional, into)]
    tabindex: OptProp<i32>,
    /// Недоступно.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Только для чтения.
    #[prop(optional, into)]
    readonly: Prop<bool>,
    /// Обязательное.
    #[prop(optional, into)]
    required: Prop<bool>,
    /// Размер.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Вариант; если не задан, берётся `PrimeConfig.input_variant`.
    #[prop(optional, into)]
    variant: OptProp<Variant>,
    /// Недопустимое значение.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Фокус на первой ячейке при отрисовке.
    #[prop(optional, into)]
    autofocus: Prop<bool>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Вызывается при изменении значения (аргумент — склеенная строка).
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,
    /// Вызывается при получении фокуса ячейкой.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается при потере фокуса ячейкой.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Шаблон вместо стандартного поля каждой ячейки.
    #[prop(optional, into)]
    input: Option<Callback<InputOtpSlot, AnyView>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<InputOtpPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let initial_length = length.get_untracked().unwrap_or(DEFAULT_LENGTH);
    let initial = value.map(|v| v.with_untracked(|s| tokens_of(s, initial_length))).unwrap_or_default();
    let ctx = Arc::new(Ctx {
        tokens: RwSignal::new(initial),
        value,
        length,
        integer_only,
        readonly,
        disabled,
        size,
        variant,
        invalid,
        mask,
        name,
        tabindex,
        required,
        autofocus,
        config_variant: use_prime_config().input_variant,
        on_change,
        on_focus,
        on_blur,
    });

    // Внешнее изменение `value` перекладывается в токены; собственная запись совпадает с ними и пропускается.
    #[cfg(target_arch = "wasm32")]
    if let Some(v) = value {
        let ctx = ctx.clone();
        Effect::new(move |_| {
            let external = v.get();
            let len = ctx.len();
            if ctx.tokens.with_untracked(|t| join(t)) != external {
                ctx.tokens.set(tokens_of(&external, len));
            }
        });
    }

    let root_class = {
        let (ctx, pt) = (ctx.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            cn.add("p-inputotp p-component");
            ctx.style_class_into(&mut cn, &style_class);
            cn.add_user(pt.root.extra_class());
            cn.finish_cow()
        }
    };
    let root_style = {
        let pt = pt.clone();
        move || {
            let style = style.with(|st| pt.root.merge_style(st.as_deref().unwrap_or_default()));
            (!style.is_empty()).then_some(style)
        }
    };
    // Замыкание списка читает только `length`: чтение токенов здесь пересоздавало бы ячейки и роняло фокус.
    let cells = {
        let (ctx, pt) = (ctx.clone(), pt.clone());
        move || (0..ctx.len()).map(|i| cell_view(&ctx, &pt, input, i)).collect_view()
    };
    let root_attrs = pt.root.attrs();

    view! {
        <div class={root_class} style={root_style} {..root_attrs}>
            {cells}
        </div>
    }
}

/// Одна ячейка: стандартное поле или шаблон.
fn cell_view(ctx: &Arc<Ctx>, pt: &Arc<InputOtpPt>, template: Option<Callback<InputOtpSlot, AnyView>>, index: usize) -> AnyView {
    let events = InputOtpEvents {
        on_input: {
            let ctx = ctx.clone();
            Callback::new(move |e: web_sys::Event| ctx.input(index, &e))
        },
        on_keydown: {
            let ctx = ctx.clone();
            Callback::new(move |e: web_sys::KeyboardEvent| ctx.keydown(index, &e))
        },
        on_focus: {
            let ctx = ctx.clone();
            Callback::new(move |e: web_sys::FocusEvent| ctx.focus(&e))
        },
        on_blur: {
            let ctx = ctx.clone();
            Callback::new(move |e: web_sys::FocusEvent| ctx.blur(&e))
        },
        on_paste: {
            let ctx = ctx.clone();
            Callback::new(move |e: web_sys::ClipboardEvent| ctx.paste(&e))
        },
    };
    if let Some(template) = template {
        let token = ctx.token(index);
        return template.run(InputOtpSlot { token, index, events });
    }

    let class = {
        let (ctx, pt) = (ctx.clone(), pt.clone());
        move || {
            let mut cn = ClassNames::new();
            write_classes(&mut cn, &ctx.input_state());
            cn.add("p-inputotp-input");
            cn.add_user(pt.input.extra_class());
            cn.finish_cow()
        }
    };
    let style = {
        let pt = pt.clone();
        move || pt.input.has_style().then(|| pt.input.merge_style(""))
    };
    let data = {
        let ctx = ctx.clone();
        move || data_p(&ctx.input_state())
    };
    let ty = {
        let ctx = ctx.clone();
        move || if ctx.mask.get() { "password" } else { "text" }
    };
    let inputmode = {
        let ctx = ctx.clone();
        move || if ctx.integer_only.get() { "numeric" } else { "text" }
    };
    let maxlength = {
        let ctx = ctx.clone();
        move || if index == 0 { ctx.len() } else { 1 }
    };
    let current = {
        let ctx = ctx.clone();
        move || ctx.token(index)
    };
    macro_rules! reactive {
        ($($field:ident),+) => {
            $(let $field = { let ctx = ctx.clone(); move || ctx.$field.get() };)+
        };
    }
    reactive!(name, tabindex, required, readonly, disabled);
    let autofocus = {
        let ctx = ctx.clone();
        move || index == 0 && ctx.autofocus.get()
    };
    let attrs = pt.input.attrs();

    view! {
        <input
            type={ty}
            class={class}
            style={style}
            inputmode={inputmode}
            maxlength={maxlength}
            name={name}
            tabindex={tabindex}
            required={required}
            readonly={readonly}
            disabled={disabled}
            autofocus={autofocus}
            value={current.clone()}
            prop:value={current}
            data-p={data}
            on:input={move |e| events.on_input.run(e)}
            on:keydown={move |e| events.on_keydown.run(e)}
            on:focus={move |e| events.on_focus.run(e)}
            on:blur={move |e| events.on_blur.run(e)}
            on:paste={move |e| events.on_paste.run(e)}
            {..attrs}
        />
    }
    .into_any()
}

/// Состояние и свойства экземпляра.
struct Ctx {
    tokens: RwSignal<Vec<String>>,
    value: Option<RwSignal<String>>,
    length: OptProp<usize>,
    integer_only: Prop<bool>,
    readonly: Prop<bool>,
    disabled: Prop<bool>,
    size: OptProp<Size>,
    variant: OptProp<Variant>,
    invalid: Prop<bool>,
    mask: Prop<bool>,
    name: OptProp<String>,
    tabindex: OptProp<i32>,
    required: Prop<bool>,
    autofocus: Prop<bool>,
    config_variant: Option<Variant>,
    on_change: Option<Callback<String>>,
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    on_blur: Option<Callback<web_sys::FocusEvent>>,
}

impl Ctx {
    fn len(&self) -> usize {
        self.length.get().unwrap_or(DEFAULT_LENGTH)
    }

    fn len_untracked(&self) -> usize {
        self.length.get_untracked().unwrap_or(DEFAULT_LENGTH)
    }

    fn token(&self, index: usize) -> String {
        self.tokens.with(|t| t.get(index).cloned().unwrap_or_default())
    }

    fn input_state(&self) -> InputTextState {
        InputTextState {
            filled: false,
            size: self.size.get(),
            invalid: self.invalid.get(),
            variant: self.variant.get().or(self.config_variant),
            fluid: false,
        }
    }

    fn style_class_into(&self, cn: &mut ClassNames, style_class: &OptProp<String>) {
        style_class.with(|c| cn.add_user(c.as_deref()));
    }

    fn locked(&self) -> bool {
        self.readonly.get_untracked() || self.disabled.get_untracked()
    }

    /// Записывает новые токены: сигнал значения и `on_change`.
    fn commit(&self, tokens: Vec<String>) {
        let joined = join(&tokens);
        self.tokens.set(tokens);
        if let Some(v) = self.value {
            v.set(joined.clone());
        }
        if let Some(cb) = self.on_change {
            cb.run(joined);
        }
    }

    fn input(&self, index: usize, e: &web_sys::Event) {
        let text = event_target_value(e);
        let input_type = e.dyn_ref::<web_sys::InputEvent>().map(web_sys::InputEvent::input_type).unwrap_or_default();
        let len = self.len_untracked();
        if index == 0 && text.chars().count() > 1 {
            // Вставка или автозаполнение в первую ячейку.
            match paste_value(&text, len, self.integer_only.get_untracked()) {
                Some(pasted) => {
                    let last = pasted.chars().count().min(len).saturating_sub(1);
                    self.commit(tokens_of(&pasted, len));
                    if let Some(el) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                        focus_at(&el, |_| last);
                    }
                }
                None => self.restore(e, index),
            }
            return;
        }
        let mut tokens = self.tokens.get_untracked();
        tokens.resize(len.max(tokens.len()), String::new());
        tokens[index] = text;
        tokens.truncate(len);
        self.commit(tokens);
        match input_move(&input_type) {
            Move::Prev => focus_nth(e, -1),
            Move::Next => focus_nth(e, 1),
            Move::None => {}
        }
    }

    /// Возвращает в поле токен ячейки (отклонённый ввод не меняет сигнал, поэтому DOM нужно исправить вручную).
    fn restore(&self, e: &web_sys::Event, index: usize) {
        if let Some(el) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
            el.set_value(&self.token(index));
        }
    }

    fn keydown(&self, index: usize, e: &web_sys::KeyboardEvent) {
        let filled = self.tokens.with_untracked(|t| join(t).chars().count());
        let has_selection = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            .is_some_and(|el| el.selection_start().ok().flatten() != el.selection_end().ok().flatten());
        let cell_empty = self.tokens.with_untracked(|t| t.get(index).is_none_or(String::is_empty));
        let action = key_decision(
            &e.key(),
            e.alt_key() || e.ctrl_key() || e.meta_key(),
            self.integer_only.get_untracked(),
            has_selection,
            filled,
            self.len_untracked(),
            cell_empty,
        );
        match action {
            KeyAction::Allow => {}
            KeyAction::Prevent => e.prevent_default(),
            KeyAction::Prev => {
                e.prevent_default();
                focus_nth(e, -1);
            }
            KeyAction::Next => {
                e.prevent_default();
                focus_nth(e, 1);
            }
        }
    }

    fn focus(&self, e: &web_sys::FocusEvent) {
        if let Some(el) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
            el.select();
        }
        if let Some(cb) = self.on_focus {
            cb.run(e.clone());
        }
    }

    fn blur(&self, e: &web_sys::FocusEvent) {
        if let Some(cb) = self.on_blur {
            cb.run(e.clone());
        }
    }

    fn paste(&self, e: &web_sys::ClipboardEvent) {
        e.prevent_default();
        if self.locked() {
            return;
        }
        let text = e.clipboard_data().and_then(|d| d.get_data("text").ok()).unwrap_or_default();
        let len = self.len_untracked();
        if let Some(pasted) = paste_value(&text, len, self.integer_only.get_untracked()) {
            let last = pasted.chars().count().min(len).saturating_sub(1);
            self.commit(tokens_of(&pasted, len));
            if let Some(el) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                focus_at(&el, |_| last);
            }
        }
    }
}

/// Поля `<input>` рядом с целью (в шаблоне между ними могут быть другие элементы) и позиция цели среди них.
fn siblings_of(target: &web_sys::Element) -> Option<(Vec<web_sys::HtmlInputElement>, usize)> {
    let children = target.parent_element()?.children();
    let inputs: Vec<web_sys::HtmlInputElement> =
        (0..children.length()).filter_map(|i| children.item(i)).filter_map(|el| el.dyn_into().ok()).collect();
    let pos = inputs.iter().position(|el| el.is_same_node(Some(target)))?;
    Some((inputs, pos))
}

/// Фокус на ячейке, сдвинутой на `delta` от целевой (но не дальше крайних).
fn focus_nth(e: &web_sys::Event, delta: isize) {
    let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
    focus_at(&target, |pos| pos.saturating_add_signed(delta));
}

/// Фокус на ячейке с номером, вычисленным по позиции цели; номер ограничивается крайними ячейками.
fn focus_at(target: &web_sys::Element, index: impl FnOnce(usize) -> usize) {
    let Some((inputs, pos)) = siblings_of(target) else { return };
    if let Some(input) = inputs.get(index(pos).min(inputs.len().saturating_sub(1))) {
        let _ = input.focus();
    }
}
