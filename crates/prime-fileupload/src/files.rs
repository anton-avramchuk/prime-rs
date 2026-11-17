//! Pure FileUpload logic: file validation, sizes, limits, labels (after PrimeNG `fileupload.ts`, no DOM).

/// Default size units (`fileSizeTypes`).
pub const SIZE_UNITS: [&str; 9] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

/// File information needed for the checks.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FileInfo {
    /// File name.
    pub name: String,
    /// MIME type (may be empty).
    pub mime: String,
    /// Size in bytes.
    pub size: u64,
}

impl FileInfo {
    /// File information.
    pub fn new(name: impl Into<String>, mime: impl Into<String>, size: u64) -> Self {
        Self { name: name.into(), mime: mime.into(), size }
    }
}

/// Reason a file was rejected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Rejection {
    /// The type is not in `accept`.
    Type,
    /// The size exceeds `max_file_size`.
    Size,
}

/// Size as "a number with three decimals and a unit" (`formatSize`).
pub fn format_size(bytes: u64, units: &[&str]) -> String {
    let unit = |i: usize| units.get(i).copied().unwrap_or("");
    if bytes == 0 {
        return format!("0 {}", unit(0));
    }
    let k = 1024f64;
    let i = ((bytes as f64).ln() / k.ln()).floor() as usize;
    // `toFixed(3)` rounds half up, while `{:.3}` in Rust rounds to even: 976.5625 → 976.563, not 976.562
    let value = (bytes as f64 / k.powi(i as i32) * 1000.0).round() / 1000.0;
    format!("{value:.3} {}", unit(i))
}

/// File extension with the dot (`getFileExtension`): for a name without a dot — the whole name.
pub fn file_extension(name: &str) -> String {
    format!(".{}", name.rsplit('.').next().unwrap_or(name))
}

/// The file type matches one of the `accept` entries (`isFileTypeValid`).
pub fn is_type_valid(file: &FileInfo, accept: &str) -> bool {
    accept.split(',').map(str::trim).any(|kind| {
        if kind.contains('*') {
            type_class(&file.mime) == type_class(kind)
        } else {
            file.mime == kind || file_extension(&file.name).to_lowercase() == kind.to_lowercase()
        }
    })
}

/// The part of the type before `/` (`getTypeClass`); without `/` — an empty string.
fn type_class(kind: &str) -> &str {
    kind.find('/').map_or("", |i| &kind[..i])
}

/// Checks a file against `accept` and `max_file_size` (`validate`).
pub fn validate(file: &FileInfo, accept: Option<&str>, max_size: Option<u64>) -> Result<(), Rejection> {
    if let Some(accept) = accept.filter(|a| !a.is_empty())
        && !is_type_valid(file, accept)
    {
        return Err(Rejection::Type);
    }
    if let Some(max) = max_size.filter(|m| *m > 0)
        && file.size > max
    {
        return Err(Rejection::Size);
    }
    Ok(())
}

/// The file is already selected: name, type and size match (`isFileSelected`).
pub fn is_duplicate(selected: &[FileInfo], file: &FileInfo) -> bool {
    selected.iter().any(|f| f.name == file.name && f.mime == file.mime && f.size == file.size)
}

/// `file_limit` is exceeded (`isFileLimitExceeded`).
pub fn limit_exceeded(auto: bool, selected: usize, uploaded: usize, limit: Option<usize>) -> bool {
    let total = if auto { selected } else { selected + uploaded };
    limit.is_some_and(|l| l < total)
}

/// Choosing files is unavailable: the limit is reached (`isChooseDisabled`).
pub fn choose_disabled(auto: bool, selected: usize, uploaded: usize, limit: Option<usize>) -> bool {
    let total = if auto { selected } else { selected + uploaded };
    limit.is_some_and(|l| l <= total)
}

/// Label of the selection in `basic` mode (`basicFileChosenLabel`).
pub fn basic_label(auto: bool, choose_label: &str, names: &[String], chosen_template: &str, none: &str) -> String {
    if auto {
        choose_label.to_owned()
    } else if names.len() == 1 {
        names[0].clone()
    } else if !names.is_empty() {
        chosen_template.replace("{0}", &names.len().to_string())
    } else {
        none.to_owned()
    }
}

