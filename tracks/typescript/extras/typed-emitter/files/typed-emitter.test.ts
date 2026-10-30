// Part of the Deeper TypeScript module (tracks/typescript/extras/typed-emitter). Don't
// edit this file: `learn` overwrites it. Your code goes in typed-emitter.ts.

import { test } from "node:test";
import assert from "node:assert/strict";
import { Emitter } from "./typed-emitter.ts";

type Events = {
  message: [from: string, text: string];
  tick: [n: number];
  reset: [];
};

test("c01_emit_calls_a_listener_with_the_arguments", () => {
  const em = new Emitter<Events>();
  const got: string[] = [];
  em.on("message", (from, text) => got.push(`${from}: ${text}`));
  em.emit("message", "ada", "hi");
  em.emit("message", "bo", "hello");
  assert.deepStrictEqual(got, ["ada: hi", "bo: hello"]);
});

test("c02_listeners_run_in_the_order_they_were_added", () => {
  const em = new Emitter<Events>();
  const order: string[] = [];
  em.on("tick", (n) => order.push(`first ${n}`));
  em.on("tick", (n) => order.push(`second ${n}`));
  em.on("tick", (n) => order.push(`third ${n}`));
  em.emit("tick", 1);
  assert.deepStrictEqual(order, ["first 1", "second 1", "third 1"]);
});

test("c03_emit_returns_how_many_listeners_ran", () => {
  const em = new Emitter<Events>();
  em.on("reset", () => {});
  em.on("reset", () => {});
  assert.equal(em.emit("reset"), 2);
});

test("c04_emit_with_no_listeners_does_nothing", () => {
  const em = new Emitter<Events>();
  assert.equal(em.emit("tick", 5), 0);
  em.on("tick", () => {});
  assert.equal(em.emit("reset"), 0, "a listener for another event doesn't count");
});

test("c05_events_are_kept_apart", () => {
  const em = new Emitter<Events>();
  const ticks: number[] = [];
  let resets = 0;
  em.on("tick", (n) => ticks.push(n));
  em.on("reset", () => resets++);
  em.emit("tick", 1);
  em.emit("reset");
  em.emit("tick", 2);
  assert.deepStrictEqual(ticks, [1, 2]);
  assert.equal(resets, 1);
});

test("c06_the_returned_function_unsubscribes", () => {
  const em = new Emitter<Events>();
  const ticks: number[] = [];
  const unsubscribe = em.on("tick", (n) => ticks.push(n));
  em.emit("tick", 1);
  unsubscribe();
  em.emit("tick", 2);
  unsubscribe(); // a second call is harmless
  assert.deepStrictEqual(ticks, [1]);
  assert.equal(em.emit("tick", 3), 0);
});

test("c07_off_removes_one_listener_and_keeps_the_others", () => {
  const em = new Emitter<Events>();
  const got: string[] = [];
  const a = (n: number) => got.push(`a${n}`);
  const b = (n: number) => got.push(`b${n}`);
  em.on("tick", a);
  em.on("tick", b);
  em.off("tick", a);
  em.off("tick", () => {}); // never added: nothing happens
  em.off("reset", () => {}); // event with no listeners: nothing happens
  em.emit("tick", 1);
  assert.deepStrictEqual(got, ["b1"]);
});

test("c08_a_listener_can_unsubscribe_itself_while_running", () => {
  const em = new Emitter<Events>();
  const got: string[] = [];
  const unsubscribe = em.on("tick", (n) => {
    got.push(`once ${n}`);
    unsubscribe();
  });
  em.on("tick", (n) => got.push(`always ${n}`));
  assert.equal(em.emit("tick", 1), 2);
  assert.equal(em.emit("tick", 2), 1);
  assert.deepStrictEqual(got, ["once 1", "always 1", "always 2"], "removing a listener mid-emit must not skip the next one");
});

test("c09_listener_added_mid_emit_runs_from_the_next_emit", () => {
  const em = new Emitter<Events>();
  const got: number[] = [];
  let added = false;
  em.on("tick", () => {
    if (!added) {
      added = true;
      em.on("tick", (n) => got.push(n));
    }
  });
  assert.equal(em.emit("tick", 1), 1);
  assert.equal(em.emit("tick", 2), 2);
  assert.deepStrictEqual(got, [2]);
});
