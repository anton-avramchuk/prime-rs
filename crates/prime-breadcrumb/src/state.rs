//! Breadcrumb item model and the click decision (after PrimeNG `Breadcrumb.onClick`).

use leptos::prelude::*;

/// Press on an item: the event and the item.
#[derive(Clone)]
pub struct BreadcrumbClick {
    /// Original event.
    pub event: web_sys::MouseEvent,
    /// Pressed item.
    pub item: BreadcrumbItem,
}

/// Breadcrumb item (instead of PrimeNG `MenuItem`, without Angular router fields).
#[derive(Clone)]
pub struct BreadcrumbItem {
    /// Identifier of the `li` element.
    pub id: Option<String>,
    /// Label.
    pub label: Option<String>,
    /// Icon (for example, an icon from `prime-icons`).
    pub icon: Option<ViewFn>,
    /// Link address; without it following the link is cancelled.
    pub url: Option<String>,
    /// Link `target`.
    pub target: Option<String>,
    /// Link `title`.
    pub title: Option<String>,
    /// Disabled: pressing triggers nothing.
    pub disabled: bool,
    /// Visible (yes by default).
    pub visible: bool,
    /// Link `tabindex` (`0` by default).
    pub tabindex: Option<String>,
    /// Extra class of the `li`.
    pub style_class: Option<String>,
    /// Inline style of the `li`.
    pub style: Option<String>,
    /// Extra class of the link.
    pub link_class: Option<String>,
    /// Badge value.
    pub badge: Option<String>,
    /// Command called on press.
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
    /// Item without properties.
    pub fn new() -> Self {
        Self::default()
    }

    /// Label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Icon.
    pub fn icon(mut self, icon: impl Into<ViewFn>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Link address.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Link `target`.
    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Link `title`.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Disables the item.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Shows or hides the item.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Badge value.
    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    /// Command on press.
    pub fn command(mut self, command: impl Into<Callback<BreadcrumbClick>>) -> Self {
        self.command = Some(command.into());
        self
    }

    /// Extra class of the `li`.
    pub fn style_class(mut self, class: impl Into<String>) -> Self {
        self.style_class = Some(class.into());
        self
    }

    /// Extra class of the link.
    pub fn link_class(mut self, class: impl Into<String>) -> Self {
        self.link_class = Some(class.into());
        self
    }
}

/// What to do when an item is pressed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClickPlan {
    /// Cancel the default action (following the link).
    pub prevent_default: bool,
    /// Call the item's `command` and `on_item_click`.
    pub run_commands: bool,
}

/// Click decision: disabled - cancel without calls, without `url` - cancel with calls.
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
