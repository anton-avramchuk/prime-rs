//! The `Fluid` context: descendants without an explicit `fluid` stretch if they have a `Fluid` ancestor
//! (`hasFluid = fluid() ?? !!pcFluid` in PrimeNG).

use crate::{OptProp, Prop};
use leptos::prelude::*;

/// Marker "we are inside `Fluid`", put into the context by the `Fluid` component.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FluidContext;

/// Makes the content of the current owner a descendant of `Fluid`.
pub fn provide_fluid() {
    provide_context(FluidContext);
}

/// Whether there is a `Fluid` among the ancestors.
pub fn in_fluid() -> bool {
    use_context::<FluidContext>().is_some()
}

/// Resulting `fluid`: an explicitly set one (including `false`) takes precedence over the context.
///
/// ```
/// use leptos::prelude::*;
/// use prime_rs_core::fluid::{in_fluid, provide_fluid, resolve_fluid};
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

/// Turns a component's optional `fluid` into the resulting `Prop<bool>`: a set value takes precedence over the `Fluid` context, and without a value the
/// presence of a `Fluid` ancestor is used (determined once when the component is created). A signal remains a signal.
///
/// ```
/// use leptos::prelude::*;
/// use prime_rs_core::{OptProp, fluid::{fluid_prop, provide_fluid}};
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
        // intermediate owners must live as long as the descendant lives: the link to the ancestor is weak
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
