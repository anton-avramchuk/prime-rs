//! `cargo xtask gen-docs`: данные сайта документации (`showcase/src/generated.rs`) из исходников.
//!
//! Для каждой страницы `showcase/src/pages/<имя>.rs` собираются: код демонстраций (компонент, на который ссылается
//! `<Demo>`), строки `use`, свойства и события компонентов крейта `crates/prime-<имя>`, части Pass Through,
//! CSS-классы темы, заметки из `docs/<имя>.md`. Разбор — текстовый: исходники отформатированы `rustfmt`.

use std::fmt::Write as _;
use std::path::Path;

/// Демонстрация страницы.
#[derive(Debug, Eq, PartialEq)]
pub struct DemoRef {
    pub name: String,
    pub title: String,
    pub component: String,
    /// Содержимое элемента `<Demo>` (на случай демо без отдельного компонента).
    pub inner: String,
}

/// Вид параметра компонента.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    Prop,
    Event,
    Pt,
}

/// Параметр `#[component]`.
#[derive(Debug, Eq, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: String,
    pub default: Option<String>,
    pub required: bool,
    pub doc: String,
    pub kind: Kind,
}

/// Компонент крейта.
#[derive(Debug, Eq, PartialEq)]
pub struct Comp {
    pub name: String,
    pub params: Vec<Param>,
}

/// Структура Pass Through и её части.
#[derive(Debug, Eq, PartialEq)]
pub struct PtStruct {
    pub name: String,
    pub fields: Vec<(String, String)>,
}

/// Находит `<Demo name=".." title="..">` и компонент внутри.
pub fn parse_page_demos(src: &str) -> Vec<DemoRef> {
    let mut out = Vec::new();
    let mut rest = src;
    while let Some(at) = rest.find("<Demo") {
        let after = &rest[at + 5..];
        let Some(end) = after.find('>') else { break };
        let attrs = &after[..end];
        let body = &after[end + 1..];
        let component = body
            .trim_start()
            .strip_prefix('<')
            .map(|t| t.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect::<String>())
            .unwrap_or_default();
        if let (Some(name), Some(title)) = (attr(attrs, "name"), attr(attrs, "title")) {
            let inner = body.find("</Demo>").map_or("", |e| &body[..e]).to_owned();
            out.push(DemoRef { name, title, component, inner });
        }
        rest = body;
    }
    out
}

fn attr(attrs: &str, key: &str) -> Option<String> {
    let pat = format!("{key}=\"");
    let at = attrs.find(&pat)? + pat.len();
    let end = attrs[at..].find('"')?;
    Some(attrs[at..at + end].to_owned())
}

/// Текст `#[component] fn Имя` вместе с атрибутами и `///` до закрывающей скобки на нулевом отступе.
pub fn extract_fn(src: &str, name: &str) -> Option<String> {
    let needle = [format!("\nfn {name}("), format!("\npub fn {name}(")];
    let at = needle.iter().filter_map(|n| src.find(n.as_str())).min()? + 1;
    let mut start = at;
    // поднимаемся к атрибутам и документации над функцией
    let head = &src[..at];
    let mut lines: Vec<&str> = head.lines().collect();
    while let Some(last) = lines.last() {
        if last.starts_with("#[") || last.starts_with("///") {
            start -= last.len() + 1;
            lines.pop();
        } else {
            break;
        }
    }
    let end = src[at..].find("\n}\n").map(|e| at + e + 2)?;
    Some(src[start..end].to_owned())
}

/// Код демо без отдельного компонента: содержимое `<Demo>` как `view!`, с общим отступом снят.
pub fn inline_code(inner: &str) -> String {
    let lines: Vec<&str> = inner.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.is_empty() {
        return String::new();
    }
    let indent = lines.iter().map(|l| l.len() - l.trim_start().len()).min().unwrap_or(0);
    let body: Vec<String> = lines.iter().map(|l| format!("    {}", &l[indent..])).collect();
    format!(
        "view! {{
{}
}}",
        body.join(
            "
"
        )
    )
}

