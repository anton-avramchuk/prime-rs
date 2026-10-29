//! Контекст `Fluid`: потомки без явного `fluid` растягиваются, если у них есть предок `Fluid`
//! (`hasFluid = fluid() ?? !!pcFluid` в PrimeNG).

use crate::{OptProp, Prop};
use leptos::prelude::*;

/// Маркер «мы внутри `Fluid`», кладётся в контекст компонентом `Fluid`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FluidContext;

/// Делает содержимое текущего владельца потомком `Fluid`.
pub fn provide_fluid() {
    provide_context(FluidContext);
}

/// Есть ли среди предшественников `Fluid`.
pub fn in_fluid() -> bool {
    use_context::<FluidContext>().is_some()
}

/// Итоговое `fluid`: заданное явно (в том числе `false`) главнее контекста.
///
/// ```
/// use leptos::prelude::*;
/// use prime_core::fluid::{in_fluid, provide_fluid, resolve_fluid};
///
/// Owner::new().with(|| {
///     assert!(!resolve_fluid(None));
///     provide_fluid();
///     assert!(in_fluid() && resolve_fluid(None));
///     assert!(!resolve_fluid(Some(false)));
/// });
/// ```
pub fn resolve_fluid(explicit: Option<bool>) -> bool {
    explicit.unwrap_or_else(in_fluid)
}

/// Превращает необязательное `fluid` компонента в итоговое `Prop<bool>`: заданное главнее контекста `Fluid`, а без значения берётся
/// наличие предка `Fluid` (определяется один раз при создании компонента). Сигнал остаётся сигналом.
///
/// ```
/// use leptos::prelude::*;
/// use prime_core::{OptProp, fluid::{fluid_prop, provide_fluid}};
///
/// Owner::new().with(|| {
///     let unset: OptProp<bool> = OptProp::default();
///     assert!(!fluid_prop(unset.clone()).get());
///     provide_fluid();
///     assert!(fluid_prop(unset).get());
///     let off: OptProp<bool> = false.into();
///     assert!(!fluid_prop(off).get());
///     let live = RwSignal::new(None::<bool>);
///     let prop = fluid_prop(live.into());
///     assert!(prop.get());
///     live.set(Some(false));
///     assert!(!prop.get());
/// });
/// ```
pub fn fluid_prop(explicit: OptProp<bool>) -> Prop<bool> {
    let inherited = in_fluid();
    match explicit {
        Prop::Static(value) => Prop::Static(value.unwrap_or(inherited)),
        Prop::Reactive(signal) => Prop::Reactive(Signal::derive(move || signal.get().unwrap_or(inherited))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outside_fluid_nothing_is_inherited() {
        Owner::new().with(|| {
            assert!(!in_fluid());
            assert!(!resolve_fluid(None));
            assert!(resolve_fluid(Some(true)));
            assert!(!resolve_fluid(Some(false)));
        });
    }

    #[test]
    fn descendants_inherit_at_any_depth_and_explicit_value_wins() {
        let root = Owner::new();
        root.with(provide_fluid);
        // промежуточные владельцы должны жить, пока жив потомок: связь с предком слабая
        let child = root.with(|| Owner::current().unwrap().child());
        let deep = child.with(|| Owner::current().unwrap().child());
        assert!(deep.with(in_fluid));
        assert!(deep.with(|| resolve_fluid(None)));
        assert!(deep.with(|| resolve_fluid(Some(true))));
        assert!(!deep.with(|| resolve_fluid(Some(false))));
    }

    #[test]
    fn sibling_owner_does_not_see_the_context() {
        let provider = Owner::new();
        provider.with(provide_fluid);
        Owner::new().with(|| assert!(!in_fluid()));
    }
}
