// Part of the Rust in Practice module (tracks/rust/extras/custom-iterator).
// Don't edit this file: `learn` overwrites it. Your code goes in src/lib.rs.

use custom_iterator::*;

fn steps(start: i64, end: i64, step: i64) -> Vec<i64> {
    Steps::new(start, end, step).collect()
}

#[test]
fn c01_steps_counts_up_to_the_end() {
    assert_eq!(steps(0, 10, 3), vec![0, 3, 6, 9]);
    assert_eq!(steps(0, 9, 3), vec![0, 3, 6]);
}

#[test]
fn c02_steps_counts_down_with_a_negative_step() {
    assert_eq!(steps(5, 0, -2), vec![5, 3, 1]);
    assert_eq!(steps(0, -3, -1), vec![0, -1, -2]);
}

#[test]
fn c03_steps_is_empty_when_it_has_nowhere_to_go() {
    assert_eq!(steps(10, 0, 1), Vec::<i64>::new());
    assert_eq!(steps(0, 10, -1), Vec::<i64>::new());
    assert_eq!(steps(3, 3, 1), Vec::<i64>::new());
    assert_eq!(steps(0, 10, 0), Vec::<i64>::new());
}

#[test]
fn c04_steps_works_with_iterator_adapters() {
    let sum: i64 = Steps::new(1, 10, 1).filter(|n| n % 2 == 0).map(|n| n * n).sum();
    assert_eq!(sum, 4 + 16 + 36 + 64);
    assert_eq!(Steps::new(0, 100, 7).count(), 15);
    assert_eq!(Steps::new(0, 100, 7).last(), Some(98));
}

#[test]
fn c05_steps_is_lazy() {
    let first: Vec<i64> = Steps::new(0, i64::MAX, 1).take(3).collect();
    assert_eq!(first, vec![0, 1, 2]);
}

#[test]
fn c06_steps_stops_instead_of_overflowing() {
    assert_eq!(steps(i64::MAX - 1, i64::MAX, 5), vec![i64::MAX - 1]);
    assert_eq!(steps(i64::MIN + 1, i64::MIN, -5), vec![i64::MIN + 1]);
}

#[test]
fn c07_words_yields_slices_of_the_original_text() {
    let text = String::from("  one two\tthree\n");
    let words: Vec<&str> = Words::new(&text).collect();
    assert_eq!(words, vec!["one", "two", "three"]);
    // The words point into `text`; they aren't copies.
    let start = text.as_ptr() as usize;
    assert_eq!(words[1].as_ptr() as usize - start, 6);
}

#[test]
fn c08_words_of_blank_text_is_empty() {
    assert_eq!(Words::new("").count(), 0);
    assert_eq!(Words::new(" \t\n ").count(), 0);
}

#[test]
fn c09_grid_cells_go_row_by_row() {
    let cells: Vec<(u32, u32)> = Grid { width: 3, height: 2 }.cells().collect();
    assert_eq!(cells, vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)]);
    assert_eq!(Grid { width: 0, height: 5 }.cells().count(), 0);
}

#[test]
fn c10_grid_cells_can_be_chained_further() {
    let grid = Grid { width: 4, height: 4 };
    let diagonal: Vec<(u32, u32)> = grid.cells().filter(|(x, y)| x == y).collect();
    assert_eq!(diagonal, vec![(0, 0), (1, 1), (2, 2), (3, 3)]);
    assert_eq!(grid.cells().skip(5).next(), Some((1, 1)));
}
