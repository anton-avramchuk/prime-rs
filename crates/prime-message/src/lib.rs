//! Message — порт `p-message` из PrimeNG 21.1.10.
//!
//! Разметка и классы как в PrimeNG: корень `div.p-message[role=alert]` (вместо хоста `<p-message>`), внутри
//! `div.p-message-content-wrapper` > `div.p-message-content` с иконкой, текстом и кнопкой закрытия.
//!
//! Как и в PrimeNG, при заданном `text` выводится `span.p-message-text` с текстом, а следом всегда второй `span.p-message-text`
//! с дочерним содержимым. Закрытое сообщение скрывается (`p-message-leave-active` и `display: none`) без анимации.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_message::{Message, MessageSeverity};
//!
//! let html = view! { <Message severity=MessageSeverity::Success text="Saved" closable=true /> }.to_html();
//! assert!(html.contains("p-message-success") && html.contains("p-message-close-button") && html.contains("Saved"));
//! ```

mod state;

use leptos::either::Either;
use leptos::prelude::*;
use prime_core::{ClassNames, OptProp, Prop, Pt, Size, all_static, use_prime_config};
use prime_icons::TimesIcon;
use std::sync::Arc;

pub use state::{MessageSeverity, MessageState, MessageVariant, data_p, write_classes};

/// Событие закрытия: исходное событие браузера.
#[derive(Clone, Debug)]
pub struct MessageClose {
    /// Исходное событие `click`.
    pub original_event: web_sys::Event,
}

/// Pass-through частей Message.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MessagePt {
    /// Корневой элемент.
    pub root: Pt,
    /// Обёртка содержимого (`p-message-content-wrapper`).
    pub content_wrapper: Pt,
    /// Содержимое (`p-message-content`).
    pub content: Pt,
    /// Иконка-класс (`i.p-message-icon`).
    pub icon: Pt,
    /// Текст (`span.p-message-text`, оба экземпляра).
    pub text: Pt,
    /// Кнопка закрытия.
    pub close_button: Pt,
    /// Иконка закрытия (для `close_icon`-класса — атрибуты, для встроенной иконки — только класс).
    pub close_icon: Pt,
}