/// Операторы `use prime_…;` страницы.
pub fn page_imports(src: &str) -> String {
    let mut out = Vec::new();
    let mut rest = src;
    while let Some(at) = rest.find("use prime_") {
        let at_line_start = at == 0 || rest.as_bytes()[at - 1] == b'\n';
        let Some(end) = rest[at..].find(';') else { break };
        if at_line_start {
            out.push(rest[at..at + end + 1].to_owned());
        }
        rest = &rest[at + end + 1..];
    }
    out.join("\n")
}

/// Первый абзац `//!`-документации крейта, одной строкой.
pub fn crate_description(lib: &str) -> String {
    let mut parts = Vec::new();
    for line in lib.lines() {
        let Some(t) = line.strip_prefix("//!") else { break };
        let t = t.trim();
        if t.is_empty() {
            if parts.is_empty() {
                continue;
            }
            break;
        }
        parts.push(t.trim_start_matches('#').trim().to_owned());
    }
    parts.join(" ")
}

fn depth_delta(text: &str) -> i32 {
    let mut depth = 0;
    let mut in_str = false;
    let mut prev = ' ';
    for c in text.chars() {
        if in_str {
            if c == '"' && prev != '\\' {
                in_str = false;
            }
        } else {
            match c {
                '"' => in_str = true,
                '(' | '[' | '{' | '<' => depth += 1,
                ')' | ']' | '}' => depth -= 1,
                '>' if prev != '-' => depth -= 1,
                _ => {}
            }
        }
        prev = c;
    }
    depth
}

/// Содержимое скобок после `from` (первая `(` и её пара), без внешних скобок.
fn paren_body(text: &str, from: usize) -> Option<&str> {
    let open = from + text[from..].find('(')?;
    let mut depth = 0;
    let mut in_str = false;
    let mut prev = ' ';
    for (i, c) in text[open..].char_indices() {
        if in_str {
            if c == '"' && prev != '\\' {
                in_str = false;
            }
        } else {
            match c {
                '"' => in_str = true,
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(&text[open + 1..open + i]);
                    }
                }
                _ => {}
            }
        }
        prev = c;
    }
    None
}

fn default_of(attr: &str) -> Option<String> {
    let at = attr.find("default =")? + "default =".len();
    let rest = &attr[at..];
    let mut depth = 0;
    let mut in_str = false;
    let mut prev = ' ';
    let mut end = rest.len();
    for (i, c) in rest.char_indices() {
        if in_str {
            if c == '"' && prev != '\\' {
                in_str = false;
            }
        } else {
            match c {
                '"' => in_str = true,
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => {
                    if depth == 0 {
                        end = i;
                        break;
                    }
                    depth -= 1;
                }
                ',' if depth == 0 => {
                    end = i;
                    break;
                }
                _ => {}
            }
        }
        prev = c;
    }
    let mut value = rest[..end].trim().to_owned();
    for wrapper in ["Prop::Static(", "OptProp::Static("] {
        if let Some(inner) = value.strip_prefix(wrapper).and_then(|v| v.strip_suffix(')')) {
            value = inner.to_owned();
        }
    }
    Some(value)
}

fn parse_params(body: &str) -> Vec<Param> {
    let mut out = Vec::new();
    let (mut docs, mut attrs, mut decl): (Vec<String>, String, String) = (Vec::new(), String::new(), String::new());
    let mut finish = |docs: &mut Vec<String>, attrs: &mut String, decl: &mut String| {
        let d = decl.trim().trim_end_matches(',').trim().to_owned();
        if let Some((name, ty)) = d.split_once(':') {
            let name = name.trim().to_owned();
            if name != "children" {
                let kind = if name == "pt" {
                    Kind::Pt
                } else if name.starts_with("on_") {
                    Kind::Event
                } else {
                    Kind::Prop
                };
                out.push(Param {
                    required: !attrs.contains("optional") && kind == Kind::Prop,
                    default: default_of(attrs),
                    doc: docs.join(" "),
                    ty: ty.split_whitespace().collect::<Vec<_>>().join(" "),
                    name,
                    kind,
                });
            }
        }
        docs.clear();
        attrs.clear();
        decl.clear();
    };
    let mut attr_depth = 0;
    for line in body.lines() {
        let t = line.trim();
        if !decl.is_empty() {
            decl.push(' ');
            decl.push_str(t);
            if depth_delta(&decl) == 0 && t.ends_with(',') {
                finish(&mut docs, &mut attrs, &mut decl);
            }
        } else if attr_depth > 0 {
            attrs.push(' ');
            attrs.push_str(t);
            attr_depth += depth_delta(t);
        } else if let Some(d) = t.strip_prefix("///") {
            docs.push(d.trim().to_owned());
        } else if t.starts_with("#[") {
            attrs.push_str(t);
            attr_depth = depth_delta(t).max(0);
        } else if !t.is_empty() && !t.starts_with("//") {
            decl.push_str(t);
            if depth_delta(&decl) == 0 && t.ends_with(',') {
                finish(&mut docs, &mut attrs, &mut decl);
            }
        }
    }
    if !decl.is_empty() {
        finish(&mut docs, &mut attrs, &mut decl);
    }
    out
}

