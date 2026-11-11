// Stage 8 of the task tracker project. Don't edit this file: `learn`
// overwrites it. Your code goes in feed.ts.

import { test } from "node:test";
import assert from "node:assert/strict";
import { follow, linesOf, parseEvents } from "./feed.ts";
import { TaskStore } from "./store.ts";

async function* from<T>(items: T[], pause = 0): AsyncGenerator<T> {
  for (const item of items) {
    if (pause) await new Promise((r) => setTimeout(r, pause));
    yield item;
  }
}

async function collect<T>(it: AsyncIterable<T>): Promise<T[]> {
  const out: T[] = [];
  for await (const x of it) out.push(x);
  return out;
}

const task = (n: number) => ({ id: `t-${n}`, title: `task ${n}`, status: "todo", priority: 2, assignee: null, due: null });
const line = (event: object) => JSON.stringify(event);

test("c45_linesof_joins_chunks_into_lines", async () => {
  assert.deepEqual(await collect(linesOf(from(["ab", "c\nd", "e\n\nf"]))), ["abc", "de", "", "f"]);
  assert.deepEqual(await collect(linesOf(from(["x\n"]))), ["x"], "no empty line after a final newline");
  assert.deepEqual(await collect(linesOf(from<string>([]))), []);
});

test("c46_parseevents_yields_events_and_numbered_errors", async () => {
  const lines = [line({ type: "created", task: task(1) }), "", "{oops", line({ type: "moved", id: "t-1", to: "done" }), line({ type: "moved", id: "t-1", to: "blocked" }), line({ type: "nap" })];
  const got = await collect(parseEvents(from(lines)));
  assert.deepEqual(
    got.map((r) => (r.ok ? r.value.type : r.error)),
    ["created", "line 3: not JSON", "moved", "line 5: not a task event", "line 6: not a task event"],
  );
});

test("c47_follow_applies_events_to_the_store", async () => {
  const store = new TaskStore();
  const lines = [line({ type: "created", task: task(1) }), line({ type: "created", task: task(2) }), line({ type: "deleted", id: "t-9" }), "nope", line({ type: "moved", id: "t-2", to: "doing" })];
  const result = await follow(store, from(lines));
  assert.deepEqual(result, { applied: 3, errors: ["no task t-9", "line 4: not JSON"] });
  assert.equal(store.board.get("t-2" as never)?.status, "doing");
});

test("c48_follow_stops_when_the_signal_aborts", async () => {
  const store = new TaskStore();
  const controller = new AbortController();
  store.on("created", (e) => {
    if (e.task.id === "t-2") controller.abort();
  });
  const lines = [1, 2, 3, 4].map((n) => line({ type: "created", task: task(n) }));
  const result = await follow(store, from(lines, 5), controller.signal);
  assert.equal(result.applied, 2);
  assert.equal(store.board.size, 2, "nothing after the abort was applied");
});

test("c49_parseevents_reads_only_what_is_asked_for", async () => {
  let pulled = 0;
  async function* endless() {
    for (let n = 1; ; n++) {
      pulled++;
      yield line({ type: "created", task: task(n) });
    }
  }
  const it = parseEvents(endless());
  await it.next();
  await it.next();
  await it.return(undefined);
  assert.equal(pulled, 2);
});
