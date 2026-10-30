# Closures in TypeScript

A function defined inside another function keeps access to the outer
function's variables, even after the outer function has returned. The inner
function together with the variables it captured is a closure. Each call of the
outer function creates new variables, so every closure it returns has its own
private state. A function that returns a function declares that with a function
type, such as `() => number`.

```typescript
function makeGreeter(greeting: string): (name: string) => string {
  let calls = 0;
  return (name) => {
    calls += 1;
    return `${greeting}, ${name} (#${calls})`;
  };
}

const hi = makeGreeter("Hi");
const yo = makeGreeter("Yo");
console.log(hi("Ada")); // Hi, Ada (#1)
console.log(hi("Lin")); // Hi, Lin (#2)
console.log(yo("Ada")); // Yo, Ada (#1)
```

<!-- from:javascript -->
## Coming from JavaScript
- Closures behave exactly as in JavaScript: they capture the variable itself,
  not a copy of its value.
- What is new is the function type in signatures: `(name: string) => string`.
  The parameter name inside the type is documentation only.
- The returned arrow's parameter types come from the declared return type, so
  `(name) =>` needs no annotation of its own.
<!-- /from -->

<!-- from:default -->
## Key points
- Keep the state in a `let` inside the factory function.
- Every call of the factory creates fresh state for the closure it returns.
- A function type is written `(params) => ReturnType`.
<!-- /from -->

## Common mistake
Declaring the state outside the factory. A `let count` at module level is
shared by every closure, so two counters advance each other. Declare it inside
the function that builds the closure.

## In this exercise
Implement `export function makeCounter(start: number, step: number): () => number`
in `make-counter/make-counter.ts`. The tests create one or two counters and
call each of them several times.
