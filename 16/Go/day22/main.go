package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

type IpRange struct {
	lower int
	upper int
}

type IpRangeArray []IpRange

func (u IpRangeArray) Len() int {
	return len(u)
}

func (u IpRangeArray) Swap(i, j int) {
	u[i], u[j] = u[j], u[i]
}

func (u IpRangeArray) Less(i, j int) bool {
	return u[i].lower < u[j].lower
}

func ParseRange(s string) IpRange {
	parts := strings.Split(s, "-")
	lower, err := strconv.Atoi(parts[0])
	if err != nil {
		panic("Bad lower")
	}
	upper, err := strconv.Atoi(parts[1])
	if err != nil {
		panic("Bad upper")
	}
	return IpRange{lower, upper}
}

func LowestAllowedIp(lines []string) int {
	ipRanges := make(IpRangeArray, len(lines))
	for i, s := range lines {
		ipRanges[i] = ParseRange(s)
	}
	sort.Sort(ipRanges)
	i := 0
	r := 0
	for {
		if r < len(ipRanges) && (ipRanges[r].lower <= i && i <= ipRanges[r].upper) {
			i = ipRanges[r].upper + 1
		} else {
			if r < len(ipRanges)-1 && !(i >= ipRanges[r+1].lower && i <= ipRanges[r+1].upper) {
				return i
			}
		}
		for r < len(ipRanges) && i > ipRanges[r].upper {
			r++
		}
	}
}

func AllAllowedIps(lines []string) int {
	ipRanges := make(IpRangeArray, len(lines))
	for i, s := range lines {
		ipRanges[i] = ParseRange(s)
	}
	sort.Sort(ipRanges)

	var mergedRanges []IpRange
	current := ipRanges[0]

	for i := 1; i < len(ipRanges); i++ {
		next := ipRanges[i]
		if next.lower <= current.upper+1 {
			// Merge ranges
			if next.upper > current.upper {
				current.upper = next.upper
			}
		} else {
			// No overlap, push current and move on
			mergedRanges = append(mergedRanges, current)
			current = next
		}
	}
	mergedRanges = append(mergedRanges, current)

	// Count allowed IPs
	allowed := 0
	prevUpper := -1
	for _, r := range mergedRanges {
		if r.lower > prevUpper+1 {
			allowed += r.lower - (prevUpper + 1)
		}
		if r.upper > prevUpper {
			prevUpper = r.upper
		}
	}
	// Final range to 2^32-1
	const maxIp = 4294967295
	if prevUpper < maxIp {
		allowed += maxIp - prevUpper
	}

	return allowed
}

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

func MinMoves(nodes []Node) int {
	// Implement BFS or A* to find shortest path moving df entries to adjacent nodes.
	// Probably need to put this in a grid or a hashmap so we can do fast lookups based on x and y

	return 0
}

func main() {
	fmt.Print("Starting day 12\n")
	inputs := adv.GetInput("22", true, "\n", true)
	nodes := ParseNodes(inputs)
	part1 := ViablePairs(nodes)
	fmt.Printf("Part 1: %v\n", part1)
}
