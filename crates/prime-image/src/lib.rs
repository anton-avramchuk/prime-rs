//! Image — порт `p-image` из PrimeNG 21.1.10.
//!
//! Корень `div.p-image.p-component` с обычным `<img>`. При `preview` поверх картинки лежит кнопка
//! `button.p-image-preview-mask` (по наведению — значок глаза); клик по ней открывает в `Portal` модальный просмотр:
//! маска `div.p-image-mask.p-overlay-mask[role=dialog]`, панель действий (поворот на ±90°, масштаб с шагом 0.1, закрытие) и
//! увеличенная картинка `img.p-image-original`. Просмотр закрывается кнопкой, кликом по маске и клавишей `Escape`, блокирует
//! прокрутку страницы, держит фокус внутри и занимает слой группы `Modal`. Просмотр выводится только в браузере.
//!
//! ```
//! use leptos::prelude::*;
//! use prime_image::Image;
//!
//! let html = Owner::new().with(|| view! { <Image src="a.jpg" alt="Photo" width="250" preview=true /> }.to_html());
//! assert!(html.contains("p-image-preview-mask") && html.contains("src=\"a.jpg\"") && html.contains("width=\"250\""));
//! ```

mod state;

use leptos::prelude::*;
use prime_core::{OptProp, Prop, Pt};
use prime_dom::Portal;
use prime_icons::{EyeIcon, RefreshIcon, SearchMinusIcon, SearchPlusIcon, TimesIcon, UndoIcon};
use std::sync::Arc;

pub use state::{
    ActionKind, ImageLoading, ZOOM_DEFAULT, ZOOM_MAX, ZOOM_MIN, ZOOM_STEP, action_classes, dimension_style, mask_should_close,
    preview_style, root_classes, transform, zoom_in, zoom_in_disabled, zoom_out, zoom_out_disabled,
};

#[cfg(target_arch = "wasm32")]
use leptos::leptos_dom::helpers::{TimeoutHandle, set_timeout_with_handle};
#[cfg(target_arch = "wasm32")]
use prime_core::{ZGroup, use_prime_config};
#[cfg(target_arch = "wasm32")]
use prime_dom::{FocusTrap, block_body_scroll, unblock_body_scroll};
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Запасное время выхода: без стилей или при отключённой анимации `animationend` не придёт.
#[cfg(target_arch = "wasm32")]
const LEAVE_FALLBACK: Duration = Duration::from_millis(400);
/// Задержка фокуса после показа и после `Escape` (как в PrimeNG).
#[cfg(target_arch = "wasm32")]
const FOCUS_DELAY: Duration = Duration::from_millis(25);

/// Контекст представления `preview_content`.
#[derive(Clone)]
pub struct ImagePreviewContext {
    /// Класс увеличенной картинки (`p-image-original` и состояние анимации).
    pub class: String,
    /// Стиль с `transform` (поворот и масштаб), меняется при действиях панели.
    pub style: Signal<String>,
    /// Вызывается при клике по картинке: такой клик не закрывает просмотр.
    pub preview_callback: Callback<()>,
}

/// Pass-through частей Image.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ImagePt {
    /// Корневой элемент.
    pub root: Pt,
    /// Основная картинка.
    pub image: Pt,
    /// Кнопка просмотра.
    pub preview_mask: Pt,
    /// Значок кнопки просмотра (только класс).
    pub preview_icon: Pt,
    /// Маска просмотра.
    pub mask: Pt,
    /// Панель действий.
    pub toolbar: Pt,
    /// Кнопка поворота вправо.
    pub rotate_right_button: Pt,
    /// Кнопка поворота влево.
    pub rotate_left_button: Pt,
    /// Кнопка уменьшения.
    pub zoom_out_button: Pt,
    /// Кнопка увеличения.
    pub zoom_in_button: Pt,
    /// Кнопка закрытия.
    pub close_button: Pt,
    /// Увеличенная картинка.
    pub original: Pt,
}

