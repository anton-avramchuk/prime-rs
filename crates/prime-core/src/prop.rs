//! Свойство компонента: значение или сигнал.
//!
//! `Signal<T>` и `MaybeProp<T>` Leptos кладут даже константу в арену реактивной системы (несколько аллокаций на каждое
//! свойство каждого экземпляра). [`Prop`] хранит константу как есть, а сигнал — только когда его действительно передали.

use leptos::prelude::*;

/// Значение свойства: готовое (`Static`) или сигнал (`Reactive`).
///
/// Вызов остаётся прежним: значение, `RwSignal`, `ReadSignal`, `Memo` или `Signal` приводятся через `Into`.
///
/// ```
/// use leptos::prelude::*;
/// use prime_core::Prop;
///
/// Owner::new().with(|| {
///     let fixed: Prop<bool> = true.into();
///     assert!(fixed.is_static() && fixed.get());
///
///     let flag = RwSignal::new(false);
///     let live: Prop<bool> = flag.into();
///     assert!(!live.is_static());
///     flag.set(true);
///     assert!(live.get());
///
///     // не переданное свойство — статическое значение по умолчанию
///     assert!(Prop::<bool>::default().is_static());
/// });
/// ```
#[derive(Clone)]
pub enum Prop<T: Send + Sync + 'static> {
    /// Готовое значение: реактивных примитивов нет.
    Static(T),
    /// Сигнал: чтение подписывает читающее замыкание.
    Reactive(Signal<T>),
}

/// Необязательное свойство: `None` — «не задано».
pub type OptProp<T> = Prop<Option<T>>;

impl<T: Send + Sync + 'static> Prop<T> {
    /// Значение статическое (сигнала нет).
    pub fn is_static(&self) -> bool {
        matches!(self, Self::Static(_))
    }

    /// Читает значение; для сигнала — с подпиской.
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        match self {
            Self::Static(v) => v.clone(),
            Self::Reactive(s) => s.get(),
        }
    }

    /// Читает значение без подписки.
    pub fn get_untracked(&self) -> T
    where
        T: Clone,
    {
        match self {
            Self::Static(v) => v.clone(),
            Self::Reactive(s) => s.get_untracked(),
        }
    }

    /// Применяет функцию к значению без клонирования; для сигнала — с подпиской.
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        match self {
            Self::Static(v) => f(v),
            Self::Reactive(s) => s.with(f),
        }
    }
}

impl<T: Default + Send + Sync + 'static> Default for Prop<T> {
    fn default() -> Self {
        Self::Static(T::default())
    }
}

impl<T: Send + Sync + 'static> From<T> for Prop<T> {
    fn from(value: T) -> Self {
        Self::Static(value)
    }
}

impl<T: Send + Sync + 'static> From<Signal<T>> for Prop<T> {
    fn from(s: Signal<T>) -> Self {
        Self::Reactive(s)
    }
}

impl<T: Send + Sync + 'static> From<ReadSignal<T>> for Prop<T> {
    fn from(s: ReadSignal<T>) -> Self {
        Self::Reactive(s.into())
    }
}

impl<T: Send + Sync + 'static> From<RwSignal<T>> for Prop<T> {
    fn from(s: RwSignal<T>) -> Self {
        Self::Reactive(s.into())
    }
}

impl<T: Send + Sync + 'static> From<Memo<T>> for Prop<T> {
    fn from(m: Memo<T>) -> Self {
        Self::Reactive(m.into())
    }
}

impl From<&str> for Prop<String> {
    fn from(s: &str) -> Self {
        Self::Static(s.to_owned())
    }
}

/// `OptProp<T>` из значения: `Some(value)`.
impl<U: Send + Sync + 'static> From<U> for Prop<Option<U>> {
    fn from(value: U) -> Self {
        Self::Static(Some(value))
    }
}

impl From<&str> for Prop<Option<String>> {
    fn from(s: &str) -> Self {
        Self::Static(Some(s.to_owned()))
    }
}

/// Истинно, если все перечисленные свойства (`Prop`/`OptProp`) статические.
///
/// Проверка без аллокаций и подписок; компонент вызывает её один раз при создании экземпляра, чтобы выбрать
/// путь отрисовки без реактивных замыканий.
///
/// ```
/// use leptos::prelude::*;
/// use prime_core::{OptProp, Prop, all_static};
///
/// Owner::new().with(|| {
///     let label: OptProp<String> = "Сохранить".into();
///     let loading: Prop<bool> = Prop::default();
///     assert!(all_static!(label, loading));
///
///     let live: Prop<bool> = RwSignal::new(false).into();
///     assert!(!all_static!(label, loading, live));
/// });
/// ```
#[macro_export]
macro_rules! all_static {
    ($($prop:expr),* $(,)?) => { true $(&& $prop.is_static())* };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_value_reads_back_and_is_static() {
        let p: Prop<String> = "5".into();
        assert!(p.is_static());
        assert_eq!(p.get(), "5");
        assert_eq!(p.with(|s| s.len()), 1);
    }

    #[test]
    fn signal_is_reactive() {
        Owner::new().with(|| {
            let s = RwSignal::new(1);
            let p: Prop<i32> = s.into();
            assert!(!p.is_static());
            assert_eq!(p.get(), 1);
            s.set(2);
            assert_eq!(p.get(), 2);
            assert_eq!(p.with(|v| *v * 10), 20);
        });
    }

    #[test]
    fn every_signal_kind_converts() {
        Owner::new().with(|| {
            let rw = RwSignal::new(1);
            let (r, _w) = signal(2);
            let m = Memo::new(move |_| rw.get() + 1);
            let s = Signal::derive(move || 10);
            assert_eq!(Prop::<i32>::from(rw).get(), 1);
            assert_eq!(Prop::<i32>::from(r).get(), 2);
            assert_eq!(Prop::<i32>::from(m).get(), 2);
            assert_eq!(Prop::<i32>::from(s).get(), 10);
        });
    }

    #[test]
    fn all_static_checks_every_property() {
        Owner::new().with(|| {
            let a: Prop<bool> = true.into();
            let b: OptProp<String> = "x".into();
            let c: Prop<i32> = RwSignal::new(1).into();
            assert!(all_static!(a, b));
            assert!(!all_static!(a, b, c));
            assert!(!all_static!(c));
            assert!(all_static!());
        });
    }

    #[test]
    fn default_is_static_default() {
        assert!(Prop::<bool>::default().is_static() && !Prop::<bool>::default().get());
        assert_eq!(OptProp::<String>::default().get(), None);
    }

    #[test]
    fn opt_prop_accepts_value_str_and_signal() {
        Owner::new().with(|| {
            let a: OptProp<String> = "x".into();
            let b: OptProp<String> = String::from("y").into();
            let sig = RwSignal::new(None::<String>);
            let c: OptProp<String> = sig.into();
            assert_eq!((a.get(), b.get(), c.get()), (Some("x".into()), Some("y".into()), None));
            assert!(a.is_static() && b.is_static() && !c.is_static());
            sig.set(Some("z".into()));
            assert_eq!(c.get(), Some("z".into()));
            let n: OptProp<i32> = 5.into();
            assert_eq!(n.get(), Some(5));
        });
    }

    #[test]
    fn reactive_read_subscribes_a_memo() {
        Owner::new().with(|| {
            let s = RwSignal::new(false);
            let p: Prop<bool> = s.into();
            let memo = Memo::new(move |_| if p.get() { "yes" } else { "no" });
            assert_eq!(memo.get_untracked(), "no");
            s.set(true);
            assert_eq!(memo.get_untracked(), "yes");
        });
    }
}
