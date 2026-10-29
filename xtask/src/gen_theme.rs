//! `gen-theme`: статический CSS темы Aura для `prime-theme`.
//!
//! CSS строит тот же код, что и в PrimeNG (`@primeuix/styled`, `@primeuix/styles`, пресет `@primeuix/themes/aura`):
//! скрипт `theme/extract.mjs` запускается в Node в каталоге кэша вне репозитория и печатает JSON.
//! Rust-часть записывает результат в `crates/prime-theme`; закоммиченные файлы не требуют Node при обычной сборке.

use crate::upstream::Upstream;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;

const EXTRACT_JS: &str = include_str!("../theme/extract.mjs");
const BEGIN: &str = "# BEGIN GENERATED FEATURES";
const END: &str = "# END GENERATED FEATURES";

/// Результат работы скрипта.
#[derive(Debug, PartialEq, Eq)]
pub struct Css {
    pub base: String,
    pub components: BTreeMap<String, String>,
}

/// Разбирает JSON скрипта и проверяет, что запрошенные компоненты присутствуют и не пусты.
pub fn parse_output(json: &str, wanted: &[String]) -> Result<Css, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("вывод extract.mjs не JSON: {e}"))?;
    let base = v["base"].as_str().filter(|s| !s.is_empty()).ok_or("в выводе нет base")?.to_owned();
    let obj = v["components"].as_object().ok_or("в выводе нет components")?;
    let mut components = BTreeMap::new();
    for name in wanted {
        let css = obj.get(name).and_then(Value::as_str).filter(|s| !s.is_empty()).ok_or_else(|| format!("нет CSS компонента `{name}`"))?;
        components.insert(name.clone(), css.to_owned());
    }
    // Нерешённый `dt('…')` в CSS означает, что значения токенов не подставлены и стиль работать не будет.
    for (name, css) in std::iter::once((&"base".to_owned(), &base)).chain(components.iter()) {
        if let Some(i) = css.find("dt(") {
            return Err(format!("в CSS `{name}` остался нерешённый dt(): {}", &css[i..(i + 40).min(css.len())]));
        }
    }
    Ok(Css { base, components })
}

/// `CSS` для `src/generated.rs`: константы с `include_str!` за feature-флагами.
pub fn render_rust(names: &[String]) -> String {
    let mut s = String::from("// Файл сгенерирован `cargo xtask gen-theme`; не редактировать вручную.\n\n");
    for n in names {
        let _ = writeln!(
            s,
            "/// CSS компонента `{n}`.\n#[cfg(feature = {n:?})]\npub const {}: &str = include_str!(\"../css/components/{n}.css\");\n",
            n.to_uppercase()
        );
    }
    s.push_str("/// Пары «имя — CSS» для включённых компонентов.\npub(crate) static COMPONENTS: &[(&str, &str)] = &[\n");
    for n in names {
        let _ = writeln!(s, "    #[cfg(feature = {n:?})]\n    ({n:?}, {}),", n.to_uppercase());
    }
    s.push_str("];\n");
    s
}

/// Подставляет features между маркерами в `Cargo.toml`.
pub fn render_features(cargo_toml: &str, names: &[String]) -> Result<String, String> {
    let (Some(b), Some(e)) = (cargo_toml.find(BEGIN), cargo_toml.find(END)) else {
        return Err(format!("в Cargo.toml нет маркеров `{BEGIN}` / `{END}`"));
    };
    let mut block = format!("{BEGIN}\n");
    for n in names {
        let _ = writeln!(block, "{n} = []");
    }
    let all = names.iter().map(|n| format!("{n:?}")).collect::<Vec<_>>().join(", ");
    let _ = writeln!(block, "all-components = [{all}]");
    Ok(format!("{}{block}{}", &cargo_toml[..b], &cargo_toml[e..]))
}

fn write_if_changed(path: &Path, text: &str) -> Result<(), String> {
    if std::fs::read_to_string(path).ok().as_deref() != Some(text) {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        }
        std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))?;
    }
    Ok(())
}

