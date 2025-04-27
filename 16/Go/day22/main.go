package main

import (
	"container/heap"
	"fmt"
	"strconv"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

// Node represents each cell
type Node struct {
	size int
	used int
}

func (n *Node) Avail() int {
	return n.size - n.used
}

func (n *Node) Empty() bool {
	return n.used == 0
}

// Grid: map of (x,y) → Node
type Grid map[[2]int]Node

// State: where the goal is, where the empty is, how many steps
type State struct {
	goalX, goalY   int
	emptyX, emptyY int
	steps          int
	priority       int // f = g + h
}

// PriorityQueue for A*
type PriorityQueue []*State

func (pq PriorityQueue) Len() int            { return len(pq) }
func (pq PriorityQueue) Less(i, j int) bool  { return pq[i].priority < pq[j].priority }
func (pq PriorityQueue) Swap(i, j int)       { pq[i], pq[j] = pq[j], pq[i] }
func (pq *PriorityQueue) Push(x interface{}) { *pq = append(*pq, x.(*State)) }
func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[0 : n-1]
	return item
}

// Heuristic: how far goal is from (0,0)
func (s *State) Heuristic() int {
	return s.goalX + s.goalY
}

// Hash: uniquely identifies a state
func (s *State) Hash() string {
	return fmt.Sprintf("%d,%d-%d,%d", s.goalX, s.goalY, s.emptyX, s.emptyY)
}

// Parse input
func ParseNodes(lines []string) (Grid, int, int, int, int) {
	grid := make(Grid)
	var emptyX, emptyY int
	var maxX, maxY int
	for _, line := range lines[2:] {
		parts := strings.Fields(line)
		if len(parts) < 4 {
			continue
		}
		name := parts[0]
		size, _ := strconv.Atoi(strings.TrimSuffix(parts[1], "T"))
		used, _ := strconv.Atoi(strings.TrimSuffix(parts[2], "T"))

		name = strings.ReplaceAll(name, "x", "")
		name = strings.ReplaceAll(name, "y", "")
		coords := strings.Split(name, "-")

		x, _ := strconv.Atoi(coords[1])
		y, _ := strconv.Atoi(coords[2])

		if used == 0 {
			emptyX, emptyY = x, y
		}
		grid[[2]int{x, y}] = Node{size, used}

		if x > maxX {
			maxX = x
		}
		if y > maxY {
			maxY = y
		}
	}
	return grid, emptyX, emptyY, maxX, maxY
}

// Generate moves from the current empty spot
func GenMoves(grid Grid, emptyX, emptyY int) [][2]int {
	moves := make([][2]int, 0, 4)
	dirs := [][2]int{{0, 1}, {1, 0}, {0, -1}, {-1, 0}} // Down, Right, Up, Left
	empty := grid[[2]int{emptyX, emptyY}]
	for _, d := range dirs {
		nx, ny := emptyX+d[0], emptyY+d[1]
		neighbor, ok := grid[[2]int{nx, ny}]
		if !ok {
			continue
		}
		if empty.size >= neighbor.used {
			moves = append(moves, [2]int{nx, ny})
		}
	}
	return moves
}

// Main A* solver
func MinMoves(grid Grid, emptyX, emptyY, maxX, maxY int) int {
	initial := &State{goalX: maxX, goalY: 0, emptyX: emptyX, emptyY: emptyY, steps: 0}
	initial.priority = initial.steps + initial.Heuristic()

	visited := make(map[string]bool)
	pq := make(PriorityQueue, 0)
	heap.Init(&pq)
	heap.Push(&pq, initial)

	for pq.Len() > 0 {
		current := heap.Pop(&pq).(*State)
		h := current.Hash()
		if visited[h] {
			continue
		}
		visited[h] = true

		// Goal reached
		if current.goalX == 0 && current.goalY == 0 {
			return current.steps
		}

		for _, move := range GenMoves(grid, current.emptyX, current.emptyY) {
			nx, ny := move[0], move[1]

			newGoalX, newGoalY := current.goalX, current.goalY
			// If moving the empty under the goal, the goal moves
			if nx == current.goalX && ny == current.goalY {
				newGoalX, newGoalY = current.emptyX, current.emptyY
			}

			newState := &State{
				goalX: newGoalX, goalY: newGoalY,
				emptyX: nx, emptyY: ny,
				steps: current.steps + 1,
			}
			newState.priority = newState.steps + newState.Heuristic()

			heap.Push(&pq, newState)
		}
	}
	return -1
}

func main() {
	fmt.Print("Starting day 22\n")
	inputs := adv.GetInput("22", true, "\n", true)
	grid, emptyX, emptyY, maxX, maxY := ParseNodes(inputs)

	part2 := MinMoves(grid, emptyX, emptyY, maxX, maxY)
	fmt.Printf("Part 2: %v\n", part2)
}
