package withtimeout

import (
	"context"
	"errors"
	"time"
)

// ErrTimeout is returned by WithTimeout when the work takes too long.
var ErrTimeout = errors.New("timed out")

// WithTimeout runs work in a goroutine. If work returns within limit, return
// its result and a nil error. Otherwise return 0 and ErrTimeout as soon as the
// limit is reached, without waiting for work to finish.
func WithTimeout(work func() int, limit time.Duration) (int, error) {
	panic("TODO: implement WithTimeout")
}

// WithContext runs work in a goroutine. If work returns before ctx is done,
// return its result and a nil error. If ctx is cancelled or its deadline
// passes first, return 0 and ctx.Err() right away.
func WithContext(ctx context.Context, work func() int) (int, error) {
	panic("TODO: implement WithContext")
}