/// Кэш зависимостей Node: вне репозитория, `PRIME_RS_CACHE` переопределяет путь.
fn cache_dir(up: &Upstream) -> PathBuf {
    let base = std::env::var_os("PRIME_RS_CACHE").map(PathBuf::from).unwrap_or_else(|| {
        let home =
            std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
        home.join(".cache").join("prime-rs")
    });
    let p = &up.primeuix;
    base.join(format!("theme-{}-{}-{}-{}", p.themes, p.styles, p.styled, p.utils))
}

fn run_checked(cmd: &mut Command, what: &str) -> Result<std::process::Output, String> {
    let out = cmd.output().map_err(|e| format!("{what}: {e}"))?;
    if out.status.success() { Ok(out) } else { Err(format!("{what}: {}", String::from_utf8_lossy(&out.stderr).trim())) }
}

/// Берёт `css`-литерал из `basestyle.ts` PrimeNG (`.p-hidden-accessible`, `.p-overflow-hidden`).
fn extra_base_css(up: &Upstream) -> Result<String, String> {
    let file = up.src_dir().join("base/style/basestyle.ts");
    let src = std::fs::read_to_string(&file).map_err(|e| format!("{}: {e}", file.display()))?;
    let from = src.find("const css = /*css*/ `").ok_or("в basestyle.ts не найден const css")? + "const css = /*css*/ `".len();
    let len = src[from..].find('`').ok_or("в basestyle.ts не закрыт css")?;
    Ok(src[from..from + len].to_owned())
}

/// Часть `css`-литерала после маркера `/* For PrimeNG */` или `/* For PrimeNG (пояснение) */` (до закрывающего обратного апострофа),
/// если маркер есть.
fn primeng_extra_block(src: &str) -> Option<String> {
    const MARK: &str = "/* For PrimeNG";
    let start = src.find(MARK)? + MARK.len();
    let end = start + src[start..].find("*/")?;
    // после имени маркера допустимо только пояснение в скобках: `/* For PrimeNG Something */` не маркер
    let note = src[start..end].trim();
    if !(note.is_empty() || (note.starts_with('(') && note.ends_with(')'))) {
        return None;
    }
    let from = end + "*/".len();
    let len = src[from..].find('`')?;
    Some(src[from..from + len].trim().to_owned())
}

/// Блоки `For PrimeNG` перечисленных в `primeng_extra` компонентов: имя → исходный текст с `dt('…')`.
fn extra_component_css(up: &Upstream) -> Result<serde_json::Map<String, Value>, String> {
    let mut map = serde_json::Map::new();
    for name in &up.theme.primeng_extra {
        let file = up.src_dir().join(format!("{name}/style/{name}style.ts"));
        let src = std::fs::read_to_string(&file).map_err(|e| format!("{}: {e}", file.display()))?;
        let block = primeng_extra_block(&src)
            .ok_or_else(|| format!("в {} нет блока `/* For PrimeNG */` (допустимо с пояснением в скобках)", file.display()))?;
        map.insert(name.clone(), Value::String(block));
    }
    Ok(map)
}

