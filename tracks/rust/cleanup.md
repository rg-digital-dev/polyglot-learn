# Cleanup in Rust

In Rust you rarely write cleanup code at all. A type that owns something implements `Drop`, and the compiler inserts the `drop` call wherever a value goes out of scope: at the end of the block, on `return`, on `?`, and while a panic unwinds. A collection owns its elements, so dropping a `Vec<File>` closes every file in it. That means "open several, fail on the third" needs no special handling: return the error with `?`, and the vector of opened resources is dropped on the way out. To close something early, end its scope or call `drop(value)`.

```rust
struct Lock(&'static str);

impl Drop for Lock {
    fn drop(&mut self) {
        println!("unlock {}", self.0);
    }
}

fn take(names: &[&'static str]) -> Result<(), String> {
    let mut held = Vec::new();
    for &name in names {
        if name.is_empty() {
            return Err("empty name".into()); // prints "unlock a" for every lock taken so far
        }
        held.push(Lock(name));
    }
    Ok(()) // and here, at the end of the function
}

fn main() {
    let _ = take(&["a", "b", ""]);
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Every value works like a `using` variable: you don't opt in, the type decides.
- No `try`/`finally` around `?`: the early return still drops everything in scope.
- `Vec` drops its elements front to back; separate variables are dropped in reverse order of declaration.
<!-- /from -->

<!-- from:default -->
## Key points
- Resources clean up in `Drop`; you don't call it yourself.
- `?` and `return` drop every value in scope on the way out.
- Keep a group of resources in a `Vec`: dropping it drops them all.
<!-- /from -->

## Common mistake
`std::mem::forget` or an `Rc` cycle skips `Drop`, and so does storing the value somewhere that outlives the call. Otherwise, there is little to get wrong here.

## In this exercise
`resource::open` returns a `Resource` that's closed when it's dropped. `read_all` opens them all into a `Vec`, then reads each one; the `?` operator handles both failure cases. Rust in Practice's `scoped-guard` shows how such a `Drop` is written.
