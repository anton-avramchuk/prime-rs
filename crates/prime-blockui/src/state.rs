//! Чистая логика BlockUI: классы маски, `aria-busy`, инлайн-стиль (по `blockuistyle.ts` и `block`/`unblock` PrimeNG).

use prime_core::ClassNames;
use std::borrow::Cow;

/// Классы маски: `p-blockui p-blockui-mask`, у документа `p-blockui-mask-document`, затем `p-overlay-mask` и класс анимации
/// входа или выхода.
pub fn mask_classes(document: bool, leaving: bool, extra: Option<&str>) -> Cow<'static, str> {
    let mut cn = ClassNames::new();
    cn.add("p-blockui p-blockui-mask")
        .add_if(document, "p-blockui-mask-document")
        .add("p-overlay-mask")
        .add(if leaving { "p-overlay-mask-leave-active" } else { "p-overlay-mask-enter-active" })
        .add_user(extra);
    cn.finish_cow()
}

/// Значение `aria-busy`: пока маска не уходит — `true`.
pub fn aria_busy(leaving: bool) -> &'static str {
    if leaving { "false" } else { "true" }
}

/// Инлайн-стиль маски: `display: flex`, слой (если выдан) и пользовательские части.
pub fn mask_style(z_index: Option<i32>, extra: &[&str]) -> String {
    let mut out = String::from("display: flex");
    if let Some(z) = z_index {
        out.push_str(&format!("; z-index: {z}"));
    }
    for part in extra {
        let part = part.trim().trim_end_matches(';').trim();
        if !part.is_empty() {
            out.push_str("; ");
            out.push_str(part);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_table() {
        assert_eq!(mask_classes(false, false, None), "p-blockui p-blockui-mask p-overlay-mask p-overlay-mask-enter-active");
        assert_eq!(
            mask_classes(true, false, None),
            "p-blockui p-blockui-mask p-blockui-mask-document p-overlay-mask p-overlay-mask-enter-active"
        );
        assert_eq!(mask_classes(false, true, None), "p-blockui p-blockui-mask p-overlay-mask p-overlay-mask-leave-active");
        assert_eq!(
            mask_classes(true, true, Some("mine")),
            "p-blockui p-blockui-mask p-blockui-mask-document p-overlay-mask p-overlay-mask-leave-active mine"
        );
    }

    #[test]
    fn aria_busy_follows_leaving() {
        assert_eq!(aria_busy(false), "true");
        assert_eq!(aria_busy(true), "false");
    }

    #[test]
    fn style_table() {
        assert_eq!(mask_style(None, &[]), "display: flex");
        assert_eq!(mask_style(Some(1101), &[]), "display: flex; z-index: 1101");
        assert_eq!(mask_style(Some(5), &["color: red;", "  ", "opacity: .5"]), "display: flex; z-index: 5; color: red; opacity: .5");
    }
}
