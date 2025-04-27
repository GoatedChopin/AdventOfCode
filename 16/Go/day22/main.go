package main

import (
	"container/list"
	"fmt"
	"strconv"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

type Node struct {
	x    int
	y    int
	size int
	used int
}

func (n *Node) Avail() int {
	return n.size - n.used
}

func (n *Node) Empty() bool {
	return n.used == 0
}

func (a *Node) Less(b *Node) bool {
	if a.y < b.y {
		return true
	}
	if a.y == b.y && a.x < b.x {
		return true
	}
	return false
}

func (a *Node) Adjacent(b *Node) bool {
	xdiff := a.x - b.x
	if xdiff < 0 {
		xdiff = -xdiff
	}
	ydiff := a.y - b.y
	if ydiff < 0 {
		ydiff = -ydiff
	}
	if xdiff+ydiff == 1 {
		return true
	}
	return false
}

func ParseNodes(lines []string) []Node {
	nodes := make([]Node, len(lines)-2)
	for _, line := range lines[2:] {
		parts := strings.Split(line, " ")
		parts[0] = strings.ReplaceAll(parts[0], "x", "")
		parts[0] = strings.ReplaceAll(parts[0], "y", "")
		parts[1] = strings.ReplaceAll(parts[1], "T", "")
		parts[2] = strings.ReplaceAll(parts[2], "T", "")

		xyparts := strings.Split(parts[0], "-")
		x, err := strconv.Atoi(xyparts[1])
		if err != nil {
			panic("bad x val")
		}
		y, err := strconv.Atoi(xyparts[2])
		if err != nil {
			panic("bad y val")
		}

		size, err := strconv.Atoi(parts[1])
		if err != nil {
			panic("bad size val")
		}
		used, err := strconv.Atoi(parts[2])
		if err != nil {
			panic("bad used val")
		}
		nodes = append(nodes, Node{x, y, size, used})
	}
	return nodes
}

func ViablePairs(nodes []Node) int {
	pairs := 0
	for c := range adv.FixedLengthCombinations(len(nodes), 2, false, 2) {
		a, b := c[0], c[1]
		if a == b {
			continue
		}
		if !nodes[b].Empty() && nodes[a].Avail() > nodes[b].used {
			pairs++
		} else if !nodes[a].Empty() && nodes[b].Avail() > nodes[a].used {
			pairs++
		}
	}
	return pairs
}

func GenViablePairs(nodes []Node) <-chan []int {
	ch := make(chan []int)
	go func(ch chan []int) {
		defer close(ch)
		for c := range adv.FixedLengthCombinations(len(nodes), 2, false, 2) {
			a, b := c[0], c[1]
			if a == b {
				continue
			}
			if !nodes[a].Adjacent(&nodes[b]) {
				continue
			}
			if !nodes[b].Empty() && nodes[a].Avail() > nodes[b].used {
				ch <- c
			} else if !nodes[a].Empty() && nodes[b].Avail() > nodes[a].used {
				ch <- c
			}
		}
	}(ch)
	return ch
}

type State struct {
	ogX, ogY, steps int
	nodes           []Node
}

func (s *State) Hash() string {
	var b strings.Builder
	for _, n := range s.nodes {
		b.WriteString(fmt.Sprintf("|%d,%d,%d", n.x, n.y, n.used))
	}
	return b.String()
}

func MinMoves(nodes []Node) int {
	targetX, targetY := 0, 0
	for _, n := range nodes {
		if n.x > targetX {
			targetX = n.x
		}
	}
	visited := make(map[string]bool)
	queue := list.New()
	queue.PushBack(State{targetX, targetY, 0, nodes})
	for queue.Len() > 0 {
		state := queue.Remove(queue.Front()).(State)
		if state.ogX == 0 && state.ogY == 0 {
			return state.steps
		}
		for pair := range GenViablePairs(state.nodes) {
			a, b := state.nodes[pair[0]], state.nodes[pair[1]]
			ogX, ogY := state.ogX, state.ogY
			newNodes := make([]Node, len(nodes))
			copy(newNodes, state.nodes)
			if a.Avail() >= b.used {
				newNodes[pair[0]] = Node{a.x, a.y, a.size, a.used + b.used}
				newNodes[pair[1]] = Node{b.x, b.y, b.size, 0}
				if b.x == ogX && b.y == ogY {
					ogX, ogY = a.x, a.y
				}
			} else {
				newNodes[pair[1]] = Node{b.x, b.y, b.size, a.used + b.used}
				newNodes[pair[0]] = Node{a.x, a.y, a.size, 0}
				if a.x == ogX && a.y == ogY {
					ogX, ogY = b.x, b.y
				}
			}
			newState := State{ogX, ogY, state.steps + 1, newNodes}
			h := newState.Hash()
			if !visited[h] {
				visited[h] = true
			} else {
				continue
			}
			queue.PushBack(newState)
		}
	}
	return -1
}

func main() {
	fmt.Print("Starting day 12\n")
	inputs := adv.GetInput("22", true, "\n", true)
	nodes := ParseNodes(inputs)
	part1 := ViablePairs(nodes)
	fmt.Printf("Part 1: %v\n", part1)
	part2 := MinMoves(nodes)
	fmt.Printf("Part 2: %v\n", part2)
}
