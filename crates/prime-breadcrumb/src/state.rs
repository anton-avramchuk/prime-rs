//! Модель пункта Breadcrumb и решение о клике (по `Breadcrumb.onClick` PrimeNG).

use leptos::prelude::*;

/// Нажатие на пункт: событие и пункт.
#[derive(Clone)]
pub struct BreadcrumbClick {
    /// Исходное событие.
    pub event: web_sys::MouseEvent,
    /// Нажатый пункт.
    pub item: BreadcrumbItem,
}

/// Пункт Breadcrumb (вместо `MenuItem` PrimeNG, без полей маршрутизатора Angular).
#[derive(Clone)]
pub struct BreadcrumbItem {
    /// Идентификатор элемента `li`.
    pub id: Option<String>,
    /// Подпись.
    pub label: Option<String>,
    /// Значок (например, иконка из `prime-icons`).
    pub icon: Option<ViewFn>,
    /// Адрес ссылки; без него переход по ссылке отменяется.
    pub url: Option<String>,
    /// `target` ссылки.
    pub target: Option<String>,
    /// `title` ссылки.
    pub title: Option<String>,
    /// Отключён: нажатие ничего не вызывает.
    pub disabled: bool,
    /// Виден (по умолчанию да).
    pub visible: bool,
    /// `tabindex` ссылки (по умолчанию `0`).
    pub tabindex: Option<String>,
    /// Дополнительный класс `li`.
    pub style_class: Option<String>,
    /// Встроенный стиль `li`.
    pub style: Option<String>,
    /// Дополнительный класс ссылки.
    pub link_class: Option<String>,
    /// Значение бейджа.
    pub badge: Option<String>,
    /// Команда, вызываемая при нажатии.
    pub command: Option<Callback<BreadcrumbClick>>,
}

impl Default for BreadcrumbItem {
    fn default() -> Self {
        Self {
            id: None,
            label: None,
            icon: None,
            url: None,
            target: None,
            title: None,
            disabled: false,
            visible: true,
            tabindex: None,
            style_class: None,
            style: None,
            link_class: None,
            badge: None,
            command: None,
        }
    }
}

impl BreadcrumbItem {
    /// Пункт без свойств.
    pub fn new() -> Self {
        Self::default()
    }

    /// Подпись.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Значок.
    pub fn icon(mut self, icon: impl Into<ViewFn>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Адрес ссылки.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// `target` ссылки.
    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// `title` ссылки.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Отключает пункт.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Показывает или скрывает пункт.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Значение бейджа.
    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    /// Команда при нажатии.
    pub fn command(mut self, command: impl Into<Callback<BreadcrumbClick>>) -> Self {
        self.command = Some(command.into());
        self
    }

    /// Дополнительный класс `li`.
    pub fn style_class(mut self, class: impl Into<String>) -> Self {
        self.style_class = Some(class.into());
        self
    }

    /// Дополнительный класс ссылки.
    pub fn link_class(mut self, class: impl Into<String>) -> Self {
        self.link_class = Some(class.into());
        self
    }
}

/// Что делать при нажатии на пункт.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClickPlan {
    /// Отменить действие по умолчанию (переход по ссылке).
    pub prevent_default: bool,
    /// Вызвать `command` пункта и `on_item_click`.
    pub run_commands: bool,
}

/// Решение о клике: отключённый — отмена без вызовов, без `url` — отмена с вызовами.
pub fn click_plan(disabled: bool, has_url: bool) -> ClickPlan {
    if disabled {
        return ClickPlan { prevent_default: true, run_commands: false };
    }
    ClickPlan { prevent_default: !has_url, run_commands: true }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn click_plan_table() {
        let table = [(true, true, true, false), (true, false, true, false), (false, true, false, true), (false, false, true, true)];
        for (disabled, has_url, prevent, run) in table {
            assert_eq!(click_plan(disabled, has_url), ClickPlan { prevent_default: prevent, run_commands: run }, "{disabled} {has_url}");
        }
    }

    #[test]
    fn defaults_are_visible_and_enabled() {
        let item = BreadcrumbItem::new();
        assert!(item.visible && !item.disabled);
    }
}
