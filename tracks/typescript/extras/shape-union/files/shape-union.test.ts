// Part of the Deeper TypeScript module (tracks/typescript/extras/shape-union). Don't
// edit this file: `learn` overwrites it. Your code goes in shape-union.ts.

import { test } from "node:test";
import assert from "node:assert/strict";
import { area, describe, totalArea, type Shape } from "./shape-union.ts";

// The untouched stub throws this; it doesn't count as the expected error.
const notStub = (e: unknown) => !(e instanceof Error && e.message.startsWith("TODO: implement"));

// Floating-point results are compared with a small tolerance.
const near = (got: number, want: number) =>
  assert.ok(Math.abs(got - want) < 1e-9, `expected about ${want}, got ${got}`);

// A value the type system says can't exist: a shape with a kind Shape doesn't have.
const hexagon = { kind: "hexagon", side: 2 } as unknown as Shape;

test("c01_area_of_a_circle", () => {
  near(area({ kind: "circle", radius: 1 }), Math.PI);
  near(area({ kind: "circle", radius: 2 }), 4 * Math.PI);
});

test("c02_area_of_a_rect", () => {
  assert.equal(area({ kind: "rect", width: 2, height: 3 }), 6);
  assert.equal(area({ kind: "rect", width: 0, height: 5 }), 0);
});

test("c03_area_of_a_triangle_is_half_base_times_height", () => {
  assert.equal(area({ kind: "triangle", base: 4, height: 3 }), 6);
  assert.equal(area({ kind: "triangle", base: 5, height: 1 }), 2.5);
});

test("c04_describe_labels_each_kind", () => {
  assert.equal(describe({ kind: "circle", radius: 2 }), "circle r=2");
  assert.equal(describe({ kind: "rect", width: 2, height: 3 }), "rect 2x3");
  assert.equal(describe({ kind: "triangle", base: 4, height: 3 }), "triangle b=4 h=3");
});

test("c05_total_area_of_mixed_shapes", () => {
  const shapes: Shape[] = [
    { kind: "rect", width: 2, height: 3 },
    { kind: "triangle", base: 4, height: 3 },
    { kind: "circle", radius: 1 },
  ];
  near(totalArea(shapes), 12 + Math.PI);
});

test("c06_total_area_of_no_shapes_is_zero", () => {
  assert.equal(totalArea([]), 0);
});

test("c07_area_throws_for_an_unknown_kind", () => {
  assert.throws(
    () => area(hexagon),
    (e: unknown) => notStub(e) && e instanceof Error && e.message.includes("hexagon"),
    "area should reach assertNever, whose message includes the value as JSON",
  );
});

test("c08_describe_throws_for_an_unknown_kind", () => {
  assert.throws(
    () => describe(hexagon),
    (e: unknown) => notStub(e) && e instanceof Error && e.message.includes("hexagon"),
    "describe should reach assertNever, whose message includes the value as JSON",
  );
});
