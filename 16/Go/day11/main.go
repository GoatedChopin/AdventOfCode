package main

import (
	"container/heap"
	"fmt"
	"slices"
	"strconv"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

type State struct {
	building [][]RTG // Your state representation
	steps    int     // Steps taken so far (g(state))
	floor    int
	priority int // steps + Heuristic(building) (f(state))
	index    int // For heap implementation
}

type MinHeap []State

func (h MinHeap) Len() int {
	return len(h)
}

// Less returns true if the element with index i should sort before the element with index j.
func (h MinHeap) Less(i, j int) bool {
	return h[i].priority < h[j].priority
}

// Swap swaps the elements with indexes i and j.
func (h MinHeap) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
}

// Push pushes the element x onto the heap.
func (h *MinHeap) Push(x any) {
	*h = append(*h, x.(State))
}

// Pop removes and returns the minimum element (according to Less) from the heap.
func (h *MinHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

type RTG struct {
	id        string
	generator bool
	microchip bool
}

func (a RTG) Compare(b RTG) int {
	if a.id == b.id &&
		a.generator == b.generator &&
		a.microchip == b.microchip {
		return 0
	}
	if a.id < b.id {
		return -1
	} else {
		return 1
	}
}

func SafeFloor(floor []RTG) bool {
	generators := map[string]bool{}
	microchips := map[string]bool{}

	for _, r := range floor {
		if r.generator {
			generators[r.id] = true
		}
		if r.microchip {
			microchips[r.id] = true
		}
	}

	// If there are no generators, all chips are safe
	if len(generators) == 0 {
		return true
	}

	// Each unpaired microchip must not be exposed
	for id := range microchips {
		if !generators[id] {
			// Unpaired chip, and generators are present: unsafe!
			return false
		}
	}

	return true
}

type BuildingState struct {
	floor    int
	steps    int
	building [][]RTG
	cost     int
}

func (a BuildingState) Compare(b BuildingState) int {
	if a.cost < b.cost {
		return -1
	} else {
		return 1
	}
}

func Serialize(building [][]RTG, floor int) string {
	out := ""
	out += strconv.Itoa(floor)
	itemIndex := 0
	lookup := make(map[string]string)
	for f := range building {
		for rtg := range building[f] {
			if lookup[building[f][rtg].id] == "" {
				lookup[building[f][rtg].id] = strconv.Itoa(itemIndex)
				itemIndex++
			}
			out += lookup[building[f][rtg].id]
			if building[f][rtg].generator {
				out += "G"
			} else if building[f][rtg].microchip {
				out += "M"
			}
		}
		out += "\n"
	}
	return out
}

func Done(building [][]RTG) bool {
	top := len(building) - 1
	for i := 0; i < top; i++ {
		if len(building[i]) > 0 {
			return false
		}
	}
	return true
}

func WalkElevatorConstraint(floor int, steps int, building [][]RTG) int {
	pq := &MinHeap{}
	heap.Init(pq)
	initialState := State{
		building: building,
		steps:    0,
		floor:    0,
		priority: Heuristic(building),
	}
	heap.Push(pq, initialState)
	visited := make(map[string]int)
	visited[Serialize(building, floor)] = -1

	maxSteps := 0

	for pq.Len() > 0 {
		state := heap.Pop(pq).(State)
		// fmt.Println(state) // Use custom String() method
		floor = state.floor
		if state.steps > maxSteps {
			fmt.Printf("Searching steps %v\n", state.steps)
			maxSteps = state.steps
		}
		steps = state.steps
		building = state.building

		if Done(building) {
			return steps
		}

		// Move up
		if floor < len(building)-1 {
			for combo := range adv.FixedLengthCombinations(len(building[floor]), 2, true, 1) {
				if len(combo) == 0 {
					continue
				}
				armfull := make([]RTG, len(combo))
				for i, idx := range combo {
					armfull[i] = building[floor][idx]
				}
				currentFloor := make([]RTG, 0, len(building[floor])-len(armfull))
				for _, rtg := range building[floor] {
					if !slices.Contains(armfull, rtg) {
						currentFloor = append(currentFloor, rtg)
					}
				}
				nextFloor := slices.Concat(building[floor+1], armfull)
				if SafeFloor(currentFloor) && SafeFloor(nextFloor) {
					newBuilding := make([][]RTG, len(building))
					for i := range building {
						newBuilding[i] = make([]RTG, len(building[i]))
						copy(newBuilding[i], building[i])
					}
					newBuilding[floor] = currentFloor
					newBuilding[floor+1] = nextFloor
					stateKey := Serialize(newBuilding, floor+1)
					if visited[stateKey] == 0 || visited[stateKey] > steps+1 {
						visited[stateKey] = steps + 1
						heap.Push(pq, State{newBuilding, steps + 1, floor + 1, steps + Heuristic(newBuilding), 0})
					}
				}
			}
		}

		// Move down (now independent)
		elementsBelowUs := 0
		for f := range floor {
			for range building[f] {
				elementsBelowUs++
			}
		}
		if floor > 0 && elementsBelowUs > 0 {
			for combo := range adv.FixedLengthCombinations(len(building[floor]), 2, true, 1) {
				if len(combo) == 0 {
					continue
				}
				armfull := make([]RTG, len(combo))
				for i, idx := range combo {
					armfull[i] = building[floor][idx]
				}
				currentFloor := make([]RTG, 0, len(building[floor])-len(armfull))
				for _, rtg := range building[floor] {
					if !slices.Contains(armfull, rtg) {
						currentFloor = append(currentFloor, rtg)
					}
				}
				prevFloor := slices.Concat(building[floor-1], armfull)
				if SafeFloor(currentFloor) && SafeFloor(prevFloor) {
					newBuilding := make([][]RTG, len(building))
					for i := range building {
						newBuilding[i] = make([]RTG, len(building[i]))
						copy(newBuilding[i], building[i])
					}
					newBuilding[floor] = currentFloor
					newBuilding[floor-1] = prevFloor
					stateKey := Serialize(newBuilding, floor-1)
					if visited[stateKey] == 0 || visited[stateKey] > steps+1 {
						visited[stateKey] = steps + 1
						heap.Push(pq, State{newBuilding, steps + 1, floor - 1, steps + Heuristic(newBuilding), 0})
					}
				}
			}
		}
	}
	return -1
}

func Cost(b BuildingState) int {
	cost := 0
	for i, floor := range b.building {
		for range floor {
			cost += len(b.building) - i
		}
	}
	return cost
}

func Heuristic(b [][]RTG) int {
	topFloor := len(b) - 1
	count := 0
	for i := range topFloor {
		count += len(b[i])
	}
	return count
}

func (s BuildingState) String() string {
	header := "Steps " + strconv.Itoa(s.steps) + ":\n"
	var b strings.Builder
	for i := len(s.building) - 1; i >= 0; i-- {
		// Print floor number
		fmt.Fprintf(&b, "%d ", i+1)

		// Elevator position
		if s.floor == i {
			b.WriteString("E ")
		} else {
			b.WriteString("  ")
		}

		// Sort items by ID to keep consistent ordering
		floor := s.building[i]
		items := make([]string, 0, len(floor))
		for _, rtg := range floor {
			prefix := strings.ToUpper(rtg.id)
			if rtg.generator {
				items = append(items, prefix+"G")
			}
			if rtg.microchip {
				items = append(items, prefix+"M")
			}
		}
		slices.Sort(items)
		if len(items) > 0 {
			b.WriteString(strings.Join(items, ","))
		} else {
			b.WriteString("--")
		}
		b.WriteString("\n")
	}
	return header + b.String()
}

func Building(lines []string) [][]RTG {
	building := make([][]RTG, len(lines))
	for i, line := range lines {
		fmt.Printf("%v\n", line)
		if line == "--" {
			building[i] = []RTG{}
		} else {
			parts := strings.Split(line, ",")
			building[i] = make([]RTG, len(parts))
			for p, part := range parts {
				chars := strings.Split(part, "")
				building[i][p] = RTG{chars[0], chars[1] == "G", chars[1] == "M"}
			}
		}
	}
	fmt.Printf("Len of lines %d, building %d", len(lines), len(building))
	return building
}

func main() {
	fmt.Printf("Starting day 11\n")
	inputs := adv.GetInput("11.2", true, "\n", true)
	building1 := Building(inputs)
	part1 := WalkElevatorConstraint(0, 0, building1)
	fmt.Printf("%v\n", part1)
}