/// Сообщение.
///
/// Каждое свойство принимает значение или сигнал.
#[component]
pub fn Message(
    /// Серьёзность (по умолчанию `info`).
    #[prop(optional, into)]
    severity: Prop<MessageSeverity>,
    /// Вариант оформления.
    #[prop(optional, into)]
    variant: OptProp<MessageVariant>,
    /// Размер.
    #[prop(optional, into)]
    size: OptProp<Size>,
    /// Текст сообщения.
    #[prop(optional, into)]
    text: OptProp<String>,
    /// CSS-класс иконки (`<i class="p-message-icon …">`).
    #[prop(optional, into)]
    icon: OptProp<String>,
    /// CSS-класс иконки закрытия.
    #[prop(optional, into)]
    close_icon: OptProp<String>,
    /// Показывать кнопку закрытия.
    #[prop(optional)]
    closable: bool,
    /// Время жизни в миллисекундах; по истечении сообщение скрывается.
    #[prop(optional)]
    life: Option<u32>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Своя иконка вместо `icon` (выводится без обёртки).
    #[prop(optional, into)]
    icon_template: Option<ViewFn>,
    /// Своя иконка кнопки закрытия.
    #[prop(optional, into)]
    close_icon_template: Option<ViewFn>,
    /// Своё содержимое вместо текста и дочерних элементов; получает колбэк закрытия.
    #[prop(optional, into)]
    container_template: Option<Callback<Callback<web_sys::Event>, AnyView>>,
    /// Вызывается после закрытия пользователем.
    #[prop(optional, into)]
    on_close: Option<Callback<MessageClose>>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<MessagePt>,
    /// Содержимое сообщения.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let is_static = all_static!(severity, variant, size, text, icon, close_icon, style, style_class) && !closable && life.is_none();
    let visible = (!is_static).then(|| RwSignal::new(true));
    let props = Arc::new(Props {
        severity,
        variant,
        size,
        text,
        icon,
        close_icon,
        closable,
        style,
        style_class,
        icon_template,
        close_icon_template,
        visible,
    });

    if let (Some(ms), Some(visible)) = (life, visible) {
        #[cfg(target_arch = "wasm32")]
        {
            // Таймер ставится только в браузере; при SSR сообщение остаётся видимым.
            set_timeout(move || visible.set(false), std::time::Duration::from_millis(u64::from(ms)));
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = (ms, visible);
    }

    let close = Callback::new(move |e: web_sys::Event| {
        if let Some(visible) = visible {
            visible.set(false);
        }
        if let Some(cb) = on_close {
            cb.run(MessageClose { original_event: e });
        }
    });
    let kids = children.map(|c| c().into_any());
    let aria_close = use_prime_config().locale.text("aria.close").to_owned();
    let (root_attrs, wrapper_attrs, content_attrs) = (pt.root.attrs(), pt.content_wrapper.attrs(), pt.content.attrs());

    if is_static {
        let (p, t) = (&*props, &*pt);
        let data = data_of(p);
        let (root_class, root_style) = (root_class(p, t), root_style(p, t));
        let (wrapper_class, content_class) =
            (t.content_wrapper.merge_class("p-message-content-wrapper"), t.content.merge_class("p-message-content"));
        let icon = icon_of(p, t, &data);
        let body = match container_template {
            Some(tpl) => tpl.run(close),
            None => view! {
                {text_of(t, &data, p.text.get())}
                <span class={text_class(t)} data-p={data.clone()} {..t.text.attrs()}>{kids}</span>
            }
            .into_any(),
        };
        let close_view = ();
        Either::Left(message_markup!(
            root_class,
            root_style,
            data,
            wrapper_class,
            content_class,
            icon,
            body,
            close_view,
            root_attrs,
            wrapper_attrs,
            content_attrs
        ))
    } else {
        let (p, t) = (&props, &pt);
        let root_class = rx(p, t, root_class);
        let root_style = rx(p, t, root_style);
        let data = rx(p, t, |p, _| data_of(p));
        let wrapper_class = rx(p, t, |_, t| t.content_wrapper.merge_class("p-message-content-wrapper"));
        let content_class = rx(p, t, |_, t| t.content.merge_class("p-message-content"));
        let icon = rx(p, t, |p, t| icon_of(p, t, &data_of(p)));
        let body = match container_template {
            Some(tpl) => tpl.run(close),
            None => {
                let text = rx(p, t, |p, t| text_of(t, &data_of(p), p.text.get()));
                let data_kids = data.clone();
                let kids_class = text_class(t);
                let kids_attrs = t.text.attrs();
                view! {
                    {text}
                    <span class={kids_class} data-p={data_kids} {..kids_attrs}>{kids}</span>
                }
                .into_any()
            }
        };
        let close_view = {
            let (props, pt) = (props.clone(), pt.clone());
            move || close_of(&props, &pt, &data_of(&props), close, &aria_close)
        };
        Either::Right(message_markup!(
            root_class,
            root_style,
            data,
            wrapper_class,
            content_class,
            icon,
            body,
            close_view,
            root_attrs,
            wrapper_attrs,
            content_attrs
        ))
    }
}

/// Разметка Message: одна для статической и реактивной ветвей (меняются только выражения атрибутов).
macro_rules! message_markup {
    (
        $root_class:expr, $root_style:expr, $data:expr, $wrapper_class:expr, $content_class:expr, $icon:expr, $body:expr, $close:expr,
        $root_attrs:expr, $wrapper_attrs:expr, $content_attrs:expr
    ) => {{
        let data_wrapper = $data.clone();
        let data_content = $data.clone();
        view! {
            <div role="alert" aria-live="polite" class={$root_class} style={$root_style} data-p={$data} {..$root_attrs}>
                <div class={$wrapper_class} data-p={data_wrapper} {..$wrapper_attrs}>
                    <div class={$content_class} data-p={data_content} {..$content_attrs}>
                        {$icon}
                        {$body}
                        {$close}
                    </div>
                </div>
            </div>
        }
    }};
}
use message_markup;

/// Замыкание над свойствами экземпляра: клоны `Arc` и указатель на функцию без захвата.
fn rx<T>(props: &Arc<Props>, pt: &Arc<MessagePt>, f: fn(&Props, &MessagePt) -> T) -> impl Fn() -> T + Clone + Send + Sync + 'static + use<T>
where
    T: 'static,
{
    let (props, pt) = (props.clone(), pt.clone());
    move || f(&props, &pt)
}

fn state_of(props: &Props) -> MessageState {
    MessageState { severity: props.severity.get(), variant: props.variant.get(), size: props.size.get() }
}

fn data_of(props: &Props) -> String {
    data_p(&state_of(props))
}

fn is_hidden(props: &Props) -> bool {
    props.visible.is_some_and(|v| !v.get())
}

fn root_class(props: &Props, pt: &MessagePt) -> String {
    let mut cn = ClassNames::new();
    write_classes(&mut cn, &state_of(props));
    props.style_class.with(|c| cn.add_opt(c.as_deref()));
    cn.add_opt(pt.root.extra_class()).add_if(is_hidden(props), "p-message-leave-active");
    cn.finish()
}

fn root_style(props: &Props, pt: &MessagePt) -> Option<String> {
    let hidden = is_hidden(props);
    props.style.with(|st| {
        (st.is_some() || pt.root.has_style() || hidden).then(|| {
            let base = pt.root.merge_style(st.as_deref().unwrap_or(""));
            if hidden { format!("{base} display: none;") } else { base }
        })
    })
}

fn text_class(pt: &MessagePt) -> String {
    pt.text.merge_class("p-message-text")
}

/// Первый `span.p-message-text` с текстом; без текста — ничего.
fn text_of(pt: &MessagePt, data: &str, text: Option<String>) -> Option<AnyView> {
    let text = text.filter(|t| !t.is_empty())?;
    Some(view! { <span class={text_class(pt)} data-p={data.to_owned()} {..pt.text.attrs()}>{text}</span> }.into_any())
}

fn icon_of(props: &Props, pt: &MessagePt, data: &str) -> AnyView {
    let template = props.icon_template.as_ref().map(|t| t.run());
    let class = props.icon.with(|icon| {
        icon.as_deref().filter(|i| !i.is_empty()).map(|icon| {
            let mut cn = ClassNames::new();
            cn.add("p-message-icon").add(icon).add_opt(pt.icon.extra_class());
            cn.finish()
        })
    });
    let glyph = class.map(|class| view! { <i class={class} data-p={data.to_owned()} {..pt.icon.attrs()}></i> });
    view! { {template} {glyph} }.into_any()
}

fn close_of(props: &Props, pt: &MessagePt, data: &str, close: Callback<web_sys::Event>, aria: &str) -> AnyView {
    if !props.closable {
        return ().into_any();
    }
    let inner = if let Some(class) = props.close_icon.get().filter(|c| !c.is_empty()) {
        let mut cn = ClassNames::new();
        cn.add("p-message-close-icon").add(&class).add_opt(pt.close_icon.extra_class());
        view! { <i class={cn.finish()} data-p={data.to_owned()} {..pt.close_icon.attrs()}></i> }.into_any()
    } else if let Some(template) = &props.close_icon_template {
        template.run()
    } else {
        view! { <TimesIcon class={pt.close_icon.merge_class("p-message-close-icon")} /> }.into_any()
    };
    view! {
        <button
            type="button"
            class={pt.close_button.merge_class("p-message-close-button")}
            aria-label={aria.to_owned()}
            data-p={data.to_owned()}
            on:click={move |e: web_sys::MouseEvent| close.run(e.into())}
            {..pt.close_button.attrs()}
        >
            {inner}
        </button>
    }
    .into_any()
}

/// Свойства экземпляра.
struct Props {
    severity: Prop<MessageSeverity>,
    variant: OptProp<MessageVariant>,
    size: OptProp<Size>,
    text: OptProp<String>,
    icon: OptProp<String>,
    close_icon: OptProp<String>,
    closable: bool,
    style: OptProp<String>,
    style_class: OptProp<String>,
    icon_template: Option<ViewFn>,
    close_icon_template: Option<ViewFn>,
    visible: Option<RwSignal<bool>>,
}
