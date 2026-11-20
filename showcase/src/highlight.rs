//! Rust highlighting of the demo code blocks: HTML with `span`s of `tk-*` classes.
//!
//! Runs in the browser when a page renders, instead of shipping a pre-highlighted copy of every snippet inside the
//! wasm binary. Works on byte offsets of the source and writes into one pre-sized `String` (no per-token allocations).

use std::fmt::Write as _;

const KEYWORDS: &[&str] = &[
    "fn", "let", "mut", "use", "pub", "struct", "enum", "impl", "move", "if", "else", "match", "for", "in", "return", "true", "false",
    "as", "const", "static", "self", "Self", "where", "type", "mod", "crate", "super", "async", "await", "dyn",
];

fn escape_into(text: &str, out: &mut String) {
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(c),
        }
    }
}

fn span(out: &mut String, class: &str, text: &str) {
    let _ = write!(out, "<span class=\"{class}\">");
    escape_into(text, out);
    out.push_str("</span>");
}

/// Byte offset where `rest` stops satisfying `keep` (or its length).
fn take_while(rest: &str, keep: impl Fn(char) -> bool) -> usize {
    rest.char_indices().find(|&(_, c)| !keep(c)).map_or(rest.len(), |(at, _)| at)
}

/// Highlights Rust (and `view!` markup) source as HTML.
pub fn highlight(code: &str) -> String {
    let mut out = String::with_capacity(code.len() * 2);
    let mut i = 0;
    while let Some(c) = code[i..].chars().next() {
        let rest = &code[i..];
        if rest.starts_with("//") {
            let end = rest.find('\n').unwrap_or(rest.len());
            span(&mut out, "tk-c", &rest[..end]);
            i += end;
        } else if c == '"' {
            // The closing quote is the first `"` not preceded by a backslash.
            let mut prev = '"';
            let mut end = rest.len();
            for (at, ch) in rest.char_indices().skip(1) {
                if ch == '"' && prev != '\\' {
                    end = at + 1;
                    break;
                }
                prev = ch;
            }
            span(&mut out, "tk-s", &rest[..end]);
            i += end;
        } else if c.is_alphabetic() || c == '_' {
            let end = take_while(rest, |x| x.is_alphanumeric() || x == '_');
            let word = &rest[..end];
            let mut before = code[..i].chars().rev();
            let (prev, prev2) = (before.next().unwrap_or(' '), before.next());
            let class = if KEYWORDS.contains(&word) {
                Some("tk-k")
            } else if rest[end..].starts_with('!') {
                Some("tk-m")
            } else if (prev == '<' || (prev == '/' && prev2 == Some('<'))) && word.starts_with(char::is_uppercase) {
                Some("tk-t")
            } else if word.starts_with(char::is_uppercase) {
                Some("tk-y")
            } else {
                None
            };
            match class {
                Some(class) => span(&mut out, class, word),
                None => out.push_str(word),
            }
            i += end;
        } else if c.is_ascii_digit() {
            let end = take_while(rest, |x| x.is_ascii_alphanumeric() || x == '.' || x == '_');
            span(&mut out, "tk-n", &rest[..end]);
            i += end;
        } else {
            escape_into(&rest[..c.len_utf8()], &mut out);
            i += c.len_utf8();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::highlight;

    #[test]
    fn token_classes() {
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
    fn whole_output() {
        assert_eq!(
            highlight("</Card> Vec<u8> s = \"q\\\"x\" & 1.5e3 é"),
            "&lt;/<span class=\"tk-t\">Card</span>&gt; <span class=\"tk-y\">Vec</span>&lt;u8&gt; s = \
             <span class=\"tk-s\">\"q\\\"x\"</span> &amp; <span class=\"tk-n\">1.5e3</span> é"
        );
    }

    #[test]
    fn unterminated_tokens_run_to_the_end() {
        assert_eq!(highlight("\"open"), "<span class=\"tk-s\">\"open</span>");
        assert_eq!(highlight("// tail"), "<span class=\"tk-c\">// tail</span>");
        assert_eq!(highlight(""), "");
    }
}
