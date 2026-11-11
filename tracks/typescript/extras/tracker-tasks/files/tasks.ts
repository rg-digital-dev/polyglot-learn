// Tasks and their IDs.

// TODO: these types are too loose: tasks.types.ts checks that the type checker
// can tell a TaskId from any string, that only real statuses and priorities
// fit, and that a task's fields can't be changed. Tighten each one.
export type TaskId = string;

export const STATUSES = ["todo", "doing", "done"];
export type Status = string;
export type Priority = number;

export interface Task {
  id: TaskId;
  title: string;
  status: Status;
  priority: Priority;
  assignee: string | null;
  /** A YYYY-MM-DD date, or null. */
  due: string | null;
}

/** "t-" followed by digits. Throw "not a task id: <raw>" otherwise. */
export function taskId(raw: string): TaskId {
  throw new Error("TODO: implement taskId");
}

/** Whether `value` is one of the three statuses. */
export function isStatus(value: unknown): value is Status {
  throw new Error("TODO: implement isStatus");
}

/** A title, trimmed: 1 to 80 characters. Throw "title must be 1 to 80 characters" otherwise. */
export function checkTitle(title: string): string {
  throw new Error("TODO: implement checkTitle");
}

/** Whether `s` is a real calendar date written as YYYY-MM-DD (no 2026-02-30). */
export function isDate(s: string): boolean {
  throw new Error("TODO: implement isDate");
}

/**
 * A new task: status "todo", priority 2 unless given, no assignee, the title
 * checked with checkTitle. A due date that isn't a real date throws
 * "bad due date: <due>". The task is frozen.
 */
export function createTask(id: TaskId, title: string, options: { priority?: Priority; due?: string } = {}): Task {
  throw new Error("TODO: implement createTask");
}
