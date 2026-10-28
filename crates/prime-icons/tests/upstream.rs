//! Сверка с исходниками PrimeNG: каждый `d` встречается в отрисованной иконке.
//! Тест пропускается, если клон PrimeNG недоступен (`PRIMENG_PATH` или `C:/Projects/primeng`).
#![cfg(all(feature = "ssr", feature = "all-icons"))]

use leptos::prelude::*;
use std::path::PathBuf;

fn primeng_icons() -> Option<PathBuf> {
    let root = std::env::var("PRIMENG_PATH").unwrap_or_else(|_| "C:/Projects/primeng".into());
    let dir = PathBuf::from(root).join("packages/primeng/src/icons");
    dir.is_dir().then_some(dir)
}

fn source_paths(src: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = src;
    while let Some(i) = rest.find("<svg:path") {
        rest = &rest[i..];
        let end = rest.find("/>").unwrap();
        let tag = &rest[..end];
        let from = tag.find(" d=\"").unwrap() + 4;
        let len = tag[from..].find('"').unwrap();
        out.push(tag[from..from + len].split_whitespace().collect::<Vec<_>>().join(" "));
        rest = &rest[end..];
    }
    out
}

#[test]
fn every_icon_has_the_same_paths_as_primeng() {
    let Some(dir) = primeng_icons() else {
        eprintln!("PrimeNG не найден, тест пропущен");
        return;
    };
    let mut checked = 0;
    for entry in std::fs::read_dir(&dir).unwrap().flatten() {
        let name = entry.file_name().into_string().unwrap();
        if name == "baseicon" || name == "blank" || !entry.path().is_dir() {
            continue;
        }
        let file = std::fs::read_dir(entry.path())
            .unwrap()
            .flatten()
            .map(|e| e.path())
            .find(|p| p.extension().is_some_and(|x| x == "ts") && p.file_name().unwrap() != "public_api.ts")
            .unwrap();
        let src = std::fs::read_to_string(&file).unwrap();
        let class = src.split("export class ").nth(1).unwrap().split_whitespace().next().unwrap();
        let html = render(class);
        let expected = source_paths(&src);
        assert!(!expected.is_empty(), "{name}");
        for d in &expected {
            assert!(html.contains(&format!(r#"d="{d}""#)), "{name}: нет контура {d}");
        }
        assert_eq!(html.matches("<path").count(), expected.len(), "{name}: число контуров");
        assert!(html.contains(r#"viewBox="0 0 14 14""#), "{name}");
        checked += 1;
    }
    assert_eq!(checked, 53);
}

macro_rules! render_match {
    ($class:expr, $($name:ident),+ $(,)?) => {
        match $class {
            $(stringify!($name) => view! { <prime_icons::$name /> }.to_html(),)+
            other => panic!("нет иконки {other}"),
        }
    };
}

fn render(class: &str) -> String {
    render_match!(
        class,
        AngleDoubleDownIcon,
        AngleDoubleLeftIcon,
        AngleDoubleRightIcon,
        AngleDoubleUpIcon,
        AngleDownIcon,
        AngleLeftIcon,
        AngleRightIcon,
        AngleUpIcon,
        ArrowDownIcon,
        ArrowDownLeftIcon,
        ArrowDownRightIcon,
        ArrowLeftIcon,
        ArrowRightIcon,
        ArrowUpIcon,
        BanIcon,
        BarsIcon,
        CalendarIcon,
        CaretLeftIcon,
        CaretRightIcon,
        CheckIcon,
        ChevronDownIcon,
        ChevronLeftIcon,
        ChevronRightIcon,
        ChevronUpIcon,
        ExclamationTriangleIcon,
        EyeIcon,
        EyeSlashIcon,
        FilterIcon,
        FilterFillIcon,
        FilterSlashIcon,
        HomeIcon,
        InfoCircleIcon,
        MinusIcon,
        PencilIcon,
        PlusIcon,
        RefreshIcon,
        SearchIcon,
        SearchMinusIcon,
        SearchPlusIcon,
        SortAltIcon,
        SortAmountDownIcon,
        SortAmountUpAltIcon,
        SpinnerIcon,
        StarIcon,
        StarFillIcon,
        ThLargeIcon,
        TimesIcon,
        TimesCircleIcon,
        TrashIcon,
        UndoIcon,
        UploadIcon,
        WindowMaximizeIcon,
        WindowMinimizeIcon,
    )
}
