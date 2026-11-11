// Stage 7 of the task tracker project. Don't edit this file: `learn`
// overwrites it. Your code goes in persist.ts.

import { test } from "node:test";
import assert from "node:assert/strict";
import { apply, emptyBoard, type Board } from "./board.ts";
import { isTask, parseBoard, toJSON } from "./persist.ts";
import { createTask, taskId } from "./tasks.ts";

const TASK = { id: "t-1", title: "Write docs", status: "todo", priority: 2, assignee: null, due: "2026-12-01" };
const text = (tasks: unknown[], version: unknown = 1) => JSON.stringify({ version, tasks });

function error(input: string): string {
  const r = parseBoard(input);
  assert.equal(r.ok, false, `${input} should be rejected`);
  return r.ok ? "" : r.error;
}

test("c39_a_board_survives_a_round_trip", () => {
  let board: Board = emptyBoard;
  for (const [n, title] of [[1, "Write docs"], [2, 'Say "hi"']] as const) {
    const r = apply(board, { type: "created", task: createTask(taskId(`t-${n}`), title, { due: "2026-12-0" + n }) });
    assert.ok(r.ok);
    board = r.value;
  }
  const json = toJSON(board);
  assert.equal(JSON.parse(json).version, 1, "the format has a version");
  const back = parseBoard(json);
  assert.ok(back.ok, JSON.stringify(back));
  assert.deepEqual(back.value, board);
  assert.ok([...back.value.values()].every(Object.isFrozen), "loaded tasks are frozen");
});

test("c40_bad_json_is_an_error_not_an_exception", () => {
  assert.match(error("{ not json"), /^not JSON/);
  assert.match(error(""), /^not JSON/);
  assert.equal(error("[1, 2]"), "expected an object");
});

test("c41_the_version_and_the_tasks_array_are_checked", () => {
  assert.equal(error(text([], 2)), "unsupported version: 2");
  assert.equal(error(JSON.stringify({ tasks: [] })), "unsupported version: undefined");
  assert.equal(error(JSON.stringify({ version: 1, tasks: {} })), "tasks: expected an array");
});

test("c42_each_field_is_checked_and_named_with_its_path", () => {
  const cases: [Record<string, unknown>, string][] = [
    [{ id: "1" }, "tasks[1].id: expected a task id like t-1"],
    [{ title: "" }, "tasks[1].title: expected 1 to 80 characters"],
    [{ status: "blocked" }, "tasks[1].status: expected todo, doing or done"],
    [{ priority: "2" }, "tasks[1].priority: expected 1, 2 or 3"],
    [{ assignee: 7 }, "tasks[1].assignee: expected a string or null"],
    [{ due: "2026-02-30" }, "tasks[1].due: expected a YYYY-MM-DD date or null"],
  ];
  for (const [change, message] of cases) {
    assert.equal(error(text([TASK, { ...TASK, id: "t-2", ...change }])), message);
  }
  assert.equal(error(text([TASK, null])), "tasks[1]: expected an object");
});

test("c43_duplicate_ids_are_rejected", () => {
  assert.equal(error(text([TASK, { ...TASK, title: "again" }])), "tasks[1].id: duplicate t-1");
});

test("c44_istask_is_a_type_guard_for_unknown_values", () => {
  const values: unknown[] = [TASK, { ...TASK, priority: 4 }, "t-1", null, { ...TASK, due: null }];
  assert.deepEqual(values.map(isTask), [true, false, false, false, true]);
});
