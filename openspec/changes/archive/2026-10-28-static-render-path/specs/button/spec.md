## ADDED Requirements

### Requirement: Static render path of Button
If all properties of `Button` are static, it SHALL be rendered with ready values of the attributes and blocks (icon, label, badge), without reactive closures and `RenderEffect` on the attributes and blocks; if at least one property is passed as a signal, the reactive path SHALL be used. Both paths SHALL produce the same markup and behaviour (events, `pt`, ripple, autofocus), and the markup SHALL be described once.

#### Scenario: Identical markup and behaviour
- **WHEN** the same set of values is passed statically and through signals
- **THEN** the HTML matches, and the click, blocking and focus work in the same way

#### Scenario: Memory of a static button
- **WHEN** 1000 static buttons with a label and a severity are mounted in the browser
- **THEN** the growth of wasm linear memory is noticeably lower than on the reactive path, and it is recorded in `docs/benchmarks.md` (the goal is no more than 3 KB per button against 6.7 KB)

#### Scenario: Reactive button
- **WHEN** `loading` or another property is passed as a signal and changes
- **THEN** the class, attributes, blocking and the indicator are updated, as before
