//! `gen-spec`: заготовка спеки компонента из TypeScript-исходников PrimeNG.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::Path;

/// Что удалось извлечь из исходников компонента.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Extracted {
    pub selectors: Vec<String>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub classes: Vec<String>,
}

fn ident_at_start(s: &str) -> Option<&str> {
    let end = s.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '$')).unwrap_or(s.len());
    (end > 0).then(|| &s[..end])
}

/// Пропускает декоратор `@Name(...)` с учётом вложенных скобок; возвращает остаток строки.
fn skip_decorator<'a>(line: &'a str, name: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(name)?;
    if !rest.starts_with('(') {
        return None;
    }
    let mut depth = 0usize;
    for (i, c) in rest.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(rest[i + 1..].trim());
                }
            }
            _ => {}
        }
    }
    None
}

fn member_name(rest: &str) -> Option<&str> {
    let rest = rest.trim_start();
    let rest = ["get ", "set ", "public ", "readonly "].iter().fold(rest, |r, p| r.strip_prefix(p).unwrap_or(r));
    ident_at_start(rest)
}

/// Разбирает файл компонента: селекторы, входы и выходы.
pub fn parse_component(source: &str) -> Extracted {
    let mut out = Extracted::default();
    let mut inputs = BTreeSet::new();
    let mut outputs = BTreeSet::new();
    let lines: Vec<&str> = source.lines().map(str::trim).collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if let Some(rest) = line.strip_prefix("selector:") {
            let sel = rest.trim().trim_end_matches(',').trim_matches(['\'', '"', '`']);
            if !sel.is_empty() && !out.selectors.contains(&sel.to_owned()) {
                out.selectors.push(sel.to_owned());
            }
        }
        for (deco, set) in [("@Input", &mut inputs), ("@Output", &mut outputs)] {
            if let Some(rest) = skip_decorator(line, deco) {
                let rest = if rest.is_empty() { lines.get(i + 1).copied().unwrap_or("") } else { rest };
                if let Some(n) = member_name(rest) {
                    set.insert(n.to_owned());
                }
            }
        }
        if let Some((name, rhs)) = line.split_once('=') {
            let name = name.trim();
            let rhs = rhs.trim_start();
            let is_ident = ident_at_start(name) == Some(name);
            if is_ident
                && ["input", "model"].iter().any(|f| {
                    rhs.starts_with(&format!("{f}(")) || rhs.starts_with(&format!("{f}<")) || rhs.starts_with(&format!("{f}.required"))
                })
            {
                inputs.insert(name.to_owned());
            }
            if is_ident && (rhs.starts_with("output(") || rhs.starts_with("output<")) {
                outputs.insert(name.to_owned());
            }
        }
        i += 1;
    }
    out.inputs = inputs.into_iter().collect();
    out.outputs = outputs.into_iter().collect();
    out
}

/// Собирает имена классов `p-*` из блока `const classes = { ... }` файла стилей.
pub fn parse_classes(style_source: &str) -> Vec<String> {
    let Some(start) = style_source.find("const classes") else {
        return Vec::new();
    };
    let body = &style_source[start..];
    let end = body.find("\n};").map_or(body.len(), |e| e + 3);
    let body = &body[..end];
    let mut set = BTreeSet::new();
    let mut parts = body.split(['\'', '"', '`']);
    parts.next();
    for (idx, part) in parts.enumerate() {
        if idx % 2 == 0 {
            for token in part.split_whitespace() {
                let token = token.trim_matches(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '_'));
                if token.starts_with("p-") && !token.contains("${") {
                    set.insert(token.to_owned());
                }
            }
        }
    }
    set.into_iter().collect()
}

fn list(items: &[String], empty: &str) -> String {
    if items.is_empty() {
        return format!("- {empty}\n");
    }
    items.iter().fold(String::new(), |mut s, i| {
        let _ = writeln!(s, "- `{i}`");
        s
    })
}

