// Part of the Deeper Go module (tracks/go/extras/with-timeout). Don't edit
// this file: `learn` overwrites it. Your code goes in withtimeout.go.
//
// Timings are deliberately loose so slow machines don't fail: "slow" work
// sleeps for seconds, and "right away" means well under a second.

package withtimeout

import (
	"context"
	"errors"
	"testing"
	"time"
)

// sleepy returns work that sleeps for d, then returns v.
func sleepy(d time.Duration, v int) func() int {
	return func() int {
		time.Sleep(d)
		return v
	}
}

func TestWithTimeout(t *testing.T) {
	t.Run("c01_fast_work_returns_its_result", func(t *testing.T) {
		defer failOnPanic(t)
		got, err := WithTimeout(sleepy(20*time.Millisecond, 42), time.Second)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if got != 42 {
			t.Fatalf("got %d, want 42", got)
		}
	})
	t.Run("c02_slow_work_returns_errtimeout", func(t *testing.T) {
		defer failOnPanic(t)
		got, err := WithTimeout(sleepy(time.Second, 42), 50*time.Millisecond)
		if !errors.Is(err, ErrTimeout) {
			t.Fatalf("got error %v, want ErrTimeout", err)
		}
		if got != 0 {
			t.Fatalf("got %d with the error, want 0", got)
		}
	})
	t.Run("c03_a_timeout_does_not_wait_for_the_work_to_finish", func(t *testing.T) {
		defer failOnPanic(t)
		start := time.Now()
		_, err := WithTimeout(sleepy(2*time.Second, 1), 50*time.Millisecond)
		elapsed := time.Since(start)
		if !errors.Is(err, ErrTimeout) {
			t.Fatalf("got error %v, want ErrTimeout", err)
		}
		if elapsed > time.Second {
			t.Fatalf("took %v: return when the limit is reached, not when the work ends", elapsed.Round(time.Millisecond))
		}
	})
	t.Run("c04_a_timeout_waits_at_least_the_limit", func(t *testing.T) {
		defer failOnPanic(t)
		start := time.Now()
		_, err := WithTimeout(sleepy(2*time.Second, 1), 200*time.Millisecond)
		elapsed := time.Since(start)
		if !errors.Is(err, ErrTimeout) {
			t.Fatalf("got error %v, want ErrTimeout", err)
		}
		if elapsed < 150*time.Millisecond {
			t.Fatalf("gave up after %v, but the limit was 200ms", elapsed.Round(time.Millisecond))
		}
	})
}

func TestWithContext(t *testing.T) {
	t.Run("c05_work_that_beats_the_deadline_returns_its_result", func(t *testing.T) {
		defer failOnPanic(t)
		ctx, cancel := context.WithTimeout(context.Background(), time.Second)
		defer cancel()
		got, err := WithContext(ctx, sleepy(20*time.Millisecond, 7))
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if got != 7 {
			t.Fatalf("got %d, want 7", got)
		}
	})
	t.Run("c06_a_passed_deadline_returns_deadlineexceeded", func(t *testing.T) {
		defer failOnPanic(t)
		ctx, cancel := context.WithTimeout(context.Background(), 50*time.Millisecond)
		defer cancel()
		start := time.Now()
		got, err := WithContext(ctx, sleepy(2*time.Second, 7))
		elapsed := time.Since(start)
		if !errors.Is(err, context.DeadlineExceeded) {
			t.Fatalf("got error %v, want context.DeadlineExceeded", err)
		}
		if got != 0 {
			t.Fatalf("got %d with the error, want 0", got)
		}
		if elapsed > time.Second {
			t.Fatalf("took %v: return when ctx.Done() is closed", elapsed.Round(time.Millisecond))
		}
	})
	t.Run("c07_cancelling_the_context_stops_the_wait", func(t *testing.T) {
		defer failOnPanic(t)
		ctx, cancel := context.WithCancel(context.Background())
		defer cancel()
		time.AfterFunc(50*time.Millisecond, cancel)
		start := time.Now()
		_, err := WithContext(ctx, sleepy(2*time.Second, 7))
		elapsed := time.Since(start)
		if !errors.Is(err, context.Canceled) {
			t.Fatalf("got error %v, want context.Canceled", err)
		}
		if elapsed > time.Second {
			t.Fatalf("took %v: return when ctx.Done() is closed", elapsed.Round(time.Millisecond))
		}
	})
	t.Run("c08_an_already_cancelled_context_returns_right_away", func(t *testing.T) {
		defer failOnPanic(t)
		ctx, cancel := context.WithCancel(context.Background())
		cancel()
		start := time.Now()
		_, err := WithContext(ctx, sleepy(2*time.Second, 7))
		elapsed := time.Since(start)
		if !errors.Is(err, context.Canceled) {
			t.Fatalf("got error %v, want context.Canceled", err)
		}
		if elapsed > time.Second {
			t.Fatalf("took %v: a cancelled context should return at once", elapsed.Round(time.Millisecond))
		}
	})
}

// failOnPanic turns a panic (like the stub's) into a normal test failure, so
// one panicking case doesn't stop the others from running.
func failOnPanic(t *testing.T) {
	if r := recover(); r != nil {
		t.Fatalf("panicked: %v", r)
	}
}
