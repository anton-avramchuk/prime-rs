# Rating

Port of PrimeNG 21.1.10's `p-rating` (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-rating::Rating` | `packages/primeng/src/rating/rating.ts`, `rating/style/ratingstyle.ts` |
| CSS | `@primeuix/styles/rating` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/rating/*-doc.ts` (`basic`, `disabled`, `numberofstars`, `readonly`, `template`) |

Spec: `openspec/specs/rating`.

## What matches PrimeNG

- Structure: root `div.p-rating`, for each star a `div.p-rating-option` with a hidden radio button (`span.p-hidden-accessible > input[type=radio]`) and the icon `p-rating-icon p-rating-on-icon` / `p-rating-off-icon`.
- Classes of the root, option and icons and their order: `p-rating`, `p-readonly`, `p-disabled`; `p-rating-option`, `p-rating-option-active`, `p-focus-visible`; `p-invalid` on the icons. The root's `data-p` is `readonly disabled`.
- A group of native radio buttons with a shared `name`: the keyboard arrows work without any code of our own. The default name is `pn_id_<n>_name`, the number from the hydration context (the shared helper `prime_core::ids::next_unique`, extracted from `collapsible`).
- A click on a star selects it, a repeated click on the current value resets it; after a click focus moves to the star's radio button and is not considered visible.
- A star's `aria-label` comes from the locale: `aria.star` for the first, `aria.stars` with `{star}` substituted for the rest.
- Default icons are `StarFillIcon` and `StarIcon`; the `on_icon_template` and `off_icon_template` templates receive the star number and the icon class (`{ $implicit, class }`).

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the `<p-rating>` host | The host in Angular is a framework detail. |
| `value: RwSignal<Option<u32>>` instead of `ngModel`; without it, internal state (`default_value`) | Two-way binding without forms infrastructure. |
| A keyboard arrow selects the neighbouring star and does not reset the value | In PrimeNG `onOptionSelect` resets the value if the focus index equals the selected one, and `focus` arrives before `change`; this looks like a side effect, in our case `change` always sets the star's value. The value cannot be reset from the keyboard, only by a click. |
| A click on the radio button itself is handled by `change`, not `click` | Arrows and a programmatic `click()` send both events, so the value would be set twice. |
| The radio button's `value` attribute is the star number | In PrimeNG `[attr.value]="modelValue()"` overrides `[value]`, and every button's `value` equals the current value, which breaks native form submission (RadioButton does the same). |
| Visible focus is the radio button's `:focus-visible` | `sourceCapabilities.firesTouchEvents` is absent from `web-sys` and Firefox. |
| `on_icon`/`off_icon` are a `ViewFn` in a `span` with the icon classes instead of `iconOnClass`/`iconOffClass`/`iconOnStyle`/`iconOffStyle` | `prime-icons` has no font; styles are set by `style`/`pt`. |
| No `autofocus`, Angular Forms integration, `/* For PrimeNG */` block | Out of scope of the change. |
| The `withoutcancel` demo is not ported | In 21.1.10 Rating has no cancel button, the demo duplicates `basic`. |
| In the `template` demo the `♥` and `♡` characters instead of external images | No dependency on a CDN. |

The star list is built by a closure over `stars` and does not read the value: otherwise a selection would recreate the stars and drop focus; everything that depends on the value is separate closures inside a star.

## Performance

SSR: 159 allocations per instance with five stars (budget 175), HTML ≈ 9 KB because of the SVG paths in each icon; contribution to raw wasm +258 KB (budget 300 KB).