/// Все `#[component]` исходника с параметрами.
pub fn parse_components(src: &str) -> Vec<Comp> {
    let mut out = Vec::new();
    for chunk in src.split("#[component]").skip(1) {
        let Some(fi) = chunk.find("fn ") else { continue };
        let name: String = chunk[fi + 3..].chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
        if let Some(body) = paren_body(chunk, fi) {
            out.push(Comp { name, params: parse_params(body) });
        }
    }
    out
}

/// Структуры `*Pt` с полями.
pub fn parse_pt(src: &str) -> Vec<PtStruct> {
    let mut out = Vec::new();
    for chunk in src.split("pub struct ").skip(1) {
        let name: String = chunk.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
        if !name.ends_with("Pt") || name == "Pt" {
            continue;
        }
        let Some(open) = chunk.find('{') else { continue };
        let Some(close) = chunk[open..].find("\n}") else { continue };
        let mut fields = Vec::new();
        let mut doc = String::new();
        for line in chunk[open + 1..open + close].lines() {
            let t = line.trim();
            if let Some(d) = t.strip_prefix("///") {
                doc = d.trim().to_owned();
            } else if let Some((n, _)) = t.strip_prefix("pub ").and_then(|f| f.split_once(':')) {
                fields.push((n.trim().to_owned(), std::mem::take(&mut doc)));
            }
        }
        out.push(PtStruct { name, fields });
    }
    out
}

/// Классы `p-…` из CSS.
pub fn css_classes(css: &str) -> Vec<String> {
    let mut set = std::collections::BTreeSet::new();
    let mut i = 0;
    while let Some(at) = css[i..].find(".p-") {
        let start = i + at + 1;
        let end = start + css[start..].bytes().take_while(|b| b.is_ascii_alphanumeric() || *b == b'-' || *b == b'_').count();
        set.insert(css[start..end].to_owned());
        i = end;
    }
    set.into_iter().collect()
}

fn esc(c: char, out: &mut String) {
    match c {
        '&' => out.push_str("&amp;"),
        '<' => out.push_str("&lt;"),
        '>' => out.push_str("&gt;"),
        _ => out.push(c),
    }
}

const KEYWORDS: &[&str] = &[
    "fn", "let", "mut", "use", "pub", "struct", "enum", "impl", "move", "if", "else", "match", "for", "in", "return", "true", "false",
    "as", "const", "static", "self", "Self", "where", "type", "mod", "crate", "super", "async", "await", "dyn",
];

