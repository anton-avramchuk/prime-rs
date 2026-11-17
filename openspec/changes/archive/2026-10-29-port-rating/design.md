# Design

## Context

Requirements are in `specs/rating`. PrimeNG 21.1.10 sources: `packages/primeng/src/rating/rating.ts`, `rating/style/ratingstyle.ts`, the demos in `apps/showcase/doc/rating`. Models: `crates/prime-radiobutton` (a native `<input>` in a group, `docs/radiobutton.md`) and `crates/prime-selectbutton`/`prime-togglebutton` (reactive `rx` closures, tracking rules). `prime-icons` has `StarIcon` and `StarFillIcon`, `prime-core` has the `aria.star` and `aria.stars` translations, and `p-hidden-accessible` is already in the theme's base CSS.

Observations about the source that affected the decisions:
- Rating has no cancel button (in the `withoutcancel` demo nothing differs from `basic`); a reset is selecting the current value again.
- PrimeNG's `onOptionSelect` event resets the value if `focusedOptionIndex === value || value === this.value`; the focus index is set by the `focus` handler, which fires before `change` on an arrow transition, so in PrimeNG an arrow on a "focused" star apparently resets the value. This looks like an implementation side effect rather than intended behaviour, and is not ported (see D2).
- `[attr.value]="modelValue()"` on the `<input>` overrides `[value]="star + 1"`, so every radio button's `value` equals the current value. This breaks native form submission and is not ported (as with RadioButton).

## Decisions

### D1. Value
`value: Option<RwSignal<Option<u32>>>`; without it an internal signal with `default_value` (`OptProp<u32>`, read once) is created. Star numbers start from 1; the value `0` is not used (PrimeNG has `value || null`).

### D2. Selection
The pure function `next_value(current, star) -> Option<u32>` (`state.rs`): an equal value is reset to `None`, otherwise `Some(star)`. A click on `div.p-rating-option` selects as in PrimeNG: with `readonly`/`disabled` nothing happens; otherwise the value changes, `on_rate` is called, then focus moves to the star's radio button (`onOptionClick`). The radio button's `change` event (arrows, programmatic `click()`) sets the star's value **without** toggling to `None`: a native `change` arrives only when the selected radio button changes, so the value there is new by definition. The value cannot be reset from the keyboard (as with a native group), only by a click.

### D3. Markup and reactivity
The root is a `<div>`; the stars are built once by a closure over `stars` (the star count is the only thing it reads), and everything that depends on the value (option classes, `checked`, the icon) is separate closures inside a star. The list closure does not read `value`: reading the value signal in it would recreate the stars and drop focus (the lesson of `port-selectbutton`; only `*_untracked` inside the list). Option class order: `p-rating-option`, `p-rating-option-active`, `p-focus-visible`, then `pt.option`; icons: `p-rating-icon p-rating-on-icon|p-rating-off-icon`, `p-invalid`, `pt`. The root's `data-p`: `readonly disabled`; root classes: `p-rating`, `p-readonly`, `p-disabled`, `style_class`, `pt.root`. The pure logic (classes, `data-p`, `next_value`) is in `state.rs` with a table test.

### D4. Focus and `p-focus-visible`
In PrimeNG visible focus is determined by `sourceCapabilities.firesTouchEvents`, which is absent from `web-sys` and Firefox. Instead, the `focus` handler checks `input.matches(":focus-visible")` and stores the star number in `visible_focus: RwSignal<Option<u32>>`; `blur` resets it. Focus after a mouse click (programmatic) is not considered visible by the browser, so `p-focus-visible` does not appear on such a star, as in PrimeNG. `on_focus`/`on_blur` are called on the radio button's `focus`/`blur` (`on_focus` only if not `readonly` and not `disabled`).

### D5. Group name
`name: OptProp<String>`, otherwise `pn_id_<n>_name`, where `<n>` is a number from the shared hydration context (identical on the server and in the browser). The logic already exists in `prime-core::collapsible` (`next_unique`); we move it to `prime-core::ids` (`pub fn next_unique`) and use it in `collapsible` and `Rating`. The behaviour of `Panel`/`Fieldset`/`Accordion` does not change (their tests and snapshots stay as they are).

### D6. Icons and slots
By default `StarFillIcon`/`StarIcon` with the icon classes and `pt`. `on_icon`/`off_icon: Option<ViewFn>` replace the star; `on_icon_template`/`off_icon_template: Callback<RatingIcon, AnyView>` (`RatingIcon { star: u32, class: String }`, like PrimeNG's `{ $implicit, class }` context) replace the icon entirely, priority: the template, then `on_icon`/`off_icon`, then the star. The class-based `icon_on_class` and the like are not ported: there is no icon font.

### D7. Labels
A star's `aria-label`: `aria.star` for the first, otherwise `aria.stars` with `{star}` substituted; the strings are taken from `use_prime_config().locale` once when the component is created (as in other components with a locale).

### D8. CSS
`rating` in `theme.components` (`upstream.toml`), `cargo xtask gen-theme`. The `/* For PrimeNG */` block contains only Angular Forms' `p-rating.ng-invalid.ng-dirty` rule, so `primeng_extra` is not extended. Icons are painted and outlined via the theme's `stroke`; for `p-invalid`, `rating.invalid.icon.color` is used.

### D9. Demos
`basic`, `disabled`, `numberofstars`, `readonly`, `template`. Demos with forms and `withoutcancel` are skipped. In `template`, instead of external CDN images, the custom icons are the `♥` and `♡` characters in a `span` (via `on_icon_template`/`off_icon_template`). `basic` has an `echo` of the value for e2e.

### D10. Tests
SSR: structure, star count, classes and `data-p`, `checked`, labels, the `name`/`required`/`readonly`/`disabled` attributes, icons and slots, `invalid`, `pt`, parity of static and reactive properties. Browser (`tests/web.rs`): select and reset by click, `readonly`/`disabled`, one `on_rate` per action, radio button switching (`input.click()`) without a reset, external signal, reactive `stars`, `p-focus-visible` and focus, keeping the star elements and focus after a selection. The crate with `tests/web.rs` is added to the browser job of `ci.yml` (3 places). e2e additionally checks the keyboard arrows.

## Risks / Trade-offs

- [Five stars with four nodes and several closures each cost more than one button] → budget by measurement; if exceeded, collapse the star's closures (a shared `rx` per star).
- [Divergence from PrimeNG on arrows (D2) and in determining visible focus (D4)] → recorded in `docs/rating.md`; the spec describes predictable behaviour.
- [Extracting `next_unique` affects `collapsible`] → a purely mechanical move, the existing `prime-core` and component tests (Panel, Fieldset, Accordion) must stay green without edits.

## Migration Plan

No migration: a new crate and an internal helper. Rollback is `git revert`.

## Open Questions

None.
