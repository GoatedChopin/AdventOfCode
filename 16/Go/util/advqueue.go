package adv

import (
	"fmt"
)

type Queue struct {
	List []int
}

// function to add an element in the queue
func (q *Queue) Enqueue(element int) {
	q.List = append(q.List, element)
}

// function to delete an element in the queue
func (q *Queue) Dequeue() int {
	if q.isEmpty() {
		fmt.Println("Queue is empty.")
		return 0
	}
	element := q.List[0]
	q.List = q.List[1:]
	return element
}

// function checks whether the queue is empty or not
func (q *Queue) isEmpty() bool {
	return len(q.List) == 0
}
