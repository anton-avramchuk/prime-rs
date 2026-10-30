//! Password — порт `p-password` из PrimeNG 21.1.10.
//!
//! Корень `span.p-password` с полем [`InputText`], необязательными значками очистки и показа пароля и панелью-подсказкой
//! `div.p-password-overlay` со шкалой надёжности. Значение — необязательный двусторонний `value: RwSignal<String>`.
//!
//! Надёжность по умолчанию считается [`strength`] (то же, что регулярные выражения PrimeNG по умолчанию); свои правила
//! задаёт свойство `strength`.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_password::Password;
//!
//! let html = view! { <Password toggle_mask=true placeholder="Пароль" /> }.to_html();
//! assert!(html.contains("p-password-input") && html.contains(r#"type="password""#) && html.contains("p-password-unmask-icon"));
//! ```

mod panel;
mod state;

use leptos::either::Either;
use leptos::prelude::*;
use panel::Panel;
use prime_core::{ClassNames, OptProp, Prop, Pt, Size, Variant, use_prime_config};
use prime_icons::{EyeIcon, EyeSlashIcon, TimesIcon};
use prime_inputtext::InputText;
use std::sync::Arc;

pub use state::{PasswordStrength, Phase, RootState, root_classes, strength};

/// Pass-through частей [`Password`].
#[derive(Clone, Debug, Default)]
pub struct PasswordPt {
    /// Корень.
    pub root: Pt,
    /// Значок очистки.
    pub clear_icon: Pt,
    /// Значок «скрыть пароль».
    pub mask_icon: Pt,
    /// Значок «показать пароль».
    pub unmask_icon: Pt,
    /// Панель.
    pub overlay: Pt,
    /// Содержимое панели.
    pub content: Pt,
    /// Шкала.
    pub meter: Pt,
    /// Метка шкалы.
    pub meter_label: Pt,
    /// Текст под шкалой.
    pub meter_text: Pt,
}

