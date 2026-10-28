//! Чистая логика Message: классы и `data-p` (по `messagestyle.ts` и `Message.dataP` PrimeNG).

use prime_core::{ClassNames, DataP, Size};

/// Серьёзность сообщения (`p-message-<severity>`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum MessageSeverity {
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
}

impl MessageSeverity {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
            Self::Secondary => "secondary",
            Self::Contrast => "contrast",
        }
    }

    fn class(self) -> &'static str {
        match self {
            Self::Success => "p-message-success",
            Self::Info => "p-message-info",
            Self::Warn => "p-message-warn",
            Self::Error => "p-message-error",
            Self::Secondary => "p-message-secondary",
            Self::Contrast => "p-message-contrast",
        }
    }
}

/// Вариант оформления (`p-message-<variant>`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MessageVariant {
    /// `outlined`.
    Outlined,
    /// `text`.
    Text,
    /// `simple`.
    Simple,
}

impl MessageVariant {
    /// Значение, как в PrimeNG.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Outlined => "outlined",
            Self::Text => "text",
            Self::Simple => "simple",
        }
    }

    fn class(self) -> &'static str {
        match self {
            Self::Outlined => "p-message-outlined",
            Self::Text => "p-message-text",
            Self::Simple => "p-message-simple",
        }
    }
}

/// Снимок свойств, влияющих на классы и `data-p`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MessageState {
    /// Серьёзность.
    pub severity: MessageSeverity,
    /// Вариант.
    pub variant: Option<MessageVariant>,
    /// Размер.
    pub size: Option<Size>,
}

/// Добавляет классы корня в порядке `messagestyle.ts`.
pub fn write_classes(cn: &mut ClassNames, s: &MessageState) {
    cn.add("p-message p-component").add(s.severity.class()).add_opt(s.variant.map(MessageVariant::class));
    cn.add_if(s.size == Some(Size::Small), "p-message-sm").add_if(s.size == Some(Size::Large), "p-message-lg");
}

/// Значение `data-p`: `outlined`, `simple`, серьёзность, размер.
pub fn data_p(s: &MessageState) -> String {
    let mut d = DataP::new();
    d.add_if(s.variant == Some(MessageVariant::Outlined), "outlined")
        .add_if(s.variant == Some(MessageVariant::Simple), "simple")
        .add(s.severity.as_str())
        .add_opt(s.size.map(Size::as_str));
    d.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classes(s: &MessageState) -> String {
        let mut cn = ClassNames::new();
        write_classes(&mut cn, s);
        cn.finish()
    }

    #[test]
    fn classes_and_data_p_table() {
        let st = |severity, variant, size| MessageState { severity, variant, size };
        let cases = [
            (MessageState::default(), "p-message p-component p-message-info", "info"),
            (st(MessageSeverity::Success, None, None), "p-message p-component p-message-success", "success"),
            (
                st(MessageSeverity::Error, Some(MessageVariant::Outlined), Some(Size::Small)),
                "p-message p-component p-message-error p-message-outlined p-message-sm",
                "outlined error small",
            ),
            (
                st(MessageSeverity::Warn, Some(MessageVariant::Simple), Some(Size::Large)),
                "p-message p-component p-message-warn p-message-simple p-message-lg",
                "simple warn large",
            ),
            (
                st(MessageSeverity::Contrast, Some(MessageVariant::Text), None),
                "p-message p-component p-message-contrast p-message-text",
                "contrast",
            ),
        ];
        for (s, class, data) in cases {
            assert_eq!(classes(&s), class, "{s:?}");
            assert_eq!(data_p(&s), data, "{s:?}");
        }
    }
}
