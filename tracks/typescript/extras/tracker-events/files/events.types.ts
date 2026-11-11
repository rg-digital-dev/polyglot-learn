// Stage 2 of the task tracker project. Don't edit this file: `learn` overwrites it.
//
// Never run, only type-checked: this is case 8, "types check". Each
// `@ts-expect-error` line MUST be a type error. If your types let it through,
// tsc reports "Unused '@ts-expect-error' directive" for that line.

import { assertNever, type EventType, type TaskEvent } from "./events.ts";
import { taskId, type Status } from "./tasks.ts";

declare const e: TaskEvent;

if (e.type === "moved") {
  // Narrowed: a moved event has `to`...
  const to: Status = e.to;
  // @ts-expect-error ...and no title
  e.title;
}
if (e.type === "renamed") {
  const title: string = e.title;
}

const t: EventType = "deleted";
// @ts-expect-error "archived" isn't an event type
const archived: EventType = "archived";

// @ts-expect-error a renamed event needs its title
const missing: TaskEvent = { type: "renamed", id: taskId("t-1") };
// @ts-expect-error a moved event can't go to a made-up status
const wrong: TaskEvent = { type: "moved", id: taskId("t-1"), to: "blocked" };

// Every event type handled, so the default branch gets `never`.
function name(event: TaskEvent): string {
  switch (event.type) {
    case "created":
    case "renamed":
    case "moved":
    case "assigned":
    case "deleted":
      return event.type;
    default:
      return assertNever(event);
  }
}

// One type left out, so `event` isn't `never` there.
function incomplete(event: TaskEvent): string {
  switch (event.type) {
    case "created":
    case "renamed":
    case "moved":
    case "assigned":
      return event.type;
    default:
      // @ts-expect-error "deleted" isn't handled
      return assertNever(event);
  }
}

export { archived, incomplete, missing, name, t, wrong };
