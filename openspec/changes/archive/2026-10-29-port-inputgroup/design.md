# Design

## Context

Requirements are in `specs/inputgroup`. PrimeNG 21.1.10 sources: `packages/primeng/src/inputgroup/inputgroup.ts`, `inputgroup/style/inputgroupstyle.ts`, `inputgroupaddon/inputgroupaddon.ts`, `inputgroupaddon/style/inputgroupaddonstyle.ts`, demos `apps/showcase/doc/inputgroup`. Reference implementation - `crates/prime-toolbar` (`docs/toolbar.md`): static and reactive branches, markup `macro_rules!`, `pt` by parts. There is no shared logic or client code.

## Decisions

### D1. One crate, two components
`InputGroup` and `InputGroupAddon` live in `prime-inputgroup`: the addon is not used on its own, and they share the theme CSS (`@primeuix/styles/inputgroup`).

### D2. Branches
Static - when `style_class` and `style` are static. Reactive - closures over `Arc<Props>` and `Arc<...Pt>` only for the mutable root attributes; `children` are built once. The markup is a shared `macro_rules!`.

### D3. Classes and attributes
`InputGroup`: `p-inputgroup` + `style_class` + the class from `pt.root`. `InputGroupAddon`: `p-inputgroupaddon` + the same. Both have `style` (in PrimeNG `InputGroup` does not have it, but it is harmless and consistent; record the difference in the docs). The `fluid` property is not ported: in PrimeNG 21.1.10 `InputGroup` has no such input, `instance.fluid` is always `undefined`.

### D4. CSS
`theme.components` gets `inputgroup`; the `/*For PrimeNG*/` block is not needed (`primeng_extra` does not change), since the rules concern the Angular wrappers `p-button`, `p-inputwrapper`, `p-inputmask`. The rounding of the outer `Button` and `InputText` inside the group is provided by the rules of `@primeuix/styles/inputgroup`; if the visuals diverge, fix it in the showcase, not in the generator.

### D5. Demos
`basic` (icon, text, addons around `InputText`), `button` (`Button` next to the input), `checkbox` (`Checkbox` in an addon), `multiple` (several addons). `floatlabel` and `iftalabel` are skipped (no components), record the difference in `docs/inputgroup.md`.

### D6. Tests
SSR: structure and order, classes, `style`, `pt`, branch parity, nesting with `InputText`/`Button`. No browser tests: there is no client behaviour.

## Risks / Trade-offs

- [The rounding of outer elements depends on the DOM structure of `Button` and `InputText` in Leptos] -> check with e2e and a screenshot of the `/inputgroup` page; if it diverges, add the missing rule to the showcase CSS and record it in the docs.

## Migration Plan

No migration: a new crate. Rollback is `git revert`.

## Open Questions

None.