/// Формирует Markdown-заготовку спеки.
pub fn render(name: &str, commit: &str, version: &str, e: &Extracted) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "<!-- Заготовка создана `cargo xtask gen-spec {name}`; довести вручную. -->");
    let _ = writeln!(s, "<!-- Источник: packages/primeng/src/{name}/{name}.ts, PrimeNG {version}, коммит {commit}. -->\n");
    let _ = writeln!(s, "## ADDED Requirements\n");
    let _ = writeln!(
        s,
        "### Requirement: Props\nКомпонент `prime-{name}` SHALL принимать те же свойства, что и PrimeNG (имена приведены к snake_case Rust).\n"
    );
    let _ = writeln!(s, "Селекторы PrimeNG:\n{}", list(&e.selectors, "не найдены"));
    let _ = writeln!(s, "Входы PrimeNG:\n{}", list(&e.inputs, "не найдены"));
    let _ = writeln!(s, "#### Scenario: TODO\n- **WHEN** TODO\n- **THEN** TODO\n");
    let _ = writeln!(s, "### Requirement: События\nКомпонент SHALL вызывать те же события, что и PrimeNG.\n");
    let _ = writeln!(s, "Выходы PrimeNG:\n{}", list(&e.outputs, "не найдены"));
    let _ = writeln!(s, "#### Scenario: TODO\n- **WHEN** TODO\n- **THEN** TODO\n");
    let _ = writeln!(s, "### Requirement: DOM и классы\nРазметка SHALL совпадать с PrimeNG и использовать те же классы.\n");
    let _ = writeln!(s, "Классы PrimeNG:\n{}", list(&e.classes, "не найдены"));
    let _ = writeln!(s, "#### Scenario: TODO\n- **WHEN** TODO\n- **THEN** TODO\n");
    let _ = writeln!(
        s,
        "### Requirement: Доступность\nКомпонент SHALL выставлять те же ARIA-атрибуты и роли, что и PrimeNG.\n\n#### Scenario: TODO\n- **WHEN** TODO\n- **THEN** TODO\n"
    );
    let _ = writeln!(
        s,
        "### Requirement: Клавиатура\nКомпонент SHALL поддерживать те же клавиши, что и PrimeNG.\n\n#### Scenario: TODO\n- **WHEN** TODO\n- **THEN** TODO\n"
    );
    let _ = writeln!(
        s,
        "### Requirement: Производительность\nРендер компонента SHALL укладываться в бюджеты `benches/budgets.toml`.\n\n#### Scenario: TODO\n- **WHEN** TODO\n- **THEN** TODO"
    );
    s
}

/// Читает исходники компонента из `src_dir` и возвращает заготовку.
pub fn generate(src_dir: &Path, name: &str, commit: &str, version: &str) -> Result<String, String> {
    let dir = src_dir.join(name);
    let main = dir.join(format!("{name}.ts"));
    let source = std::fs::read_to_string(&main).map_err(|_| format!("компонент `{name}` не найден: нет файла {}", main.display()))?;
    let mut extracted = parse_component(&source);
    if let Ok(style) = std::fs::read_to_string(dir.join("style").join(format!("{name}style.ts"))) {
        extracted.classes = parse_classes(&style);
    }
    Ok(render(name, commit, version, &extracted))
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMPONENT: &str = r#"
@Component({
    selector: 'p-badge',
    template: `<span></span>`
})
export class Badge {
    @Input({ transform: booleanAttribute }) text: boolean = false;
    @Input() size: 'small' | 'large' | undefined;
    @Input()
    get label(): string | undefined { return this._label; }
    fluid = input(undefined, { transform: booleanAttribute });
    value = model<number>();
    @Output() onClick: EventEmitter<MouseEvent> = new EventEmitter();
    closed = output<void>();
}
"#;

    const STYLE: &str = r#"
const classes = {
    root: ({ instance }) => [
        'p-badge p-component',
        { 'p-badge-sm': instance.size === 'small', [`p-badge-${instance.severity}`]: true }
    ],
    label: 'p-badge-label'
};

export class BadgeStyle {}
const other = 'p-ignored';
"#;

    #[test]
    fn parses_component() {
        let e = parse_component(COMPONENT);
        assert_eq!(e.selectors, vec!["p-badge"]);
        assert_eq!(e.inputs, vec!["fluid", "label", "size", "text", "value"]);
        assert_eq!(e.outputs, vec!["closed", "onClick"]);
    }

    #[test]
    fn parses_classes() {
        assert_eq!(parse_classes(STYLE), vec!["p-badge", "p-badge-label", "p-badge-sm", "p-component"]);
    }

    #[test]
    fn renders_snapshot() {
        let mut e = parse_component(COMPONENT);
        e.classes = parse_classes(STYLE);
        insta::assert_snapshot!(render("badge", "fe82b291d6", "21.1.10", &e));
    }

    #[test]
    fn unknown_component_is_error_and_creates_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let err = generate(dir.path(), "nope", "c", "v").unwrap_err();
        assert!(err.contains("nope"));
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
    }
}
