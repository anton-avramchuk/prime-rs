//! Построитель списка CSS-классов (аналог `cn` из `@primeuix/utils`).

use leptos::prelude::*;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::sync::{Mutex, OnceLock, PoisonError};

/// Размер стекового буфера, байт: типичный набор классов компонента (с модификаторами) умещается без кучи.
const STACK: usize = 192;

/// Максимум записей в таблице интернирования: константных комбинаций классов у компонента десятки.
const MAX_INTERNED: usize = 4096;

/// Буфер строки: стек, при переполнении — куча.
#[derive(Clone, Debug)]
enum Buf {
    Stack { data: [u8; STACK], len: usize },
    Heap(String),
}

impl Buf {
    fn as_str(&self) -> &str {
        match self {
            // Записываются только целые `&str`, поэтому границы символов сохраняются.
            Self::Stack { data, len } => std::str::from_utf8(&data[..*len]).unwrap_or_default(),
            Self::Heap(s) => s,
        }
    }

    fn push_str(&mut self, s: &str) {
        match self {
            Self::Stack { data, len } if *len + s.len() <= STACK => {
                data[*len..*len + s.len()].copy_from_slice(s.as_bytes());
                *len += s.len();
            }
            Self::Stack { .. } => {
                let mut heap = String::with_capacity((self.as_str().len() + s.len()).max(2 * STACK));
                heap.push_str(self.as_str());
                heap.push_str(s);
                *self = Self::Heap(heap);
            }
            Self::Heap(h) => h.push_str(s),
        }
    }

    fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }
}

/// Собирает классы в одну строку: без пустых и повторяющихся классов, на стековом буфере.
///
/// `finish()` отдаёт `String`. `finish_cow()` для строк из одних констант компонента (без [`add_user`](Self::add_user))
/// возвращает интернированный `&'static str` без аллокации.
///
/// ```
/// use prime_core::ClassNames;
///
/// let mut cn = ClassNames::new();
/// cn.add("p-button p-component").add_if(true, "p-button-sm").add_if(false, "p-disabled");
/// assert_eq!(cn.finish(), "p-button p-component p-button-sm");
/// ```
#[derive(Debug, Clone)]
pub struct ClassNames {
    buf: Buf,
    dynamic: bool,
}

impl Default for ClassNames {
    fn default() -> Self {
        Self::new()
    }
}

impl ClassNames {
    /// Пустой построитель.
    pub fn new() -> Self {
        Self { buf: Buf::Stack { data: [0; STACK], len: 0 }, dynamic: false }
    }

    /// Пустой построитель; ёмкость не нужна (стековый буфер), параметр оставлен для совместимости.
    pub fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }

    /// Добавляет один или несколько классов через пробел.
    pub fn add(&mut self, classes: &str) -> &mut Self {
        for class in classes.split_whitespace() {
            if !self.contains(class) {
                if !self.buf.is_empty() {
                    self.buf.push_str(" ");
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

    /// Добавляет пользовательские классы (`style_class`, `pt`): строка перестаёт быть константной и не интернируется.
    pub fn add_user(&mut self, classes: Option<&str>) -> &mut Self {
        if let Some(c) = classes.filter(|c| !c.trim().is_empty()) {
            self.dynamic = true;
            self.add(c);
        }
        self
    }

    fn contains(&self, class: &str) -> bool {
        self.buf.as_str().split(' ').any(|c| c == class)
    }

    /// Готовая строка.
    pub fn finish(self) -> String {
        match self.buf {
            Buf::Heap(s) => s,
            Buf::Stack { .. } => self.buf.as_str().to_owned(),
        }
    }

    /// Готовая строка без аллокации для константных комбинаций классов (интернирование), иначе `String`.
    pub fn finish_cow(self) -> Cow<'static, str> {
        if self.dynamic { Cow::Owned(self.finish()) } else { intern(self.buf.as_str()) }
    }
}

/// Находит или добавляет строку в глобальную таблицу; сверх лимита возвращает `String`.
fn intern(s: &str) -> Cow<'static, str> {
    static TABLE: OnceLock<Mutex<BTreeSet<&'static str>>> = OnceLock::new();
    let mut table = TABLE.get_or_init(Default::default).lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(found) = table.get(s) {
        return Cow::Borrowed(found);
    }
    if table.len() >= MAX_INTERNED {
        return Cow::Owned(s.to_owned());
    }
    let leaked: &'static str = Box::leak(s.to_owned().into_boxed_str());
    table.insert(leaked);
    Cow::Borrowed(leaked)
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

    #[test]
    fn finish_cow_interns_constant_strings() {
        let build = || {
            let mut cn = ClassNames::new();
            cn.add("p-a p-b").add_if(true, "p-c");
            cn.finish_cow()
        };
        let (first, second) = (build(), build());
        assert_eq!(first, "p-a p-b p-c");
        assert!(matches!((&first, &second), (Cow::Borrowed(a), Cow::Borrowed(b)) if std::ptr::eq(*a, *b)));
    }

    #[test]
    fn user_classes_are_not_interned() {
        let mut cn = ClassNames::new();
        cn.add("p-a").add_user(Some("mine")).add_user(None).add_user(Some("  "));
        let out = cn.finish_cow();
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out, "p-a mine");
        let mut empty_user = ClassNames::new();
        empty_user.add("p-a").add_user(Some(""));
        assert!(matches!(empty_user.finish_cow(), Cow::Borrowed("p-a")));
    }

    #[test]
    fn long_strings_overflow_the_stack_buffer() {
        let mut cn = ClassNames::new();
        for i in 0..40 {
            cn.add(&format!("class-number-{i}"));
        }
        let out = cn.finish();
        assert!(out.len() > STACK && out.starts_with("class-number-0 class-number-1") && out.ends_with("class-number-39"));
    }
}
