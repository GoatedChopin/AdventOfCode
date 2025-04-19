package adv

/*
For generating a sequence of combinations based on an array's
index and the length of each group
*/
func FixedLengthCombinations(arrayLength int, groupLength int, upTo bool, from int) <-chan []int {
	ch := make(chan []int)
	items := make([]int, arrayLength)
	for i := range arrayLength {
		items[i] = i
	}
	// numCombos := Factorial(arrayLength) / (Factorial(groupLength) * Factorial(arrayLength-groupLength))
	// combinations := make([][]int, numCombos)
	go func(ch chan []int) {
		defer close(ch)
		add(ch, []int{}, items, groupLength, upTo, from)
	}(ch)
	return ch
}

func add(ch chan []int, current []int, items []int, groupLength int, upTo bool, from int) {
	if len(current) == groupLength {
		ch <- current
		return
	} else if upTo && len(current) < groupLength && len(current) >= from {
		ch <- current
	}
	for i, item := range items {
		add(ch, append(current, item), items[i+1:], groupLength, upTo, from)
	}
}
