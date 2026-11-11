// Saving a board as JSON, and loading one back from text we don't trust.

import { err, ok, type Board, type Result } from "./board.ts";
import { isDate, isStatus, type Task, type TaskId } from "./tasks.ts";

export const FORMAT_VERSION = 1;

/** `{"version":1,"tasks":[...]}`, the tasks in board order. */
export function toJSON(board: Board): string {
  throw new Error("TODO: implement toJSON");
}

/** Whether `v` is a valid task: the same rules parseBoard checks for each one. */
export function isTask(v: unknown): v is Task {
  throw new Error("TODO: implement isTask");
}

/**
 * A board from JSON text. Never throws: every problem is an error naming
 * where it is. In order:
 *   "not JSON: <parser message>"     "expected an object"
 *   "unsupported version: <version as JSON>"     "tasks: expected an array"
 *   "tasks[i]: expected an object"
 *   "tasks[i].id: expected a task id like t-1"
 *   "tasks[i].title: expected 1 to 80 characters"
 *   "tasks[i].status: expected todo, doing or done"
 *   "tasks[i].priority: expected 1, 2 or 3"
 *   "tasks[i].assignee: expected a string or null"
 *   "tasks[i].due: expected a YYYY-MM-DD date or null"
 *   "tasks[i].id: duplicate <id>"
 * Loaded tasks are frozen.
 */
export function parseBoard(text: string): Result<Board> {
  throw new Error("TODO: implement parseBoard");
}
