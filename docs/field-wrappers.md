# Field wrappers: ButtonGroup, Fluid, IconField, FloatLabel, IftaLabel

Port of `p-buttongroup`, `p-fluid`, `p-iconfield` with `p-inputicon`, `p-floatlabel` and `p-iftalabel` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-buttongroup::ButtonGroup` | `buttongroup/buttongroup.ts`, `buttongroup/style/buttongroupstyle.ts` |
| `prime-fluid::Fluid` | `fluid/fluid.ts` (no styles: `FluidStyle` only sets the `p-fluid` class) |
| `prime-iconfield::IconField`, `InputIcon` | `iconfield/iconfield.ts`, `inputicon/inputicon.ts`, their `style/*style.ts` |
| `prime-floatlabel::FloatLabel` | `floatlabel/floatlabel.ts`, `floatlabel/style/floatlabelstyle.ts` |
| `prime-iftalabel::IftaLabel` | `iftalabel/iftalabel.ts`, `iftalabel/style/iftalabelstyle.ts` |
| CSS | `@primeuix/styles/{buttongroup,floatlabel,iftalabel,iconfield}` — `cargo xtask gen-theme`; there are no modules for `fluid` and `inputicon` (the `.p-inputicon` rules are part of `iconfield`) |
| Demos | `apps/showcase/doc/{buttongroup,fluid,iconfield,floatlabel,iftalabel}/*-doc.ts`, plus `floatlabel` and `iftalabel` on the InputText, InputNumber, Select, Textarea pages |

Specs: `openspec/specs/{buttongroup,fluid,iconfield,floatlabel,iftalabel}`.

All five are containers without logic: `children`, `style`, `style_class` and `pt.root`. The field and `<label for>` stay in the user's markup, as in PrimeNG.

## What matches PrimeNG

- Root classes and their order: `p-buttongroup p-component` (`role="group"`), `p-fluid`, `p-iconfield p-iconfield-left|right`, `p-inputicon`, `p-floatlabel p-floatlabel-over|on|in`, `p-iftalabel`; then `style_class`, then the class from `pt.root`.
- Icon position, label lift and size, joined buttons, colors and rounding are entirely in the theme CSS; the components only set classes.
- `iconPosition` defaults to `left` (the PrimeNG docs say "right", but the code sets `left`); since the CSS positions the icon by `:first-child`/`:last-child`, the `p-iconfield-left|right` class is informational.
- `Fluid` makes `Button`, `InputText`, `Textarea`, `InputNumber` and `Select` behave as with `fluid = true` when the descendant has no explicit `fluid`; an explicit value (including `false`) takes precedence — `hasFluid = fluid() ?? !!pcFluid`.

## Differences

| Difference | Reason |
|---|---|
| Roots are `<span>`/`<div>`, not `<p-…>` hosts | The host in Angular is a framework detail. |
| The `Fluid` context is `prime_rs_core::fluid` (`provide_fluid`, `in_fluid`, `resolve_fluid`, `fluid_prop`); consumers read it once at creation | The ancestor does not change during the descendant's lifetime. Replaces `inject(Fluid)` in PrimeNG. |
| `Fluid` puts the context into a child owner and keeps it until the parent is cleaned up | Otherwise `provide_context` in the component would leak to siblings that follow `Fluid` in the same owner. |
| `fluid` on `Button`, `InputText`, `Textarea`, `InputNumber`, `Select` is `OptProp<bool>`, not `Prop<bool>` | "Not set" must be distinguished from `false`. Calls with `fluid=true` and `fluid=signal_Option` are compatible; a `bool` signal is now wrapped in `Some`. `ToggleButton` and `SelectButton` do not change: they have no inheritance in PrimeNG. |
| `/* For PrimeNG */` blocks are not carried over | They target the `p-button` hosts (`.p-buttongroup p-button …`) and the Angular Forms class `.ng-invalid.ng-dirty`; the theme base CSS already contains `.p-buttongroup .p-button …` and `:has(.p-invalid) label`. |
| `FloatLabelVariant` is an enum, `variant` may be a signal | The root class is replaced without recreating the children; `write_classes` is a pure function with a table test. |
| The demos lack the `user`, `lock`, `bookmark`, `heart`, `time`, `trash` icons (available are `search`, `eye`, `check`, `times`, `plus`, `pencil`, `trash`, `spinner`); the IconField `template` uses a simple SVG instead of the PrimeVue logo | They are not in `prime-icons` or need a CDN. |
| Demos with Angular Forms are not carried over; `invalid` in `floatlabel` and `iftalabel` follows the emptiness of the field | Forms are out of scope. |
| The Fluid demo uses `InputNumber`, `Select`, `Textarea` instead of `AutoComplete`, `DatePicker`, `Password`; the same set without `Fluid` is shown next to it | These components do not exist in the project yet. |
| `fluid` has neither CSS nor a theme | As in PrimeNG: the `p-fluid` class styles nothing, the descendants set the width themselves. |

Other components that read `Fluid` in PrimeNG (`AutoComplete`, `DatePicker`, `Password`, `InputMask`, `MultiSelect` and others) will get the same context once they exist: `fluid_prop(fluid)` at the start of the component is enough.

## Performance

SSR, allocations per instance (budget in `benches/alloc-budgets.toml`): ButtonGroup with three stub buttons — 45 (50), Fluid with `InputText` and `Button` — 31 (34), IconField with an icon and a field — 21 (23), FloatLabel — 9 (10), IftaLabel — 9 (10). Contribution to raw wasm is in the table below (budgets in `benches/budgets.toml`).

| Component | Contribution to raw wasm, B | Budget, B |
|---|---|---|
| Buttongroup | 73687 | 81000 |
| Fluid | 75238 | 83000 |
| Iconfield | 112697 | 124000 |
| Floatlabel | 91149 | 100000 |
| Iftalabel | 71920 | 79000 |

Tests: crate SSR tests (structure, classes, order, `style`/`style_class`/`pt`, parity of the static and reactive branches, variants and positions, `Fluid` inheritance by all five consumers, explicit `fluid = false`, nesting and siblings outside `Fluid`), SSR inheritance tests in each of the five consumers, Playwright e2e (icon placement, label lift, joined buttons, stretching to full width).
