package wrappederrors

import "errors"

// ErrNotFound is the sentinel error for a key that isn't in the config.
var ErrNotFound = errors.New("not found")

// ParseError reports a config line that has no "=". Line is 1-based.
type ParseError struct {
	Line int
	Text string
}

// Error makes *ParseError an error. Format: `line 3: no "=" in "oops"`.
func (e *ParseError) Error() string {
	panic("TODO: implement ParseError.Error")
}

// Lookup finds key in config lines of the form "key=value" and returns the
// value. Split each line at its first "=" and trim spaces around key and
// value. Skip blank lines and lines starting with "#".
//
//   - A line without "=" (checked before the key is found): return a
//     *ParseError for that line.
//   - No line has the key: return an error that wraps ErrNotFound and
//     mentions the key, e.g. `key "port": not found`.
func Lookup(lines []string, key string) (string, error) {
	panic("TODO: implement Lookup")
}

// LookupIn calls Lookup and, on failure, wraps its error with the file name in
// front: `app.conf: key "port": not found`. The original error must still be
// reachable with errors.Is and errors.As.
func LookupIn(file string, lines []string, key string) (string, error) {
	panic("TODO: implement LookupIn")
}

// Describe sums up an error from Lookup or LookupIn: "ok" for nil, "missing"
// if it wraps ErrNotFound, "bad line N" if it wraps a *ParseError, and
// "unknown" for anything else.
func Describe(err error) string {
	panic("TODO: implement Describe")
}
