// Stage 4 of the task tracker project. Don't edit this file: `learn`
// overwrites it. Your code goes in store.ts.
//
// Case 20, "types check", isn't here: learn runs tsc on store.ts and
// store.types.ts (and every earlier stage's types file) and reports the
// result as that case.

import { test } from "node:test";
import assert from "node:assert/strict";
import { describe, type TaskEvent } from "./events.ts";
import { TaskStore } from "./store.ts";
import { createTask, taskId } from "./tasks.ts";

const t1 = taskId("t-1");
const create = (id = t1): TaskEvent => ({ type: "created", task: createTask(id, `task ${id}`) });

test("c21_dispatch_applies_and_logs_the_event", () => {
  const store = new TaskStore();
  assert.equal(store.board.size, 0);
  const r = store.dispatch(create());
  assert.ok(r.ok);
  assert.equal(store.board.get(t1)?.title, "task t-1");
  assert.deepEqual(store.log.map(describe), ['created t-1 "task t-1"']);
});

test("c22_handlers_get_only_their_event_type", () => {
  const store = new TaskStore();
  const seen: string[] = [];
  store.on("moved", (e) => seen.push(`moved:${e.to}`));
  store.on("created", (e) => seen.push(`created:${e.task.id}`));
  store.on("moved", (e) => seen.push(`again:${e.id}`));
  store.dispatch(create());
  store.dispatch({ type: "moved", id: t1, to: "doing" });
  store.dispatch({ type: "renamed", id: t1, title: "x" });
  assert.deepEqual(seen, ["created:t-1", "moved:doing", "again:t-1"]);
});

test("c23_the_returned_function_unsubscribes", () => {
  const store = new TaskStore();
  let count = 0;
  const stop = store.on("created", () => count++);
  store.dispatch(create());
  stop();
  store.dispatch(create(taskId("t-2")));
  assert.equal(count, 1);
  stop();
});

test("c24_a_failed_dispatch_changes_nothing", () => {
  const store = new TaskStore();
  store.dispatch(create());
  const before = store.board;
  let told = 0;
  store.on("created", () => told++);
  store.onAny(() => told++);
  assert.deepEqual(store.dispatch(create()), { ok: false, error: "t-1 already exists" });
  assert.equal(store.board, before);
  assert.equal(store.log.length, 1);
  assert.equal(told, 0, "no handler hears about a failed event");
});

test("c25_onany_sees_every_event_with_the_new_board", () => {
  const start = new TaskStore().board;
  const store = new TaskStore(start);
  const seen: string[] = [];
  const stop = store.onAny((e, board) => seen.push(`${e.type}:${board.size}`));
  store.dispatch(create());
  store.dispatch(create(taskId("t-2")));
  store.dispatch({ type: "deleted", id: t1 });
  stop();
  store.dispatch({ type: "deleted", id: taskId("t-2") });
  assert.deepEqual(seen, ["created:1", "created:2", "deleted:1"]);
});
