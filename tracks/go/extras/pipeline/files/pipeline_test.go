// Part of the Deeper Go module (tracks/go/extras/pipeline). Don't edit this
// file: `learn` overwrites it. Your code goes in pipeline.go.

package pipeline

import (
	"slices"
	"testing"
	"time"
)

func TestPipeline(t *testing.T) {
	t.Run("c01_generate_sends_every_number_in_order", func(t *testing.T) {
		got := collect(t, func() <-chan int { return Generate(3, 1, 2) })
		wantInts(t, got, []int{3, 1, 2})
	})
	t.Run("c02_generate_returns_before_the_values_are_read", func(t *testing.T) {
		nums := make([]int, 100)
		// Nobody reads from the channel here: Generate must not send before returning.
		within(t, "Generate didn't return: send from a goroutine, not before returning", func() <-chan int {
			return Generate(nums...)
		})
	})
	t.Run("c03_generate_with_no_numbers_closes_right_away", func(t *testing.T) {
		got := collect(t, func() <-chan int { return Generate() })
		wantInts(t, got, nil)
	})
	t.Run("c04_square_squares_each_value_in_order", func(t *testing.T) {
		got := collect(t, func() <-chan int { return Square(source(1, 2, 3, -4)) })
		wantInts(t, got, []int{1, 4, 9, 16})
	})
	t.Run("c05_square_of_an_empty_stream_is_empty", func(t *testing.T) {
		got := collect(t, func() <-chan int { return Square(source()) })
		wantInts(t, got, nil)
	})
	t.Run("c06_merge_forwards_every_value_from_every_input", func(t *testing.T) {
		got := collect(t, func() <-chan int { return Merge(source(1, 2, 3), source(10, 20), source(100)) })
		slices.Sort(got) // values from different inputs may arrive in any order
		wantInts(t, got, []int{1, 2, 3, 10, 20, 100})
	})
	t.Run("c07_merge_with_one_input_keeps_its_order", func(t *testing.T) {
		got := collect(t, func() <-chan int { return Merge(source(5, 4, 3, 2, 1)) })
		wantInts(t, got, []int{5, 4, 3, 2, 1})
	})
	t.Run("c08_merge_with_no_inputs_closes_right_away", func(t *testing.T) {
		got := collect(t, func() <-chan int { return Merge() })
		wantInts(t, got, nil)
	})
	t.Run("c09_sum_adds_every_value_until_the_channel_closes", func(t *testing.T) {
		got := within(t, "Sum didn't return", func() int { return Sum(source(4, 5, 6)) })
		if got != 15 {
			t.Fatalf("got %d, want 15", got)
		}
	})
	t.Run("c10_stages_chain_into_a_pipeline", func(t *testing.T) {
		got := within(t, "the pipeline didn't finish", func() int {
			// Two branches squared in parallel, merged, then summed:
			// 1+4 + 9+16 = 30.
			return Sum(Merge(Square(Generate(1, 2)), Square(Generate(3, 4))))
		})
		if got != 30 {
			t.Fatalf("got %d, want 30", got)
		}
	})
}

// source returns a closed, buffered channel holding nums.
func source(nums ...int) <-chan int {
	ch := make(chan int, len(nums))
	for _, n := range nums {
		ch <- n
	}
	close(ch)
	return ch
}

// collect calls start and reads everything from the channel it returns.
func collect(t *testing.T, start func() <-chan int) []int {
	t.Helper()
	return within(t, "the channel was never closed (or Generate never returned)", func() []int {
		var out []int
		for n := range start() {
			out = append(out, n)
		}
		return out
	})
}

// within runs f in a goroutine and fails the test if f panics or doesn't
// return within 2 seconds, so a missing close() can't hang the test run.
func within[T any](t *testing.T, timeoutMsg string, f func() T) T {
	t.Helper()
	type result struct {
		value    T
		panicked any
		ok       bool
	}
	done := make(chan result, 1)
	go func() {
		defer func() {
			if r := recover(); r != nil {
				done <- result{panicked: r}
			}
		}()
		done <- result{value: f(), ok: true}
	}()
	select {
	case r := <-done:
		if !r.ok {
			t.Fatalf("panicked: %v", r.panicked)
		}
		return r.value
	case <-time.After(2 * time.Second):
		t.Fatalf("timed out after 2s: %s", timeoutMsg)
	}
	var zero T
	return zero
}

func wantInts(t *testing.T, got, want []int) {
	t.Helper()
	if !slices.Equal(got, want) {
		t.Fatalf("got %v, want %v", got, want)
	}
}
