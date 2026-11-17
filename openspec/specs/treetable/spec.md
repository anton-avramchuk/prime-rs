# treetable Specification

## Purpose
TreeTable is a hierarchical table with expansion, sorting, a paginator and selection.

## Requirements

### Requirement: Tree rows
Visible rows SHALL be built by a depth-first traversal: roots and descendants of expanded nodes (expansion is stored by keys). A row SHALL have a level, a position among siblings and a sibling count (`aria-level`, `aria-posinset`, `aria-setsize`), `aria-expanded` only on expandable nodes (with children or `leaf = false`). The `p-treetable-toggler` button SHALL be rendered in the `toggler_column` column, be visible only on expandable nodes and have an indent `level × 16px`.

#### Scenario: Expanding
- **WHEN** the "Applications" node button is clicked
- **THEN** its children are shown right after it, `aria-expanded = true`

#### Scenario: Leaf
- **WHEN** a node has no children
- **THEN** the button is hidden (`visibility: hidden`), `aria-expanded` is absent

### Requirement: Sorting and pages
Sorting SHALL apply to siblings of every level (stable, single and multi-level, as in Table) and return to the first page. The paginator SHALL count root nodes: the page `[first, first + rows)` of roots together with their visible descendants.

#### Scenario: Descending
- **WHEN** sorting by size descending
- **THEN** roots and the children of every expanded node go in descending size order

#### Scenario: Root page
- **WHEN** five roots, `rows = 2`, going to the second page
- **THEN** the third and fourth roots are shown with their expanded descendants

### Requirement: Selection
A click on a row SHALL select the node in `single` and `multiple` modes (`meta_key_selection` as in Table); a click on the expand button or the checkbox SHALL NOT select the row. In `checkbox` mode the node checkbox SHALL select the node and all descendants, and the parent only when all its children are selected; unselecting SHALL unselect descendants and all ancestors. The node checkbox SHALL be checked for a selected node and show a partial state if some descendants are selected.

#### Scenario: Propagation
- **WHEN** the only child of the "React" node is selected
- **THEN** the child and "React" are selected, "Applications" is shown as partially selected

#### Scenario: Root
- **WHEN** "Applications" is selected
- **THEN** its whole subtree is selected; a repeated click unselects everything

### Requirement: Markup
`TreeTable` SHALL render `div.p-treetable.p-component` with blocks: loading mask, header, top paginator, table container (`table.p-treetable-table`, `thead`, `tbody`, `tfoot` when there are labels), bottom paginator, footer. The header of a sortable column SHALL have `p-sortable-column`, `aria-sort`, the icon `p-treetable-sort-icon`; when there are no nodes `p-treetable-empty-message` SHALL be shown (if not loading).

#### Scenario: Empty
- **WHEN** there are no nodes
- **THEN** "No results found" is rendered across the full width
