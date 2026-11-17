# Benchmarks

Benchmarks live in `benches/`: the `render-bench` crate (SSR rendering and allocations) and `wasm-size` (an application for measuring the wasm size).
Reports are written to `target/bench/*.json` in the format `{"metrics": {"<name>": <number>}}`; less is better.

## Render time

```sh
cargo bench -p render-bench --bench render   # divan: detailed statistics per scenario
cargo bench -p render-bench --bench time     # median in target/bench/time.json (for comparison with the baseline)
```

## Allocations

```sh
cargo bench -p render-bench --bench alloc    # dhat; each scenario in a separate process -> target/bench/alloc.json
```

For each scenario the number of allocations, the total size, the peak memory and the HTML size are printed.

## wasm size

```sh
cargo xtask wasm-size                        # target/bench/wasm.json, non-zero exit code when the budget is exceeded
cargo xtask wasm-size --budgets file.toml
```

A component's contribution is the size of the raw `.wasm` (release, without `wasm-opt`) minus `case-baseline` of the `benches/wasm-size` application.
The budgets are in `benches/budgets.toml`. A new component adds `case-<name>` to `benches/wasm-size` and a line to the budgets.

## Allocation budget

```sh
cargo bench -p render-bench --bench alloc   # target/bench/alloc.json
cargo xtask alloc-budget                    # non-zero exit code when benches/alloc-budgets.toml is exceeded
```

For each `<component>-1000` scenario a maximum number of allocations per instance is set (the number of allocations of the scenario is divided by the number of instances).
A new component adds a scenario to `benches/render/src/lib.rs` and a line to `benches/alloc-budgets.toml`; the check is part of the `apps` job in `ci.yml`.

## Comparison with the baseline

```sh
cargo xtask bench-compare <baseline directory> <current reports directory> [--threshold 10] [--fail-over]
```

Prints a Markdown table for the PR comment. A regression is growth above the threshold (10 % by default). The exit code is 0 until `--fail-over` is given:
blocking is enabled after the baseline has accumulated. Time on shared CI runners is noisy, so it is trusted less than allocations and size.

## A new scenario

Add an entry to `SCENARIOS` (`benches/render/src/lib.rs`): the name `<component>-<number of instances>` and the render function. The scenario immediately gets into all three benchmarks.
Each component needs scenarios for one instance and for 1000 instances.

## Current values

Measured on Windows 11, release (a one-off measurement, not a reference):

| Scenario | Median | Allocations | Peak memory |
|---|---:|---:|---:|
| check-icon-1 | 1.6 µs | 15 | 2.5 KB |
| check-icon-1000 | 1.64 ms | 5 020 | 2.3 MB |
| spinner-icon-1000 | 2.42 ms | 6 020 | 3.3 MB |
| all-icons-1 (54 icons) | 183 µs | 725 | 140 KB |
| all-icons-20 | 3.36 ms | 14 220 | 2.6 MB |

Contribution to the wasm size: one icon +8 176 B, all icons +127 367 B (see `docs/icons.md`).

## Badge and Button (`port-button`)

Measured on Windows 11, release (a one-off measurement).

| Scenario | Median | Allocations | Allocations per instance | Peak memory |
|---|---:|---:|---:|---:|
| badge-1 | 3.5 µs | 51 | 51 | 2.3 KB |
| badge-1000 | 3.45 ms | 41 030 | 41 | 2.1 MB |
| button-1 | 4.9 µs | 83 | 83 | 5.0 KB |
| button-1000 | 5.97 ms | 70 032 | 70 | 4.1 MB |
| button-full-1000 (icon, label, badge) | 12.9 ms | 131 059 | 131 | 8.0 MB |

Contribution to the wasm size (raw `.wasm`, release, without `wasm-opt`) relative to `case-baseline` (an empty `div`):

| Case | Contribution, bytes | Budget, bytes |
|---|---:|---:|
| `case-badge` | 113 015 | 170 000 |
| `case-button` | 379 196 | 570 000 |

Conclusions:
- The budgets are the measured contribution with a margin of ≈ 50 %. The contribution of Badge includes the reactive environment of Leptos, which is absent in the `baseline`, and Button adds to Badge
  the events, `Effect`, `Ripple`, `SpinnerIcon` and `Callback` (≈ 266 KB). For the next components the real increase will be noticeably smaller.
- About 41 allocations per Badge and 70 per Button during SSR go mainly to reactive primitives (`Memo`, closures, `Signal::derive`), which
  are not needed when rendering a string. This is a candidate for optimization (static properties without `Signal`, one `Memo` per component) as a separate change;
  in this change the goal is to fix the baseline.
- The wasm size in a real application is lower: `wasm-opt -Oz` and `opt-level = "z"` compress the binary more than the raw release build.

## Optimization of reactive properties (`optimize-reactive-props`)

Component properties are `prime_core::Prop`/`OptProp` (a value does not create a signal), without `Memo` and with one `Arc<Props>` per instance.

SSR, release, allocations per instance (1000 instances), budgets — `benches/alloc-budgets.toml`:

| Scenario | Before | After | Change | Peak memory before → after |
|---|---:|---:|---:|---:|
| badge-1000 | 41 | 8 | −80 % | 2.2 MB → 0.27 MB |
| button-1000 | 67 | 20 | −70 % | 4.3 MB → 1.9 MB |
| button-full-1000 (icon, label, badge) | 128 | 50 | −61 % | 8.3 MB → 3.3 MB |

### Second pass: `optimize-allocs`

