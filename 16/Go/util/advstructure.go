package adv

type IntegerHeap []int

func (iheap *IntegerHeap) Len() int { return len(*iheap) }

func (iheap *IntegerHeap) Less(i, j int) bool {
	return (*iheap)[i] < (*iheap)[j]
}

func (iheap *IntegerHeap) Swap(i, j int) {
	(*iheap)[i], (*iheap)[j] = (*iheap)[j], (*iheap)[i]
}

func (iheap *IntegerHeap) Push(x any) {
	*iheap = append(*iheap, x.(int))
}

func (iheap *IntegerHeap) Pop() any {
	var previous IntegerHeap = *iheap
	n := len(previous)
	x1 := previous[n-1]
	*iheap = previous[0 : n-1]
	return x1
}
