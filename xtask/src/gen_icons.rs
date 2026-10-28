//! `gen-icons`: генерация `prime-icons` из исходников иконок PrimeNG (`packages/primeng/src/icons`).
//!
//! У всех иконок PrimeNG общий `viewBox="0 0 14 14"` (задаётся в `BaseIcon`); форма иконки — один или
//! несколько `<svg:path d="…">`, иногда внутри `<svg:g clip-path>` с `clipPath`, равным всему `viewBox`
//! (его мы не воспроизводим: он ничего не обрезает). `blank` — отдельный случай (прозрачный `rect`) и пишется вручную.

use std::fmt::Write as _;
use std::path::Path;

/// Одна иконка PrimeNG.
#[derive(Debug, PartialEq, Eq)]
pub struct Icon {
    /// Имя каталога PrimeNG; оно же имя feature-флага (`check`, `chevrondown`).
    pub feature: String,
    /// Имя типа (`CheckIcon`).
    pub class: String,
    /// Значения `d` всех `<svg:path>` по порядку.
    pub paths: Vec<String>,
}

const ALLOWED_TAGS: [&str; 5] = ["g", "path", "defs", "clipPath", "rect"];
/// Каталоги, которые не генерируются.
const SKIP: [&str; 2] = ["baseicon", "blank"];
const VIEW_BOX: &str = "0 0 14 14";

fn between<'a>(s: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let from = s.find(start)? + start.len();
    let len = s[from..].find(end)?;
    Some(&s[from..from + len])
}

/// Разбирает исходник иконки.
pub fn parse_icon(feature: &str, source: &str) -> Result<Icon, String> {
    let err = |m: &str| format!("{feature}: {m}");
    let class = between(source, "export class ", " extends BaseIcon").ok_or_else(|| err("не найден `export class … extends BaseIcon`"))?;
    let template = between(source, "template: `", "`").ok_or_else(|| err("не найден template"))?;

    let mut rest = template;
    while let Some(i) = rest.find("<svg:") {
        rest = &rest[i + "<svg:".len()..];
        let tag: String = rest.chars().take_while(|c| c.is_ascii_alphanumeric()).collect();
        if !ALLOWED_TAGS.contains(&tag.as_str()) {
            return Err(err(&format!("неожиданный тег <svg:{tag}>")));
        }
    }

    let mut paths = Vec::new();
    let mut rest = template;
    while let Some(i) = rest.find("<svg:path") {
        rest = &rest[i..];
        let tag_end = rest.find("/>").ok_or_else(|| err("не закрыт <svg:path>"))?;
        let d = between(&rest[..tag_end], " d=\"", "\"").ok_or_else(|| err("у <svg:path> нет d"))?;
        paths.push(d.split_whitespace().collect::<Vec<_>>().join(" "));
        rest = &rest[tag_end..];
    }
    if paths.is_empty() {
        return Err(err("нет ни одного <svg:path>"));
    }
    Ok(Icon { feature: feature.to_owned(), class: class.trim().to_owned(), paths })
}

/// Проверяет, что `BaseIcon` по-прежнему задаёт `viewBox`, на который рассчитан `prime-icons`.
pub fn check_view_box(base_icon_source: &str) -> Result<(), String> {
    let found = between(base_icon_source, "viewBox: '", "'").ok_or("в baseicon.ts не найден viewBox")?;
    if found == VIEW_BOX { Ok(()) } else { Err(format!("viewBox PrimeNG изменился: {found}, ожидалось {VIEW_BOX}")) }
}

/// Читает все иконки из каталога `icons`.
pub fn read_icons(icons_dir: &Path) -> Result<Vec<Icon>, String> {
    let base = std::fs::read_to_string(icons_dir.join("baseicon/baseicon.ts")).map_err(|e| format!("baseicon.ts: {e}"))?;
    check_view_box(&base)?;
    let mut dirs: Vec<_> = std::fs::read_dir(icons_dir)
        .map_err(|e| format!("{}: {e}", icons_dir.display()))?
        .flatten()
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| !SKIP.contains(&n.as_str()))
        .collect();
    dirs.sort();
    dirs.iter()
        .map(|name| {
            let dir = icons_dir.join(name);
            // Исходник иконки — единственный .ts, кроме public_api.ts (у `filterfill` он называется `filter.ts`).
            let file = std::fs::read_dir(&dir)
                .map_err(|e| format!("{name}: {e}"))?
                .flatten()
                .map(|e| e.path())
                .find(|p| p.extension().is_some_and(|x| x == "ts") && p.file_name().is_some_and(|f| f != "public_api.ts"))
                .ok_or_else(|| format!("{name}: нет исходника"))?;
            parse_icon(name, &std::fs::read_to_string(&file).map_err(|e| format!("{}: {e}", file.display()))?)
        })
        .collect()
}

