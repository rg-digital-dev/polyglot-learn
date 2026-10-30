// Part of the Deeper Go module (tracks/go/extras/wrapped-errors). Don't edit
// this file: `learn` overwrites it. Your code goes in wrappederrors.go.

package wrappederrors

import (
	"errors"
	"fmt"
	"strings"
	"testing"
)

var config = []string{
	"# server settings",
	"host = example.org",
	"",
	"port=8080",
}

func TestLookup(t *testing.T) {
	t.Run("c01_lookup_returns_the_value_for_a_key", func(t *testing.T) {
		defer failOnPanic(t)
		wantValue(t, config, "port", "8080")
		wantValue(t, config, "host", "example.org")
	})
	t.Run("c02_blank_lines_and_comments_are_skipped", func(t *testing.T) {
		defer failOnPanic(t)
		wantValue(t, []string{"", "# port=1", "   ", "port=2"}, "port", "2")
	})
	t.Run("c03_a_missing_key_wraps_errnotfound", func(t *testing.T) {
		defer failOnPanic(t)
		_, err := Lookup(config, "user")
		if !errors.Is(err, ErrNotFound) {
			t.Fatalf("got error %v, want one that wraps ErrNotFound", err)
		}
		if err == ErrNotFound {
			t.Fatalf("got ErrNotFound itself; wrap it with fmt.Errorf and %%w to add the key")
		}
		if !strings.Contains(err.Error(), `"user"`) {
			t.Fatalf("error %q should mention the key \"user\"", err)
		}
	})
	t.Run("c04_a_line_without_an_equals_sign_is_a_parseerror", func(t *testing.T) {
		defer failOnPanic(t)
		_, err := Lookup([]string{"a=1", "oops", "port=80"}, "port")
		var pe *ParseError
		if !errors.As(err, &pe) {
			t.Fatalf("got error %v, want a *ParseError", err)
		}
		if pe.Line != 2 || pe.Text != "oops" {
			t.Fatalf("got ParseError{Line: %d, Text: %q}, want {Line: 2, Text: \"oops\"}", pe.Line, pe.Text)
		}
	})
	t.Run("c05_parseerror_message_names_the_line_and_text", func(t *testing.T) {
		defer failOnPanic(t)
		got := (&ParseError{Line: 3, Text: "oops"}).Error()
		if want := `line 3: no "=" in "oops"`; got != want {
			t.Fatalf("got %q, want %q", got, want)
		}
	})
	t.Run("c06_lookupin_puts_the_file_name_in_front", func(t *testing.T) {
		defer failOnPanic(t)
		got, err := LookupIn("app.conf", config, "port")
		if err != nil || got != "8080" {
			t.Fatalf("LookupIn found port: got (%q, %v), want (\"8080\", nil)", got, err)
		}
		_, err = LookupIn("app.conf", config, "user")
		if err == nil {
			t.Fatalf("expected an error for a missing key")
		}
		if want := `app.conf: key "user": not found`; err.Error() != want {
			t.Fatalf("got %q, want %q", err.Error(), want)
		}
	})
	t.Run("c07_errors_is_sees_through_both_layers", func(t *testing.T) {
		defer failOnPanic(t)
		_, err := LookupIn("app.conf", config, "user")
		if !errors.Is(err, ErrNotFound) {
			t.Fatalf("errors.Is(%v, ErrNotFound) = false: use %%w, not %%v", err)
		}
	})
	t.Run("c08_errors_as_sees_through_both_layers", func(t *testing.T) {
		defer failOnPanic(t)
		_, err := LookupIn("app.conf", []string{"oops"}, "port")
		var pe *ParseError
		if !errors.As(err, &pe) {
			t.Fatalf("errors.As(%v, &pe) = false: use %%w, not %%v", err)
		}
		if pe.Line != 1 {
			t.Fatalf("got Line %d, want 1", pe.Line)
		}
	})
	t.Run("c09_describe_names_each_kind_of_error", func(t *testing.T) {
		defer failOnPanic(t)
		cases := []struct {
			err  error
			want string
		}{
			{nil, "ok"},
			{fmt.Errorf("a: %w", fmt.Errorf("b: %w", ErrNotFound)), "missing"},
			{fmt.Errorf("x.conf: %w", &ParseError{Line: 4, Text: "?"}), "bad line 4"},
			{errors.New("disk full"), "unknown"},
		}
		for _, c := range cases {
			if got := Describe(c.err); got != c.want {
				t.Fatalf("Describe(%v) = %q, want %q", c.err, got, c.want)
			}
		}
	})
}

func wantValue(t *testing.T, lines []string, key, want string) {
	t.Helper()
	got, err := Lookup(lines, key)
	if err != nil {
		t.Fatalf("Lookup(%q): unexpected error: %v", key, err)
	}
	if got != want {
		t.Fatalf("Lookup(%q) = %q, want %q", key, got, want)
	}
}

// failOnPanic turns a panic (like the stub's) into a normal test failure, so
// one panicking case doesn't stop the others from running.
func failOnPanic(t *testing.T) {
	if r := recover(); r != nil {
		t.Fatalf("panicked: %v", r)
	}
}
