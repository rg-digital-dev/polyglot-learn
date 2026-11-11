// Stage 6 of the task tracker project. Don't edit this file: `learn` overwrites it.
//
// Never run, only type-checked: this is case 32, "types check". Each
// `@ts-expect-error` line MUST be a type error. If your types let it through,
// tsc reports "Unused '@ts-expect-error' directive" for that line.

import { query } from "./query.ts";
import type { Priority, Task, TaskId } from "./tasks.ts";

declare const tasks: Task[];

query(tasks).where("status", "done");
query(tasks).where("priority", (p) => p > 1);
// @ts-expect-error "color" isn't a field of Task
query(tasks).where("color", "red");
// @ts-expect-error "blocked" isn't a status
query(tasks).where("status", "blocked");
// @ts-expect-error the test gets a Priority, a number: no toUpperCase
query(tasks).where("priority", (p) => p.toUpperCase() === "HIGH");
// @ts-expect-error there's no "size" field to sort by
query(tasks).sortBy("size");

const rows = query(tasks).select("id", "priority");
const id: TaskId = rows[0].id;
const priority: Priority = rows[0].priority;
// @ts-expect-error only id and priority were selected
rows[0].title;

// The builder works on any object type, not only tasks.
const people = query([{ name: "ada", age: 36 }]).where("age", (a) => a > 30).select("name");
const name: string = people[0].name;
// @ts-expect-error age wasn't selected
people[0].age;

export { id, name, priority };
