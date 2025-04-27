package main

import (
	"container/heap"
	"fmt"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

func ParseGrid(lines []string) ([][]bool, map[[2]int]bool, [2]int) {
	var startPoint [2]int
	pointsOfInterest := make(map[[2]int]bool)
	walls := make([][]bool, len(lines))
	for i, line := range lines {
		walls[i] = make([]bool, len(line))
		for col, r := range line {
			if r == '#' {
				walls[i][col] = true
			} else if r == '.' {
				// do nothing
			} else if r == '0' {
				startPoint[0] = i
				startPoint[1] = col
			} else {
				pointsOfInterest[[2]int{i, col}] = true
			}
		}
	}
	return walls, pointsOfInterest, startPoint
}

type RoboState struct {
	x, y, steps, cost int
	visited           map[[2]int]bool
	returning         bool
}

type RoboQueue []RoboState

func (r RoboQueue) Less(a, b int) bool {
	return r[a].cost < r[b].cost
}

func (r RoboQueue) Swap(a, b int) {
	r[a], r[b] = r[b], r[a]
}

func (r RoboQueue) Len() int {
	return len(r)
}

func (h *RoboQueue) Push(x any) {
	*h = append(*h, x.(RoboState))
}

func (h *RoboQueue) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func Manhattan(a, b, c, d int) int {
	xd := a - c
	yd := b - d
	if xd < 0 {
		xd = -xd
	}
	if yd < 0 {
		yd = -yd
	}
	return xd + yd
}

func TotalCost(x, y int, pois *map[[2]int]bool, visited *map[[2]int]bool) int {
	cost := 0
	for key := range *pois {
		if (*visited)[key] {
			continue
		}
		cost += Manhattan(x, y, key[0], key[1])
	}
	return cost
}

func inBounds(x, y int, walls *[][]bool) bool {
	if x < 0 || x >= len(*walls) {
		return false
	}
	if y < 0 || y >= len((*walls)[0]) {
		return false
	}
	return true
}

func GenMoves(x, y int, walls *[][]bool) [][2]int {
	out := make([][2]int, 0)
	for _, dir := range [][2]int{[2]int{0, 1}, [2]int{0, -1}, [2]int{1, 0}, [2]int{-1, 0}} {
		nx, ny := x+dir[0], y+dir[1]
		if inBounds(nx, ny, walls) {
			if (*walls)[nx][ny] {
				continue
			}
			out = append(out, [2]int{nx, ny})
		}
	}
	return out
}

func CopyVisited(visited *map[[2]int]bool) map[[2]int]bool {
	out := make(map[[2]int]bool)
	for key := range *visited {
		out[key] = (*visited)[key]
	}
	return out
}

func Render(walls *[][]bool, pois *map[[2]int]bool, state *RoboState) {
	for r := range len(*walls) {
		for c := range len((*walls)[0]) {
			if (*walls)[r][c] {
				fmt.Print("#")
			} else if (*state).x == r && (*state).y == c {
				fmt.Print("X")
			} else if (*state).visited[[2]int{r, c}] {
				fmt.Print("$")
			} else if (*pois)[[2]int{r, c}] {
				fmt.Print("@")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Print("\n")
	}
	fmt.Print("\n")
}

func SerializeVisited(visited map[[2]int]bool) string {
	keys := make([][2]int, 0, len(visited))
	for k := range visited {
		keys = append(keys, k)
	}
	// It's fine to just iterate without sorting since the set is small.
	// You could sort keys if needed for deterministic output.
	out := ""
	for _, k := range keys {
		out += fmt.Sprintf("%d,%d|", k[0], k[1])
	}
	return out
}

func FindPath(walls [][]bool, pois map[[2]int]bool, startPoint [2]int) int {
	pq := RoboQueue{}
	heap.Init(&pq)
	heap.Push(&pq, RoboState{startPoint[0], startPoint[1], 0, 0, make(map[[2]int]bool), false})
	visitedStates := make(map[string]int)
	for pq.Len() > 0 {
		state := heap.Pop(&pq).(RoboState)
		if len(state.visited) == len(pois) {
			return state.steps
		}
		// Render(&walls, &pois, &state)
		for _, move := range GenMoves(state.x, state.y, &walls) {
			newVisited := state.visited
			if pois[move] && !state.visited[move] {
				newVisited = CopyVisited(&state.visited)
				newVisited[move] = true
			}
			key := fmt.Sprintf("%d,%d|%s", move[0], move[1], SerializeVisited(newVisited))
			if prevSteps, ok := visitedStates[key]; ok && prevSteps <= state.steps+1 {
				continue // we've already reached here faster
			}
			visitedStates[key] = state.steps + 1
			totalCost := (2 * (state.steps + 1)) + TotalCost(move[0], move[1], &pois, &newVisited)
			heap.Push(&pq, RoboState{move[0], move[1], state.steps + 1, totalCost, newVisited, false})
		}
	}
	return -1
}

func FindPathAndReturn(walls [][]bool, pois map[[2]int]bool, startPoint [2]int) int {
	pq := RoboQueue{}
	heap.Init(&pq)
	heap.Push(&pq, RoboState{startPoint[0], startPoint[1], 0, 0, make(map[[2]int]bool), false})
	visitedStates := make(map[string]int)
	for pq.Len() > 0 {
		state := heap.Pop(&pq).(RoboState)
		if len(state.visited) == len(pois) {
			state.returning = true
		}
		// Render(&walls, &pois, &state)
		for _, move := range GenMoves(state.x, state.y, &walls) {
			if !state.returning {
				newVisited := state.visited
				if pois[move] && !state.visited[move] {
					newVisited = CopyVisited(&state.visited)
					newVisited[move] = true
				}
				key := fmt.Sprintf("0%d,%d|%s", move[0], move[1], SerializeVisited(newVisited))
				if prevSteps, ok := visitedStates[key]; ok && prevSteps <= state.steps+1 {
					continue // we've already reached here faster
				}
				visitedStates[key] = state.steps + 1
				totalCost := (2 * (state.steps + 1)) + TotalCost(move[0], move[1], &pois, &newVisited)
				heap.Push(&pq, RoboState{move[0], move[1], state.steps + 1, totalCost, newVisited, false})
			} else {
				if move == startPoint {
					return state.steps + 1
				}
				key := fmt.Sprintf("1%d,%d|%s", move[0], move[1], SerializeVisited(state.visited))
				if prevSteps, ok := visitedStates[key]; ok && prevSteps <= state.steps+1 {
					continue // we've already reached here faster
				}
				visitedStates[key] = state.steps + 1
				totalCost := (2 * (state.steps + 1)) + Manhattan(state.x, state.y, startPoint[0], startPoint[1])
				heap.Push(&pq, RoboState{move[0], move[1], state.steps + 1, totalCost, state.visited, false})
			}
		}
	}
	return -1
}

func main() {
	fmt.Print("Starting day 24\n")
	inputs := adv.GetInput("24", true, "\n", true)
	walls, pois, startPoint := ParseGrid(inputs)
	part1 := FindPath(walls, pois, startPoint)
	fmt.Printf("Part 1: %v\n", part1)
	part2 := FindPathAndReturn(walls, pois, startPoint)
	fmt.Printf("Part 1: %v\n", part2)
}
