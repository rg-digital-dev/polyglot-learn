// Part of the Deeper TypeScript module (tracks/typescript/extras/pick-fields). Don't
// edit this file: `learn` overwrites it. Your code goes in pick-fields.ts.

import { test } from "node:test";
import assert from "node:assert/strict";
import { groupBy, pick, pluck } from "./pick-fields.ts";

interface Book {
  title: string;
  author: string;
  year: number;
  subtitle?: string;
}

const books: Book[] = [
  { title: "Dune", author: "Herbert", year: 1965 },
  { title: "Emma", author: "Austen", year: 1815, subtitle: "A Novel" },
  { title: "Persuasion", author: "Austen", year: 1817 },
  { title: "Solaris", author: "Lem", year: 1961 },
];

test("c01_pick_copies_the_chosen_fields", () => {
  assert.deepStrictEqual(pick(books[0], ["title", "year"]), { title: "Dune", year: 1965 });
  assert.deepStrictEqual(pick({ x: 1, y: 2, z: 3 }, ["z"]), { z: 3 });
});

test("c02_pick_returns_a_new_object", () => {
  const book = { title: "Dune", author: "Herbert", year: 1965 };
  const picked = pick(book, ["title", "author", "year"]);
  assert.notStrictEqual(picked, book, "pick should build a new object, not return obj");
  assert.deepStrictEqual(picked, book);
  assert.deepStrictEqual(book, { title: "Dune", author: "Herbert", year: 1965 });
});

test("c03_pick_with_no_keys_gives_an_empty_object", () => {
  assert.deepStrictEqual(pick(books[1], []), {});
});

test("c04_pick_leaves_out_a_missing_optional_field", () => {
  // books[0] has no subtitle: the result must not get `subtitle: undefined`.
  assert.deepStrictEqual(pick(books[0], ["title", "subtitle"]), { title: "Dune" });
  assert.deepStrictEqual(pick(books[1], ["title", "subtitle"]), { title: "Emma", subtitle: "A Novel" });
});

test("c05_pluck_collects_one_field_from_each_item", () => {
  assert.deepStrictEqual(pluck(books, "year"), [1965, 1815, 1817, 1961]);
  assert.deepStrictEqual(pluck(books, "author"), ["Herbert", "Austen", "Austen", "Lem"]);
});

test("c06_pluck_of_no_items_is_empty", () => {
  assert.deepStrictEqual(pluck([] as Book[], "title"), []);
});

test("c07_groupby_keeps_input_order_inside_each_group", () => {
  // Spread into a plain object so a null-prototype result compares equal too.
  assert.deepStrictEqual({ ...groupBy(books, (b) => b.author) }, {
    Herbert: [books[0]],
    Austen: [books[1], books[2]],
    Lem: [books[3]],
  });
});

test("c08_groupby_with_number_keys", () => {
  const byCentury = groupBy(books, (b) => Math.floor(b.year / 100) + 1);
  assert.deepStrictEqual({ ...byCentury }, { 19: [books[1], books[2]], 20: [books[0], books[3]] });
});

test("c09_groupby_of_no_items_is_an_empty_object", () => {
  assert.deepStrictEqual({ ...groupBy([] as Book[], (b) => b.author) }, {});
});
