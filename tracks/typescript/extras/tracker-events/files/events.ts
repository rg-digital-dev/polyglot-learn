// Everything that can happen to a task.

import type { Status, Task, TaskId } from "./tasks.ts";

// TODO: one loose shape lets anything through: events.types.ts checks that a
// moved event has `to` but no title, that a renamed event needs its title,
// and that a switch over all five types leaves `never`. Make TaskEvent a
// discriminated union of five event types, each with its own fields:
//   created  { task }            renamed  { id, title }
//   moved    { id, to: Status }  assigned { id, assignee: string | null }
//   deleted  { id }
export type TaskEvent = {
  type: string;
  task?: Task;
  id?: TaskId;
  title?: string;
  to?: Status;
  assignee?: string | null;
};

// TODO: the union of the five type names.
export type EventType = string;

/** For the default branch of an exhaustive switch. Throws if it's ever reached. */
export function assertNever(value: never): never {
  throw new Error("TODO: implement assertNever");
}

/** The task an event is about: a created event's task.id, or the event's id. */
export function taskIdOf(event: TaskEvent): TaskId {
  throw new Error("TODO: implement taskIdOf");
}

/**
 * One line per event:
 *   created t-1 "Write docs"     renamed t-1 to "New title"     moved t-1 to doing
 *   assigned t-1 to ada          unassigned t-1 (assignee null)  deleted t-1
 */
export function describe(event: TaskEvent): string {
  throw new Error("TODO: implement describe");
}

/** The events about task `id`, in order. */
export function eventsFor(events: readonly TaskEvent[], id: TaskId): TaskEvent[] {
  throw new Error("TODO: implement eventsFor");
}