/// Подсветка Rust: HTML со `span` классов `tk-*`.
pub fn highlight(code: &str) -> String {
    let chars: Vec<char> = code.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '/' && chars.get(i + 1) == Some(&'/') {
            let end = chars[i..].iter().position(|&x| x == '\n').map_or(chars.len(), |p| i + p);
            out.push_str("<span class=\"tk-c\">");
            chars[i..end].iter().for_each(|&x| esc(x, &mut out));
            out.push_str("</span>");
            i = end;
        } else if c == '"' {
            let mut end = i + 1;
            while end < chars.len() && !(chars[end] == '"' && chars[end - 1] != '\\') {
                end += 1;
            }
            let end = (end + 1).min(chars.len());
            out.push_str("<span class=\"tk-s\">");
            chars[i..end].iter().for_each(|&x| esc(x, &mut out));
            out.push_str("</span>");
            i = end;
        } else if c.is_alphabetic() || c == '_' {
            let end = i + chars[i..].iter().take_while(|x| x.is_alphanumeric() || **x == '_').count();
            let word: String = chars[i..end].iter().collect();
            let prev = if i > 0 { chars[i - 1] } else { ' ' };
            let class = if KEYWORDS.contains(&word.as_str()) {
                Some("tk-k")
            } else if chars.get(end) == Some(&'!') {
                Some("tk-m")
            } else if (prev == '<' || (prev == '/' && i > 1 && chars[i - 2] == '<')) && word.starts_with(char::is_uppercase) {
                Some("tk-t")
            } else if word.starts_with(char::is_uppercase) {
                Some("tk-y")
            } else {
                None
            };
            match class {
                Some(cl) => {
                    let _ = write!(out, "<span class=\"{cl}\">{word}</span>");
                }
                None => out.push_str(&word),
            }
            i = end;
        } else if c.is_ascii_digit() {
            let end = i + chars[i..].iter().take_while(|x| x.is_ascii_alphanumeric() || **x == '.' || **x == '_').count();
            let num: String = chars[i..end].iter().collect();
            let _ = write!(out, "<span class=\"tk-n\">{num}</span>");
            i = end;
        } else {
            esc(c, &mut out);
            i += 1;
        }
    }
    out
}

/// Markdown → HTML. Заголовок первого уровня (название документа) опускается, остальные сдвигаются до `h4`+,
/// чтобы на странице оставался один `h2` и разделы `h3`.
pub fn markdown(md: &str) -> String {
    use pulldown_cmark::{Event, HeadingLevel, Tag, TagEnd};
    let shift = |l: HeadingLevel| match l {
        HeadingLevel::H1 | HeadingLevel::H2 => HeadingLevel::H4,
        HeadingLevel::H3 => HeadingLevel::H5,
        _ => HeadingLevel::H6,
    };
    let mut skipping_title = false;
    let events =
        pulldown_cmark::Parser::new_ext(md, pulldown_cmark::Options::ENABLE_TABLES | pulldown_cmark::Options::ENABLE_STRIKETHROUGH)
            .filter_map(|e| match e {
                Event::Start(Tag::Heading { level: HeadingLevel::H1, .. }) => {
                    skipping_title = true;
                    None
                }
                Event::End(TagEnd::Heading(HeadingLevel::H1)) => {
                    skipping_title = false;
                    None
                }
                _ if skipping_title => None,
                Event::Start(Tag::Heading { level, id, classes, attrs }) => {
                    Some(Event::Start(Tag::Heading { level: shift(level), id, classes, attrs }))
                }
                Event::End(TagEnd::Heading(level)) => Some(Event::End(TagEnd::Heading(shift(level)))),
                other => Some(other),
            });
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, events);
    html
}

/// Пункты заметок о доступности: строки с `aria-`, `role=` или клавишами.
pub fn accessibility_notes(md: &str) -> String {
    let lines: Vec<&str> = md
        .lines()
        .filter(|l| l.trim_start().starts_with("- ") && (l.contains("aria-") || l.contains("role=") || l.contains("клавиш")))
        .collect();
    if lines.is_empty() { String::new() } else { markdown(&lines.join("\n")) }
}

/// Данные одной страницы.
pub struct PageData {
    pub name: String,
    pub description: String,
    pub imports: String,
    pub demos: Vec<(DemoRef, String)>,
    pub comps: Vec<Comp>,
    pub pt: Vec<PtStruct>,
    pub classes: Vec<String>,
    pub notes: String,
    pub a11y: String,
}

fn q(s: &str) -> String {
    format!("{s:?}")
}

