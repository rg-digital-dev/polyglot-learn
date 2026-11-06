// Declare your statics here: an AtomicU64 for the IDs, a LazyLock for the
// colour table, an AtomicUsize that counts how often the table was built, and
// a OnceLock for the app name.
//
// use std::collections::HashMap;
// use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
// use std::sync::{LazyLock, OnceLock};

/// A new ID on every call: 1, 2, 3, ... never the same one twice, even when
/// many threads ask at once.
pub fn next_id() -> u64 {
    todo!("implement next_id")
}

/// The RGB value of a colour name, ignoring case. The table is built the first
/// time it's needed, and only once: black 0x000000, white 0xffffff,
/// red 0xff0000, green 0x00ff00, blue 0x0000ff.
pub fn color_code(name: &str) -> Option<u32> {
    todo!("implement color_code({name:?})")
}

/// How many times the colour table has been built so far (0 or 1).
pub fn table_builds() -> usize {
    todo!("implement table_builds")
}

/// Set the app name. Only the first call succeeds; later calls return the error
/// "app name is already `<the name>`".
pub fn set_app_name(name: &str) -> Result<(), String> {
    todo!("implement set_app_name({name:?})")
}

/// The app name, if it has been set.
pub fn app_name() -> Option<&'static str> {
    todo!("implement app_name")
}
