package genericset

import "cmp"

// Set holds distinct values of any comparable type. The zero value is an
// empty set that is ready to use, so Add must create the map when it is nil.
type Set[T comparable] struct {
	items map[T]struct{}
}

// New returns a set holding the given items (duplicates count once).
func New[T comparable](items ...T) *Set[T] {
	panic("TODO: implement New")
}

// Add puts v in the set; adding a value twice keeps one copy.
func (s *Set[T]) Add(v T) {
	panic("TODO: implement Set.Add")
}

// Has reports whether v is in the set.
func (s *Set[T]) Has(v T) bool {
	panic("TODO: implement Set.Has")
}

// Remove takes v out of the set. Removing a missing value does nothing.
func (s *Set[T]) Remove(v T) {
	panic("TODO: implement Set.Remove")
}

// Len returns the number of values in the set.
func (s *Set[T]) Len() int {
	panic("TODO: implement Set.Len")
}

// Sorted returns the set's values in ascending order. It is a function, not a
// method, because it needs the stricter cmp.Ordered constraint to sort.
func Sorted[T cmp.Ordered](s *Set[T]) []T {
	panic("TODO: implement Sorted")
}

// Map returns f applied to every item, in order.
func Map[T, U any](items []T, f func(T) U) []U {
	panic("TODO: implement Map")
}

// Filter returns the items for which keep returns true, in order.
func Filter[T any](items []T, keep func(T) bool) []T {
	panic("TODO: implement Filter")
}
