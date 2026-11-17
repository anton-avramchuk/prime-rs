# Design

## Context

The requirements are in `specs/progressspinner`. PrimeNG 21.1.10 sources: `packages/primeng/src/progressspinner/progressspinner.ts`, `progressspinner/style/progressspinnerstyle.ts`, demos `apps/showcase/doc/progressspinner` (`basic`, `custom`). The theme CSS will be obtained through `cargo xtask gen-theme`; the `/* For PrimeNG */` block is absent from `progressspinnerstyle.ts`. The crate template is `prime-toolbar` (two branches, `macro_rules!`, `Props`).

## Decisions

### D1. Attribute values are strings
`stroke_width`, `fill` and `animation_duration` are `Prop<String>` with the same default values as in Angular (`"2"`, `"none"`, `"2s"`): the values go into the SVG/CSS as they are and match the input of the demos (`strokeWidth="8"`, `animationDuration=".5s"`). Numbers are not used, so as not to lose forms like `.5s`.

### D2. Branches
The static branch — when all properties are static; the markup is in a `macro_rules!`, only the attribute expressions change. `animation-duration` is assembled into the `style` string of the `svg` (`animation-duration: 2s`); `pt.spin` adds its own style through `merge_style`.

### D3. Root attributes
`role="progressbar"` and `aria-busy="true"` are static; `aria-label` is `OptProp<String>`, not rendered when `None`. There is no `data-p`: `progressspinnerstyle.ts` does not set it.

### D4. SVG in `view!`
The `svg` and `circle` elements from `leptos::svg` with the attributes `viewBox`, `cx`, `cy`, `r`, `stroke-miterlimit`. It is checked that the SSR markup matches the expected string (the name `viewBox` with a capital letter), and that on hydration the nodes match (e2e).

## Risks / Trade-offs

- [The case of `viewBox` during hydration] → an SSR test for the substring `viewBox=`, e2e for the absence of hydration errors in the console.
- [`pt.spin` and `animation-duration` share the `style`] → the order: the base values, then pt (as in the other components).

## Migration Plan

No migration: a new crate. Rollback — `git revert`.

## Open Questions

None.
