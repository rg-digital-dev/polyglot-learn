# Mapped types

A mapped type builds a new object type by going over the keys of another one:
`{ [K in keyof T]: Something<T[K]> }` has the same keys as `T`, and each
value type is computed from the original. Built-ins such as `Partial<T>` and
`Readonly<T>` are mapped types. Combined with a generic class, one type map
can describe a whole API: here the keys are event names, the values are tuples
of arguments, and a mapped type turns them into "the list of listeners for
each event". Calling a method with a wrong event name or wrong arguments is
then a compile error.

```typescript
type Settings = { volume: number; muted: boolean; theme: "light" | "dark" };

// One validator per setting, each taking that setting's own type.
type Validators<T> = { [K in keyof T]: (value: T[K]) => boolean };

class Store<T extends object> {
  private values: T;
  private checks: Validators<T>;
  constructor(initial: T, checks: Validators<T>) {
    this.values = { ...initial };
    this.checks = checks;
  }
  set<K extends keyof T>(key: K, value: T[K]): boolean {
    if (!this.checks[key](value)) return false;
    this.values[key] = value;
    return true;
  }
  get<K extends keyof T>(key: K): T[K] {
    return this.values[key];
  }
}

const store = new Store<Settings>(
  { volume: 5, muted: false, theme: "light" },
  { volume: (v) => v >= 0 && v <= 10, muted: () => true, theme: () => true },
);
console.log(store.set("volume", 11), store.get("volume")); // false 5
// store.set("theme", "blue");
// error TS2345: Argument of type '"blue"' is not assignable to parameter of type '"light" | "dark"'.
```

The editor (or `tsc`) checks event names and argument types. Node only strips
the types, so the tests check behaviour: call order, return values and
unsubscribing.

<!-- from:javascript -->
## Coming from JavaScript
- At runtime this is the emitter you'd write in JavaScript: an object of
  arrays. The mapped type only tells the compiler which array holds which
  listener type.
- A tuple type such as `[from: string, text: string]` describes a rest
  parameter list, so `(...args: Events[K]) => void` gets real parameter names
  and types in the editor.
- Declare the event map with `type`, not `interface`: an interface doesn't
  count as a `Record<string, unknown[]>`, so the constraint rejects it.
<!-- /from -->

<!-- from:default -->
## Key points
- `[K in keyof T]` loops over keys at the type level; `T[K]` is the value type
  for each key.
- Add `?` in a mapped type (`[K in keyof T]?:`) when not every key has a value
  yet.
- Return an unsubscribe function from `on`: the caller keeps it next to the
  code that subscribed and doesn't need the listener reference.
<!-- /from -->

## Common mistake
Removing from the array you're looping over. When a listener unsubscribes
itself during `emit`, `splice` shifts the rest down by one and the loop skips
the next listener without any error. Loop over a copy (`[...list]`) instead.

## In this exercise
Implement `on`, `off` and `emit` on the `Emitter` class in
`typed-emitter/typed-emitter.ts`. The `listeners` field and its mapped type are
already declared; store the listeners there.
