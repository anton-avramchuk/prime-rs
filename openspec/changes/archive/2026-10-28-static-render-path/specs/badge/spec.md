## ADDED Requirements

### Requirement: Static render path of Badge
If all properties of `Badge` are static, it SHALL be rendered with ready values of the attributes and the text, without reactive closures and `RenderEffect` in the browser; if at least one property is passed as a signal, the reactive path SHALL be used. Both paths SHALL produce the same markup for the same values, and the markup SHALL be described once.

#### Scenario: Identical markup
- **WHEN** the same set of values is passed statically and through signals
- **THEN** the HTML of both variants matches

#### Scenario: Static Badge in the browser
- **WHEN** 1000 static `Badge` instances are mounted in the browser
- **THEN** the growth of wasm linear memory is noticeably lower than on the reactive path (the values are recorded in `docs/benchmarks.md`), and no `RenderEffect` is created for the attributes

#### Scenario: Reactive Badge
- **WHEN** at least one property of `Badge` is passed as a signal and changes
- **THEN** the text, the classes and `data-p` are updated
