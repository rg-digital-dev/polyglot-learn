/// The first word of `text`: everything before the first space, or the whole
/// text if there is no space. Return a slice of `text`, not a copy.
pub fn first_word(text: &str) -> &str {
    todo!("implement first_word({text:?})")
}

/// Whichever string has more characters; `a` when they're the same length.
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!("implement longest({a:?}, {b:?})")
}

/// A view into some text. It holds a borrow, so it can't outlive the text.
pub struct Excerpt<'a> {
    pub text: &'a str,
}

impl<'a> Excerpt<'a> {
    /// The sentences of the text: split on '.', each one trimmed, empty ones
    /// left out. Each sentence is a slice of the original text.
    pub fn sentences(&self) -> Vec<&'a str> {
        todo!("implement Excerpt::sentences for {:?}", self.text)
    }

    /// The sentence with the most characters (the first one on a tie), or
    /// `None` when there are no sentences.
    pub fn longest_sentence(&self) -> Option<&'a str> {
        todo!("implement Excerpt::longest_sentence for {:?}", self.text)
    }
}
