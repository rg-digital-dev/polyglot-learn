# Channels and pipelines

A channel is a typed pipe between goroutines: `ch <- v` sends, `v := <-ch` receives. On an unbuffered channel (`make(chan int)`) a send waits until another goroutine receives, so the two meet. The sender calls `close(ch)` when it has nothing more to send; `for v := range ch` then reads every remaining value and stops. That makes a **pipeline**: each stage is a function that starts a goroutine, reads from an input channel, writes to an output channel it closes when done, and returns the output right away. Several channels can be **fanned in** to one with a goroutine per input and a `sync.WaitGroup` that tells when all of them are finished.

```go
package main

import (
	"fmt"
	"strings"
)

// Words sends each word of text, then closes the channel.
func Words(text string) <-chan string {
	out := make(chan string)
	go func() {
		defer close(out)
		for _, w := range strings.Fields(text) {
			out <- w
		}
	}()
	return out // the caller can read while the goroutine still sends
}

// Long passes on only the words longer than n letters.
func Long(in <-chan string, n int) <-chan string {
	out := make(chan string)
	go func() {
		defer close(out)
		for w := range in { // ends when Words closes its channel
			if len(w) > n {
				out <- w
			}
		}
	}()
	return out
}

func main() {
	for w := range Long(Words("go makes channels feel natural"), 4) {
		fmt.Println(w) // makes, channels, natural
	}
}
```

<!-- from:typescript -->
## Coming from TypeScript
- A receive-only `<-chan T` is a bit like an async iterator, but the producer runs in parallel instead of being pulled step by step.
- Closing a channel is the "done" signal; there is no `{ done: true }` value. Receiving from a closed channel gives the zero value at once, so `range` is the safe way to read.
- Nothing is buffered by default. A send with nobody receiving blocks forever, and if every goroutine is blocked, the program dies with a deadlock error.
- The direction in the type, `<-chan int` (receive only) or `chan<- int` (send only), is checked by the compiler.
<!-- /from -->

<!-- from:default -->
## Key points
- The goroutine that sends on a channel is the one that closes it, with `defer close(out)`; receivers never close.
- Return the channel before any value is sent; the sending happens in the goroutine.
- For fan-in, close the output only after `wg.Wait()`, in a goroutine of its own, so the function can still return immediately.
- Values from one channel keep their order; values merged from several channels arrive in any order.
<!-- /from -->

## Common mistake
Sending before returning the channel, in the same goroutine:

```go
func Words(text string) <-chan string {
	out := make(chan string)
	for _, w := range strings.Fields(text) {
		out <- w   // nobody can receive yet: blocks forever
	}
	close(out)
	return out
}
// fatal error: all goroutines are asleep - deadlock!
```

Wrap the loop in `go func() { defer close(out); ... }()` and return `out` after it.

## In this exercise
Implement `Generate`, `Square`, `Merge` and `Sum` in `pipeline.go`. You'll need to import `sync` for Merge. The tests read each channel with a 2-second limit, so a missing `close` fails a case instead of hanging.
