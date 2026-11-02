//! Сообщение Toast (аналог `ToastMessageOptions` PrimeNG) и серьёзность.

/// Серьёзность сообщения: определяет класс `p-toast-message-<severity>` и значок по умолчанию.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ToastSeverity {
    /// `success`.
    Success,
    /// `info` (по умолчанию).
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
    /// Свой вид (как `severity: 'custom'` в PrimeNG): без класса серьёзности, значок информации.
    Custom(String),
}

impl ToastSeverity {
    /// Значение, как в PrimeNG.
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

/// Сообщение: что показать, кому (по ключу) и как долго.
#[derive(Clone, Debug, PartialEq)]
pub struct ToastMessage {
    /// Серьёзность.
    pub severity: ToastSeverity,
    /// Заголовок.
    pub summary: Option<String>,
    /// Подробности.
    pub detail: Option<String>,
    /// Ключ `Toast`, которому адресовано сообщение; `None` — `Toast` без ключа.
    pub key: Option<String>,
    /// Атрибут `id` сообщения.
    pub id: Option<String>,
    /// Время показа, мс; `None` — значение `Toast`.
    pub life: Option<u32>,
    /// Не закрывать по времени.
    pub sticky: bool,
    /// Показывать кнопку закрытия.
    pub closable: bool,
    /// CSS-класс значка (вместо значка серьёзности).
    pub icon: Option<String>,
    /// CSS-класс значка кнопки закрытия.
    pub close_icon: Option<String>,
    /// Дополнительный класс сообщения.
    pub style_class: Option<String>,
    /// Дополнительный класс содержимого сообщения.
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
    /// Сообщение с серьёзностью, заголовком и подробностями.
    pub fn new(severity: ToastSeverity, summary: impl Into<String>, detail: impl Into<String>) -> Self {
        Self { severity, summary: Some(summary.into()), detail: Some(detail.into()), ..Self::default() }
    }

    /// Ключ адресата.
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// Атрибут `id`.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Время показа, мс.
    pub fn life(mut self, ms: u32) -> Self {
        self.life = Some(ms);
        self
    }

    /// Не закрывать по времени.
    pub fn sticky(mut self, sticky: bool) -> Self {
        self.sticky = sticky;
        self
    }

    /// Показывать кнопку закрытия.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// CSS-класс значка.
    pub fn icon(mut self, class: impl Into<String>) -> Self {
        self.icon = Some(class.into());
        self
    }

    /// CSS-класс значка кнопки закрытия.
    pub fn close_icon(mut self, class: impl Into<String>) -> Self {
        self.close_icon = Some(class.into());
        self
    }

    /// Дополнительный класс сообщения.
    pub fn style_class(mut self, class: impl Into<String>) -> Self {
        self.style_class = Some(class.into());
        self
    }

    /// Дополнительный класс содержимого.
    pub fn content_style_class(mut self, class: impl Into<String>) -> Self {
        self.content_style_class = Some(class.into());
        self
    }
}
