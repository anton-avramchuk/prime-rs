//! Confirmation description (PrimeNG `Confirmation`).

use leptos::prelude::*;
use prime_button::ButtonSeverity;
use prime_rs_core::props::Size;
use wasm_bindgen::JsCast;

/// How the confirmation window was closed (PrimeNG `ConfirmEventType`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfirmEventType {
    /// Accepted with the button.
    Accept,
    /// Rejected with the button.
    Reject,
    /// Window closed without a choice: close icon, `Escape`, click on the mask.
    Cancel,
}

impl ConfirmEventType {
    /// Value as in PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::Reject => "reject",
            Self::Cancel => "cancel",
        }
    }
}

/// Which element receives focus when shown.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ConfirmFocus {
    /// Accept button (default).
    #[default]
    Accept,
    /// Reject button.
    Reject,
    /// Close button (ConfirmDialog).
    Close,
    /// Focus is not moved.
    None,
}

impl ConfirmFocus {
    /// Value as in PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::Reject => "reject",
            Self::Close => "close",
            Self::None => "none",
        }
    }
}

/// Properties of a confirmation button (`acceptButtonProps` and `rejectButtonProps` in PrimeNG, relevant part).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConfirmButtonProps {
    /// Label.
    pub label: Option<String>,
    /// Color.
    pub severity: Option<ButtonSeverity>,
    /// Button with an outline.
    pub outlined: Option<bool>,
    /// Text button.
    pub text: Option<bool>,
    /// Size.
    pub size: Option<Size>,
}

impl ConfirmButtonProps {
    /// Empty properties.
    pub fn new() -> Self {
        Self::default()
    }

    /// Label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Color.
    pub fn severity(mut self, severity: ButtonSeverity) -> Self {
        self.severity = Some(severity);
        self
    }

    /// Button with an outline.
    pub fn outlined(mut self, outlined: bool) -> Self {
        self.outlined = Some(outlined);
        self
    }

    /// Text button.
    pub fn text(mut self, text: bool) -> Self {
        self.text = Some(text);
        self
    }

    /// Size.
    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }
}

/// Confirmation description. Unset fields are `None`.
#[derive(Clone, Default)]
pub struct Confirmation {
    /// Key: only a component with the same `key` shows this confirmation (if neither is set, that is also a match).
    pub key: Option<String>,
    /// Question text.
    pub message: Option<String>,
    /// Header (ConfirmDialog).
    pub header: Option<String>,
    /// Icon next to the text.
    pub icon: Option<ViewFn>,
    /// Element the panel is shown for (ConfirmPopup) and to which focus returns.
    pub target: Option<web_sys::HtmlElement>,
    /// Called on accept.
    pub accept: Option<Callback<()>>,
    /// Called on reject.
    pub reject: Option<Callback<()>>,
    /// Accept button label.
    pub accept_label: Option<String>,
    /// Reject button label.
    pub reject_label: Option<String>,
    /// Accept button icon.
    pub accept_icon: Option<ViewFn>,
    /// Reject button icon.
    pub reject_icon: Option<ViewFn>,
    /// Whether to show the accept button (default: yes).
    pub accept_visible: Option<bool>,
    /// Whether to show the reject button (default: yes).
    pub reject_visible: Option<bool>,
    /// Extra CSS class of the accept button.
    pub accept_button_style_class: Option<String>,
    /// Extra CSS class of the reject button.
    pub reject_button_style_class: Option<String>,
    /// Accept button properties.
    pub accept_button_props: Option<ConfirmButtonProps>,
    /// Reject button properties.
    pub reject_button_props: Option<ConfirmButtonProps>,
    /// Focus when shown.
    pub default_focus: Option<ConfirmFocus>,
    /// Whether to close with the `Escape` key (default: yes).
    pub close_on_escape: Option<bool>,
    /// Whether to close on click outside the window (default: yes for ConfirmPopup).
    pub dismissable_mask: Option<bool>,
    /// Whether to block page scrolling (ConfirmDialog).
    pub block_scroll: Option<bool>,
    /// Whether there is a close button (ConfirmDialog).
    pub closable: Option<bool>,
    /// Window position (ConfirmDialog).
    pub position: Option<String>,
    /// Whether the window is modal (ConfirmDialog).
    pub modal: Option<bool>,
}

