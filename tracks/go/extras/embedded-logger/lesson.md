# Embedding and implicit interfaces

Go has no classes and no inheritance. To share behaviour, a struct **embeds** another type: a field written as just a type name, with no field name. The embedded type's fields and methods are **promoted**, so they can be used as if the outer struct declared them. If the outer type declares a method with the same name, that method wins and the promoted one is hidden, but still reachable through the field, which is named after its type. Interfaces work the same quiet way: a type satisfies an interface simply by having its methods. Promoted methods count, so embedding is often how a type gets the methods an interface asks for.

```go
package main

import "fmt"

type Counter interface {
	Inc()
	Total() int
}

type Tally struct{ n int }

func (t *Tally) Inc()       { t.n++ }
func (t *Tally) Total() int { return t.n }

// Votes embeds Tally: it gets Inc and Total for free.
type Votes struct {
	Tally
	Topic string
}

// Double embeds Tally and overrides Inc.
type Double struct{ Tally }

func (d *Double) Inc() {
	d.Tally.Inc() // call the hidden, promoted method explicitly
	d.Tally.Inc()
}

func main() {
	counters := []Counter{&Votes{Topic: "lunch"}, &Double{}}
	for _, c := range counters {
		c.Inc()
		fmt.Println(c.Total()) // 1, then 2
	}
}
```

<!-- from:typescript -->
## Coming from TypeScript
- Interfaces are structural, like in TypeScript, but there is no `implements` clause at all: having the methods is the whole declaration.
- Embedding is not `extends`. A `*Votes` is not a `*Tally`; you can't pass one where the other is expected. Only the interface sees them as the same.
- There is no `super`. To call the method you are overriding, name the embedded field: `d.Tally.Inc()`.
- Inside a promoted method the receiver is the embedded value, not the outer struct, so `Tally.Inc` never calls `Double.Inc`. There is no virtual dispatch through embedding.
<!-- /from -->

<!-- from:default -->
## Key points
- A field written as only a type name embeds it; its methods are promoted to the outer type.
- A method declared on the outer type hides a promoted method of the same name; `outer.Inner.Method()` still reaches it.
- A type satisfies an interface by having the methods, promoted ones included. `var _ Counter = &Votes{}` checks it at compile time.
- Methods with a pointer receiver belong to `*T`, so it is `&Votes{}`, not `Votes{}`, that satisfies `Counter`.
<!-- /from -->

## Common mistake
Giving the field a name, which stops the promotion:

```go
type Votes struct{ t Tally }   // a named field: nothing is promoted
var c Counter = &Votes{}
// cannot use &Votes{} (value of type *Votes) as Counter value in variable declaration:
// *Votes does not implement Counter (missing method Inc)
type Votes struct{ Tally }     // fixed: embedded
```

## In this exercise
Implement `Base.Log` and `Base.Lines` in `embeddedlogger.go`, then embed `Base` in `Plain` and `Shouty` and give `Shouty` its own `Log`. `NewPlain` and `NewShouty` return them as a `Logger`; the tests check through reflection that both really embed `Base`.
