// Stage 1 of the task tracker project. Don't edit this file: `learn`
// overwrites it. Your code goes in tasks.ts.
//
// Case 1, "types check", isn't here: learn runs tsc on tasks.ts and
// tasks.types.ts and reports the result as that case.

import { test } from "node:test";
import assert from "node:assert/strict";
import { createTask, isStatus, taskId } from "./tasks.ts";

// The untouched stub throws this; it doesn't count as the expected error.
const notStub = (e: unknown) => !(e instanceof Error && e.message.startsWith("TODO: implement"));
const withMessage = (message: string) => (e: unknown) => notStub(e) && e instanceof Error && e.message === message;

test("c02_taskid_accepts_t_and_digits_only", () => {
  assert.equal(taskId("t-1"), "t-1");
  assert.equal(taskId("t-2048"), "t-2048");
  for (const raw of ["t-", "t1", "T-1", "t-1a", " t-1", "task-1", ""]) {
    assert.throws(() => taskId(raw), withMessage(`not a task id: ${raw}`), JSON.stringify(raw));
  }
});

test("c03_createtask_fills_in_the_defaults", () => {
  assert.deepEqual(createTask(taskId("t-1"), "Write docs"), {
    id: "t-1",
    title: "Write docs",
    status: "todo",
    priority: 2,
    assignee: null,
    due: null,
  });
  const t = createTask(taskId("t-2"), "Ship", { priority: 3, due: "2026-12-01" });
  assert.equal(t.priority, 3);
  assert.equal(t.due, "2026-12-01");
});

test("c04_createtask_trims_and_checks_the_title", () => {
  assert.equal(createTask(taskId("t-1"), "  Tidy up  ").title, "Tidy up");
  assert.equal(createTask(taskId("t-1"), "x".repeat(80)).title.length, 80);
  for (const title of ["", "   ", "x".repeat(81)]) {
    assert.throws(() => createTask(taskId("t-1"), title), withMessage("title must be 1 to 80 characters"), `${title.length} characters`);
  }
});

test("c05_createtask_checks_the_due_date", () => {
  for (const due of ["2026-13-01", "2026-02-30", "1/12/2026", "2026-1-5", "soon"]) {
    assert.throws(() => createTask(taskId("t-1"), "x", { due }), withMessage(`bad due date: ${due}`), due);
  }
  assert.equal(createTask(taskId("t-1"), "x", { due: "2028-02-29" }).due, "2028-02-29", "a leap day is a date");
});

test("c06_tasks_are_frozen", () => {
  const t = createTask(taskId("t-1"), "x");
  assert.ok(Object.isFrozen(t), "createTask should return Object.freeze({...})");
  assert.throws(() => {
    (t as { title: string }).title = "changed";
  }, TypeError);
});

test("c07_isstatus_knows_the_three_statuses", () => {
  assert.deepEqual(["todo", "doing", "done", "blocked", "TODO", 1, null].map(isStatus), [true, true, true, false, false, false, false]);
});