The `class` and `data-p` strings are assembled on the stack and interned for constant combinations (`ClassNames::finish_cow`, `DataP::finish_cow`,
`Pt::merge_class`): a repeated render of the same state does not allocate. User classes (`style_class`, `pt`) are marked
`add_user` and are not interned. The rest: `Arc<Props>`/`Arc<Pt>` are created only in the reactive branch of display components, icons insert
the ready outline markup (`inner_html`, without `collect_view`), `.into_any()` is replaced by `Either`/`EitherOfN`, string formatting goes through `write!`.
Lesson: each `.into_any()` is a separate allocation, and `format!` into a temporary string is one more.

| Scenario | Before | After | Change |
|---|---:|---:|---:|
| badge-1000 | 8 | 5 | −38 % |
| tag-1000 | 11 | 6 | −45 % |
| avatar-1000 | 15 | 9 | −40 % |
| divider-1000 | 22 | 13 | −41 % |
| card-1000 | 29 | 22 | −24 % |
| button-1000 | 23 | 19 | −17 % |
| inputtext-1000 | 13 | 10 | −23 % |
| checkbox-1000 | 34 | 20 | −41 % |
| radiobutton-1000 | 23 | 17 | −26 % |
| toggleswitch-1000 | 33 | 23 | −30 % |
| message-1000 | 36 | 17 | −53 % |
| progressbar-1000 | 36 | 16 | −56 % |
| avatargroup-1000 | 6 | 4 | −33 % |

Third pass (`optimize-allocs-button-chip`): `Arc<Props>`/`Arc<Pt>` only in the reactive branch, `NodeRef` only on `wasm32`, blocks without `.into_any()`.

| Scenario | Before | After | Change |
|---|---:|---:|---:|
| button-1000 | 19 | 9 | −53 % |
| button-full-1000 | 45 | 28 | −38 % |
| chip-1000 | 10 | 6 | −40 % |
| chip-removable-1000 | 35 | 22 | −37 % |

The Leptos floor for a `div` with a `span` in a loop is 2 allocations per instance, `Children` costs another 2 (the closure box and `AnyView`), so these values cannot go much lower without changing the API. Button and Chip were optimized in the third pass (see below).

Wasm size (`cargo xtask wasm-size`, raw release): the Badge contribution 113 015 → 84 320 B (−25 %), Button 379 196 → 326 754 B (−14 %).

What gave the gain (an experiment on `render-bench`, allocations per instance): a "bare" `<span>` — 2, `Pt` — 2, one `Memo` — +9, one signal property — +3.
Attribute closures during SSR cost ≈ 1 allocation more than a ready string (5 against 4), ready `AnyAttribute`s cost 7; therefore a fast path for static attributes during SSR is not needed.

Browser (Chrome, release build of the showcase, 1000 buttons `Button label severity`, scenario `e2e/tests/memory.spec.ts`; the JS heap does not include Rust memory,
so the size of the wasm linear memory is read through `window.wasmMemoryBytes()`):

| Metric | Before | After |
|---|---:|---:|
| wasm memory after mounting 1000 buttons | +9.44 MB | +6.75 MB (−28 %) |
| wasm memory after mounting 1000 icons (unchanged) | — | +0.33 MB |
| DOM nodes for 1000 buttons | 7001 | 7001 |
| JS heap for 1000 buttons | 0.96 MB | 0.96 MB |
| Mounting time for 1000 buttons | 35.7 ms | 38.8 ms (within noise) |

Conclusion for D2: in the browser a button still takes ≈ 6.7 KB of linear memory, an icon 0.33 KB. The difference is a `RenderEffect` for each attribute closure
(a button has about twelve: `class`, `style`, `type`, `aria-label`, `tabindex`, `disabled`, three `data-p*` and the dynamic blocks). For static buttons
these effects are not needed. This is a candidate for a separate change: a static path with ready strings (expressing "a string or a closure" as one value
is not possible in Leptos 0.8, a separate markup branch is needed).

## Static render path (`static-render-path`)

If all properties of a component are static (`all_static!`), it renders the markup with ready values, without closures and `RenderEffect`; otherwise the reactive path is used.
The markup is written once in a `macro_rules!` and substituted into both branches (`Either`); the parity tests compare the HTML of both branches, and the browser tests of Button
run in both branches. The `Effect` for autofocus and ripple is created only when it is needed.

Wasm linear memory per instance in the browser (Chrome, release, 1000 instances, `e2e/tests/memory.spec.ts`):

| Scenario | Before | After | Change |
|---|---:|---:|---:|
| Badge, static properties | 3 408 B | **393 B** | −88 % |
| Badge, value as a signal | 3 473 B | 2 228 B | −36 % |
| **Button, static properties** | 9 437 B | **1 966 B** | **−79 %** |
| Button, `loading` as a signal | — | 7 471 B | — |
| Icon (for comparison) | 328 B | 328 B | — |

Mounting time of 1000 static buttons: 52.6 → 19.5 ms; Badge: 15 → 7 ms.

Cost: SSR allocations per Button instance 23 (was 20), per full button 56 (was 50), Badge unchanged (8); contribution to raw wasm +14 %
(Badge 84 320 → 96 303 B, Button 326 754 → 373 874 B). Budgets: `benches/alloc-budgets.toml` (28 and 68 per button), `benches/budgets.toml` unchanged.

Observation: the growth of wasm memory after 20 cycles of "mount and remove" 1000 reactive buttons (≈ 14 MB; for static ones — 1 MB) comes from the shared external signal
`loading`: its list of subscribers grows at each mount and is cleaned lazily, on the next notification. This is Leptos behaviour, not a leak of the
component; for pages where the signal changes, the growth is limited by this cleaning.