/// Генерирует CSS и Rust-обвязку.
pub fn run(root: &Path, up: &Upstream) -> Result<(), String> {
    let cache = cache_dir(up);
    let node = cache.join("node_modules");
    if !node.is_dir() {
        std::fs::create_dir_all(&cache).map_err(|e| format!("{}: {e}", cache.display()))?;
        let p = &up.primeuix;
        std::fs::write(cache.join("package.json"), "{\"name\":\"prime-rs-theme\",\"private\":true,\"type\":\"module\"}\n")
            .map_err(|e| e.to_string())?;
        let npm = if cfg!(windows) { "npm.cmd" } else { "npm" };
        let pkgs = [
            format!("@primeuix/themes@{}", p.themes),
            format!("@primeuix/styles@{}", p.styles),
            format!("@primeuix/styled@{}", p.styled),
            format!("@primeuix/utils@{}", p.utils),
        ];
        run_checked(
            Command::new(npm).current_dir(&cache).args(["install", "--silent", "--no-audit", "--no-fund", "--ignore-scripts"]).args(&pkgs),
            "npm install",
        )?;
    }
    std::fs::write(cache.join("extract.mjs"), EXTRACT_JS).map_err(|e| e.to_string())?;

    let options = serde_json::json!({
        "darkModeSelector": up.theme.dark_mode_selector,
        "components": up.theme.components,
        "extraBaseCss": extra_base_css(up)?,
        "extraComponentCss": extra_component_css(up)?,
    });
    let out = run_checked(Command::new("node").current_dir(&cache).arg("extract.mjs").arg(options.to_string()), "node extract.mjs")?;
    let css = parse_output(&String::from_utf8_lossy(&out.stdout), &up.theme.components)?;

    let krate = root.join("crates/prime-theme");
    write_if_changed(&krate.join("css/base.css"), &format!("{}\n", css.base))?;
    let dir = krate.join("css/components");
    for (name, text) in &css.components {
        write_if_changed(&dir.join(format!("{name}.css")), &format!("{text}\n"))?;
    }
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for e in entries.flatten() {
            let stem = e.path().file_stem().and_then(|s| s.to_str()).map(str::to_owned).unwrap_or_default();
            if !css.components.contains_key(&stem) {
                std::fs::remove_file(e.path()).map_err(|e| e.to_string())?;
            }
        }
    }
    let names: Vec<String> = css.components.keys().cloned().collect();
    write_if_changed(&krate.join("src/generated.rs"), &render_rust(&names))?;
    let manifest_path = krate.join("Cargo.toml");
    let manifest = std::fs::read_to_string(&manifest_path).map_err(|e| format!("{}: {e}", manifest_path.display()))?;
    write_if_changed(&manifest_path, &render_features(&manifest, &names)?)?;
    println!("тема: base.css ({} Б) и {} компонентов", css.base.len(), names.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const JSON: &str = r#"{"base":":root{--p-x:1}","components":{"badge":".p-badge{}","button":".p-button{}"}}"#;

    fn names(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn extracts_block_after_marker() {
        let src = "const style = /*css*/ `
${s}

/* For PrimeNG */
.a { x: dt('accordion.header.padding'); }
.b { y: 1; }
`;
const classes = {};";
        assert_eq!(
            primeng_extra_block(src).as_deref(),
            Some(
                ".a { x: dt('accordion.header.padding'); }
.b { y: 1; }"
            )
        );
        assert_eq!(primeng_extra_block("const style = `${s}`;"), None);
    }

    #[test]
    fn marker_may_carry_a_note() {
        let src = "const style = `
${s}

    /* For PrimeNG (iconPos) */
    .a { order: 1; }
`;";
        assert_eq!(primeng_extra_block(src).as_deref(), Some(".a { order: 1; }"));
        assert_eq!(primeng_extra_block("`/* For PrimeNG Something */ .a{}`"), None);
    }

    #[test]
    fn parses_and_filters_wanted_components() {
        let css = parse_output(JSON, &names(&["button"])).unwrap();
        assert_eq!(css.base, ":root{--p-x:1}");
        assert_eq!(css.components.keys().collect::<Vec<_>>(), vec!["button"]);
    }

    #[test]
    fn unresolved_dt_is_rejected() {
        let json = r#"{"base":":root{}","components":{"button":".p-button{color:dt('button.primary.color')}"}}"#;
        let err = parse_output(json, &names(&["button"])).unwrap_err();
        assert!(err.contains("button") && err.contains("dt("), "{err}");
    }

    #[test]
    fn missing_component_or_bad_json_is_error() {
        assert!(parse_output(JSON, &names(&["card"])).unwrap_err().contains("card"));
        assert!(parse_output("not json", &[]).is_err());
        assert!(parse_output(r#"{"base":"","components":{}}"#, &[]).is_err());
    }

    #[test]
    fn renders_rust_snapshot() {
        insta::assert_snapshot!(render_rust(&names(&["badge", "button"])));
    }

    #[test]
    fn features_are_idempotent() {
        let toml = "[features]\n# BEGIN GENERATED FEATURES\n# END GENERATED FEATURES\n";
        let once = render_features(toml, &names(&["badge"])).unwrap();
        assert!(once.contains("badge = []") && once.contains("all-components = [\"badge\"]"));
        assert_eq!(render_features(&once, &names(&["badge"])).unwrap(), once);
    }
}
