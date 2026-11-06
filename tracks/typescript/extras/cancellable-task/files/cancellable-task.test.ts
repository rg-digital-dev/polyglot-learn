// Part of the TypeScript in Practice module (tracks/typescript/extras/cancellable-task). Don't
// edit this file: `learn` overwrites it. Your code goes in cancellable-task.ts.

import { test } from "node:test";
import assert from "node:assert/strict";
import { delay, firstOf, withTimeout } from "./cancellable-task.ts";

// The untouched stub throws this; it doesn't count as the expected error.
const notStub = (e: unknown) => !(e instanceof Error && e.message.startsWith("TODO: implement"));

const elapsedSince = (start: number) => performance.now() - start;

test("c01_delay_resolves_after_the_time", async () => {
  const start = performance.now();
  await delay(40);
  assert.ok(elapsedSince(start) >= 35, "resolved too early");
});

test("c02_delay_rejects_at_once_when_already_aborted", async () => {
  const controller = new AbortController();
  controller.abort("stop");
  const start = performance.now();
  await assert.rejects(delay(1000, controller.signal), (e) => notStub(e) && e === "stop");
  assert.ok(elapsedSince(start) < 100, "waited although the signal was already aborted");
});

test("c03_delay_stops_waiting_when_aborted", async () => {
  const controller = new AbortController();
  setTimeout(() => controller.abort(new Error("cancelled")), 20);
  const start = performance.now();
  await assert.rejects(delay(1000, controller.signal), (e) => notStub(e) && e instanceof Error && e.message === "cancelled");
  assert.ok(elapsedSince(start) < 300, "kept waiting after the abort");
});

test("c04_withtimeout_returns_the_result_of_fast_work", async () => {
  const result = await withTimeout(async (signal) => {
    await delay(10, signal);
    return "done";
  }, 500);
  assert.equal(result, "done");
});

test("c05_withtimeout_throws_a_timeout_error_with_a_cause", async () => {
  const isTimeout = (e: unknown) =>
    notStub(e) &&
    e instanceof Error &&
    e.message === "timed out after 30 ms" &&
    e.cause instanceof DOMException &&
    e.cause.name === "TimeoutError";
  await assert.rejects(withTimeout((signal) => delay(1000, signal), 30), isTimeout);
  // Work that ignores the signal still times out.
  await assert.rejects(withTimeout(() => new Promise<never>(() => {}), 30), isTimeout);
});

test("c06_withtimeout_aborts_the_work_s_signal", async () => {
  let seen: AbortSignal | undefined;
  await assert.rejects(
    withTimeout((signal) => {
      seen = signal;
      return delay(1000, signal);
    }, 20),
    notStub,
  );
  assert.equal(seen?.aborted, true);
});

test("c07_withtimeout_passes_other_errors_through", async () => {
  const boom = new Error("boom");
  await assert.rejects(
    withTimeout(async () => {
      throw boom;
    }, 500),
    (e) => e === boom,
  );
});

test("c08_firstof_returns_the_fastest_result", async () => {
  const result = await firstOf([
    async (s) => (await delay(80, s), "slow"),
    async (s) => (await delay(10, s), "fast"),
    async (s) => (await delay(40, s), "medium"),
  ]);
  assert.equal(result, "fast");
});

test("c09_firstof_aborts_the_slower_tasks", async () => {
  const signals: AbortSignal[] = [];
  const start = performance.now();
  await firstOf([
    async (s) => (signals.push(s), await delay(10, s), 1),
    async (s) => (signals.push(s), await delay(2000, s), 2),
  ]);
  assert.equal(signals.length, 2);
  assert.ok(signals.every((s) => s.aborted), "the slower task's signal wasn't aborted");
  assert.ok(elapsedSince(start) < 1000);
});

test("c10_firstof_skips_failures_and_throws_when_all_fail", async () => {
  const result = await firstOf([
    async () => {
      throw new Error("first fails");
    },
    async (s) => (await delay(20, s), "second"),
  ]);
  assert.equal(result, "second");
  await assert.rejects(
    firstOf([
      async () => {
        throw new Error("a");
      },
      async () => {
        throw new Error("b");
      },
    ]),
    (e) => notStub(e) && e instanceof AggregateError && e.errors.length === 2,
  );
});
