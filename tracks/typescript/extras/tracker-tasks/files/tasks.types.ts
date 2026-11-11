// Stage 1 of the task tracker project. Don't edit this file: `learn` overwrites it.
//
// Never run, only type-checked: this is case 1, "types check". Each
// `@ts-expect-error` line MUST be a type error. If your types let it through,
// tsc reports "Unused '@ts-expect-error' directive" for that line.

import { createTask, taskId } from "./tasks.ts";
import type { Priority, Status, Task, TaskId } from "./tasks.ts";

function load(id: TaskId): TaskId {
  return id;
}

const id = taskId("t-1");
load(id);
// @ts-expect-error a plain string isn't a TaskId: only taskId() makes one
load("t-1");

const s: Status = "doing";
// @ts-expect-error "blocked" isn't a status
const blocked: Status = "blocked";

const p: Priority = 3;
// @ts-expect-error priorities are 1, 2 or 3
const urgent: Priority = 4;

const t: Task = createTask(id, "Write docs", { priority: p });
// @ts-expect-error a task's fields are readonly
t.title = "changed";
// @ts-expect-error a task's status is readonly too
t.status = s;

// A TaskId is still a string where a string is wanted.
const asText: string = t.id;

export { asText, blocked, urgent };
