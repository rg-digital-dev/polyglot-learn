// Part of the Deeper TypeScript module (tracks/typescript/extras/parse-unknown). Don't
// edit this file: `learn` overwrites it. Your code goes in parse-unknown.ts.

import { test } from "node:test";
import assert from "node:assert/strict";
import { isUser, parseUser, parseUsers } from "./parse-unknown.ts";

// The untouched stub throws this; it doesn't count as the expected error.
const notStub = (e: unknown) => !(e instanceof Error && e.message.startsWith("TODO: implement"));

// An expected error whose message starts with `prefix`.
const startsWith = (prefix: string) => (e: unknown) =>
  notStub(e) && e instanceof Error && e.message.startsWith(prefix);

test("c01_isuser_accepts_a_valid_user", () => {
  assert.equal(isUser({ name: "Ada", age: 36, tags: ["admin"] }), true);
  assert.equal(isUser({ name: "Bo", age: 0, tags: [] }), true);
});

test("c02_isuser_accepts_an_optional_email", () => {
  assert.equal(isUser({ name: "Ada", age: 36, tags: [], email: "ada@example.com" }), true);
});

test("c03_isuser_rejects_anything_else", () => {
  const bad: unknown[] = [
    null,
    undefined,
    "Ada",
    42,
    [],
    {},
    { name: "Ada", age: "36", tags: [] },
    { name: "Ada", age: 36 },
    { name: "Ada", age: 36, tags: "admin" },
    { name: "Ada", age: 36, tags: ["admin", 7] },
    { name: "Ada", age: 36, tags: [], email: null },
    { name: 7, age: 36, tags: [] },
  ];
  for (const value of bad) {
    assert.equal(isUser(value), false, `isUser(${JSON.stringify(value)}) should be false`);
  }
});

test("c04_parseuser_returns_the_typed_user", () => {
  assert.deepStrictEqual(parseUser('{"name":"Ada","age":36,"tags":["admin","dev"]}'), {
    name: "Ada",
    age: 36,
    tags: ["admin", "dev"],
  });
  assert.deepStrictEqual(parseUser('{"name":"Bo","age":20,"tags":[],"email":"bo@example.com"}'), {
    name: "Bo",
    age: 20,
    tags: [],
    email: "bo@example.com",
  });
});

test("c05_parseuser_keeps_only_the_user_fields", () => {
  assert.deepStrictEqual(parseUser('{"name":"Ada","age":36,"tags":[],"password":"hunter2"}'), {
    name: "Ada",
    age: 36,
    tags: [],
  });
});

test("c06_parseuser_names_the_bad_field", () => {
  assert.throws(() => parseUser('{"name":"Ada","age":"36","tags":[]}'), startsWith("age"));
  assert.throws(() => parseUser('{"age":36,"tags":[]}'), startsWith("name"));
  assert.throws(() => parseUser('{"name":"Ada","age":36,"tags":[1]}'), startsWith("tags"));
  assert.throws(() => parseUser('{"name":"Ada","age":36,"tags":[],"email":false}'), startsWith("email"));
});

test("c07_parseuser_rejects_a_value_that_is_not_an_object", () => {
  for (const json of ["null", "[1,2]", '"Ada"', "36"]) {
    assert.throws(() => parseUser(json), notStub, `parseUser(${json}) should throw`);
  }
});

test("c08_parseuser_rejects_malformed_json", () => {
  assert.throws(() => parseUser('{"name": "Ada"'), notStub);
  assert.throws(() => parseUser(""), notStub);
});

test("c09_parseusers_parses_an_array_of_users", () => {
  assert.deepStrictEqual(parseUsers('[{"name":"Ada","age":36,"tags":[]},{"name":"Bo","age":20,"tags":["x"],"extra":1}]'), [
    { name: "Ada", age: 36, tags: [] },
    { name: "Bo", age: 20, tags: ["x"] },
  ]);
  assert.deepStrictEqual(parseUsers("[]"), []);
});

test("c10_parseusers_reports_the_index_of_a_bad_entry", () => {
  assert.throws(() => parseUsers('[{"name":"Ada","age":36,"tags":[]},{"name":"Bo","age":"x","tags":[]}]'), startsWith("[1] age"));
  assert.throws(() => parseUsers('[{"name":"Ada","age":36,"tags":[]}, 5]'), startsWith("[1]"));
  assert.throws(() => parseUsers('{"name":"Ada","age":36,"tags":[]}'), notStub, "an object is not an array");
});
