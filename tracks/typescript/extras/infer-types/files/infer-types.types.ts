// Part of the TypeScript in Practice module (tracks/typescript/extras/infer-types). Don't
// edit this file: `learn` overwrites it. Your code goes in infer-types.ts.
//
// Never run, only type-checked: this is case 1, "types check". Every
// `Expect<Equal<A, B>>` needs A and B to be exactly the same type, and each
// `@ts-expect-error` line MUST be a type error.

import { eventName, eventOf, first, memoize } from "./infer-types.ts";
import type { ElementOf, EventName, EventOf, FirstArg, ReturnOf, Unwrap } from "./infer-types.ts";

/** True only when A and B are identical (not just assignable to each other). */
type Equal<A, B> = (<T>() => T extends A ? 1 : 2) extends <T>() => T extends B ? 1 : 2 ? true : false;
/** A type error unless T is `true`. */
type Expect<T extends true> = T;

export type Checks = [
  Expect<Equal<ElementOf<string[]>, string>>,
  Expect<Equal<ElementOf<readonly [1, "a"]>, 1 | "a">>,
  Expect<Equal<ElementOf<number>, never>>,

  Expect<Equal<Unwrap<Promise<number>>, number>>,
  Expect<Equal<Unwrap<Promise<Promise<string[]>>>, string[]>>,
  Expect<Equal<Unwrap<boolean>, boolean>>,

  Expect<Equal<ReturnOf<() => Date>, Date>>,
  Expect<Equal<ReturnOf<(a: number, b: string) => boolean>, boolean>>,
  Expect<Equal<ReturnOf<string>, never>>,

  Expect<Equal<FirstArg<(a: number, b: string) => void>, number>>,
  Expect<Equal<FirstArg<(user: { id: string }) => void>, { id: string }>>,
  Expect<Equal<FirstArg<42>, never>>,

  Expect<Equal<EventName<"click">, "onClick">>,
  Expect<Equal<EventName<"keyDown">, "onKeyDown">>,
  Expect<Equal<EventName<"focus" | "blur">, "onFocus" | "onBlur">>,

  Expect<Equal<EventOf<"onClick">, "click">>,
  Expect<Equal<EventOf<"onKeyDown">, "keyDown">>,
  Expect<Equal<EventOf<"click">, never>>,
];

// The functions' return types follow their arguments.
const n: number | undefined = first([1, 2, 3]);
const tagged: "a" | "b" | undefined = first(["a", "b"] as const);
const onClick: "onClick" = eventName("click");
// @ts-expect-error eventName("click") is "onClick", with a capital C
const wrong: "onclick" = eventName("click");
const click: "click" = eventOf("onClick");

const length = memoize((s: string) => s.length);
const size: number = length("abc");
// @ts-expect-error memoize keeps the return type: number, not string
const notText: string = length("abc");
// @ts-expect-error memoize keeps the argument type: string, not number
length(3);

export { click, n, notText, onClick, size, tagged, wrong };
