//! Описание подтверждения (`Confirmation` PrimeNG).

use leptos::prelude::*;
use prime_button::ButtonSeverity;
use prime_core::props::Size;
use wasm_bindgen::JsCast;

/// Как закрыто окно подтверждения (`ConfirmEventType` PrimeNG).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfirmEventType {
    /// Согласие кнопкой.
    Accept,
    /// Отказ кнопкой.
    Reject,
    /// Окно закрыто без выбора: крестик, `Escape`, клик по маске.
    Cancel,
}

impl ConfirmEventType {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::Reject => "reject",
            Self::Cancel => "cancel",
        }
    }
}

/// Какой элемент получает фокус при показе.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ConfirmFocus {
    /// Кнопка согласия (по умолчанию).
    #[default]
    Accept,
    /// Кнопка отказа.
    Reject,
    /// Кнопка закрытия (ConfirmDialog).
    Close,
    /// Фокус не переносится.
    None,
}

impl ConfirmFocus {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::Reject => "reject",
            Self::Close => "close",
            Self::None => "none",
        }
    }
}

/// Свойства кнопки подтверждения (`acceptButtonProps` и `rejectButtonProps` PrimeNG, нужная часть).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConfirmButtonProps {
    /// Подпись.
    pub label: Option<String>,
    /// Цвет.
    pub severity: Option<ButtonSeverity>,
    /// Кнопка с рамкой.
    pub outlined: Option<bool>,
    /// Текстовая кнопка.
    pub text: Option<bool>,
    /// Размер.
    pub size: Option<Size>,
}

impl ConfirmButtonProps {
    /// Пустые свойства.
    pub fn new() -> Self {
        Self::default()
    }

    /// Подпись.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Цвет.
    pub fn severity(mut self, severity: ButtonSeverity) -> Self {
        self.severity = Some(severity);
        self
    }

    /// Кнопка с рамкой.
    pub fn outlined(mut self, outlined: bool) -> Self {
        self.outlined = Some(outlined);
        self
    }

    /// Текстовая кнопка.
    pub fn text(mut self, text: bool) -> Self {
        self.text = Some(text);
        self
    }

    /// Размер.
    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }
}

/// Описание подтверждения. Не заданные поля — `None`.
#[derive(Clone, Default)]
pub struct Confirmation {
    /// Ключ: показывает подтверждение только компонент с таким же `key` (оба не заданы — тоже совпадение).
    pub key: Option<String>,
    /// Текст вопроса.
    pub message: Option<String>,
    /// Заголовок (ConfirmDialog).
    pub header: Option<String>,
    /// Значок рядом с текстом.
    pub icon: Option<ViewFn>,
    /// Элемент, у которого показывается панель (ConfirmPopup) и на который возвращается фокус.
    pub target: Option<web_sys::HtmlElement>,
    /// Вызывается при согласии.
    pub accept: Option<Callback<()>>,
    /// Вызывается при отказе.
    pub reject: Option<Callback<()>>,
    /// Подпись кнопки согласия.
    pub accept_label: Option<String>,
    /// Подпись кнопки отказа.
    pub reject_label: Option<String>,
    /// Значок кнопки согласия.
    pub accept_icon: Option<ViewFn>,
    /// Значок кнопки отказа.
    pub reject_icon: Option<ViewFn>,
    /// Показывать ли кнопку согласия (по умолчанию да).
    pub accept_visible: Option<bool>,
    /// Показывать ли кнопку отказа (по умолчанию да).
    pub reject_visible: Option<bool>,
    /// Дополнительный класс кнопки согласия.
    pub accept_button_style_class: Option<String>,
    /// Дополнительный класс кнопки отказа.
    pub reject_button_style_class: Option<String>,
    /// Свойства кнопки согласия.
    pub accept_button_props: Option<ConfirmButtonProps>,
    /// Свойства кнопки отказа.
    pub reject_button_props: Option<ConfirmButtonProps>,
    /// Фокус при показе.
    pub default_focus: Option<ConfirmFocus>,
    /// Закрывать ли клавишей `Escape` (по умолчанию да).
    pub close_on_escape: Option<bool>,
    /// Закрывать ли кликом вне окна (по умолчанию для ConfirmPopup да).
    pub dismissable_mask: Option<bool>,
    /// Блокировать ли прокрутку страницы (ConfirmDialog).
    pub block_scroll: Option<bool>,
    /// Есть ли кнопка закрытия (ConfirmDialog).
    pub closable: Option<bool>,
    /// Положение окна (ConfirmDialog).
    pub position: Option<String>,
    /// Модальное ли окно (ConfirmDialog).
    pub modal: Option<bool>,
}

