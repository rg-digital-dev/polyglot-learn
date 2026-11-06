/// Numbers from `start` towards `end` (not included), `step` apart. A negative
/// step counts down. A step of 0, or a start already at or past the end, yields
/// nothing. Stops instead of overflowing `i64`.
#[derive(Debug, Clone)]
pub struct Steps {
    current: i64,
    end: i64,
    step: i64,
    done: bool,
}

impl Steps {
    pub fn new(start: i64, end: i64, step: i64) -> Steps {
        Steps { current: start, end, step, done: step == 0 }
    }
}

impl Iterator for Steps {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        todo!("implement next for {self:?}")
    }
}

/// The words of a text (split on whitespace), as slices of the text itself:
/// nothing is copied.
#[derive(Debug, Clone)]
pub struct Words<'a> {
    rest: &'a str,
}

impl<'a> Words<'a> {
    pub fn new(text: &'a str) -> Words<'a> {
        Words { rest: text }
    }
}

impl<'a> Iterator for Words<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        todo!("implement next for {self:?}")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
}

impl Grid {
    /// Every cell as (x, y): the whole first row (y = 0) from x = 0, then the
    /// next row. Build it from ranges and adapters; no new struct needed.
    pub fn cells(&self) -> impl Iterator<Item = (u32, u32)> {
        std::iter::from_fn(|| todo!("implement cells"))
    }
}
