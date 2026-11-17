# Design

## Context

The requirements are in `specs/toolbar`. PrimeNG 21.1.10 sources: `packages/primeng/src/toolbar/toolbar.ts`, `toolbar/style/toolbarstyle.ts`, demos `apps/showcase/doc/toolbar`. The template is `crates/prime-card` (`docs/card.md`): the `ViewFn` slots, the static and reactive branches, the `macro_rules!` markup, `pt` per part. Toolbar has neither shared logic nor client code.

## Decisions

### D1. Slots
`start_template`, `center_template`, `end_template` — `Option<ViewFn>`; an area is drawn only for `Some`. `children` (optional) come before the areas, like `<ng-content>` in the PrimeNG template.

### D2. Branches
Static — when `aria_labelledby`, `style`, `style_class` are static. Reactive — closures over `Arc<Props>` and `Arc<ToolbarPt>`, only for the mutable root attributes; the areas and slots are built once (the slots do not depend on the property signals). The markup — a shared `macro_rules!`.

### D3. Classes and attributes
The root: `p-toolbar p-component` + `style_class` + the class from `pt.root`, `role="toolbar"`, `aria-labelledby`. The order of the areas is fixed. Toolbar has no `data-p` in PrimeNG.

### D4. Demos
`basic` is built from the components available: buttons with icons (`Button`, `text`, `secondary`), an `InputText` with a search icon instead of IconField, a "Save" `Button` instead of SplitButton. `custom` — a logo (SVG), text buttons and an avatar. The differences are recorded in `docs/toolbar.md`.

### D5. Tests
SSR: structure and order, role, empty areas, `children` before the areas, `aria-labelledby`, `style`, `pt`, branch parity. There are no browser tests: there is no client behaviour (the same is true of Card in the project).

## Risks / Trade-offs

- [The `basic` demo differs from PrimeNG in its components] → the difference is recorded in the docs; the screenshot comparison remains a report, not a blocker.

## Migration Plan

No migration: a new crate. Rollback — `git revert`.

## Open Questions

None.
