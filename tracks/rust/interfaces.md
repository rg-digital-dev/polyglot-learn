# Traits in Rust

Rust's version of an interface is a trait: a named set of method signatures. A type opts in with an explicit `impl Trait for Type` block. Code that should work with several types can be generic (`fn f<T: Trait>(x: &T)`), which is resolved at compile time, or use a trait object, `dyn Trait`, which is resolved at run time. A trait object has no fixed size, so it always sits behind a pointer such as `Box<dyn Trait>` or `&dyn Trait`. That is how one `Vec` can hold values of different concrete types.

```rust
trait Notifier {
    fn channel(&self) -> String;
    fn cost_cents(&self) -> i64;
}

struct Email { address: String }
struct Sms { number: String, segments: i64 }

impl Notifier for Email {
    fn channel(&self) -> String { format!("email {}", self.address) }
    fn cost_cents(&self) -> i64 { 0 }
}

impl Notifier for Sms {
    fn channel(&self) -> String { format!("sms {}", self.number) }
    fn cost_cents(&self) -> i64 { 5 * self.segments }
}

fn total_cost(targets: &[Box<dyn Notifier>]) -> i64 {
    targets.iter().map(|t| t.cost_cents()).sum()
}

fn main() {
    let targets: Vec<Box<dyn Notifier>> = vec![
        Box::new(Email { address: "a@example.org".to_string() }),
        Box::new(Sms { number: "555-0100".to_string(), segments: 2 }),
    ];
    assert_eq!(total_cost(&targets), 10);
    assert_eq!(targets[1].channel(), "sms 555-0100");
}
```

<!-- from:typescript -->
## Coming from TypeScript
- TypeScript checks shapes structurally; Rust only accepts types that wrote `impl Trait for Type`.
- There is no inheritance between structs. Shared behaviour lives in traits, which may supply default method bodies.
- `Box<dyn Notifier>` plays the role of a variable typed as an interface; `Box` owns the value on the heap.
<!-- /from -->

<!-- from:default -->
## Key points
- The trait's methods are callable on a type only where the trait is in scope.
- `&[Box<dyn Trait>]` is a borrowed list of mixed types; the function can read it but not reorder it.
- `sort_by` is stable, so equal keys keep their original order.
<!-- /from -->

## Common mistake
Sorting the borrowed slice you were given:

```rust
shapes.sort_by(|a, b| b.area().cmp(&a.area()));
// error[E0596]: cannot borrow `*shapes` as mutable, as it is behind a `&` reference
let mut sorted: Vec<_> = shapes.iter().collect();  // fixed: sort your own Vec of references
```

The compiler suggests `&mut [..]` instead, but the tests call the function with a shared slice.

## In this exercise
Fill in the `impl Shape for Rect` and `impl Shape for Square` skeletons, then implement `pub fn describe_all(shapes: &[Box<dyn Shape>]) -> Vec<String>`. It should only call `area()` and `label()`, never check which type a shape is.