/// Поле ввода пароля с индикатором надёжности.
///
/// Каждое свойство принимает значение или сигнал; `value` — двусторонний сигнал.
#[component]
pub fn Password(
    /// Двусторонний сигнал значения; без него значение хранится внутри.
    #[prop(optional)]
    value: Option<RwSignal<String>>,
    /// Показывать панель с шкалой надёжности (по умолчанию да).
    #[prop(default = Prop::Static(true), into)]
    feedback: Prop<bool>,
    /// Значок показа пароля.
    #[prop(optional, into)]
    toggle_mask: Prop<bool>,
    /// Значок очистки (виден при непустом значении).
    #[prop(optional, into)]
    show_clear: Prop<bool>,
    /// Своя оценка надёжности; пустое значение оценивать не нужно (оно считается подсказкой).
    #[prop(optional, into)]
    strength: Option<Callback<String, Option<PasswordStrength>>>,
    /// Подсказка при пустом значении; по умолчанию из локали (`passwordPrompt`).
    #[prop(optional, into)]
    prompt_label: OptProp<String>,
    /// Подпись слабого пароля; по умолчанию из локали (`weak`).
    #[prop(optional, into)]
    weak_label: OptProp<String>,
    /// Подпись среднего пароля; по умолчанию из локали (`medium`).
    #[prop(optional, into)]
    medium_label: OptProp<String>,
    /// Подпись сильного пароля; по умолчанию из локали (`strong`).
    #[prop(optional, into)]
    strong_label: OptProp<String>,
    /// Подсказка в пустом поле.
    #[prop(optional, into)]
    placeholder: OptProp<String>,
    /// Атрибут `autocomplete` поля.
    #[prop(optional, into)]
    autocomplete: OptProp<String>,
    /// Атрибут `id` поля.
    #[prop(optional, into)]
    input_id: OptProp<String>,
    /// Атрибут `aria-label` поля.
    #[prop(optional, into)]
    aria_label: OptProp<String>,
    /// Максимальная длина.
    #[prop(optional, into)]
    max_length: OptProp<i32>,
    /// Недоступно.
    #[prop(optional, into)]
    disabled: Prop<bool>,
    /// Только для чтения.
    #[prop(optional, into)]
    readonly: Prop<bool>,
    /// Обязательное.
    #[prop(optional, into)]
    required: Prop<bool>,
    /// Недопустимое значение.
    #[prop(optional, into)]
    invalid: Prop<bool>,
    /// Вариант оформления.
    #[prop(optional, into)]
    variant: OptProp<Variant>,
    /// Размер.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// На всю ширину.
    #[prop(optional, into)]
    fluid: OptProp<bool>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Встроенный стиль поля.
    #[prop(optional, into)]
    input_style: OptProp<String>,
    /// Дополнительный класс поля.
    #[prop(optional, into)]
    input_style_class: OptProp<String>,
    /// Заголовок панели.
    #[prop(optional, into)]
    header: Option<ViewFn>,
    /// Подвал панели.
    #[prop(optional, into)]
    footer: Option<ViewFn>,
    /// Своё содержимое панели вместо шкалы.
    #[prop(optional, into)]
    content: Option<ViewFn>,
    /// Значок «показать пароль».
    #[prop(optional, into)]
    show_icon: Option<ViewFn>,
    /// Значок «скрыть пароль».
    #[prop(optional, into)]
    hide_icon: Option<ViewFn>,
    /// Значок очистки.
    #[prop(optional, into)]
    clear_icon: Option<ViewFn>,
    /// Вызывается при получении фокуса полем.
    #[prop(optional, into)]
    on_focus: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается при потере фокуса полем.
    #[prop(optional, into)]
    on_blur: Option<Callback<web_sys::FocusEvent>>,
    /// Вызывается после очистки значка.
    #[prop(optional, into)]
    on_clear: Option<Callback<()>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<PasswordPt>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let config = use_prime_config();
    let config_variant = config.input_variant;
    let defaults = Arc::new([
        config.locale.text("passwordPrompt").to_owned(),
        config.locale.text("weak").to_owned(),
        config.locale.text("medium").to_owned(),
        config.locale.text("strong").to_owned(),
    ]);
    let value = value.unwrap_or_default();
    let fluid = prime_core::fluid::fluid_prop(fluid);
    let focused = RwSignal::new(false);
    let unmasked = RwSignal::new(false);
    let panel = Panel::new();
    panel.on_cleanup();
    #[cfg(target_arch = "wasm32")]
    {
        let handle = StoredValue::new_local(Some(window_event_listener(leptos::ev::resize, move |_| panel.close())));
        on_cleanup(move || {
            if let Some(Some(handle)) = handle.try_update_value(Option::take) {
                handle.remove();
            }
        });
    }

    let level = Signal::derive(move || {
        value.with(|v| match strength {
            Some(custom) if !v.is_empty() => custom.run(v.clone()),
            Some(_) => None,
            None => state::strength(v),
        })
    });
    let text = {
        let (prompt_label, weak_label, medium_label, strong_label, defaults) =
            (prompt_label, weak_label, medium_label, strong_label, defaults);
        move || {
            let (label, index) = match level.get() {
                None => (&prompt_label, 0),
                Some(PasswordStrength::Weak) => (&weak_label, 1),
                Some(PasswordStrength::Medium) => (&medium_label, 2),
                Some(PasswordStrength::Strong) => (&strong_label, 3),
            };
            label.get().unwrap_or_else(|| defaults[index].clone())
        }
    };

    let root_class = {
        let (pt, variant, fluid) = (pt.clone(), variant.clone(), fluid.clone());
        Signal::derive(move || {
            let base = root_classes(RootState {
                filled: value.with(|v| !v.is_empty()),
                focused: focused.get(),
                variant_filled: variant.get().or(config_variant) == Some(Variant::Filled),
                fluid: fluid.get(),
            });
            let mut cn = ClassNames::new();
            cn.add(&base);
            style_class.with(|c| cn.add_user(c.as_deref()));
            pt.root.merge_class(&cn.finish())
        })
    };
    let root_style = {
        let pt = pt.clone();
        Signal::derive(move || {
            let user = style.get().unwrap_or_default();
            pt.root.merge_style_parts(&["position: relative", &user])
        })
    };
    let data_p = {
        let fluid = fluid.clone();
        Signal::derive(move || fluid.get().then_some("fluid"))
    };
    let input_class = Signal::derive(move || {
        let mut cn = ClassNames::new();
        cn.add("p-password-input");
        input_style_class.with(|c| cn.add_user(c.as_deref()));
        Some(cn.finish())
    });
    let input_type = Signal::derive(move || Some(if unmasked.get() { "text" } else { "password" }.to_owned()));

    let on_input = {
        let feedback = feedback.clone();
        Callback::new(move |_: web_sys::Event| {
            if feedback.get_untracked() {
                panel.open();
            }
        })
    };
    let focus = {
        let feedback = feedback.clone();
        Callback::new(move |ev: web_sys::FocusEvent| {
            focused.set(true);
            if feedback.get_untracked() {
                panel.open();
            }
            if let Some(cb) = on_focus {
                cb.run(ev);
            }
        })
    };
    let blur = {
        let feedback = feedback.clone();
        Callback::new(move |ev: web_sys::FocusEvent| {
            focused.set(false);
            if feedback.get_untracked() {
                panel.close();
            }
            if let Some(cb) = on_blur {
                cb.run(ev);
            }
        })
    };
    let keydown = {
        let feedback = feedback.clone();
        move |ev: web_sys::KeyboardEvent| {
            if ev.key() == "Escape" && feedback.get_untracked() {
                panel.close();
            }
        }
    };

    let toggle_view = {
        let pt = pt.clone();
        move || {
            toggle_mask.get().then(|| {
                let click = move |_: web_sys::MouseEvent| unmasked.update(|u| *u = !*u);
                // Фокус остаётся в поле, иначе `blur` закроет панель раньше клика.
                let keep_focus = |ev: web_sys::MouseEvent| ev.prevent_default();
                if unmasked.get() {
                    let part = &pt.mask_icon;
                    let class = part.merge_class("p-password-toggle-mask-icon p-password-mask-icon");
                    match hide_icon.clone() {
                        Some(icon) => Either::Left(
                            view! { <span class={class} on:click={click} on:mousedown={keep_focus} {..part.attrs()}>{icon.run()}</span> },
                        ),
                        None => Either::Right(
                            view! { <EyeSlashIcon class={class.into_owned()} on:click={click} on:mousedown={keep_focus} {..part.attrs()} /> },
                        ),
                    }
                    .into_any()
                } else {
                    let part = &pt.unmask_icon;
                    let class = part.merge_class("p-password-toggle-mask-icon p-password-unmask-icon");
                    match show_icon.clone() {
                        Some(icon) => Either::Left(
                            view! { <span class={class} on:click={click} on:mousedown={keep_focus} {..part.attrs()}>{icon.run()}</span> },
                        ),
                        None => Either::Right(
                            view! { <EyeIcon class={class.into_owned()} on:click={click} on:mousedown={keep_focus} {..part.attrs()} /> },
                        ),
                    }
                    .into_any()
                }
            })
        }
    };
    let clear_view = {
        let pt = pt.clone();
        move || {
            (show_clear.get() && value.with(|v| !v.is_empty())).then(|| {
                let click = move |_: web_sys::MouseEvent| {
                    value.set(String::new());
                    if let Some(cb) = on_clear {
                        cb.run(());
                    }
                };
                let keep_focus = |ev: web_sys::MouseEvent| ev.prevent_default();
                let part = &pt.clear_icon;
                let class = part.merge_class("p-password-clear-icon");
                match clear_icon.clone() {
                    Some(icon) => Either::Left(
                        view! { <span class={class} on:click={click} on:mousedown={keep_focus} {..part.attrs()}>{icon.run()}</span> },
                    ),
                    None => Either::Right(
                        view! { <TimesIcon class={class.into_owned()} on:click={click} on:mousedown={keep_focus} {..part.attrs()} /> },
                    ),
                }
            })
        }
    };

    let rendered = Signal::derive(move || panel.phase.get() != Phase::Hidden);
    let panel_view = {
        let pt = pt.clone();
        move || {
            rendered.get().then(|| {
                let overlay_class = {
                    let pt = pt.clone();
                    move || {
                        let mut cn = ClassNames::new();
                        cn.add("p-password-overlay p-component").add(panel.phase.get().animation_class());
                        pt.overlay.merge_class(&cn.finish())
                    }
                };
                let overlay_style = pt.overlay.merge_style("position: absolute");
                let body = match content.clone() {
                    Some(custom) => custom.run(),
                    None => {
                        let meter_class = pt.meter_label.clone();
                        let (meter_style, meter_data) = (move || level.get().map(|l| format!("width: {}", l.width())), move || level.get().map(|l| l.as_str()));
                        let label_class = move || meter_class.merge_class(level.get().map_or("p-password-meter-label", PasswordStrength::meter_class));
                        let text = text.clone();
                        view! {
                            <div class={pt.content.merge_class("p-password-content")} {..pt.content.attrs()}>
                                <div class={pt.meter.merge_class("p-password-meter")} {..pt.meter.attrs()}>
                                    <div class={label_class} style={meter_style} data-p={meter_data} {..pt.meter_label.attrs()}></div>
                                </div>
                                <div class={pt.meter_text.merge_class("p-password-meter-text")} {..pt.meter_text.attrs()}>{text}</div>
                            </div>
                        }
                        .into_any()
                    }
                };
                view! {
                    <div node_ref={panel.panel_ref} class={overlay_class} style={overlay_style} data-p="overlay-self" {..pt.overlay.attrs()}>
                        {header.as_ref().map(ViewFn::run)}
                        {body}
                        {footer.as_ref().map(ViewFn::run)}
                    </div>
                }
            })
        }
    };

    view! {
        <span node_ref={panel.root_ref} class={root_class} style={root_style} data-p={data_p} on:keydown={keydown} {..pt.root.attrs()}>
            <InputText
                value=value
                input_type=input_type
                id=input_id
                placeholder=placeholder
                autocomplete=autocomplete
                aria_label=aria_label
                maxlength=max_length
                disabled=disabled
                readonly=readonly
                required=required
                invalid=invalid
                variant=variant
                size=size
                style=input_style
                style_class=input_class
                on_input=on_input
                on_focus=focus
                on_blur=blur
            />
            {clear_view}
            {toggle_view}
            {panel_view}
        </span>
    }
}
