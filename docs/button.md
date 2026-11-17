# Button and Badge

Port of `p-button` and `p-badge` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-button::Button` | `packages/primeng/src/button/button.ts` (the `Button` component, `p-button`), `button/style/buttonstyle.ts` |
| `prime-badge::Badge` | `packages/primeng/src/badge/badge.ts` (the `Badge` component, `p-badge`), `badge/style/badgestyle.ts` |
| CSS | `@primeuix/styles/button`, `@primeuix/styles/badge` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/button/*-doc.ts`, `apps/showcase/doc/badge/*-doc.ts` |

Specs: `openspec/specs/button`, `openspec/specs/badge` (after archiving the change `port-button`).

## What matches PrimeNG

- The classes `p-button*`, `p-badge*` and the attributes `data-p`, `data-p-disabled`, `data-p-severity` are computed by the same rules (`state.rs` of both crates,
  unit-test tables based on `buttonstyle.ts` and `badgestyle.ts`).
- Button content order: `children`, the icon or the loading indicator, the label, the badge.
- Properties: see the spec; Angular names are converted to snake_case (`iconPos` → `icon_pos`, `badgeSeverity` → `badge_severity`, `styleClass` → `style_class`).

## Differences and their reasons

| Difference | Reason |
|---|---|
| There are no host elements `<p-button>`; Badge uses a `<span>` instead of `<p-badge>` | The Angular host is a framework detail; the theme CSS is bound to classes. |
| The icon is a `ViewFn` inside a `span.p-button-icon` wrapper, not a PrimeIcons class string | There is no PrimeIcons font; the wrapper is needed so that the CSS (`order`, paddings) works for any icon. |
| The default loading indicator is `SpinnerIcon` without numeric classes | In PrimeNG `spinnerIcon` builds the classes `0 1 2 …` from `Object.entries` of a string; this is a side effect, not a style. |
| A custom loading indicator without the `pi-spin` class | `pi-spin` is an animation of the PrimeIcons font; `SpinnerIcon` has its own `p-icon-spin`. |
| No `pButton` directive, `buttonProps`, `unstyled` | Outside the scope of the change; see `proposal.md`. |
| The `fluid` property is `OptProp<bool>`: without a value it is taken from the `Fluid` context | As `hasFluid = fluid() ?? !!pcFluid` in PrimeNG; see `docs/field-wrappers.md`. |
| The demo icons are replaced by the closest `prime-icons` ones | The 53 PrimeIcons have no user, bookmark, bell, heart and users: `pi-user` → `InfoCircleIcon`, `pi-bookmark` → `StarIcon`, `pi-bell` → `ExclamationTriangleIcon`, `pi-heart` → `StarFillIcon`. |

## Checks

- `cargo test -p prime-button -p prime-badge --all-features` — unit and SSR tests (DOM structure, classes, `pt`, a11y, reactivity).
- `cargo test -p prime-button --target wasm32-unknown-unknown --test web` — click, `disabled`/`loading`, focus, autofocus, ripple, rewriting of `class`.
- `e2e/tests/button.spec.ts` — the `/button` page in the showcase (Playwright).
- Weekly screenshot comparison with PrimeNG (`e2e/tests/visual.spec.ts`, `pages.json`): the report `visual.json`, does not block PRs.

## Screenshot comparison with PrimeNG

The weekly job (`weekly.yml`, the run of 2026-10-28) compared 17 demos with the running PrimeNG showcase at the pinned commit.
The share of differing pixels (`pixelmatch`, threshold 0.1), largest first:

| Demo | Difference | Demo | Difference |
|---|---:|---|---:|
| button-severity | 24.3 % | button-badge | 10.7 % |
| button-icons | 23.8 % | button-loading | 8.2 % |
| button-raised | 23.8 % | button-basic | 6.0 % |
| button-rounded | 21.3 % | button-disabled | 5.9 % |
| button-sizes | 21.1 % | badge-severity | 5.3 % |
| button-iconsonly | 20.5 % | badge-size | 4.7 % |
| button-link | 12.5 % | button-outlined | 4.4 % |
| button-text | 2.4 % | button-template | 1.2 % |
| badge-basic | 1.0 % | | |

The difference comes not from the components but from the showcase environment; the pictures in `e2e-results` show three causes:

1. **Primary color.** In the PrimeNG showcase the primary color is "noir" (almost black), in our showcase it is the default emerald of the Aura preset; the `Primary` buttons and their `outlined`/`text` variants differ in color. The other severities (`secondary`, `success`, `info`, `warn`, `help`, `danger`) match in color.
2. **Font.** PrimeNG has its own showcase font, we use the system `sans-serif`: the button widths and the line height differ.
3. **Card width.** The PrimeNG card is narrower, so a row of eight buttons wraps onto a second line, while ours stays on one; this is why demos with many buttons (`severity`, `raised`, `rounded`, `icons`) differ the most.

The next step for an honest comparison (not in this change): bring the showcase to the same primary color, font and content width.
The metric stays a report and does not block PRs.

## Performance

Measurements and budgets — `docs/benchmarks.md`. After `optimize-reactive-props` Badge makes 8 and Button 23 allocations per instance with SSR
(it was 41 and 67); in the browser a static button takes 2.0 KB of wasm linear memory (it was 9.4 KB), a static badge 0.4 KB (it was 3.4 KB); contribution to raw wasm +96 KB and +374 KB.

Template for the next components:
- properties — `Prop<T>`/`OptProp<T>` from `prime-core` (a value does not create a signal, a signal subscribes the attributes);
- no `Memo`: the state borrows the values (`Option<&str>`) and is assembled inside the attribute closures;
- the instance properties live in one `Arc<Props>`, the closures clone only the reference counter;
- client effects (`Effect`, ripple, autofocus) are created only under `cfg(target_arch = "wasm32")`;
- two render branches: if `all_static!(all properties)`, the component returns `Either::Left` with ready attribute and block values (no closures and no `RenderEffect`),
  otherwise `Either::Right` with closures; the markup is written once in a `macro_rules!` (the attribute expressions are parameters), the values are computed by shared functions
  (`class_of`, `style_of`, `data_of`, `*_of` for the blocks), which are called once or inside a closure;
- parity tests: each scenario is rendered with values and with signals, and the HTML must match (`tests/parity.rs`); the browser event tests run in both branches;
- in `view!` an identifier before `{..attrs}` is wrapped in `{}` (`data-p={x} {..attrs}`), otherwise the macro reads it as a struct literal.

## A known problem found during the port

The theme generator left unresolved `dt('button.primary.color')` in the component CSS (the token values were not substituted, and the buttons looked
native). Fixed in `xtask/theme/extract.mjs` (the style text goes through the `Css` tag, as in `BaseStyle.load` of PrimeNG), `gen-theme`
now rejects CSS containing `dt(`, and the `prime-theme` tests check the token substitution.
