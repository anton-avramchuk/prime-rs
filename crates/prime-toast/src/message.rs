//! Toast message (analogue of PrimeNG `ToastMessageOptions`) and severity.

/// Message severity: determines the `p-toast-message-<severity>` class and the default icon.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ToastSeverity {
    /// `success`.
    Success,
    /// `info` (default).
    #[default]
    Info,
    /// `warn`.
    Warn,
    /// `error`.
    Error,
    /// `secondary`.
    Secondary,
    /// `contrast`.
    Contrast,
    /// Custom look (like `severity: 'custom'` in PrimeNG): no severity class, info icon.
    Custom(String),
}

impl ToastSeverity {
    /// Value, as in PrimeNG.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Success => "success",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
            Self::Secondary => "secondary",
            Self::Contrast => "contrast",
            Self::Custom(name) => name,
        }
    }
}

/// Message: what to show, to whom (by key) and for how long.
#[derive(Clone, Debug, PartialEq)]
pub struct ToastMessage {
    /// Severity.
    pub severity: ToastSeverity,
    /// Summary.
    pub summary: Option<String>,
    /// Detail.
    pub detail: Option<String>,
    /// `Toast` key the message is addressed to; `None` — the key-less `Toast`.
    pub key: Option<String>,
    /// Message `id` attribute.
    pub id: Option<String>,
    /// Display time, ms; `None` — the `Toast` value.
    pub life: Option<u32>,
    /// Do not close on a timer.
    pub sticky: bool,
    /// Show the close button.
    pub closable: bool,
    /// Icon CSS class (instead of the severity icon).
    pub icon: Option<String>,
    /// Close button icon CSS class.
    pub close_icon: Option<String>,
    /// Extra message class.
    pub style_class: Option<String>,
    /// Extra message content class.
    pub content_style_class: Option<String>,
}

impl Default for ToastMessage {
    fn default() -> Self {
        Self {
            severity: ToastSeverity::default(),
            summary: None,
            detail: None,
            key: None,
            id: None,
            life: None,
            sticky: false,
            closable: true,
            icon: None,
            close_icon: None,
            style_class: None,
            content_style_class: None,
        }
    }
}

impl ToastMessage {
    /// Message with severity, summary and detail.
    pub fn new(severity: ToastSeverity, summary: impl Into<String>, detail: impl Into<String>) -> Self {
        Self { severity, summary: Some(summary.into()), detail: Some(detail.into()), ..Self::default() }
    }

    /// Addressee key.
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// `id` attribute.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Display time, ms.
    pub fn life(mut self, ms: u32) -> Self {
        self.life = Some(ms);
        self
    }

    /// Do not close on a timer.
    pub fn sticky(mut self, sticky: bool) -> Self {
        self.sticky = sticky;
        self
    }

    /// Show the close button.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Icon CSS class.
    pub fn icon(mut self, class: impl Into<String>) -> Self {
        self.icon = Some(class.into());
        self
    }

    /// Close button icon CSS class.
    pub fn close_icon(mut self, class: impl Into<String>) -> Self {
        self.close_icon = Some(class.into());
        self
    }

    /// Extra message class.
    pub fn style_class(mut self, class: impl Into<String>) -> Self {
        self.style_class = Some(class.into());
        self
    }

    /// Extra message content class.
    pub fn content_style_class(mut self, class: impl Into<String>) -> Self {
        self.content_style_class = Some(class.into());
        self
    }
}
