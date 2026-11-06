use std::borrow::Cow;

/// Trim the ends and turn every run of whitespace inside into one space.
/// Borrow (a slice of `text`) when that changes nothing but the ends.
pub fn normalize_spaces(text: &str) -> Cow<'_, str> {
    todo!("implement normalize_spaces({text:?})")
}

/// Replace & < > and " with &amp; &lt; &gt; and &quot;. Borrow `text` when
/// it has none of them.
pub fn escape_html(text: &str) -> Cow<'_, str> {
    todo!("implement escape_html({text:?})")
}

/// `name` with ".<ext>" on the end. Borrow `name` when it already ends that way.
pub fn with_extension<'a>(name: &'a str, ext: &str) -> Cow<'a, str> {
    todo!("implement with_extension({name:?}, {ext:?})")
}

/// The number of characters in all items together. Works for `&[&str]` and
/// `&[String]` alike.
pub fn total_len<S: AsRef<str>>(items: &[S]) -> usize {
    let _ = items;
    todo!("implement total_len")
}
