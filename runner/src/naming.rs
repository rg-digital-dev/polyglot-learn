//! Name conversions. Spec function names are snake_case and slugs are
//! kebab-case; each backend converts them to its language's convention.

/// Split a snake_case, kebab-case or camelCase name into lowercase words.
pub fn words(name: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut prev_lower = false;
    for ch in name.chars() {
        if ch == '_' || ch == '-' || ch == ' ' {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            prev_lower = false;
            continue;
        }
        if ch.is_ascii_uppercase() && prev_lower && !cur.is_empty() {
            out.push(std::mem::take(&mut cur));
        }
        prev_lower = ch.is_ascii_lowercase() || ch.is_ascii_digit();
        cur.push(ch.to_ascii_lowercase());
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

pub fn snake(name: &str) -> String {
    words(name).join("_")
}

pub fn kebab(name: &str) -> String {
    words(name).join("-")
}

pub fn camel(name: &str) -> String {
    let w = words(name);
    let mut out = String::new();
    for (i, word) in w.iter().enumerate() {
        if i == 0 {
            out.push_str(word);
        } else {
            out.push_str(&capitalize(word));
        }
    }
    out
}

pub fn pascal(name: &str) -> String {
    words(name).iter().map(|w| capitalize(w)).collect()
}

/// Go package names: all lowercase, no separators (`leap-year` -> `leapyear`).
pub fn flat(name: &str) -> String {
    words(name).concat()
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

/// Turn a free-text case description into an identifier fragment:
/// `"zero is out of range"` -> `zero_is_out_of_range`.
pub fn ident(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.ends_with('_') && !out.is_empty() {
            out.push('_');
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    out.truncate(48);
    while out.ends_with('_') {
        out.pop();
    }
    out
}

/// Test name for case `index` (0-based): `c01_plain_name`. The `cNN_` prefix
/// keeps cases ordered and lets backends map output lines back to cases.
pub fn case_name(index: usize, description: &str) -> String {
    let rest = ident(description);
    if rest.is_empty() {
        format!("c{:02}", index + 1)
    } else {
        format!("c{:02}_{rest}", index + 1)
    }
}

/// Find a `cNN_` / `cNN` case marker in an output line and return its 0-based index.
pub fn find_case_index(line: &str) -> Option<usize> {
    let bytes = line.as_bytes();
    for i in 0..bytes.len().saturating_sub(2) {
        let boundary = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
        if boundary && bytes[i] == b'c' && bytes[i + 1].is_ascii_digit() && bytes[i + 2].is_ascii_digit() {
            let end_ok = bytes.get(i + 3).map_or(true, |b| !b.is_ascii_alphanumeric());
            if end_ok {
                let n: usize = line[i + 1..i + 3].parse().ok()?;
                return n.checked_sub(1);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_between_conventions() {
        assert_eq!(camel("sum_of_multiples"), "sumOfMultiples");
        assert_eq!(pascal("parse_port"), "ParsePort");
        assert_eq!(snake("leap-year"), "leap_year");
        assert_eq!(snake("leapYear"), "leap_year");
        assert_eq!(flat("sum-of-multiples"), "sumofmultiples");
        assert_eq!(kebab("top_scores"), "top-scores");
        assert_eq!(camel("greet"), "greet");
        assert_eq!(pascal("hello"), "Hello");
    }

    #[test]
    fn case_names_are_identifiers() {
        assert_eq!(case_name(0, "plain name"), "c01_plain_name");
        assert_eq!(case_name(9, "digits that overflow a 64-bit integer"), "c10_digits_that_overflow_a_64_bit_integer");
        assert_eq!(case_name(2, "Don't stop!"), "c03_don_t_stop");
        assert_eq!(case_name(3, "!!!"), "c04");
    }

    #[test]
    fn finds_case_markers_in_output_lines() {
        assert_eq!(find_case_index("test c03_empty_name ... ok"), Some(2));
        assert_eq!(find_case_index("--- FAIL: TestGreet/c10_x (0.00s)"), Some(9));
        assert_eq!(find_case_index("PASS   : Hello::test_c01_plain_name()"), Some(0));
        assert_eq!(find_case_index("not ok 4 - c04"), Some(3));
        assert_eq!(find_case_index("abc12 nothing here"), None);
        assert_eq!(find_case_index("c123 too long"), None);
    }
}
