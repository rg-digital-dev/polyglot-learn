// Part of the TypeScript in Practice module (tracks/typescript/extras/branded-ids). Don't
// edit this file: `learn` overwrites it. Your code goes in branded-ids.ts.
//
// Case 1, "types check", isn't here: learn runs tsc on branded-ids.ts and
// branded-ids.types.ts and reports the result as that case.

import { test } from "node:test";
import assert from "node:assert/strict";
import { addCents, cents, formatCents, orderId, priceOf, userId } from "./branded-ids.ts";

// The untouched stub throws this; it doesn't count as the expected error.
const notStub = (e: unknown) => !(e instanceof Error && e.message.startsWith("TODO: implement"));

const withMessage = (message: string) => (e: unknown) => notStub(e) && e instanceof Error && e.message === message;

test("c02_userid_accepts_a_valid_id", () => {
  assert.equal(userId("u_ada"), "u_ada");
  assert.equal(userId("u_x9"), "u_x9");
});

test("c03_userid_rejects_anything_else", () => {
  for (const raw of ["ada", "u_", "U_ada", "u_Ada", "o_1", " u_ada", "u_ada!"]) {
    assert.throws(() => userId(raw), withMessage(`not a user id: ${raw}`), `userId(${JSON.stringify(raw)})`);
  }
});

test("c04_orderid_accepts_and_rejects", () => {
  assert.equal(orderId("o_42"), "o_42");
  for (const raw of ["o_", "o_4a", "u_42", "42"]) {
    assert.throws(() => orderId(raw), withMessage(`not an order id: ${raw}`), `orderId(${JSON.stringify(raw)})`);
  }
});

test("c05_cents_accepts_whole_non_negative_numbers", () => {
  assert.equal(cents(0), 0);
  assert.equal(cents(1999), 1999);
});

test("c06_cents_rejects_fractions_negatives_and_nan", () => {
  for (const amount of [1.5, -1, Number.NaN, Number.POSITIVE_INFINITY]) {
    assert.throws(() => cents(amount), withMessage(`not an amount in cents: ${amount}`), `cents(${amount})`);
  }
});

test("c07_addcents_adds_two_amounts", () => {
  assert.equal(addCents(cents(250), cents(180)), 430);
  assert.equal(addCents(cents(0), cents(0)), 0);
});

test("c08_formatcents_writes_euros", () => {
  assert.equal(formatCents(cents(1234)), "€12.34");
  assert.equal(formatCents(cents(5)), "€0.05");
  assert.equal(formatCents(cents(0)), "€0.00");
  assert.equal(formatCents(cents(100000)), "€1000.00");
});

test("c09_priceof_looks_up_a_price", () => {
  assert.equal(priceOf("coffee"), 250);
  assert.equal(priceOf("tea"), 180);
});
