// Stage 4 of the task tracker project. Don't edit this file: `learn` overwrites it.
//
// Never run, only type-checked: this is case 20, "types check". Each
// `@ts-expect-error` line MUST be a type error. If your types let it through,
// tsc reports "Unused '@ts-expect-error' directive" for that line.

import { TaskStore, type EventOf } from "./store.ts";
import type { Status, TaskId } from "./tasks.ts";

const store = new TaskStore();

store.on("moved", (e) => {
  // The handler gets exactly the moved event...
  const to: Status = e.to;
  const id: TaskId = e.id;
  // @ts-expect-error ...which has no title
  e.title;
});

store.on("created", (e) => {
  const title: string = e.task.title;
  // @ts-expect-error a created event has a task, not an id
  e.id;
});

// @ts-expect-error "archived" isn't an event type
store.on("archived", () => {});

const moved: EventOf<"moved"> = { type: "moved", id: "t-1" as TaskId, to: "done" };
// @ts-expect-error EventOf<"moved"> is only the moved event
const notMoved: EventOf<"moved"> = { type: "deleted", id: "t-1" as TaskId };

// The board and log are read-only from outside.
// @ts-expect-error no set on a ReadonlyMap
store.board.set("t-1" as TaskId, undefined!);
// @ts-expect-error no push on a readonly array
store.log.push(moved);

const stop: () => void = store.on("deleted", () => {});

export { notMoved, stop };
