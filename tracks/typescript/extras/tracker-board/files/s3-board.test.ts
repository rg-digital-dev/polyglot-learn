// Stage 3 of the task tracker project. Don't edit this file: `learn`
// overwrites it. Your code goes in board.ts.

import { test } from "node:test";
import assert from "node:assert/strict";
import { apply, emptyBoard, replay, type Board, type Result } from "./board.ts";
import type { TaskEvent } from "./events.ts";
import { createTask, taskId } from "./tasks.ts";

const t1 = taskId("t-1");
const t2 = taskId("t-2");
const make = (id = t1, title = "Write docs"): TaskEvent => ({ type: "created", task: createTask(id, title) });

function value(r: Result<Board>): Board {
  assert.ok(r.ok, `expected ok, got the error ${JSON.stringify(!r.ok && r.error)}`);
  return r.value;
}

const start = () => value(apply(value(apply(emptyBoard, make())), make(t2, "Ship")));

test("c14_created_adds_a_task", () => {
  const board = value(apply(emptyBoard, make()));
  assert.equal(board.size, 1);
  assert.equal(board.get(t1)?.title, "Write docs");
});

test("c15_apply_never_changes_the_board_it_was_given", () => {
  const before = start();
  const copy = new Map(before);
  const after = value(apply(before, { type: "moved", id: t1, to: "done" }));
  assert.notEqual(after, before, "apply returns a new Map");
  assert.deepEqual(before, copy, "the old board is unchanged");
  assert.equal(before.get(t1)?.status, "todo", "and so is the old task");
  assert.equal(after.get(t2), before.get(t2), "untouched tasks are shared, not copied");
});

test("c16_rename_move_and_assign_change_one_task", () => {
  let b = start();
  b = value(apply(b, { type: "renamed", id: t1, title: "  Write the docs " }));
  b = value(apply(b, { type: "moved", id: t1, to: "doing" }));
  b = value(apply(b, { type: "assigned", id: t1, assignee: "ada" }));
  assert.deepEqual(b.get(t1), { id: t1, title: "Write the docs", status: "doing", priority: 2, assignee: "ada", due: null });
  assert.ok(Object.isFrozen(b.get(t1)), "changed tasks are frozen too");
  assert.equal(b.get(t2)?.title, "Ship");
});

test("c17_deleted_removes_the_task", () => {
  const b = value(apply(start(), { type: "deleted", id: t1 }));
  assert.deepEqual([...b.keys()], [t2]);
});

test("c18_invalid_events_are_errors_not_exceptions", () => {
  const b = start();
  const cases: [TaskEvent, string][] = [
    [make(), "t-1 already exists"],
    [{ type: "moved", id: taskId("t-9"), to: "done" }, "no task t-9"],
    [{ type: "deleted", id: taskId("t-9") }, "no task t-9"],
    [{ type: "moved", id: t1, to: "todo" }, "t-1 is already todo"],
    [{ type: "renamed", id: t1, title: "  " }, "title must be 1 to 80 characters"],
  ];
  for (const [event, error] of cases) {
    assert.deepEqual(apply(b, event), { ok: false, error }, JSON.stringify(event));
  }
});

test("c19_replay_stops_at_the_first_bad_event", () => {
  const events: TaskEvent[] = [make(), { type: "moved", id: t1, to: "done" }, make(t2, "Ship")];
  const b = value(replay(events) as Result<Board>);
  assert.deepEqual([...b.values()].map((t) => [t.id, t.status]), [["t-1", "done"], ["t-2", "todo"]]);
  assert.deepEqual(replay([...events, { type: "deleted", id: taskId("t-7") }, make()]), { ok: false, error: { index: 3, error: "no task t-7" } });
  assert.deepEqual(replay([]), { ok: true, value: emptyBoard });
});
