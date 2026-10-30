/// Total number of characters (not bytes) across all words. Borrows the list:
/// the caller still owns it afterwards.
pub fn total_len(words: &[String]) -> usize {
    todo!("implement total_len({words:?})")
}

/// Change every word in place: uppercase it and add "!" at the end.
/// `["hi", "there"]` becomes `["HI!", "THERE!"]`.
pub fn shout(words: &mut Vec<String>) {
    todo!("implement shout({words:?})")
}

/// Join the words with spaces into a sentence: the first letter uppercased and
/// a "." at the end. `["the", "end"]` gives `"The end."`, and no words give
/// `""`. Takes ownership: the caller can't use `words` afterwards.
pub fn into_sentence(words: Vec<String>) -> String {
    todo!("implement into_sentence({words:?})")
}
