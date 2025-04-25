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

func main() {
	fmt.Print("Starting day 15\n")
	inputs := adv.GetInput("20", true, "\n", true)
	part1 := LowestAllowedIp(inputs)
	fmt.Printf("Part 1: %v\n", part1)
	part2 := AllAllowedIps(inputs)
	fmt.Printf("Part 2: %v\n", part2)
}