/// The file is an image (`isImage`).
pub fn is_image(file: &FileInfo) -> bool {
    file.mime.starts_with("image/")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(name: &str, mime: &str, size: u64) -> FileInfo {
        FileInfo::new(name, mime, size)
    }

    #[test]
    fn format_size_table() {
        for (bytes, want) in [
            (0, "0 B"),
            (1, "1.000 B"),
            (1000, "1000.000 B"),
            (1024, "1.000 KB"),
            (1536, "1.500 KB"),
            (1_048_576, "1.000 MB"),
            (1_000_000, "976.563 KB"),
            (5_368_709_120, "5.000 GB"),
        ] {
            assert_eq!(format_size(bytes, &SIZE_UNITS), want, "{bytes}");
        }
    }

    #[test]
    fn extension() {
        assert_eq!(file_extension("a.PDF"), ".PDF");
        assert_eq!(file_extension("a.tar.gz"), ".gz");
        assert_eq!(file_extension("noext"), ".noext");
    }

    #[test]
    fn accept_table() {
        // (file, accept, expectation)
        let cases = [
            (f("a.png", "image/png", 1), "image/*", true),
            (f("a.pdf", "application/pdf", 1), "image/*", false),
            (f("a.pdf", "application/pdf", 1), "application/pdf", true),
            (f("a.pdf", "", 1), ".PDF", true),
            (f("a.pdf", "", 1), ".pdf, .doc", true),
            (f("a.txt", "text/plain", 1), "image/*, .pdf", false),
            (f("a.txt", "text/plain", 1), "image/*, text/*", true),
            (f("a.jpg", "image/jpeg", 1), " image/png , image/jpeg ", true),
        ];
        for (file, accept, want) in cases {
            assert_eq!(is_type_valid(&file, accept), want, "{} / {accept}", file.name);
        }
    }

    #[test]
    fn validate_table() {
        let pdf = f("a.pdf", "application/pdf", 2048);
        assert_eq!(validate(&pdf, None, None), Ok(()));
        assert_eq!(validate(&pdf, Some(""), None), Ok(()));
        assert_eq!(validate(&pdf, Some("image/*"), None), Err(Rejection::Type));
        assert_eq!(validate(&pdf, None, Some(1000)), Err(Rejection::Size));
        assert_eq!(validate(&pdf, None, Some(2048)), Ok(()));
        assert_eq!(validate(&pdf, None, Some(0)), Ok(()), "zero means no limit");
        assert_eq!(validate(&pdf, Some("image/*"), Some(1000)), Err(Rejection::Type), "the type is checked first");
    }

    #[test]
    fn duplicates() {
        let a = f("a.txt", "text/plain", 5);
        assert!(is_duplicate(std::slice::from_ref(&a), &a));
        assert!(!is_duplicate(std::slice::from_ref(&a), &f("a.txt", "text/plain", 6)));
        assert!(!is_duplicate(&[], &a));
    }

    #[test]
    fn limits() {
        assert!(!limit_exceeded(false, 1, 0, Some(1)));
        assert!(limit_exceeded(false, 2, 0, Some(1)));
        assert!(limit_exceeded(false, 1, 1, Some(1)), "pending and uploaded ones are counted together");
        assert!(!limit_exceeded(true, 1, 5, Some(1)), "in auto mode uploaded ones are not counted");
        assert!(!limit_exceeded(false, 9, 9, None));
        assert!(choose_disabled(false, 1, 0, Some(1)));
        assert!(!choose_disabled(false, 0, 0, Some(1)));
        assert!(choose_disabled(false, 0, 2, Some(2)));
        assert!(!choose_disabled(true, 0, 2, Some(2)));
    }

    #[test]
    fn basic_label_table() {
        let none = "No file chosen";
        assert_eq!(basic_label(true, "Choose", &[], "{0} files", none), "Choose");
        assert_eq!(basic_label(false, "Choose", &[], "{0} files", none), none);
        assert_eq!(basic_label(false, "Choose", &["a.txt".into()], "{0} files", none), "a.txt");
        assert_eq!(basic_label(false, "Choose", &["a".into(), "b".into()], "{0} files", none), "2 files");
        assert_eq!(basic_label(false, "Choose", &["a".into(), "b".into()], "Files", none), "Files");
    }

    #[test]
    fn image_detection() {
        assert!(is_image(&f("a.png", "image/png", 1)));
        assert!(!is_image(&f("a.txt", "text/plain", 1)));
    }
}
