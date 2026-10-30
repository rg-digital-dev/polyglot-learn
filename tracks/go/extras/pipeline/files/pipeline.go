package pipeline

// Generate starts a goroutine that sends each number, in order, on the
// returned channel and then closes it. It returns right away, before anyone
// reads.
func Generate(nums ...int) <-chan int {
	panic("TODO: implement Generate")
}

// Square starts a goroutine that reads every value from in, sends its square
// on the returned channel in the same order, and closes that channel once in
// is closed.
func Square(in <-chan int) <-chan int {
	panic("TODO: implement Square")
}

// Merge forwards every value from all the input channels to one output
// channel (in any order) and closes the output once every input is closed.
// Use one goroutine per input and a sync.WaitGroup to know when all are done.
func Merge(ins ...<-chan int) <-chan int {
	panic("TODO: implement Merge")
}

// Sum reads values from in until it is closed and returns their total.
func Sum(in <-chan int) int {
	panic("TODO: implement Sum")
}
