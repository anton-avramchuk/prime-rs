# Theme

`prime-theme` contains the static CSS of the Aura theme. It is not generated at runtime and does not require Node at build time:
the files live in `crates/prime-theme/css/` and are included via `include_str!`.

```rust
let css = prime_theme::stylesheet(&["button", "badge"]); // base CSS + CSS of the selected components
```

- `BASE`: the `--p-*` variables (palettes, semantic tokens, light and dark scheme), global styles and `.p-hidden-accessible`.
- The dark scheme is enabled by the `DARK_MODE_SELECTOR` selector (`.p-dark`) on the root element.
- A component's CSS is available if its feature is enabled (`button`, `badge`, ..., `all-components`); other names are skipped.

## Regeneration

```sh
cargo xtask gen-theme
```

Node and npm are required. The command installs `@primeuix/themes`, `@primeuix/styles`, `@primeuix/styled` and `@primeuix/utils` at the versions from `upstream.toml`
into a cache outside the repository (`~/.cache/prime-rs`, overridable with `PRIME_RS_CACHE`) without running package scripts, then `xtask/theme/extract.mjs`
builds the CSS with the same code as PrimeNG (`Theme.getCommon`, `Theme.getComponent`, `dt()` resolution). The result is deterministic.

The component list is set in `upstream.toml` (`[theme] components`): a new component is added to the list together with its port.
Size: `base.css` is about 19 KB, a component's CSS is 3 to 35 KB (minified).
