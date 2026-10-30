#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turn {
    Left,
    Right,
}

/// One instruction for the robot. Each variant carries different data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Move this many steps in the direction the robot faces.
    Forward(i32),
    Turn(Turn),
    /// Add the text to the robot's log.
    Say(String),
    /// Jump straight to a position, keeping the direction.
    Teleport { x: i32, y: i32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub facing: Direction,
    pub log: Vec<String>,
}

impl Direction {
    /// The direction after turning 90 degrees. North is up, East is right.
    pub fn turn(self, t: Turn) -> Direction {
        todo!("implement Direction::turn({self:?}, {t:?})")
    }
}

/// Parse one line: `forward 3`, `left`, `right`, `say <any text>`,
/// `teleport <x> <y>`. Anything else, including bad numbers, is `None`.
pub fn parse(line: &str) -> Option<Command> {
    todo!("implement parse({line:?})")
}

impl Robot {
    /// A robot at (0, 0), facing North, with an empty log.
    pub fn new() -> Robot {
        todo!("implement Robot::new")
    }

    /// Carry out one command. Moving North adds to `y`, East adds to `x`.
    pub fn apply(&mut self, cmd: &Command) {
        todo!("implement Robot::apply({cmd:?}) for {self:?}")
    }
}

/// Run a script (one command per line) on a new robot. Lines that don't parse
/// are skipped.
pub fn run(script: &str) -> Robot {
    todo!("implement run({script:?})")
}