/// Rust-код модуля `generated`.
pub fn render(pages: &[PageData]) -> String {
    let mut out = String::from(
        "//! Данные сайта документации. Создан `cargo xtask gen-docs`; руками не править.\n\n\
         /// Код демонстрации.\npub struct DemoDoc {\n    pub name: &'static str,\n    pub title: &'static str,\n    pub code: &'static str,\n    pub html: &'static str,\n}\n\n\
         /// Параметр компонента: вид (`prop`, `event`, `pt`), имя, тип, значение по умолчанию, описание, обязательность.\npub struct ParamDoc {\n    pub kind: &'static str,\n    pub name: &'static str,\n    pub ty: &'static str,\n    pub default: &'static str,\n    pub doc: &'static str,\n    pub required: bool,\n}\n\n\
         /// Компонент крейта.\npub struct CompDoc {\n    pub name: &'static str,\n    pub params: &'static [ParamDoc],\n}\n\n\
         /// Структура Pass Through.\npub struct PtDoc {\n    pub name: &'static str,\n    pub fields: &'static [(&'static str, &'static str)],\n}\n\n\
         /// Страница компонента.\npub struct PageDoc {\n    pub name: &'static str,\n    pub description: &'static str,\n    pub imports: &'static str,\n    pub demos: &'static [DemoDoc],\n    pub comps: &'static [CompDoc],\n    pub pt: &'static [PtDoc],\n    pub classes: &'static [&'static str],\n    pub notes: &'static str,\n    pub a11y: &'static str,\n}\n\n\
         /// Страницы по имени.\npub static PAGES: &[PageDoc] = &[\n",
    );
    for p in pages {
        let _ = writeln!(
            out,
            "    PageDoc {{\n        name: {},\n        description: {},\n        imports: {},",
            q(&p.name),
            q(&p.description),
            q(&p.imports)
        );
        out.push_str("        demos: &[\n");
        for (d, code) in &p.demos {
            let _ = writeln!(
                out,
                "            DemoDoc {{ name: {}, title: {}, code: {}, html: {} }},",
                q(&d.name),
                q(&d.title),
                q(code),
                q(&highlight(code))
            );
        }
        out.push_str("        ],\n        comps: &[\n");
        for c in &p.comps {
            let _ = writeln!(out, "            CompDoc {{\n                name: {},\n                params: &[", q(&c.name));
            for a in &c.params {
                let kind = match a.kind {
                    Kind::Prop => "prop",
                    Kind::Event => "event",
                    Kind::Pt => "pt",
                };
                let _ = writeln!(
                    out,
                    "                    ParamDoc {{ kind: {}, name: {}, ty: {}, default: {}, doc: {}, required: {} }},",
                    q(kind),
                    q(&a.name),
                    q(&a.ty),
                    q(a.default.as_deref().unwrap_or("")),
                    q(&a.doc),
                    a.required
                );
            }
            out.push_str("                ],\n            },\n");
        }
        out.push_str("        ],\n        pt: &[\n");
        for s in &p.pt {
            let fields = s.fields.iter().map(|(n, d)| format!("({}, {})", q(n), q(d))).collect::<Vec<_>>().join(", ");
            let _ = writeln!(out, "            PtDoc {{ name: {}, fields: &[{fields}] }},", q(&s.name));
        }
        let classes = p.classes.iter().map(|c| q(c)).collect::<Vec<_>>().join(", ");
        let _ = writeln!(
            out,
            "        ],\n        classes: &[{classes}],\n        notes: {},\n        a11y: {},\n    }},",
            q(&p.notes),
            q(&p.a11y)
        );
    }
    out.push_str("];\n");
    out
}

fn read(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok().map(|t| t.replace("\r\n", "\n"))
}

/// Файл заметок страницы: общие документы нескольких компонентов.
fn doc_file(name: &str) -> &str {
    match name {
        "badge" => "button",
        "buttongroup" | "fluid" | "iconfield" | "floatlabel" | "iftalabel" => "field-wrappers",
        "overlaybadge" | "timeline" | "metergroup" => "display-batch",
        other => other,
    }
}