/// Состояние, общее для обработчиков просмотра.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Core {
    /// Просмотр запрошен.
    open: RwSignal<bool>,
    /// Маска в DOM (пока играет анимация выхода, она ещё в DOM).
    rendered: RwSignal<bool>,
    /// Идёт анимация выхода.
    leaving: RwSignal<bool>,
    rotate: RwSignal<i32>,
    scale: RwSignal<f64>,
    /// Последний клик пришёл с увеличенной картинки: маску он не закрывает.
    preview_click: StoredValue<bool>,
    /// Выданный маске слой.
    z_index: RwSignal<Option<i32>>,
    scroll_blocked: StoredValue<bool>,
    was_open: StoredValue<bool>,
    mask: NodeRef<leptos::html::Div>,
    close_button: NodeRef<leptos::html::Button>,
    preview_button: NodeRef<leptos::html::Button>,
    on_show: Option<Callback<()>>,
    on_hide: Option<Callback<()>>,
    #[cfg(target_arch = "wasm32")]
    trap: StoredValue<Option<FocusTrap>, LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    escape: StoredValue<Option<Box<dyn FnOnce()>>, LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    timer: StoredValue<Option<TimeoutHandle>, LocalStorage>,
}

impl Core {
    /// Снимает слушатель `Escape` и ловушку фокуса.
    fn release_input(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(remove) = self.escape.try_update_value(Option::take).flatten() {
                remove();
            }
            self.trap.try_update_value(Option::take);
        }
    }

    /// Освобождает слой и блокировку прокрутки.
    fn release_layer(&self) {
        if let Some(z) = self.z_index.try_update(Option::take).flatten() {
            prime_core::zindex::revert(z);
        }
        if self.scroll_blocked.try_update_value(std::mem::take).unwrap_or(false) {
            #[cfg(target_arch = "wasm32")]
            unblock_body_scroll();
        }
    }

    /// Полное освобождение при размонтировании компонента.
    fn teardown(&self) {
        self.release_input();
        self.release_layer();
        #[cfg(target_arch = "wasm32")]
        if let Some(handle) = self.timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
    }

    /// Просмотр запрошен закрыть.
    fn close(&self) {
        self.open.set(false);
    }

    /// Фокус на кнопку просмотра (после `Escape`).
    #[cfg(target_arch = "wasm32")]
    fn focus_preview_button(self: &Arc<Self>) {
        let this = self.clone();
        leptos::prelude::set_timeout(
            move || {
                if let Some(button) = this.preview_button.get_untracked() {
                    let _ = button.focus();
                }
            },
            FOCUS_DELAY,
        );
    }

    /// Маска только что попала в DOM: слой, блокировка прокрутки, ловушка фокуса, `Escape`, фокус, `on_show`.
    ///
    /// Повторное открытие во время выхода вызывает это снова, пока слой и блокировка ещё удерживаются: второй раз их не берём.
    #[cfg(target_arch = "wasm32")]
    fn opened(self: &Arc<Self>) {
        if self.z_index.get_untracked().is_none() {
            let base = use_prime_config().z_index.base(ZGroup::Modal);
            self.z_index.set(Some(prime_core::zindex::generate(ZGroup::Modal, base)));
        }
        if !self.scroll_blocked.get_value() {
            block_body_scroll();
            self.scroll_blocked.set_value(true);
        }
        if let Some(mask) = self.mask.get_untracked() {
            let mask: web_sys::HtmlElement = mask.unchecked_into();
            self.trap.set_value(FocusTrap::attach(&mask));
        }
        let this = self.clone();
        let handle = window_event_listener(leptos::ev::keydown, move |e: web_sys::KeyboardEvent| {
            if e.key() != "Escape" || !this.open.get_untracked() {
                return;
            }
            // Поверх просмотра может лежать другой слой (например, диалог): закрывается только верхний.
            let own = this.z_index.get_untracked().unwrap_or(0);
            if own >= prime_core::zindex::current() {
                e.prevent_default();
                this.close();
                this.focus_preview_button();
            }
        });
        self.escape.set_value(Some(Box::new(move || handle.remove())));
        let this = self.clone();
        leptos::prelude::set_timeout(
            move || {
                if let Some(button) = this.close_button.get_untracked() {
                    let _ = button.focus();
                }
            },
            FOCUS_DELAY,
        );
        if let Some(cb) = self.on_show {
            cb.run(());
        }
    }

    /// Просмотр закрыт: играет анимация выхода.
    #[cfg(target_arch = "wasm32")]
    fn begin_leave(self: &Arc<Self>) {
        self.release_input();
        self.leaving.set(true);
        let this = self.clone();
        if let Ok(handle) = set_timeout_with_handle(move || this.finish_leave(), LEAVE_FALLBACK) {
            self.timer.set_value(Some(handle));
        }
    }

    /// Анимация выхода закончилась: маска убирается из DOM, слой и блокировка освобождаются, поворот и масштаб сброшены.
    #[cfg(target_arch = "wasm32")]
    fn finish_leave(&self) {
        if self.rendered.is_disposed() || !self.leaving.get_untracked() {
            return;
        }
        if let Some(handle) = self.timer.try_update_value(Option::take).flatten() {
            handle.clear();
        }
        self.release_layer();
        self.leaving.set(false);
        self.rendered.set(false);
        self.rotate.set(0);
        self.scale.set(ZOOM_DEFAULT);
        if let Some(cb) = self.on_hide {
            cb.run(());
        }
    }
}

