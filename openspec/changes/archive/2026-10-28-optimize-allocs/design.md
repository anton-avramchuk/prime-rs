# Design

## Context

`ClassNames::new()` reserves a `String` of 96 bytes, each `finish()` hands it out; `data_p()` builds another; `merge_class` a third. The Leptos floor for `div`+`span` is 2 allocations per instance (`floor-1000`), a static `AvatarGroup` is 6, of which 2 are the class string and `Arc<Pt>`.

## Goals / Non-Goals

**Goals:** remove allocations for class and `data-p` strings in typical states; remove `Arc`s not needed by the static branch; HTML does not change.
**Non-Goals:** public API, wasm size, the reactive branch (except the shared string assembly).

## Decisions

### D1. Stack buffer in `ClassNames`
A 192-byte buffer on the stack; on overflow it switches to `String`. `finish()` still returns a `String` (one allocation, as before, but without an intermediate one).

### D2. Interning
`finish_cow()`: if the classes were added only via `add`/`add_if` (component constants), the string is looked up in a global `HashSet<&'static str>` (behind a `Mutex`) and on a miss is "leaked" once (`Box::leak`). The table is limited to 4096 entries — beyond the limit a `String` is returned. If `add_opt` with `Some` was used (user `style_class`, `pt`), interning is not applied: the values are unbounded.
*Alternative*: `&'static str` tables by flag combinations — dozens of combinations per component, brittle maintenance.

### D3. `data-p`
The same interning: `DataP::finish_cow()`; the `data_p(&state)` functions return `Cow<'static, str>`.

### D4. `Arc` in the static branch
`Arc<Props>`/`Arc<Pt>` are needed by the closures of the reactive branch and by event handlers. Where the static branch does not have them, they are created only in the reactive one.

### D5. Verification
Measurements before and after — `cargo bench -p render-bench --bench alloc`; new budgets — with ≈10% headroom over the measurement. Branch parity and HTML are checked by the existing tests.

## Risks / Trade-offs

- [The leaking table grows] → limited to 4096 entries, keys are only constant combinations.
- [`Mutex` in multithreaded SSR] → the critical section is a hash table lookup; acceptable under contention; there is no locking on wasm.

## Migration Plan

Internal refactoring without an API change. Rollback — `git revert`.

## Open Questions

None.
