// Part of the Deeper TypeScript module (tracks/typescript/extras/lazy-range). Don't
// edit this file: `learn` overwrites it. Your code goes in lazy-range.ts.

import { test } from "node:test";
import assert from "node:assert/strict";
import { Countdown, map, range, take } from "./lazy-range.ts";

// The untouched stub throws this; it doesn't count as the expected error.
const notStub = (e: unknown) => !(e instanceof Error && e.message.startsWith("TODO: implement"));

// An endless sequence 0, 1, 2, ... that counts how many values were pulled
// from it, and fails instead of hanging when far too many are pulled.
function counted() {
  const stats = { pulled: 0 };
  function* naturals(): Generator<number> {
    for (let i = 0; ; i++) {
      if (stats.pulled >= 1000) throw new Error("pulled 1000 values from an endless sequence: stop earlier");
      stats.pulled++;
      yield i;
    }
  }
  return { stats, items: naturals() };
}

test("c01_range_counts_from_start_up_to_end", () => {
  assert.deepStrictEqual([...range(0, 4)], [0, 1, 2, 3]);
  assert.deepStrictEqual([...range(-2, 1)], [-2, -1, 0]);
});

test("c02_range_moves_by_step", () => {
  assert.deepStrictEqual([...range(0, 10, 3)], [0, 3, 6, 9]);
  assert.deepStrictEqual([...range(1, 2, 0.25)], [1, 1.25, 1.5, 1.75]);
});

test("c03_range_counts_down_with_a_negative_step", () => {
  assert.deepStrictEqual([...range(5, 0, -2)], [5, 3, 1]);
  assert.deepStrictEqual([...range(3, 0, -1)], [3, 2, 1]);
});

test("c04_range_is_empty_when_start_is_past_end", () => {
  assert.deepStrictEqual([...range(3, 3)], []);
  assert.deepStrictEqual([...range(5, 0)], []);
  assert.deepStrictEqual([...range(0, 5, -1)], []);
});

test("c05_range_rejects_a_step_of_zero", () => {
  assert.throws(() => [...range(0, 5, 0)], notStub);
});

test("c06_take_stops_an_endless_sequence", () => {
  assert.deepStrictEqual([...take(counted().items, 3)], [0, 1, 2]);
  assert.deepStrictEqual([...take(range(10, Infinity, 10), 4)], [10, 20, 30, 40]);
  assert.deepStrictEqual([...take([1, 2], 5)], [1, 2], "a short input just ends early");
});

test("c07_take_pulls_no_more_items_than_it_needs", () => {
  const three = counted();
  assert.deepStrictEqual([...take(three.items, 3)], [0, 1, 2]);
  assert.equal(three.stats.pulled, 3, "take(items, 3) should pull exactly 3 values");
  const none = counted();
  assert.deepStrictEqual([...take(none.items, 0)], []);
  assert.equal(none.stats.pulled, 0, "take(items, 0) should pull nothing");
});

test("c08_map_transforms_lazily", () => {
  const calls: number[] = [];
  const squares = map(counted().items, (n) => {
    calls.push(n);
    return n * n;
  });
  assert.deepStrictEqual(calls, [], "map should not call fn before a value is asked for");
  assert.deepStrictEqual([...take(squares, 4)], [0, 1, 4, 9]);
  assert.deepStrictEqual(calls, [0, 1, 2, 3]);
  assert.deepStrictEqual([...map(["a", "bb"], (s) => s.length)], [1, 2]);
});

test("c09_countdown_works_with_for_of_and_spread", () => {
  const c = new Countdown(3);
  const seen: number[] = [];
  for (const n of c) seen.push(n);
  assert.deepStrictEqual(seen, [3, 2, 1]);
  assert.deepStrictEqual([...c], [3, 2, 1], "a second loop starts again");
  assert.deepStrictEqual([...map(new Countdown(2), (n) => n * 10)], [20, 10]);
});

test("c10_countdown_from_zero_is_empty", () => {
  assert.deepStrictEqual([...new Countdown(0)], []);
});