/// Картинка с необязательным просмотром в модальном слое.
///
/// Свойства принимают значение или сигнал. Представления (`image`, `indicator`, `preview_content`, значки панели) строятся заново
/// при каждом показе.
#[component]
pub fn Image(
    /// Адрес картинки.
    #[prop(optional, into)]
    src: OptProp<String>,
    /// `srcset` картинки.
    #[prop(optional, into)]
    srcset: OptProp<String>,
    /// `sizes` картинки.
    #[prop(optional, into)]
    sizes: OptProp<String>,
    /// Адрес картинки в просмотре (по умолчанию `src`).
    #[prop(optional, into)]
    preview_image_src: OptProp<String>,
    /// `srcset` картинки в просмотре.
    #[prop(optional, into)]
    preview_image_srcset: OptProp<String>,
    /// `sizes` картинки в просмотре.
    #[prop(optional, into)]
    preview_image_sizes: OptProp<String>,
    /// Атрибут `alt`.
    #[prop(optional, into)]
    alt: OptProp<String>,
    /// Атрибут `width` (число — в пикселях; задаёт и размер кнопки просмотра).
    #[prop(optional, into)]
    width: OptProp<String>,
    /// Атрибут `height`.
    #[prop(optional, into)]
    height: OptProp<String>,
    /// Атрибут `loading`.
    #[prop(optional, into)]
    loading: OptProp<ImageLoading>,
    /// Включает просмотр.
    #[prop(optional, into)]
    preview: Prop<bool>,
    /// Класс картинки.
    #[prop(optional, into)]
    image_class: OptProp<String>,
    /// Встроенный стиль картинки.
    #[prop(optional, into)]
    image_style: OptProp<String>,
    /// Дополнительный класс корня.
    #[prop(optional, into)]
    style_class: OptProp<String>,
    /// Встроенный стиль корня.
    #[prop(optional, into)]
    style: OptProp<String>,
    /// Pass-through частей.
    #[prop(optional)]
    pt: Option<ImagePt>,
    /// Своя картинка вместо `img`; получает обратный вызов ошибки загрузки.
    #[prop(optional)]
    image: Option<Callback<Callback<web_sys::Event>, AnyView>>,
    /// Свой значок кнопки просмотра вместо глаза.
    #[prop(optional, into)]
    indicator: Option<ViewFn>,
    /// Своя увеличенная картинка вместо `img.p-image-original`.
    #[prop(optional)]
    preview_content: Option<Callback<ImagePreviewContext, AnyView>>,
    /// Значок поворота вправо.
    #[prop(optional, into)]
    rotate_right_icon: Option<ViewFn>,
    /// Значок поворота влево.
    #[prop(optional, into)]
    rotate_left_icon: Option<ViewFn>,
    /// Значок уменьшения.
    #[prop(optional, into)]
    zoom_out_icon: Option<ViewFn>,
    /// Значок увеличения.
    #[prop(optional, into)]
    zoom_in_icon: Option<ViewFn>,
    /// Значок закрытия.
    #[prop(optional, into)]
    close_icon: Option<ViewFn>,
    /// Просмотр показан.
    #[prop(optional)]
    on_show: Option<Callback<()>>,
    /// Просмотр скрыт (после анимации выхода).
    #[prop(optional)]
    on_hide: Option<Callback<()>>,
    /// Ошибка загрузки основной картинки.
    #[prop(optional)]
    on_image_error: Option<Callback<web_sys::Event>>,
) -> impl IntoView {
    let pt = Arc::new(pt.unwrap_or_default());
    let core = Arc::new(Core {
        open: RwSignal::new(false),
        rendered: RwSignal::new(false),
        leaving: RwSignal::new(false),
        rotate: RwSignal::new(0),
        scale: RwSignal::new(ZOOM_DEFAULT),
        preview_click: StoredValue::new(false),
        z_index: RwSignal::new(None),
        scroll_blocked: StoredValue::new(false),
        was_open: StoredValue::new(false),
        mask: NodeRef::new(),
        close_button: NodeRef::new(),
        preview_button: NodeRef::new(),
        on_show,
        on_hide,
        #[cfg(target_arch = "wasm32")]
        trap: StoredValue::new_local(None),
        #[cfg(target_arch = "wasm32")]
        escape: StoredValue::new_local(None),
        #[cfg(target_arch = "wasm32")]
        timer: StoredValue::new_local(None),
    });
    {
        let core = core.clone();
        on_cleanup(move || core.teardown());
    }
    // Показ, выход и побочные эффекты (слой, прокрутка, ловушка фокуса, `Escape`) — только в браузере.
    #[cfg(target_arch = "wasm32")]
    {
        let core = core.clone();
        Effect::new(move |_| {
            let shown = core.open.get();
            // Маска попадает в DOM после этого эффекта, поэтому следим и за ссылкой на неё.
            let mounted = core.mask.get().is_some();
            let was_open = core.was_open.get_value();
            if shown {
                if let Some(handle) = core.timer.try_update_value(Option::take).flatten() {
                    handle.clear();
                }
                // Повторное открытие во время выхода: маска ещё в DOM, возвращаем её в состояние «открыта».
                if core.leaving.get_untracked() {
                    core.leaving.set(false);
                }
                if !core.rendered.get_untracked() {
                    core.rendered.set(true);
                }
                if mounted && !was_open {
                    core.was_open.set_value(true);
                    core.opened();
                }
            } else if was_open {
                core.was_open.set_value(false);
                core.begin_leave();
            } else if core.rendered.get_untracked() && !core.leaving.get_untracked() {
                // Закрыли, не дождавшись появления маски в DOM: ждать нечего.
                core.rendered.set(false);
            }
        });
    }

    let on_error = move |e: web_sys::Event| {
        if let Some(cb) = on_image_error {
            cb.run(e);
        }
    };
    let preview_root = preview.clone();
    let root_class = {
        let pt = pt.clone();
        move || {
            let extra = style_class.with(|c| c.clone());
            let base = root_classes(preview_root.get(), extra.as_deref());
            pt.root.merge_class(&base).into_owned()
        }
    };
    let root_style = {
        let pt = pt.clone();
        move || {
            let merged = style.with(|s| pt.root.merge_style(s.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then_some(merged)
        }
    };
    let img_class = {
        let pt = pt.clone();
        move || {
            let merged = image_class.with(|c| pt.image.merge_class(c.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then(|| merged.into_owned())
        }
    };
    let img_style = {
        let pt = pt.clone();
        move || {
            let merged = image_style.with(|s| pt.image.merge_style(s.as_deref().unwrap_or_default()));
            (!merged.is_empty()).then_some(merged)
        }
    };
    let (width_img, height_img) = (width.clone(), height.clone());
    let (src_img, srcset_img, sizes_img) = (src.clone(), srcset.clone(), sizes.clone());
    let image_view = match image {
        Some(custom) => custom.run(Callback::new(on_error)).into_any(),
        None => {
            let attrs = pt.image.attrs();
            view! {
                <img
                    src={move || src_img.get()}
                    srcset={move || srcset_img.get()}
                    sizes={move || sizes_img.get()}
                    alt={move || alt.get()}
                    width={move || width_img.get()}
                    height={move || height_img.get()}
                    loading={move || loading.get().map(ImageLoading::as_str)}
                    class={img_class}
                    style={img_style}
                    on:error={move |e: web_sys::ErrorEvent| on_error(e.into())}
                    {..attrs}
                />
            }
            .into_any()
        }
    };

    let parts = Arc::new(Parts {
        src,
        srcset,
        sizes,
        preview_image_src,
        preview_image_srcset,
        preview_image_sizes,
        preview_content,
        rotate_right_icon,
        rotate_left_icon,
        zoom_out_icon,
        zoom_in_icon,
        close_icon,
    });

    let preview_button = {
        let (core, pt) = (core.clone(), pt.clone());
        move || {
            preview.get().then(|| {
                let icon = match indicator.clone() {
                    Some(custom) => custom.run(),
                    None => {
                        let class = pt.preview_icon.merge_class("p-image-preview-icon").into_owned();
                        view! { <EyeIcon class=class /> }.into_any()
                    }
                };
                let label = use_prime_config_text("aria.zoomImage");
                let (w, h) = (width.clone(), height.clone());
                let size = move || w.with(|w| h.with(|h| dimension_style(w.as_deref(), h.as_deref())));
                let open = {
                    let core = core.clone();
                    move |_| {
                        core.open.set(true);
                        #[cfg(not(target_arch = "wasm32"))]
                        let _ = &core;
                    }
                };
                let attrs = pt.preview_mask.attrs();
                let class = pt.preview_mask.merge_class("p-image-preview-mask").into_owned();
                view! {
                    <button type="button" class=class aria-label=label style=size node_ref=core.preview_button on:click={open} {..attrs}>
                        {icon}
                    </button>
                }
            })
        }
    };

    let preview_dialog = {
        let (core, pt, parts) = (core.clone(), pt.clone(), parts.clone());
        move || {
            let (core, pt, parts) = (core.clone(), pt.clone(), parts.clone());
            core.rendered.get().then(move || view! { <Portal>{build_preview(&core, &pt, &parts)}</Portal> })
        }
    };

    let root_attrs = pt.root.attrs();
    view! {
        <div class={root_class} style={root_style} {..root_attrs}>
            {image_view}
            {preview_button}
            {preview_dialog}
        </div>
    }
}

/// Перевод для подписи: в SSR берётся из конфигурации так же, как в браузере.
fn use_prime_config_text(key: &str) -> String {
    prime_core::use_prime_config().locale.text(key).to_owned()
}

/// Свойства, нужные только при выводе просмотра.
struct Parts {
    src: OptProp<String>,
    srcset: OptProp<String>,
    sizes: OptProp<String>,
    preview_image_src: OptProp<String>,
    preview_image_srcset: OptProp<String>,
    preview_image_sizes: OptProp<String>,
    preview_content: Option<Callback<ImagePreviewContext, AnyView>>,
    rotate_right_icon: Option<ViewFn>,
    rotate_left_icon: Option<ViewFn>,
    zoom_out_icon: Option<ViewFn>,
    zoom_in_icon: Option<ViewFn>,
    close_icon: Option<ViewFn>,
}

/// Маска просмотра с панелью действий и увеличенной картинкой.
fn build_preview(core: &Arc<Core>, pt: &Arc<ImagePt>, parts: &Arc<Parts>) -> impl IntoView + use<> {
    let (leaving, z_index, rotate, scale) = (core.leaving, core.z_index, core.rotate, core.scale);
    let mask_class = {
        let pt = pt.clone();
        move || {
            let state = if leaving.get() { "p-overlay-mask-leave-active" } else { "p-overlay-mask-enter-active" };
            pt.mask.merge_class(&format!("p-image-mask p-overlay-mask {state}")).into_owned()
        }
    };
    let mask_style = {
        let pt = pt.clone();
        move || {
            let z = z_index.get().map(|z| format!("z-index: {z}")).unwrap_or_default();
            let merged = pt.mask.merge_style(&z);
            (!merged.is_empty()).then_some(merged)
        }
    };
    let on_mask_click = {
        let core = core.clone();
        move |_| {
            if mask_should_close(core.preview_click.get_value()) {
                core.close();
            }
            core.preview_click.set_value(false);
        }
    };
    let on_animation_end = {
        let core = core.clone();
        move |e: web_sys::AnimationEvent| animation_end(&core, &e)
    };

    let zoom_out_off = Signal::derive(move || zoom_out_disabled(scale.get()));
    let zoom_in_off = Signal::derive(move || zoom_in_disabled(scale.get()));
    let never = Signal::stored(false);
    let tool =
        |kind, off, key: &'static str, icon: ViewFn, pt: &Pt, act: Callback<()>, node| action_button(kind, off, key, icon, pt, act, node);
    let buttons = view! {
        {tool(
            ActionKind::RotateRight,
            never,
            "aria.rotateRight",
            parts.rotate_right_icon.clone().unwrap_or_else(|| ViewFn::from(|| view! { <RefreshIcon /> })),
            &pt.rotate_right_button,
            Callback::new(move |()| rotate.update(|r| *r += 90)),
            None,
        )}
        {tool(
            ActionKind::RotateLeft,
            never,
            "aria.rotateLeft",
            parts.rotate_left_icon.clone().unwrap_or_else(|| ViewFn::from(|| view! { <UndoIcon /> })),
            &pt.rotate_left_button,
            Callback::new(move |()| rotate.update(|r| *r -= 90)),
            None,
        )}
        {tool(
            ActionKind::ZoomOut,
            zoom_out_off,
            "aria.zoomOut",
            parts.zoom_out_icon.clone().unwrap_or_else(|| ViewFn::from(|| view! { <SearchMinusIcon /> })),
            &pt.zoom_out_button,
            Callback::new(move |()| scale.update(|s| *s = zoom_out(*s))),
            None,
        )}
        {tool(
            ActionKind::ZoomIn,
            zoom_in_off,
            "aria.zoomIn",
            parts.zoom_in_icon.clone().unwrap_or_else(|| ViewFn::from(|| view! { <SearchPlusIcon /> })),
            &pt.zoom_in_button,
            Callback::new(move |()| scale.update(|s| *s = zoom_in(*s))),
            None,
        )}
        {tool(
            ActionKind::Close,
            never,
            "aria.close",
            parts.close_icon.clone().unwrap_or_else(|| ViewFn::from(|| view! { <TimesIcon /> })),
            &pt.close_button,
            {
                let core = core.clone();
                Callback::new(move |()| core.close())
            },
            Some(core.close_button),
        )}
    };

    let toolbar_attrs = pt.toolbar.attrs();
    let toolbar_class = pt.toolbar.merge_class("p-image-toolbar").into_owned();
    let toolbar_style = pt.toolbar.has_style().then(|| pt.toolbar.merge_style(""));

    let original = build_original(core, pt, parts);
    let mask_attrs = pt.mask.attrs();
    view! {
        <div
            node_ref=core.mask
            class=mask_class
            style=mask_style
            role="dialog"
            aria-modal="true"
            on:click=on_mask_click
            on:animationend={on_animation_end}
            {..mask_attrs}
        >
            <div class=toolbar_class style=toolbar_style on:click=|e: web_sys::MouseEvent| e.stop_propagation() {..toolbar_attrs}>
                {buttons}
            </div>
            {original}
        </div>
    }
}

/// Увеличенная картинка (или представление `preview_content`).
fn build_original(core: &Arc<Core>, pt: &Arc<ImagePt>, parts: &Arc<Parts>) -> impl IntoView + use<> {
    let (leaving, rotate, scale) = (core.leaving, core.rotate, core.scale);
    let state = move || if leaving.get() { "p-image-original-leave-active" } else { "p-image-original-enter-active" };
    let style = Signal::derive(move || preview_style(rotate.get(), scale.get()));
    let mark = {
        let core = core.clone();
        Callback::new(move |()| core.preview_click.set_value(true))
    };
    if let Some(custom) = parts.preview_content {
        let class = pt.original.merge_class(&format!("p-image-original {}", state())).into_owned();
        return custom.run(ImagePreviewContext { class, style, preview_callback: mark }).into_any();
    }
    let class = {
        let pt = pt.clone();
        move || pt.original.merge_class(&format!("p-image-original {}", state())).into_owned()
    };
    let p = parts.clone();
    let (src, srcset, sizes) = (
        move || p.preview_image_src.get().or_else(|| p.src.get()),
        {
            let p = parts.clone();
            move || p.preview_image_srcset.get().or_else(|| p.srcset.get())
        },
        {
            let p = parts.clone();
            move || p.preview_image_sizes.get().or_else(|| p.sizes.get())
        },
    );
    let attrs = pt.original.attrs();
    let style_attr = {
        let pt = pt.clone();
        move || pt.original.merge_style(&style.get())
    };
    view! { <img src=src srcset=srcset sizes=sizes class=class style=style_attr on:click=move |_| mark.run(()) {..attrs} /> }.into_any()
}

/// Кнопка панели действий.
fn action_button(
    kind: ActionKind,
    disabled: Signal<bool>,
    label_key: &'static str,
    icon: ViewFn,
    pt: &Pt,
    act: Callback<()>,
    node: Option<NodeRef<leptos::html::Button>>,
) -> impl IntoView + use<> {
    let label = use_prime_config_text(label_key);
    let base = pt.clone();
    let class = {
        let base = base.clone();
        move || base.merge_class(action_classes(kind, disabled.get())).into_owned()
    };
    let attrs = pt.attrs();
    let node = node.unwrap_or_default();
    view! {
        <button type="button" node_ref=node class=class aria-label=label disabled=move || disabled.get() on:click=move |_| act.run(()) {..attrs}>
            {icon.run()}
        </button>
    }
}

/// `animationend` самой маски (не вложенных элементов) завершает выход.
fn animation_end(core: &Arc<Core>, e: &web_sys::AnimationEvent) {
    #[cfg(target_arch = "wasm32")]
    {
        let own = e.target().zip(e.current_target()).is_some_and(|(t, c)| t == c);
        if own && core.leaving.get_untracked() {
            core.finish_leave();
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (core, e);
}