/// Собирает данные страниц из каталогов репозитория.
pub fn collect(root: &Path) -> Result<Vec<PageData>, String> {
    let pages_dir = root.join("showcase/src/pages");
    let mut names: Vec<String> = std::fs::read_dir(&pages_dir)
        .map_err(|e| format!("{}: {e}", pages_dir.display()))?
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().to_str().and_then(|n| n.strip_suffix(".rs")).map(str::to_owned))
        .filter(|n| n != "mod" && n != "home")
        .collect();
    names.sort();
    let mut pages = Vec::new();
    for name in names {
        let src = read(&pages_dir.join(format!("{name}.rs"))).ok_or_else(|| format!("страница {name}"))?;
        let demos = parse_page_demos(&src)
            .into_iter()
            .map(|d| {
                let code = extract_fn(&src, &d.component).unwrap_or_else(|| inline_code(&d.inner));
                (d, code)
            })
            .collect();
        let lib = read(&root.join(format!("crates/prime-{name}/src/lib.rs")));
        let comps = lib.as_deref().map(parse_components).unwrap_or_default();
        let pt = lib.as_deref().map(parse_pt).unwrap_or_default();
        let description = lib.as_deref().map(crate_description).unwrap_or_default();
        let classes =
            read(&root.join(format!("crates/prime-theme/css/components/{name}.css"))).map(|c| css_classes(&c)).unwrap_or_default();
        let md = read(&root.join(format!("docs/{}.md", doc_file(&name))));
        pages.push(PageData {
            imports: page_imports(&src),
            description,
            demos,
            comps,
            pt,
            classes,
            notes: md.as_deref().map(markdown).unwrap_or_default(),
            a11y: md.as_deref().map(accessibility_notes).unwrap_or_default(),
            name,
        });
    }
    Ok(pages)
}

