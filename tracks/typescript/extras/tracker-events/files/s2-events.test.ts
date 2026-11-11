// Stage 2 of the task tracker project. Don't edit this file: `learn`
// overwrites it. Your code goes in events.ts.
//
// Case 8, "types check", isn't here: learn runs tsc on events.ts and
// events.types.ts (and every earlier stage's types file) and reports the
// result as that case.

import { test } from "node:test";
import assert from "node:assert/strict";
import { assertNever, describe, eventsFor, taskIdOf, type TaskEvent } from "./events.ts";
import { createTask, taskId } from "./tasks.ts";

const t1 = taskId("t-1");
const t2 = taskId("t-2");
const created: TaskEvent = { type: "created", task: createTask(t1, "Write docs") };

const EVENTS: TaskEvent[] = [
  created,
  { type: "renamed", id: t1, title: "Write the docs" },
  { type: "created", task: createTask(t2, "Ship") },
  { type: "moved", id: t1, to: "doing" },
  { type: "assigned", id: t2, assignee: "ada" },
  { type: "deleted", id: t1 },
];

test("c09_describe_says_what_happened", () => {
  assert.deepEqual(EVENTS.map(describe), [
    'created t-1 "Write docs"',
    'renamed t-1 to "Write the docs"',
    'created t-2 "Ship"',
    "moved t-1 to doing",
    "assigned t-2 to ada",
    "deleted t-1",
  ]);
});

test("c10_assigning_null_is_unassigning", () => {
  assert.equal(describe({ type: "assigned", id: t2, assignee: null }), "unassigned t-2");
});

test("c11_taskidof_finds_the_id_in_every_event", () => {
  assert.deepEqual(EVENTS.map(taskIdOf), ["t-1", "t-1", "t-2", "t-1", "t-2", "t-1"]);
});

test("c12_eventsfor_keeps_one_task_s_events_in_order", () => {
  assert.deepEqual(eventsFor(EVENTS, t2).map(describe), ['created t-2 "Ship"', "assigned t-2 to ada"]);
  assert.deepEqual(eventsFor(EVENTS, taskId("t-9")), []);
});

test("c13_assertnever_throws_if_reached", () => {
  assert.throws(() => assertNever("surprise" as never), (e: unknown) => e instanceof Error && !e.message.startsWith("TODO: implement"));
});
