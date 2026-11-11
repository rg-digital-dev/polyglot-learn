// Stage 6 of the task tracker project. Don't edit this file: `learn`
// overwrites it. Your code goes in query.ts.
//
// Case 32, "types check", isn't here: learn runs tsc on query.ts and
// query.types.ts (and every earlier stage's types file) and reports the
// result as that case.

import { test } from "node:test";
import assert from "node:assert/strict";
import { query } from "./query.ts";
import type { Task } from "./tasks.ts";

const task = (n: number, status: Task["status"], priority: Task["priority"], due: string | null, assignee: string | null = null) =>
  ({ id: `t-${n}`, title: `task ${n}`, status, priority, due, assignee }) as Task;

const TASKS: Task[] = [
  task(1, "todo", 2, "2026-12-01"),
  task(2, "done", 3, null, "ada"),
  task(3, "doing", 1, "2026-11-20", "bob"),
  task(4, "done", 1, "2026-11-25"),
  task(5, "todo", 3, null, "ada"),
];
const ids = (ts: Task[]) => ts.map((t) => t.id);

test("c33_where_compares_a_field_with_a_value", () => {
  assert.deepEqual(ids(query(TASKS).where("status", "done").all()), ["t-2", "t-4"]);
  assert.deepEqual(ids(query(TASKS).where("assignee", "ada").where("status", "todo").all()), ["t-5"]);
  assert.deepEqual(ids(query(TASKS).where("assignee", null).all()), ["t-1", "t-4"]);
});

test("c34_where_takes_a_test_function", () => {
  assert.deepEqual(ids(query(TASKS).where("priority", (p) => p >= 3).all()), ["t-2", "t-5"]);
  assert.deepEqual(ids(query(TASKS).where("due", (d) => d !== null && d < "2026-11-30").all()), ["t-3", "t-4"]);
});

test("c35_sortby_puts_nulls_last_either_way", () => {
  assert.deepEqual(ids(query(TASKS).sortBy("due").all()), ["t-3", "t-4", "t-1", "t-2", "t-5"]);
  assert.deepEqual(ids(query(TASKS).sortBy("due", "desc").all()), ["t-1", "t-4", "t-3", "t-2", "t-5"]);
  assert.deepEqual(ids(query(TASKS).sortBy("priority", "desc").all()), ["t-2", "t-5", "t-1", "t-3", "t-4"], "ties keep their order");
});

test("c36_select_keeps_only_the_given_fields", () => {
  assert.deepEqual(query(TASKS).where("status", "doing").select("id", "assignee"), [{ id: "t-3", assignee: "bob" }]);
  assert.deepEqual(query([]).select(), []);
});

test("c37_queries_are_immutable_and_reusable", () => {
  const todo = query(TASKS).where("status", "todo");
  const urgent = todo.where("priority", 3);
  assert.deepEqual(ids(todo.all()), ["t-1", "t-5"], "adding a step made a new query");
  assert.deepEqual(ids(urgent.all()), ["t-5"]);
  const items = [...TASKS];
  const q = query(items);
  items.pop();
  assert.equal(q.count(), 5, "the query keeps its own copy of the items");
  q.sortBy("priority").all();
  assert.deepEqual(ids(q.all()), ids(TASKS), "sorting doesn't reorder the source");
});

test("c38_limit_and_count", () => {
  assert.deepEqual(ids(query(TASKS).sortBy("priority").limit(2).all()), ["t-3", "t-4"]);
  assert.equal(query(TASKS).where("status", "done").count(), 2);
  assert.equal(query(TASKS).limit(0).count(), 0);
});