/// Запуск: запись `showcase/src/generated.rs` или проверка актуальности (`check`).
pub fn run(root: &Path, check: bool) -> Result<(), String> {
    let text = render(&collect(root)?);
    let path = root.join("showcase/src/generated.rs");
    if check {
        let current = std::fs::read_to_string(&path).unwrap_or_default();
        if current.replace("\r\n", "\n") != text {
            return Err(format!("{} устарел: выполните `cargo xtask gen-docs`", path.display()));
        }
        return Ok(());
    }
    std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
    println!("документация: {}", path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = "use prime_button::Button;\nuse prime_core::Size;\n\n#[component]\nfn Basic() -> impl IntoView {\n    view! { <Button label=\"x\" /> }\n}\n\n#[component]\nfn Other() -> impl IntoView {\n    view! { <b>\"o\"</b> }\n}\n\n#[component]\npub fn ButtonPage() -> impl IntoView {\n    view! {\n        <Demo name=\"basic\" title=\"Basic\">\n            <Basic />\n        </Demo>\n        <Demo\n            name=\"other\"\n            title=\"Other\"\n        >\n            <Other />\n        </Demo>\n    }\n}\n";

    #[test]
    fn demos_and_code() {
        let demos = parse_page_demos(PAGE);
        assert_eq!(
            demos,
            vec![
                DemoRef {
                    name: "basic".into(),
                    title: "Basic".into(),
                    component: "Basic".into(),
                    inner: "
            <Basic />
        "
                    .into()
                },
                DemoRef {
                    name: "other".into(),
                    title: "Other".into(),
                    component: "Other".into(),
                    inner: "
            <Other />
        "
                    .into()
                },
            ]
        );
        let code = extract_fn(PAGE, "Basic").unwrap();
        assert!(code.starts_with("#[component]\nfn Basic()") && code.ends_with('}') && code.contains("<Button"), "{code}");
        assert!(!code.contains("Other"));
        assert!(extract_fn(PAGE, "Missing").is_none());
        assert_eq!(page_imports(PAGE), "use prime_button::Button;\nuse prime_core::Size;");
    }

    const LIB: &str = "//! Button — порт.\n//! Вторая строка.\n//!\n//! Далее.\n\n/// Pass-through частей.\n#[derive(Clone)]\npub struct ButtonPt {\n    /// Корень.\n    pub root: Pt,\n    pub icon: Pt,\n}\n\n/// Кнопка.\n#[component]\npub fn Button(\n    /// Подпись.\n    #[prop(optional, into)]\n    label: Prop<String>,\n    /// Порядок.\n    #[prop(optional, default = Prop::Static(5), into)]\n    page_links: Prop<usize>,\n    /// Обязательное.\n    key: Callback<Vec<(String, u8)>, String>,\n    /// Клик.\n    #[prop(optional, into)]\n    on_click: Option<Callback<MouseEvent>>,\n    /// Части.\n    #[prop(optional)]\n    pt: Option<ButtonPt>,\n    children: Children,\n) -> impl IntoView {\n}\n";

    #[test]
    fn components_and_pt() {
        let comps = parse_components(LIB);
        assert_eq!(comps.len(), 1);
        assert_eq!(comps[0].name, "Button");
        let p = &comps[0].params;
        assert_eq!(p.len(), 5, "children пропущен: {p:?}");
        assert_eq!((p[0].name.as_str(), p[0].required, p[0].ty.as_str()), ("label", false, "Prop<String>"));
        assert_eq!(p[0].doc, "Подпись.");
        assert_eq!(p[1].default.as_deref(), Some("5"));
        assert!(p[2].required && p[2].ty == "Callback<Vec<(String, u8)>, String>", "{:?}", p[2]);
        assert_eq!(p[3].kind, Kind::Event);
        assert_eq!(p[4].kind, Kind::Pt);
        let pt = parse_pt(LIB);
        assert_eq!(
            pt,
            vec![PtStruct {
                name: "ButtonPt".into(), fields: vec![("root".into(), "Корень.".into()), ("icon".into(), String::new())]
            }]
        );
        assert_eq!(crate_description(LIB), "Button — порт. Вторая строка.");
    }

    #[test]
    fn classes_and_highlight() {
        assert_eq!(
            css_classes(".p-button{a:b}.p-button-icon:hover,.x.p-badge-dot{}a.p-button"),
            ["p-badge-dot", "p-button", "p-button-icon"]
        );
        let html = highlight("let x = \"a<b\"; // c\nview! { <Button n=5 /> }");
        assert!(html.contains("<span class=\"tk-k\">let</span>") && html.contains("<span class=\"tk-s\">\"a&lt;b\"</span>"), "{html}");
        assert!(
            html.contains("<span class=\"tk-c\">// c</span>")
                && html.contains("<span class=\"tk-t\">Button</span>")
                && html.contains("<span class=\"tk-n\">5</span>"),
            "{html}"
        );
        assert!(html.contains("<span class=\"tk-m\">view</span>"), "{html}");
    }

    #[test]
    fn markdown_and_accessibility() {
        let md = "# T\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n- Есть `aria-sort`.\n- Иное.\n";
        let html = markdown(md);
        assert!(html.contains("<table>") && !html.contains("<h1>") && !html.contains(">T<"), "{html}");
        assert!(
            markdown(
                "## A

### B
"
            )
            .contains("<h4>A</h4>")
                && markdown(
                    "## A

### B
"
                )
                .contains("<h5>B</h5>")
        );
        let a11y = accessibility_notes(md);
        assert!(a11y.contains("aria-sort") && !a11y.contains("Иное"), "{a11y}");
        assert_eq!(accessibility_notes("- нет\n"), "");
    }

    #[test]
    fn inline_demo_code() {
        assert_eq!(
            inline_code(
                "
            <A />
              <B />
        "
            ),
            "view! {
    <A />
      <B />
}"
        );
        assert_eq!(
            inline_code(
                "  
 "
            ),
            ""
        );
        assert_eq!(doc_file("badge"), "button");
        assert_eq!(doc_file("table"), "table");
    }

    #[test]
    fn render_is_deterministic() {
        let page = || PageData {
            name: "button".into(),
            description: "d".into(),
            imports: "use a;".into(),
            demos: parse_page_demos(PAGE).into_iter().map(|d| (d, "code".to_owned())).collect(),
            comps: parse_components(LIB),
            pt: parse_pt(LIB),
            classes: vec!["p-button".into()],
            notes: "<p>n</p>".into(),
            a11y: String::new(),
        };
        let a = render(&[page()]);
        assert_eq!(a, render(&[page()]));
        assert!(a.contains("name: \"button\"") && a.contains("ParamDoc { kind: \"event\", name: \"on_click\""), "{a}");
    }
}