/// Rust-код всех иконок (`src/generated.rs`).
pub fn render_rust(icons: &[Icon]) -> String {
    let mut s = String::from(
        "// Файл сгенерирован `cargo xtask gen-icons` из иконок PrimeNG; не редактировать вручную.\n\
         // Иконки добавляются только при включённом одноимённом feature.\n\n",
    );
    for icon in icons {
        let paths = icon.paths.iter().map(|d| format!("{d:?}")).collect::<Vec<_>>().join(", ");
        let _ = writeln!(s, "#[cfg(feature = {:?})]\nicon!({}, [{}]);\n", icon.feature, icon.class, paths);
    }
    s.push_str("/// Все иконки подряд (для демо и измерений размера).\n#[cfg(feature = \"all-icons\")]\n#[::leptos::component]\n");
    s.push_str("pub fn AllIcons() -> impl ::leptos::prelude::IntoView {\n    use ::leptos::prelude::*;\n    [\n");
    s.push_str("        view! { <crate::BlankIcon /> }.into_any(),\n");
    for icon in icons {
        let _ = writeln!(s, "        view! {{ <{} /> }}.into_any(),", icon.class);
    }
    s.push_str("    ]\n    .into_iter()\n    .collect_view()\n}\n");
    s
}

const BEGIN: &str = "# BEGIN GENERATED FEATURES";
const END: &str = "# END GENERATED FEATURES";

/// Подставляет список features между маркерами в `Cargo.toml` крейта.
pub fn render_features(cargo_toml: &str, icons: &[Icon]) -> Result<String, String> {
    let (b, e) = (cargo_toml.find(BEGIN), cargo_toml.find(END));
    let (Some(b), Some(e)) = (b, e) else { return Err(format!("в Cargo.toml нет маркеров `{BEGIN}` / `{END}`")) };
    let mut block = format!("{BEGIN}\n");
    for icon in icons {
        let _ = writeln!(block, "{} = []", icon.feature);
    }
    let all = icons.iter().map(|i| format!("{:?}", i.feature)).collect::<Vec<_>>().join(", ");
    let _ = writeln!(block, "blank = []\nall-icons = [\"blank\", {all}]");
    Ok(format!("{}{block}{}", &cargo_toml[..b], &cargo_toml[e..]))
}

/// Генерирует файлы `prime-icons` из исходников PrimeNG.
pub fn run(icons_dir: &Path, crate_dir: &Path) -> Result<usize, String> {
    let icons = read_icons(icons_dir)?;
    let rs = render_rust(&icons);
    let manifest_path = crate_dir.join("Cargo.toml");
    let manifest = std::fs::read_to_string(&manifest_path).map_err(|e| format!("{}: {e}", manifest_path.display()))?;
    let manifest = render_features(&manifest, &icons)?;
    for (path, text) in [(crate_dir.join("src/generated.rs"), rs), (manifest_path, manifest)] {
        if std::fs::read_to_string(&path).ok().as_deref() != Some(text.as_str()) {
            std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
        }
    }
    Ok(icons.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SINGLE: &str = r#"
@Component({
    selector: '[data-p-icon="check"]',
    template: `
        <svg:path
            d="M4.8 11.5
               L1 2"
            fill="currentColor"
        />
    `
})
export class CheckIcon extends BaseIcon {}
"#;

    const CLIPPED: &str = r#"
@Component({
    template: `
        <svg:g [attr.clip-path]="pathId">
            <svg:path d="M1 1" fill="currentColor" />
            <svg:path d="M2 2" fill="currentColor" />
        </svg:g>
        <svg:defs><svg:clipPath [id]="pathId"><svg:rect width="14" height="14" fill="white" /></svg:clipPath></svg:defs>
    `
})
export class SpinnerIcon extends BaseIcon {}
"#;

    #[test]
    fn parses_single_path_and_normalizes_whitespace() {
        let icon = parse_icon("check", SINGLE).unwrap();
        assert_eq!(icon, Icon { feature: "check".into(), class: "CheckIcon".into(), paths: vec!["M4.8 11.5 L1 2".into()] });
    }

    #[test]
    fn parses_clipped_icon_with_several_paths() {
        let icon = parse_icon("spinner", CLIPPED).unwrap();
        assert_eq!(icon.paths, vec!["M1 1", "M2 2"]);
    }

    #[test]
    fn unexpected_tag_is_an_error() {
        let bad = SINGLE.replace("<svg:path", "<svg:circle");
        assert!(parse_icon("x", &bad).unwrap_err().contains("circle"));
    }

    #[test]
    fn missing_path_is_an_error() {
        assert!(parse_icon("x", "export class XIcon extends BaseIcon {}\ntemplate: `<svg:rect />`").is_err());
    }

    #[test]
    fn view_box_check() {
        assert!(check_view_box("viewBox: '0 0 14 14',").is_ok());
        assert!(check_view_box("viewBox: '0 0 16 16',").is_err());
    }

    #[test]
    fn renders_rust_snapshot() {
        let icons = vec![parse_icon("check", SINGLE).unwrap(), parse_icon("spinner", CLIPPED).unwrap()];
        insta::assert_snapshot!(render_rust(&icons));
    }

    #[test]
    fn renders_features_between_markers_and_is_idempotent() {
        let toml = "[features]\n# BEGIN GENERATED FEATURES\nold = []\n# END GENERATED FEATURES\n\n[deps]\n";
        let icons = vec![parse_icon("check", SINGLE).unwrap()];
        let once = render_features(toml, &icons).unwrap();
        assert!(once.contains("check = []") && !once.contains("old = []"));
        assert_eq!(render_features(&once, &icons).unwrap(), once);
        assert!(render_features("[features]\n", &icons).is_err());
    }
}
