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
	if u[i].lower < u[j].lower {
		return true
	}
	if u[i].upper < u[j].upper {
		return true
	}
	return false
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
	max := 4294967295
	ipRanges := make(IpRangeArray, len(lines))
	mergedRanges := make(IpRangeArray, 1)
	for i, s := range lines {
		ipRanges[i] = ParseRange(s)
	}
	mergedRanges[0] = ipRanges[0]
	sort.Sort(ipRanges)
	m := 0
	i, j := 0, 1
	for j < len(ipRanges) {
		for j < len(ipRanges) && ipRanges[j].lower-ipRanges[i].upper <= 1 {
			mergedRanges[m] = IpRange{mergedRanges[m].lower, ipRanges[j].upper}
			j++
		}
		if ipRanges[j].lower > ipRanges[i].lower+1 {
			mergedRanges = append(mergedRanges, ipRanges[j])
			m++
		}
		i = j
		j++
	}
	for _, r := range mergedRanges {
		max -= r.upper - r.lower
	}
	return max
}

func main() {
	fmt.Print("Starting day 15\n")
	inputs := adv.GetInput("20", true, "\n", true)
	part1 := LowestAllowedIp(inputs)
	fmt.Printf("Part 1: %v\n", part1)
	part2 := AllAllowedIps(inputs)
	fmt.Printf("Part 2: %v\n", part2)
}
