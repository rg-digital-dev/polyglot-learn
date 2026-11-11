// Stage 5 of the task tracker project. Don't edit this file: `learn`
// overwrites it. Your code goes in history.ts.

import { test } from "node:test";
import assert from "node:assert/strict";
import { apply, emptyBoard, type Result } from "./board.ts";
import type { TaskEvent } from "./events.ts";
import { History } from "./history.ts";
import { createTask, taskId } from "./tasks.ts";

const t1 = taskId("t-1");
const create = (n: number): TaskEvent => ({ type: "created", task: createTask(taskId(`t-${n}`), `task ${n}`) });
const ids = (h: History<ReadonlyMap<string, unknown>, TaskEvent>) => [...h.state.keys()];

/** A counter that can't go below zero. */
const counter = (n: number, by: number): Result<number> => (n + by < 0 ? { ok: false, error: "below zero" } : { ok: true, value: n + by });

test("c26_do_applies_events_through_the_reducer", () => {
  const h = new History(apply, emptyBoard);
  assert.equal(h.canUndo, false);
  assert.ok(h.do(create(1)).ok);
  assert.ok(h.do({ type: "moved", id: t1, to: "done" }).ok);
  assert.equal(h.state.get(t1)?.status, "done");
  assert.equal(h.canUndo, true);
});

test("c27_undo_and_redo_walk_back_and_forth", () => {
  const h = new History(apply, emptyBoard);
  h.do(create(1));
  h.do(create(2));
  h.do(create(3));
  assert.equal(h.undo(), true);
  assert.equal(h.undo(), true);
  assert.deepEqual(ids(h), ["t-1"]);
  assert.equal(h.canRedo, true);
  assert.equal(h.redo(), true);
  assert.deepEqual(ids(h), ["t-1", "t-2"]);
  h.undo();
  h.undo();
  assert.equal(h.undo(), false, "nothing left to undo");
  assert.equal(h.state, emptyBoard);
});

test("c28_a_new_action_clears_redo", () => {
  const h = new History(apply, emptyBoard);
  h.do(create(1));
  h.do(create(2));
  h.undo();
  h.do(create(3));
  assert.equal(h.canRedo, false);
  assert.equal(h.redo(), false);
  assert.deepEqual(ids(h), ["t-1", "t-3"]);
});

test("c29_a_failed_action_leaves_the_history_alone", () => {
  const h = new History(apply, emptyBoard);
  h.do(create(1));
  h.do(create(2));
  h.undo();
  assert.deepEqual(h.do(create(1)), { ok: false, error: "t-1 already exists" });
  assert.equal(h.canRedo, true, "redo survives a failed action");
  h.redo();
  assert.deepEqual(ids(h), ["t-1", "t-2"]);
});

test("c30_the_limit_keeps_only_the_latest_steps", () => {
  const h = new History(counter, 0, 3);
  for (let i = 0; i < 5; i++) h.do(1);
  assert.equal(h.state, 5);
  let undone = 0;
  while (h.undo()) undone++;
  assert.equal(undone, 3);
  assert.equal(h.state, 2);
});

test("c31_history_works_for_any_state_and_event_type", () => {
  const h = new History(counter, 0);
  h.do(5);
  assert.deepEqual(h.do(-9), { ok: false, error: "below zero" });
  h.do(-2);
  h.undo();
  assert.equal(h.state, 5);
});
