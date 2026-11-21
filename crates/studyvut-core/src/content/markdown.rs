//! What the core needs to know about Markdown: front matter, the title,
//! referenced images and unpaired math delimiters. Rendering is the
//! frontend's job (marked and KaTeX).

/// Split `+++`-delimited TOML front matter from the text. Without front
/// matter, the whole text is the body.
pub fn split_front_matter(source: &str) -> (Option<&str>, &str) {
    let source = source.strip_prefix('\u{feff}').unwrap_or(source);
    let Some(rest) = source
        .strip_prefix("+++\n")
        .or_else(|| source.strip_prefix("+++\r\n"))
    else {
        return (None, source);
    };
    // The closing line may end the file.
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        if line.trim_end() == "+++" {
            let body = &rest[offset + line.len()..];
            return (Some(&rest[..offset]), body.trim_start_matches(['\n', '\r']));
        }
        offset += line.len();
    }
    (None, source)
}

/// The text of the first `# ` heading.
pub fn first_heading(body: &str) -> Option<&str> {
    body.lines()
        .find_map(|l| l.strip_prefix("# "))
        .map(str::trim)
        .filter(|t| !t.is_empty())
}

/// The targets of `![alt](target "title")`, in order, without titles.
pub fn image_refs(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("![") {
        rest = &rest[start + 2..];
        let Some(close) = rest.find("](") else { break };
        let after = &rest[close + 2..];
        let Some(end) = after.find(')') else { break };
        let target = after[..end].trim();
        let path = target.split_whitespace().next().unwrap_or("");
        let path = path.trim_start_matches('<').trim_end_matches('>');
        if !path.is_empty() {
            out.push(path.to_owned());
        }
        rest = &after[end..];
    }
    out
}

/// Whether the `$` math delimiters pair up, ignoring `\$` and code.
pub fn dollars_balanced(text: &str) -> bool {
    let mut count = 0usize;
    let mut fenced = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        let mut in_code = false;
        let mut prev = '\0';
        for c in line.chars() {
            match c {
                '`' => in_code = !in_code,
                '$' if !in_code && prev != '\\' => count += 1,
                _ => {}
            }
            prev = if prev == '\\' && c == '\\' { '\0' } else { c };
        }
    }
    count.is_multiple_of(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn front_matter_is_split_off() {
        let (fm, body) = split_front_matter("+++\ntitle = \"A\"\n+++\n\n# B\ntext");
        assert_eq!(fm, Some("title = \"A\"\n"));
        assert_eq!(body, "# B\ntext");
        assert_eq!(split_front_matter("# Only\n"), (None, "# Only\n"));
        assert_eq!(split_front_matter("+++\nunclosed"), (None, "+++\nunclosed"));
        assert_eq!(split_front_matter("+++\nx = 1\n+++"), (Some("x = 1\n"), ""));
    }

    #[test]
    fn the_first_heading_is_the_title() {
        assert_eq!(first_heading("intro\n# Title \n## Sub"), Some("Title"));
        assert_eq!(first_heading("## Sub only"), None);
    }

    #[test]
    fn image_targets_are_found() {
        let md = "![a](images/x.svg \"Cap\") text ![](images/y.png) [link](z)";
        assert_eq!(image_refs(md), vec!["images/x.svg", "images/y.png"]);
    }

    #[test]
    fn dollars_must_pair_up() {
        assert!(dollars_balanced("$a$ and $$b$$"));
        assert!(!dollars_balanced("$a and b"));
        assert!(dollars_balanced("costs \\$5"));
        assert!(dollars_balanced("`echo $HOME`"));
        assert!(dollars_balanced("```sh\necho $X\n```\n$y$"));
        assert!(
            !dollars_balanced("\\\\$x"),
            "an escaped backslash does not escape the dollar"
        );
    }
}
