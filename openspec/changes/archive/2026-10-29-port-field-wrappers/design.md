# Design

## Context

Requirements are in `specs/{buttongroup,fluid,iconfield,floatlabel,iftalabel}`. PrimeNG 21.1.10 sources: `buttongroup/buttongroup.ts`, `fluid/fluid.ts`, `iconfield/iconfield.ts`, `inputicon/inputicon.ts`, `floatlabel/floatlabel.ts`, `iftalabel/iftalabel.ts` and their `style/*style.ts`. Models: `crates/prime-inputgroup` (a container with a slot and `pt.root`, two components in one crate), `crates/prime-toolbar`/`prime-card` (a container with `style_class`), `prime_core::config` (context via `provide_context`).

Observations about the source that affect decisions:
- All six are "dumb" containers: an `<ng-content>` template, root classes and `pt`; no logic except choosing a class by `variant` and `iconPosition`.
- Icon position, label lift, colors and rounding are entirely in the theme CSS: `@primeuix/styles/{buttongroup,floatlabel,iftalabel,iconfield}` (`iconfield` also has the `.p-inputicon` rules). `fluid` has no style module (`FluidStyle` only sets the class `p-fluid`).
- The `/* For PrimeNG */` blocks of `buttongroup` target the `p-button` host element (`.p-buttongroup p-button …`) and those of `floatlabel`/`iftalabel` target `.ng-invalid.ng-dirty` — they do not apply to our markup. The base theme CSS already contains `.p-buttongroup .p-button …` and `:has(.p-invalid) label`, which is enough; `primeng_extra` is not extended.
- `Fluid` styles nothing: descendants (`Button`, `InputText`, `Textarea`, `InputNumber`, `Select`, as well as `AutoComplete`, `DatePicker` and others we do not have yet) read `hasFluid = fluid() ?? !!pcFluid` via DI and add their own stretching class. So the real work is in the consumers.
- `iconPosition` of `IconField` defaults to `left` (the documentation says "right", but the code sets `left`); since the CSS positions the icon by `:first-child`/`:last-child`, the class `p-iconfield-left|right` is informational. We port the behavior of the code.

## Decisions

### D1. Crates
`prime-buttongroup`, `prime-fluid`, `prime-iconfield` (`IconField`, `InputIcon`), `prime-floatlabel`, `prime-iftalabel`. Each depends only on `prime-core` (`ClassNames`, `Pt`, `OptProp`/`Prop`); each has `ssr`/`hydrate` features, rustdoc with an example, SSR tests. Components follow `InputGroup`: `children: Children`, `style_class: OptProp<String>`, `style: OptProp<String>`, `pt: Option<…Pt>`, where `…Pt { root: Pt }`; single properties (`variant`, `icon_position`) are `Prop<…>`. All five have no static branch: markup of a single element with closures only where a property can be a signal (the root `class`).

### D2. Fluid context (`prime-core::fluid`)
`FluidContext` (a marker type), `provide_fluid()` (`provide_context(FluidContext)`), `in_fluid() -> bool` (`use_context::<FluidContext>().is_some()`), `resolve_fluid(explicit: Option<bool>) -> bool` (`explicit.unwrap_or_else(in_fluid)`). `Fluid` calls `provide_fluid()` on creation and renders `div.p-fluid`. Consumers read the context **once on creation** (`let inherited = in_fluid()`), because the ancestor does not change.

### D3. Fluid consumers
In `Button`, `InputText`, `Textarea`, `InputNumber` and `Select` the `fluid` property changes from `Prop<bool>` to `OptProp<bool>` (calls `fluid=true` and `fluid=signal_of_option` are compatible; `fluid=signal_of_bool` now has to be wrapped in `Some`, but there are no such calls in the showcase and tests). The effective value is `fluid.get().unwrap_or(inherited)`: in the static branch it is computed once (the `all_static!` marker is not violated — the context is static), in the reactive one it is read in a closure. `ToggleButton` and `SelectButton` do not change (they have no inheritance in PrimeNG).

### D4. FloatLabel and IftaLabel
`FloatLabelVariant { Over (default), On, In }` and classes `p-floatlabel p-floatlabel-over|on|in` (the pure function `write_classes` with a table-driven test); the `class` closure lets the variant change without recreating children. `IftaLabel` has one constant class. The `<label for>` and the field stay in the user's markup (as in PrimeNG).

### D5. IconField and InputIcon
`IconPosition { Left (default), Right }`; classes `p-iconfield p-iconfield-left|right`; `InputIcon` is `span.p-inputicon` with a slot, the icon is passed as a child element (an SVG from `prime-icons` or custom).

### D6. CSS
`buttongroup`, `floatlabel`, `iftalabel` are added to `theme.components` (`iconfield` is already there); `cargo xtask gen-theme`, then check `git diff` for unchanged CSS of other components. `fluid` has no module — not added.

### D7. Demos
Pages `/buttongroup` (`basic` — three buttons; `split` with an icon — after `button/buttongroup-doc.ts`), `/fluid` (`basic`), `/iconfield` (`basic`, `sizes`, `template`, `floatlabel`, `iftalabel`), `/floatlabel` (`basic`, `variants`, `invalid`), `/iftalabel` (`basic`, `invalid`). In `iconfield`'s `template` a simple SVG replaces the PrimeVue logo. The `floatlabel` and `iftalabel` demos from their documentation (if they exist there) are added to the InputText, InputNumber, Select and Textarea pages; e2e for each page and the updated places. Icons — `SearchIcon`, `UserIcon`/`LockIcon` are unavailable in `prime-icons`, so the demos use only the available ones (`SearchIcon`, `SpinnerIcon`, `TimesIcon`, `CheckIcon`); the difference is recorded in the documentation.

### D8. Tests
SSR: structure, classes and order, `style`/`style_class`/`pt`, variants and positions, nesting, empty content; for Fluid — all five consumers (including the priority of an explicit `false`, reactive `fluid`, nesting). No browser tests: the components are static, and behavior is checked by e2e (positioning and label lift, joined buttons, stretching to full width).

### D9. Benchmarks
Scenarios `buttongroup`, `fluid`, `iconfield`, `floatlabel`, `iftalabel` (`-1` and `-1000` each) and `case-*` for wasm; budgets by measurement.

## Risks / Trade-offs

- [Changing the `fluid` type in five existing crates] → the API stays compatible for `fluid=true` and with no value; behavior without `Fluid` does not change (tests of existing crates stay green without edits, except those that explicitly pass `Signal<bool>`).
- [Theme CSS changes only by adding modules] → check `git diff` after `gen-theme`.
- [No browser tests] → compensated by e2e with computed styles (icon position, label lift, button radii).
- [No `user`, `lock` icons in `prime-icons`, demos simplified] → recorded in `docs/field-wrappers.md`.

## Migration Plan

No migration. Rollback — `git revert`.

## Open Questions

None.
