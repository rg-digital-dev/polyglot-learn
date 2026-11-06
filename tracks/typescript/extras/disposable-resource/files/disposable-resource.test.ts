// Part of the TypeScript in Practice module (tracks/typescript/extras/disposable-resource). Don't
// edit this file: `learn` overwrites it. Your code goes in disposable-resource.ts.

import { test } from "node:test";
import assert from "node:assert/strict";
import { Transaction, openAll, runQuery, transfer } from "./disposable-resource.ts";

// The untouched stub throws this; it doesn't count as the expected error.
const notStub = (e: unknown) => !(e instanceof Error && e.message.startsWith("TODO: implement"));

const withMessage = (message: string) => (e: unknown) => notStub(e) && e instanceof Error && e.message === message;

test("c01_a_transaction_logs_begin_and_records_entries", () => {
  const log: string[] = [];
  const tx = new Transaction("save", log);
  tx.record("write header");
  tx.record("write body");
  assert.deepEqual(log, ["begin save", "save: write header", "save: write body"]);
});

test("c02_using_rolls_back_at_the_end_of_the_block", () => {
  const log: string[] = [];
  {
    using tx = new Transaction("save", log);
    tx.record("write");
  }
  assert.deepEqual(log, ["begin save", "save: write", "rollback save"]);
});

test("c03_commit_logs_commit_instead_of_rollback", () => {
  const log: string[] = [];
  {
    using tx = new Transaction("save", log);
    tx.commit();
  }
  assert.deepEqual(log, ["begin save", "commit save"]);
});

test("c04_transactions_are_disposed_in_reverse_order", () => {
  const log: string[] = [];
  {
    using first = new Transaction("first", log);
    using second = new Transaction("second", log);
    void first;
    void second;
  }
  assert.deepEqual(log, ["begin first", "begin second", "rollback second", "rollback first"]);
});

test("c05_transfer_commits_when_every_step_succeeds", () => {
  const log: string[] = [];
  transfer(log, ["debit", "credit"]);
  assert.deepEqual(log, ["begin transfer", "transfer: debit", "transfer: credit", "commit transfer"]);
});

test("c06_transfer_rolls_back_when_a_step_throws", () => {
  const log: string[] = [];
  assert.throws(() => transfer(log, ["debit", "!credit", "notify"]), withMessage("step `!credit` failed"));
  assert.deepEqual(log, ["begin transfer", "transfer: debit", "rollback transfer"]);
});

test("c07_await_using_closes_a_connection_after_the_query", async () => {
  const log: string[] = [];
  const rows = await runQuery(log, "db", "select 1");
  assert.equal(rows, "rows for select 1");
  assert.deepEqual(log, ["open db", "db: select 1", "close db"]);
});

test("c08_openall_commits_every_transaction_and_closes_them_newest_first", () => {
  const log: string[] = [];
  openAll(log, ["a", "b", "c"]);
  assert.deepEqual(log, ["begin a", "begin b", "begin c", "commit c", "commit b", "commit a"]);
});

test("c09_openall_rolls_back_what_it_opened_when_a_name_is_bad", () => {
  const log: string[] = [];
  assert.throws(() => openAll(log, ["a", "b", "!c", "d"]), withMessage("bad name `!c`"));
  assert.deepEqual(log, ["begin a", "begin b", "rollback b", "rollback a"]);
});
