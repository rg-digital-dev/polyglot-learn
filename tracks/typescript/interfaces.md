# Interfaces in TypeScript

An `interface` can list methods as well as fields, which turns it into a contract: any value that has those methods can be used where the interface is expected. A class promises to keep the contract with `implements`, and the compiler checks that every method exists with a matching signature. Code written against the interface calls the methods without knowing which class it was given.

```typescript
interface Greeter {
  greet(name: string): string;
}

class Formal implements Greeter {
  title: string;
  constructor(title: string) {
    this.title = title;
  }
  greet(name: string): string {
    return `Good evening, ${this.title} ${name}.`;
  }
}

class Casual implements Greeter {
  greet(name: string): string {
    return `hey ${name}`;
  }
}

const greeters: Greeter[] = [new Formal("Dr."), new Casual()];
for (const g of greeters) console.log(g.greet("Ada"));
```

<!-- from:javascript -->
## Coming from JavaScript
- JavaScript checks the duck at runtime; TypeScript checks it at compile time. Typing is still structural, so a plain object with a `greet` method is a `Greeter` even without `implements`.
- `implements` only checks. It adds nothing at runtime and inherits no code; `extends` is still how a class inherits.
- Interfaces are erased, so `x instanceof Greeter` is a compile error. Test a concrete class, or check for the method with `"greet" in x`.
<!-- /from -->

<!-- from:default -->
## Key points
- Type parameters and arrays with the interface (`Greeter[]`), not the classes.
- `implements` makes the compiler confirm a class has every method.
- `Array.prototype.sort` is stable since ES2019: equal keys keep their order.
<!-- /from -->

## Common mistake
Using a constructor parameter property, `constructor(public title: string) {}`. It is valid TypeScript, but it generates code, so Node's type stripping stops with `ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX`. Declare the field and assign it in the constructor body, as `Formal` does above.

## In this exercise
In `shapes/shapes.ts`, fill in `area()` and `label()` on `Rect` and `Square`, then implement `export function describeAll(shapes: Shape[]): string[]`. It should only call `area()` and `label()`, never check which class a shape is.
