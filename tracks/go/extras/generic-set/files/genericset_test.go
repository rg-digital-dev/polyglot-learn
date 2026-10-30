// Part of the Deeper Go module (tracks/go/extras/generic-set). Don't edit
// this file: `learn` overwrites it. Your code goes in genericset.go.

package genericset

import (
	"slices"
	"strconv"
	"strings"
	"testing"
)

type point struct{ X, Y int }

func TestSet(t *testing.T) {
	t.Run("c01_new_holds_the_given_items", func(t *testing.T) {
		defer failOnPanic(t)
		s := New(3, 1, 2)
		if s.Len() != 3 {
			t.Fatalf("Len() = %d, want 3", s.Len())
		}
		for _, v := range []int{1, 2, 3} {
			if !s.Has(v) {
				t.Fatalf("Has(%d) = false, want true", v)
			}
		}
	})
	t.Run("c02_duplicates_count_once", func(t *testing.T) {
		defer failOnPanic(t)
		s := New("a", "b", "a")
		s.Add("b")
		s.Add("c")
		if s.Len() != 3 {
			t.Fatalf("Len() = %d, want 3 (a, b, c)", s.Len())
		}
	})
	t.Run("c03_has_reports_missing_values", func(t *testing.T) {
		defer failOnPanic(t)
		s := New(1, 2)
		if s.Has(3) {
			t.Fatalf("Has(3) = true for a set of 1 and 2")
		}
	})
	t.Run("c04_remove_takes_a_value_out", func(t *testing.T) {
		defer failOnPanic(t)
		s := New(1, 2, 3)
		s.Remove(2)
		s.Remove(99) // missing: nothing happens
		if s.Has(2) || s.Len() != 2 {
			t.Fatalf("after Remove(2): Has(2) = %v, Len() = %d; want false, 2", s.Has(2), s.Len())
		}
	})
	t.Run("c05_the_zero_value_is_an_empty_set_ready_to_use", func(t *testing.T) {
		defer failOnPanic(t)
		var s Set[string]
		if s.Len() != 0 || s.Has("x") {
			t.Fatalf("a zero Set should be empty")
		}
		s.Add("x")
		if !s.Has("x") || s.Len() != 1 {
			t.Fatalf("after Add(\"x\"): Has = %v, Len = %d; want true, 1", s.Has("x"), s.Len())
		}
	})
	t.Run("c06_sorted_returns_numbers_in_order", func(t *testing.T) {
		defer failOnPanic(t)
		got := Sorted(New(5, -1, 3, 5, 0))
		if want := []int{-1, 0, 3, 5}; !slices.Equal(got, want) {
			t.Fatalf("got %v, want %v", got, want)
		}
	})
	t.Run("c07_sorted_works_for_strings_too", func(t *testing.T) {
		defer failOnPanic(t)
		got := Sorted(New("pear", "apple", "fig"))
		if want := []string{"apple", "fig", "pear"}; !slices.Equal(got, want) {
			t.Fatalf("got %q, want %q", got, want)
		}
	})
	t.Run("c08_a_set_can_hold_structs", func(t *testing.T) {
		defer failOnPanic(t)
		// point is comparable (== works on it), so Set[point] is allowed,
		// even though points can't be sorted.
		s := New(point{1, 2}, point{1, 2}, point{0, 0})
		if s.Len() != 2 || !s.Has(point{0, 0}) || s.Has(point{2, 1}) {
			t.Fatalf("Set[point] should hold {1 2} and {0 0} once each")
		}
	})
}

func TestMapFilter(t *testing.T) {
	t.Run("c09_map_converts_every_item_in_order", func(t *testing.T) {
		defer failOnPanic(t)
		got := Map([]int{1, 20, 300}, strconv.Itoa)
		if want := []string{"1", "20", "300"}; !slices.Equal(got, want) {
			t.Fatalf("got %q, want %q", got, want)
		}
		lengths := Map([]string{"go", "rust"}, func(s string) int { return len(s) })
		if want := []int{2, 4}; !slices.Equal(lengths, want) {
			t.Fatalf("got %v, want %v", lengths, want)
		}
	})
	t.Run("c10_filter_keeps_matching_items_in_order", func(t *testing.T) {
		defer failOnPanic(t)
		got := Filter([]string{"go", "rust", "gleam", "lua"}, func(s string) bool {
			return strings.HasPrefix(s, "g")
		})
		if want := []string{"go", "gleam"}; !slices.Equal(got, want) {
			t.Fatalf("got %q, want %q", got, want)
		}
		if none := Filter([]int{1, 3}, func(n int) bool { return n%2 == 0 }); len(none) != 0 {
			t.Fatalf("got %v, want nothing", none)
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
