# Proposal

## Why

ImageCompare is the first component of wave 4: a comparison of two images with a slider (cropping the upper one by `clip-path`). It is the smallest of the remaining components (about 150 lines in PrimeNG), it does not depend on other crates, and it allows wave 4 to start with a light step.

## What Changes

- A new crate `prime-imagecompare` (a port of `p-imagecompare` from PrimeNG 21.1.10).
- `ImageCompare`: the root `div.p-imagecompare` > the left view (`left`, usually an `<img>`) > the right view (`right`, usually an `<img>`) > `input[type=range].p-imagecompare-slider` (`min=0`, `max=100`, `value=50`).
- Properties: `left` and `right` (`ViewFn`), `tabindex`, `aria_label`, `aria_labelledby` (on the root), `style`, `style_class`, `pt` (`root`, `slider`).
- Behaviour: input into the slider sets `clip-path: polygon(0 0, V% 0, V% 100%, 0 100%)` on the element directly before the slider (the right image); in a container with `dir="rtl"` — `polygon((100−V)% 0, 100% 0, 100% 100%, (100−V)% 100%)`. Before the first input the theme rule applies (`50%`).
- The showcase page `/imagecompare` with the demos `basic` and `responsive`; e2e; benchmarks and budgets; `docs/imagecompare.md`.
- Styles: `imagecompare` is added to `theme.components`.
- **Not included:** a `MutationObserver` watching the `dir` attribute (the direction is computed on input — the result is the same, because in PrimeNG it is also applied only on input), `Bind`, `pTemplate`, the `accessibility` demo (text).

## Capabilities

### New Capabilities
- `imagecompare`: a comparison of two images with a slider.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-imagecompare` (`publish = false`); dependencies: `prime-rs-core`, `web-sys`.
- Changes to `Cargo.toml` of the workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (the browser job, three places).
