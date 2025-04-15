package main

import (
	"fmt"
	"strings"

	"container/heap"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

type CharCount struct {
	char  rune
	count int
}

func (c *CharCount) String() string {
	return fmt.Sprintf("%c: %v", c.char, c.count)
}

type CharHeap []CharCount

func (h CharHeap) Len() int { return len(h) }
func (h CharHeap) Less(i, j int) bool {
	if h[i].count == h[j].count {
		return h[i].char < h[j].char // Alphabetical order if counts are equal
	}
	return h[i].count > h[j].count // Swap directions for part two
}
func (h CharHeap) Swap(i, j int) { h[i], h[j] = h[j], h[i] }

func (h *CharHeap) Push(x any) {
	*h = append(*h, x.(CharCount))
}

func (h *CharHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func AverageChars(lines []string, cols int) string {
	counts := make([]map[rune]int, cols)
	for _, line := range lines {
		for i, c := range strings.TrimSpace(line) {
			if counts[i] == nil {
				counts[i] = make(map[rune]int)
			}
			counts[i][c]++
		}
	}
	ave := ""
	for i := range cols {
		rank := &CharHeap{}
		for key, count := range counts[i] {
			fmt.Printf("{%c -> %v}, ", key, count)
			heap.Push(rank, CharCount{key, count})
		}
		// fmt.Printf("%v\n", rank)
		top := heap.Pop(rank).(CharCount)
		fmt.Printf("\n%c ! %v\n", top.char, top.count)
		ave += string(top.char)
	}
	return ave
}

func main() {
	fmt.Println("Starting day 6")
	inputs := adv.GetInput("6", true, "\n", true)
	p1 := AverageChars(inputs, 8)
	fmt.Printf("Answer 1: %v\n", p1)
}
