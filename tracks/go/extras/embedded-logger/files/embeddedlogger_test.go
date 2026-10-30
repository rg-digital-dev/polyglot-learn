// Part of the Deeper Go module (tracks/go/extras/embedded-logger). Don't edit
// this file: `learn` overwrites it. Your code goes in embeddedlogger.go.

package embeddedlogger

import (
	"reflect"
	"testing"
)

func TestLoggers(t *testing.T) {
	t.Run("c01_plain_logs_messages_with_its_prefix", func(t *testing.T) {
		defer failOnPanic(t)
		l := NewPlain("[app] ")
		l.Log("started")
		wantLines(t, l.Lines(), "[app] started")
	})
	t.Run("c02_lines_come_back_in_the_order_they_were_logged", func(t *testing.T) {
		defer failOnPanic(t)
		l := NewPlain("")
		l.Log("one")
		l.Log("two")
		l.Log("three")
		wantLines(t, l.Lines(), "one", "two", "three")
	})
	t.Run("c03_lines_returns_a_copy", func(t *testing.T) {
		defer failOnPanic(t)
		l := NewPlain("")
		l.Log("keep me")
		got := l.Lines()
		if len(got) == 1 {
			got[0] = "changed by the caller"
		}
		wantLines(t, l.Lines(), "keep me")
	})
	t.Run("c04_a_new_logger_has_no_lines", func(t *testing.T) {
		defer failOnPanic(t)
		if got := NewPlain("x ").Lines(); len(got) != 0 {
			t.Fatalf("got %q, want no lines", got)
		}
	})
	t.Run("c05_shouty_uppercases_and_adds_an_exclamation_mark", func(t *testing.T) {
		defer failOnPanic(t)
		l := NewShouty("")
		l.Log("disk is full")
		l.Log("again")
		wantLines(t, l.Lines(), "DISK IS FULL!", "AGAIN!")
	})
	t.Run("c06_shouty_keeps_the_prefix_as_it_is", func(t *testing.T) {
		defer failOnPanic(t)
		l := NewShouty("[db] ")
		l.Log("timeout")
		wantLines(t, l.Lines(), "[db] TIMEOUT!")
	})
	t.Run("c07_each_logger_keeps_its_own_lines", func(t *testing.T) {
		defer failOnPanic(t)
		a := NewPlain("a: ")
		b := NewPlain("b: ")
		a.Log("first")
		b.Log("second")
		wantLines(t, a.Lines(), "a: first")
		wantLines(t, b.Lines(), "b: second")
	})
	t.Run("c08_broadcast_logs_to_every_logger", func(t *testing.T) {
		defer failOnPanic(t)
		plain := NewPlain("> ")
		shouty := NewShouty("> ")
		Broadcast("deploy done", plain, shouty)
		wantLines(t, plain.Lines(), "> deploy done")
		wantLines(t, shouty.Lines(), "> DEPLOY DONE!")
	})
	t.Run("c09_base_on_its_own_is_a_logger", func(t *testing.T) {
		defer failOnPanic(t)
		// *Base has Log and Lines, so it satisfies Logger with no declaration.
		var l Logger = &Base{Prefix: "base: "}
		l.Log("hello")
		wantLines(t, l.Lines(), "base: hello")
	})
	t.Run("c10_plain_and_shouty_embed_base", func(t *testing.T) {
		defer failOnPanic(t)
		checkEmbeds(t, NewPlain(""), reflect.TypeOf(&Plain{}))
		checkEmbeds(t, NewShouty(""), reflect.TypeOf(&Shouty{}))
	})
}

func wantLines(t *testing.T, got []string, want ...string) {
	t.Helper()
	if len(got) != len(want) {
		t.Fatalf("got %q, want %q", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("got %q, want %q", got, want)
		}
	}
}

func checkEmbeds(t *testing.T, l Logger, want reflect.Type) {
	t.Helper()
	got := reflect.TypeOf(l)
	if got != want {
		t.Fatalf("got a %v, want a %v", got, want)
	}
	f, ok := want.Elem().FieldByName("Base")
	if !ok || !f.Anonymous || f.Type != reflect.TypeOf(Base{}) {
		t.Fatalf("%v should embed Base (a field written as just `Base`)", want.Elem())
	}
}

// failOnPanic turns a panic (like the stub's) into a normal test failure, so
// one panicking case doesn't stop the others from running.
func failOnPanic(t *testing.T) {
	if r := recover(); r != nil {
		t.Fatalf("panicked: %v", r)
	}
}