impl Confirmation {
    /// Пустое подтверждение.
    pub fn new() -> Self {
        Self::default()
    }

    /// Ключ.
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// Текст вопроса.
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Заголовок.
    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Значок рядом с текстом.
    pub fn icon(mut self, icon: impl Into<ViewFn>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Цель — элемент.
    pub fn target(mut self, target: web_sys::HtmlElement) -> Self {
        self.target = Some(target);
        self
    }

    /// Цель — `currentTarget` события (в его отсутствие `target`).
    pub fn target_event(mut self, event: &web_sys::Event) -> Self {
        self.target = event
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
            .or_else(|| event.target().and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()));
        self
    }

    /// Обработчик согласия.
    pub fn accept(mut self, accept: impl Into<Callback<()>>) -> Self {
        self.accept = Some(accept.into());
        self
    }

    /// Обработчик отказа.
    pub fn reject(mut self, reject: impl Into<Callback<()>>) -> Self {
        self.reject = Some(reject.into());
        self
    }

    /// Подпись кнопки согласия.
    pub fn accept_label(mut self, label: impl Into<String>) -> Self {
        self.accept_label = Some(label.into());
        self
    }

    /// Подпись кнопки отказа.
    pub fn reject_label(mut self, label: impl Into<String>) -> Self {
        self.reject_label = Some(label.into());
        self
    }

    /// Значок кнопки согласия.
    pub fn accept_icon(mut self, icon: impl Into<ViewFn>) -> Self {
        self.accept_icon = Some(icon.into());
        self
    }

    /// Значок кнопки отказа.
    pub fn reject_icon(mut self, icon: impl Into<ViewFn>) -> Self {
        self.reject_icon = Some(icon.into());
        self
    }

    /// Показывать ли кнопку согласия.
    pub fn accept_visible_flag(mut self, visible: bool) -> Self {
        self.accept_visible = Some(visible);
        self
    }

    /// Показывать ли кнопку отказа.
    pub fn reject_visible_flag(mut self, visible: bool) -> Self {
        self.reject_visible = Some(visible);
        self
    }

    /// Дополнительный класс кнопки согласия.
    pub fn accept_button_style_class(mut self, class: impl Into<String>) -> Self {
        self.accept_button_style_class = Some(class.into());
        self
    }

    /// Дополнительный класс кнопки отказа.
    pub fn reject_button_style_class(mut self, class: impl Into<String>) -> Self {
        self.reject_button_style_class = Some(class.into());
        self
    }

    /// Свойства кнопки согласия.
    pub fn accept_button_props(mut self, props: ConfirmButtonProps) -> Self {
        self.accept_button_props = Some(props);
        self
    }

    /// Свойства кнопки отказа.
    pub fn reject_button_props(mut self, props: ConfirmButtonProps) -> Self {
        self.reject_button_props = Some(props);
        self
    }

    /// Фокус при показе.
    pub fn default_focus(mut self, focus: ConfirmFocus) -> Self {
        self.default_focus = Some(focus);
        self
    }

    /// Закрывать ли клавишей `Escape`.
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = Some(close);
        self
    }

    /// Закрывать ли кликом вне окна.
    pub fn dismissable_mask(mut self, dismissable: bool) -> Self {
        self.dismissable_mask = Some(dismissable);
        self
    }

    /// Блокировать ли прокрутку страницы.
    pub fn block_scroll(mut self, block: bool) -> Self {
        self.block_scroll = Some(block);
        self
    }

    /// Есть ли кнопка закрытия.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = Some(closable);
        self
    }

    /// Положение окна.
    pub fn position(mut self, position: impl Into<String>) -> Self {
        self.position = Some(position.into());
        self
    }

    /// Модальное ли окно.
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    /// Показана ли кнопка согласия (`acceptVisible !== false`).
    pub fn accept_visible(&self) -> bool {
        self.accept_visible != Some(false)
    }

    /// Показана ли кнопка отказа (`rejectVisible !== false`).
    pub fn reject_visible(&self) -> bool {
        self.reject_visible != Some(false)
    }
}

impl std::fmt::Debug for Confirmation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Confirmation")
            .field("key", &self.key)
            .field("message", &self.message)
            .field("header", &self.header)
            .field("accept_label", &self.accept_label)
            .field("reject_label", &self.reject_label)
            .field("default_focus", &self.default_focus)
            .finish_non_exhaustive()
    }
}
