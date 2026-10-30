// Part of the Deeper Rust module (tracks/rust/extras/robot-commands). Don't
// edit this file: `learn` overwrites it. Your code goes in src/lib.rs.

use robot_commands::*;

fn robot_at(x: i32, y: i32, facing: Direction) -> Robot {
    Robot { x, y, facing, log: Vec::new() }
}

#[test]
fn c01_turning_right_goes_clockwise() {
    use Direction::*;
    assert_eq!(North.turn(Turn::Right), East);
    assert_eq!(East.turn(Turn::Right), South);
    assert_eq!(South.turn(Turn::Right), West);
    assert_eq!(West.turn(Turn::Right), North);
}

#[test]
fn c02_turning_left_goes_counter_clockwise() {
    use Direction::*;
    assert_eq!(North.turn(Turn::Left), West);
    assert_eq!(West.turn(Turn::Left), South);
    assert_eq!(South.turn(Turn::Left), East);
    assert_eq!(East.turn(Turn::Left), North);
}

#[test]
fn c03_parse_reads_forward_with_a_distance() {
    assert_eq!(parse("forward 3"), Some(Command::Forward(3)));
    assert_eq!(parse("forward -2"), Some(Command::Forward(-2)));
}

#[test]
fn c04_parse_reads_turns() {
    assert_eq!(parse("left"), Some(Command::Turn(Turn::Left)));
    assert_eq!(parse("right"), Some(Command::Turn(Turn::Right)));
}

#[test]
fn c05_parse_reads_say_with_the_rest_of_the_line() {
    assert_eq!(parse("say hello there"), Some(Command::Say("hello there".to_string())));
}

#[test]
fn c06_parse_reads_teleport_with_two_numbers() {
    assert_eq!(parse("teleport 4 -7"), Some(Command::Teleport { x: 4, y: -7 }));
}

#[test]
fn c07_parse_rejects_unknown_or_malformed_lines() {
    for line in ["", "jump", "forward", "forward three", "left now", "teleport 1", "teleport a b", "say"] {
        assert_eq!(parse(line), None, "parse({line:?}) should be None");
    }
}

#[test]
fn c08_a_new_robot_starts_at_the_origin_facing_north() {
    assert_eq!(Robot::new(), robot_at(0, 0, Direction::North));
}

#[test]
fn c09_forward_moves_in_the_facing_direction() {
    let mut r = robot_at(0, 0, Direction::North);
    r.apply(&Command::Forward(2));
    assert_eq!((r.x, r.y), (0, 2));
    r.apply(&Command::Turn(Turn::Right));
    r.apply(&Command::Forward(3));
    assert_eq!((r.x, r.y, r.facing), (3, 2, Direction::East));
    r.apply(&Command::Turn(Turn::Right));
    r.apply(&Command::Forward(5));
    assert_eq!((r.x, r.y), (3, -3));
    r.apply(&Command::Teleport { x: 10, y: 10 });
    assert_eq!((r.x, r.y, r.facing), (10, 10, Direction::South));
}

#[test]
fn c10_say_adds_to_the_log() {
    let mut r = robot_at(0, 0, Direction::North);
    r.apply(&Command::Say("beep".to_string()));
    r.apply(&Command::Say("boop".to_string()));
    assert_eq!(r.log, vec!["beep", "boop"]);
}

#[test]
fn c11_run_executes_a_script_and_skips_bad_lines() {
    let r = run("forward 2\nright\nsay turning\nfly away\n\nforward 1\n");
    assert_eq!(r, Robot { x: 1, y: 2, facing: Direction::East, log: vec!["turning".to_string()] });
}
