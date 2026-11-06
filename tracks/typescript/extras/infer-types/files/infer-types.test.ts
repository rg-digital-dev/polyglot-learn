// Part of the TypeScript in Practice module (tracks/typescript/extras/infer-types). Don't
// edit this file: `learn` overwrites it. Your code goes in infer-types.ts.
//
// Case 1, "types check", isn't here: learn runs tsc on infer-types.ts and
// infer-types.types.ts and reports the result as that case.

import { test } from "node:test";
import assert from "node:assert/strict";
import { eventName, eventOf, first, memoize } from "./infer-types.ts";

test("c02_first_returns_the_first_element", () => {
  assert.equal(first([3, 1, 2]), 3);
  assert.equal(first(["only"]), "only");
  assert.equal(first([]), undefined);
});

test("c03_eventname_capitalizes_the_event", () => {
  assert.equal(eventName("click"), "onClick");
  assert.equal(eventName("keyDown"), "onKeyDown");
  assert.equal(eventName(""), "on");
});

test("c04_eventof_strips_the_on_prefix", () => {
  assert.equal(eventOf("onClick"), "click");
  assert.equal(eventOf("onKeyDown"), "keyDown");
  assert.equal(eventOf("on"), "");
});

test("c05_memoize_calls_the_function_once_per_argument", () => {
  const calls: string[] = [];
  const shout = memoize((s: string) => {
    calls.push(s);
    return s.toUpperCase();
  });
  assert.equal(shout("a"), "A");
  assert.equal(shout("b"), "B");
  assert.equal(shout("a"), "A");
  assert.deepEqual(calls, ["a", "b"]);
});

test("c06_memoize_returns_the_cached_value", () => {
  const make = memoize((name: string) => ({ name }));
  const first = make("ada");
  assert.equal(make("ada"), first, "the same object, not a new one");
  assert.notEqual(make("bo"), first);
});
