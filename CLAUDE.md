# CLAUDE.md

## Language rule

**Use English everywhere in this repository.** This applies to:

- source code comments and doc comments (`//`, `///`, `//!`);
- string literals, panic/error messages, log output, CLI output, UI text in the showcase;
- identifiers, test names, test descriptions and test fixtures;
- commit messages and PR titles/descriptions;
- all Markdown files (`README.md`, `docs/`, `openspec/` specs, proposals, designs, tasks, archived changes);
- config files (`Cargo.toml` descriptions, CI workflows, `*.toml`, `*.yml`) and snapshots.

Do not introduce Cyrillic (or any non-English text) into tracked files. Chat replies to the user may follow the user's own language preference, but everything written into the repo must be English.

Quick check for violations:

```bash
git grep -nP '[\x{0400}-\x{04FF}]'
```

The command must print nothing.
