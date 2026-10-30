# Generators and lazy sequences

A generator function, written `function*`, returns an iterator instead of
running to the end. Each time the caller asks for the next value, the body
runs until the next `yield` and pauses there with all its local variables
intact. Nothing is computed ahead of time, so a generator can describe an
endless sequence and the caller decides how much of it to use. Anything with
a `[Symbol.iterator]()` method that returns an iterator is **iterable** and
works with `for...of`, spread and `Array.from`. Generators are iterable, and
a class becomes iterable by giving it that method.

```typescript
function* fibonacci(): Generator<number> {
  let [a, b] = [0, 1];
  while (true) {
    yield a;
    [a, b] = [b, a + b];
  }
}

function* filter<T>(items: Iterable<T>, keep: (item: T) => boolean): Generator<T> {
  for (const item of items) {
    if (keep(item)) yield item;
  }
}

class Deck implements Iterable<string> {
  suits = ["hearts", "spades"];
  *[Symbol.iterator](): Generator<string> {
    for (const suit of this.suits) for (let n = 1; n <= 3; n++) yield `${n} of ${suit}`;
  }
}

for (const n of filter(fibonacci(), (n) => n % 2 === 0)) {
  if (n > 100) break; // break stops the generator; nothing more is computed
  console.log(n); // 0, 2, 8, 34
}
console.log([...new Deck()].length); // 6
```

The editor (or `tsc`) checks the element types: `Generator<number>` and
`Iterable<T>` say what each `yield` produces. Node only strips the types, so
the tests check behaviour, including how many values your code pulls from an
endless sequence.

<!-- from:javascript -->
## Coming from JavaScript
- Generators and the iteration protocol are plain JavaScript. TypeScript adds
  the element type: `Generator<number>` for what you produce, `Iterable<T>`
  for what you accept.
- Accept `Iterable<T>` rather than `T[]` when you only loop once. Arrays,
  strings, `Map`, `Set` and generators all fit.
- Spreading an endless generator, `[...fibonacci()]`, never finishes. Cut it
  with something like `take` first.
<!-- /from -->

<!-- from:default -->
## Key points
- `function*` plus `yield` gives a lazy sequence; the body runs on demand.
- A generator object can be looped over only once. An iterable class makes a
  fresh iterator on every `[Symbol.iterator]()` call, so it can be looped over
  again.
- `return` inside a generator ends it, and `break` in the caller's `for...of`
  stops it.
<!-- /from -->

## Common mistake
Checking the count before yielding in `take`:

```typescript
let count = 0;
for (const item of items) {
  if (count === n) return; // too late: this item was already pulled
  yield item;
  count++;
}
```

The loop pulls item `n + 1` before the check runs. With a generator that does
work for each value, such as reading a file or calling an API, that's wasted
work. Yield first, then check, and handle `n <= 0` before the loop.

## In this exercise
Implement `range`, `take` and `map` as generators in `lazy-range/lazy-range.ts`,
and give `Countdown` a `[Symbol.iterator]` method. The tests feed `take` and
`map` an endless sequence that counts how many values it handed out.
