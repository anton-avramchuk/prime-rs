//! Сверка сгенерированного CSS с токенами Aura и проверка выборочной загрузки стилей компонентов.

use prime_theme::{BASE, DARK_MODE_SELECTOR};

fn var<'a>(css: &'a str, name: &str) -> Option<&'a str> {
    let key = format!("{name}:");
    let from = css.find(&key)? + key.len();
    let len = css[from..].find([';', '}'])?;
    Some(&css[from..from + len])
}

#[test]
fn primary_and_surface_tokens_match_aura() {
    assert_eq!(var(BASE, "--p-primary-500"), Some("var(--p-emerald-500)"));
    assert_eq!(var(BASE, "--p-emerald-500"), Some("#10b981"));
    assert_eq!(var(BASE, "--p-border-radius-md"), Some("6px"));
    assert_eq!(var(BASE, "--p-surface-0"), Some("#ffffff"));
    assert!(var(BASE, "--p-primary-color").is_some());
}

#[test]
fn dark_mode_uses_the_configured_selector() {
    assert!(BASE.contains(&format!("{DARK_MODE_SELECTOR}{{")), "нет правил для {DARK_MODE_SELECTOR}");
}

#[test]
fn base_contains_primeng_helper_classes() {
    assert!(BASE.contains(".p-hidden-accessible"));
}

#[cfg(all(feature = "button", feature = "badge", not(feature = "card")))]
#[test]
fn only_requested_component_css_is_included() {
    use prime_theme::{component, stylesheet};
    let css = stylesheet(&["button"]);
    assert!(css.starts_with(BASE));
    assert!(css.contains(".p-button"));
    assert!(!css.contains(".p-badge-dot"), "CSS badge не запрашивался");
    assert!(component("card").is_none(), "feature card не включён");
    assert_eq!(stylesheet(&["nope"]), BASE);
}

#[cfg(feature = "badge")]
#[test]
fn badge_css_is_available_when_its_feature_is_on() {
    let css = prime_theme::stylesheet(&["badge"]);
    assert!(css.contains(".p-badge-dot"));
    assert!(css.starts_with(BASE));
}

#[test]
fn no_unresolved_dt_calls_in_generated_css() {
    assert!(!BASE.contains("dt("), "в base.css остался dt(");
    assert!(BASE.contains("--p-scrollbar-width") || !BASE.contains("scrollbar"), "base: токен scrollbar не подставлен");
}

#[cfg(feature = "button")]
#[test]
fn button_css_uses_resolved_token_variables() {
    let css = prime_theme::component("button").expect("feature button включён");
    assert!(!css.contains("dt("), "в CSS Button остался dt()");
    assert!(css.contains("var(--p-button-primary-background)"), "токены не подставлены");
    assert!(css.contains(".p-button-danger"), "нет стиля серьёзности");
}
