# CI

Workflow files live in `.github/workflows/`. They cannot be run locally as a whole: the logic for selecting crates, budgets and comparing
benchmarks is moved to `cargo xtask` and checked by tests; the YAML files themselves are checked on GitHub.

## `ci.yml` — every PR and push to `dev`

| Job | What it does |
|---|---|
| `changes` | `cargo xtask affected --base <base>`: the list of affected crates |
| `fmt` | `cargo fmt --all --check` |
| `deny` | `cargo deny check` (`deny.toml`) |
| `libs` | `clippy -D warnings`, `cargo nextest`, doc tests for the affected libraries, `--all-features` |
| `wasm` | `wasm-bindgen-test` for `prime-dom` in headless Chrome (on a full run or when `prime-dom` changes) |
| `apps` | showcase (clippy for ssr and hydrate), the wasm size budget, benchmarks, comparison with the baseline and a comment in the PR |

Rules for selecting crates (`xtask/src/affected.rs`):
- if `prime-rs-core` or `prime-dom`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/`, `.github/`, `xtask/`, `deny.toml`, `rustfmt.toml`,
  `benches/budgets.toml` changed → the whole workspace is checked;
- otherwise the changed crates and all crates depending on them (transitively);
- changes only in documentation and `openspec/` run nothing except `fmt` and `deny`.

e2e does not run on a PR. A benchmark regression of more than 10 % is reported with a comment and does not block the merge;
exceeding the wasm size budget (`benches/budgets.toml`) fails the `apps` job.

Cache: `Swatinem/rust-cache`, saved only on `dev`; before finishing, the job `cargo sweep --file` removes artifacts that were not used in this run, and `CACHE_LIMIT_MB` (6000) is the size limit of `target/`: if it is exceeded, the job fails.
The benchmark baseline is the artifact `bench-baseline` (30 days) of the last successful `dev` run.

## `weekly.yml` — once a week, manually, and on PRs labelled `run-e2e`

Clones PrimeNG at the commit from `upstream.toml`, builds its showcase and ours, and runs Playwright (`e2e/`):
`smoke` (hydration, the scheme switch), `memory` (time and memory for 1000 components, a creation and removal loop),
`visual` (comparison of the pages from `e2e/tests/pages.json` with the original). The reference screenshots are made on the fly and are not stored in the repository.
The job is marked `continue-on-error`: differences do not block PRs. The artifacts (`e2e/results`) are kept for 7 days.

Locally: start the showcase (`cd showcase && cargo leptos serve`), then `cd e2e && npm install && PW_CHANNEL=chrome npx playwright test`.
`UPSTREAM_URL` enables the comparison with PrimeNG.

## `machete.yml` — once a week

`cargo machete`: searches for unused dependencies.

## Verified on GitHub

- `ci.yml`: all jobs are green on a push to `dev`; on a PR with an artificial regression (+53 % allocations) one comment appeared in the PR
  (repeated runs update it), regressions are marked ⚠️, the job does not fail and the merge is not blocked.
- `weekly.yml`: a manual run (`workflow_dispatch`) built the PrimeNG showcase at the pinned commit and ours, the smoke and memory measurement passed
  (≈ 7 minutes), the artifact `e2e-results` was uploaded. The screenshot comparison is still empty: `e2e/tests/pages.json` has no pages, the components do not exist yet.
- The render time on shared runners is noisy (±30 % between runs even for untouched scenarios), so the reliable signals are allocations and the wasm size.
  Before enabling `--fail-over`, the time should be smoothed (several runs) or not taken into account.
