//! Построитель списка CSS-классов (аналог `cn` из `@primeuix/utils`).

use leptos::prelude::*;

/// Собирает классы в одну строку: без пустых и повторяющихся классов, одной аллокацией при достаточной ёмкости.
///
/// ```
/// use prime_core::ClassNames;
///
/// let mut cn = ClassNames::new();
/// cn.add("p-button p-component").add_if(true, "p-button-sm").add_if(false, "p-disabled");
/// assert_eq!(cn.finish(), "p-button p-component p-button-sm");
/// ```
#[derive(Debug, Default, Clone)]
pub struct ClassNames {
    buf: String,
}

impl ClassNames {
    /// Пустой построитель с ёмкостью, достаточной для типичного компонента.
    pub fn new() -> Self {
        Self::with_capacity(96)
    }

    /// Пустой построитель с заданной ёмкостью.
    pub fn with_capacity(capacity: usize) -> Self {
        Self { buf: String::with_capacity(capacity) }
    }

    /// Добавляет один или несколько классов через пробел.
    pub fn add(&mut self, classes: &str) -> &mut Self {
        for class in classes.split_whitespace() {
            if !self.contains(class) {
                if !self.buf.is_empty() {
                    self.buf.push(' ');
                }
                self.buf.push_str(class);
            }
        }
        self
    }

    /// Добавляет классы, если условие истинно.
    pub fn add_if(&mut self, condition: bool, classes: &str) -> &mut Self {
        if condition {
            self.add(classes);
        }
        self
    }

    /// Добавляет классы, если они заданы.
    pub fn add_opt(&mut self, classes: Option<&str>) -> &mut Self {
        if let Some(c) = classes {
            self.add(c);
        }
        self
    }

    fn contains(&self, class: &str) -> bool {
        self.buf.split(' ').any(|c| c == class)
    }

    /// Готовая строка.
    pub fn finish(self) -> String {
        self.buf
    }
}

/// Реактивный атрибут `class`: пересчитывается и уведомляет подписчиков только при изменении строки.
pub fn class_memo(build: impl Fn(&mut ClassNames) + Send + Sync + 'static) -> Memo<String> {
    Memo::new(move |_| {
        let mut cn = ClassNames::new();
        build(&mut cn);
        cn.finish()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_empty_and_duplicates() {
        let mut cn = ClassNames::new();
        cn.add("p-button p-component").add("").add("  ").add("p-button extra").add_opt(None).add_opt(Some("x"));
        assert_eq!(cn.finish(), "p-button p-component extra x");
    }

    #[test]
    fn conditional_classes() {
        let disabled = true;
        let loading = false;
        let mut cn = ClassNames::new();
        cn.add("p-inputtext").add_if(disabled, "p-disabled").add_if(loading, "p-loading");
        assert_eq!(cn.finish(), "p-inputtext p-disabled");
    }

    #[test]
    fn prefix_is_not_a_duplicate() {
        let mut cn = ClassNames::new();
        cn.add("p-button").add("p-button-sm");
        assert_eq!(cn.finish(), "p-button p-button-sm");
    }

    #[test]
    fn memo_updates_only_on_change() {
        let owner = Owner::new();
        owner.with(|| {
            let disabled = RwSignal::new(false);
            let memo = class_memo(move |cn| {
                cn.add("p-x").add_if(disabled.get(), "p-disabled");
            });
            assert_eq!(memo.get_untracked(), "p-x");
            disabled.set(true);
            assert_eq!(memo.get_untracked(), "p-x p-disabled");
        });
    }
}