impl Confirmation {
    /// Empty confirmation.
    pub fn new() -> Self {
        Self::default()
    }

    /// Key.
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// Question text.
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Header.
    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Icon next to the text.
    pub fn icon(mut self, icon: impl Into<ViewFn>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Target — an element.
    pub fn target(mut self, target: web_sys::HtmlElement) -> Self {
        self.target = Some(target);
        self
    }

    /// Target — the event's `currentTarget` (or `target` if absent).
    pub fn target_event(mut self, event: &web_sys::Event) -> Self {
        self.target = event
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
            .or_else(|| event.target().and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()));
        self
    }

    /// Accept handler.
    pub fn accept(mut self, accept: impl Into<Callback<()>>) -> Self {
        self.accept = Some(accept.into());
        self
    }

    /// Reject handler.
    pub fn reject(mut self, reject: impl Into<Callback<()>>) -> Self {
        self.reject = Some(reject.into());
        self
    }

    /// Accept button label.
    pub fn accept_label(mut self, label: impl Into<String>) -> Self {
        self.accept_label = Some(label.into());
        self
    }

    /// Reject button label.
    pub fn reject_label(mut self, label: impl Into<String>) -> Self {
        self.reject_label = Some(label.into());
        self
    }

    /// Accept button icon.
    pub fn accept_icon(mut self, icon: impl Into<ViewFn>) -> Self {
        self.accept_icon = Some(icon.into());
        self
    }

    /// Reject button icon.
    pub fn reject_icon(mut self, icon: impl Into<ViewFn>) -> Self {
        self.reject_icon = Some(icon.into());
        self
    }

    /// Whether to show the accept button.
    pub fn accept_visible_flag(mut self, visible: bool) -> Self {
        self.accept_visible = Some(visible);
        self
    }

    /// Whether to show the reject button.
    pub fn reject_visible_flag(mut self, visible: bool) -> Self {
        self.reject_visible = Some(visible);
        self
    }

    /// Extra CSS class of the accept button.
    pub fn accept_button_style_class(mut self, class: impl Into<String>) -> Self {
        self.accept_button_style_class = Some(class.into());
        self
    }

    /// Extra CSS class of the reject button.
    pub fn reject_button_style_class(mut self, class: impl Into<String>) -> Self {
        self.reject_button_style_class = Some(class.into());
        self
    }

    /// Accept button properties.
    pub fn accept_button_props(mut self, props: ConfirmButtonProps) -> Self {
        self.accept_button_props = Some(props);
        self
    }

    /// Reject button properties.
    pub fn reject_button_props(mut self, props: ConfirmButtonProps) -> Self {
        self.reject_button_props = Some(props);
        self
    }

    /// Focus when shown.
    pub fn default_focus(mut self, focus: ConfirmFocus) -> Self {
        self.default_focus = Some(focus);
        self
    }

    /// Whether to close with the `Escape` key.
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = Some(close);
        self
    }

    /// Whether to close on click outside the window.
    pub fn dismissable_mask(mut self, dismissable: bool) -> Self {
        self.dismissable_mask = Some(dismissable);
        self
    }

    /// Whether to block page scrolling.
    pub fn block_scroll(mut self, block: bool) -> Self {
        self.block_scroll = Some(block);
        self
    }

    /// Whether there is a close button.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = Some(closable);
        self
    }

    /// Window position.
    pub fn position(mut self, position: impl Into<String>) -> Self {
        self.position = Some(position.into());
        self
    }

    /// Whether the window is modal.
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    /// Whether the accept button is shown (`acceptVisible !== false`).
    pub fn accept_visible(&self) -> bool {
        self.accept_visible != Some(false)
    }

    /// Whether the reject button is shown (`rejectVisible !== false`).
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
