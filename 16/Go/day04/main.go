package main

import (
	"container/heap"
	"fmt"
	"strconv"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

type Datum struct {
	encryptedName string
	sectorId      uint
	checksum      string
}

func (d *Datum) String() string {
	return d.encryptedName + "-" + fmt.Sprint(d.sectorId) + "[" + d.checksum + "]"
}

func NewDatum(s string) Datum {
	// split string on [
	parts := strings.Split(s, "[")
	hash := strings.Replace(parts[len(parts)-1], "]", "", 1)
	// split left part on -
	var scale uint = 1
	var sectorId uint = 0
	encryptedName := ""
	leftSide := strings.Split(parts[0], "")
	for i := len(parts[0]) - 1; i > 0; i-- {
		if leftSide[i] == "-" {
			encryptedName = parts[0][:i]
			break
		}
		parsedInt, err := strconv.Atoi(leftSide[i])
		sectorId += uint(parsedInt) * scale
		if err != nil {
			panic("BAD UINT")
		}
		scale *= 10
	}
	// take rightside as the sectorId
	// take leftside as the encryptedName
	return Datum{encryptedName, sectorId, hash}
}

type CharCount struct {
	char  rune
	count int
}

func (c *CharCount) String() string {
	return fmt.Sprintf("%v: %v", c.char, c.count)
}

type CharHeap []CharCount

func (h CharHeap) Len() int { return len(h) }
func (h CharHeap) Less(i, j int) bool {
	if h[i].count == h[j].count {
		return h[i].char < h[j].char // Alphabetical order if counts are equal
	}
	return h[i].count > h[j].count // Higher count first
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

func TopK(s string, k int) map[rune]int {
	out := make(map[rune]int)
	for _, c := range s {
		out[c]++
	}
	return out
}

func IsDummy(s string, h string) bool {
	freq := make(map[rune]int)
	for _, c := range strings.ReplaceAll(s, "-", "") {
		freq[c]++
	}
	rank := &CharHeap{}
	for char, count := range freq {
		heap.Push(rank, CharCount{char, count})
	}
	// var lastRune rune
	top5 := make([]CharCount, 5)
	for i := 0; rank.Len() > 0; i++ {
		r := heap.Pop(rank).(CharCount)
		if i >= 5 {
			break
		}
		top5[i] = r
		// lastRune = r.char
	}
	// fmt.Printf("%v -> %v\n%v\n", top5, h, s)
	for i, c := range h {
		// fmt.Printf("Checking %v\n", c)
		if top5[i].char != c {
			// fmt.Printf("%v was not in the top 5\n\n", c)
			return true
		}
	}
	// fmt.Print("\n")
	return false
}

func Rotate(s string, i int) string {
	// startCap := 65
	// startLow := 97
	out := ""
	i = i % (26)
	for _, c := range s {
		rot := c + rune(i)
		if c <= 122 && rot > 122 {
			rot -= 26
		} else if c <= 90 && rot > 90 {
			rot -= 26
		} else if c == 32 {
			rot = 45
		} else if c == 45 {
			rot = 32
		}
		// fmt.Printf("%v -> %v\n", string(c), string(rot))
		out += string(rot)
	}
	return out
}

func NotDummies(lines []string) uint {
	var sum uint = 0
	for _, line := range lines {
		datum := NewDatum(line)
		if IsDummy(datum.encryptedName, datum.checksum) {
			continue
		}
		// fmt.Printf("%v -> %v\n\n", datum, datum.sectorId)
		sum += datum.sectorId
	}
	return sum
}

func Decrypt(lines []string) uint {
	for _, line := range lines {
		datum := NewDatum(line)
		if IsDummy(datum.encryptedName, datum.checksum) {
			continue
		}
		decryptedString := Rotate(datum.encryptedName, int(datum.sectorId))
		fmt.Printf("%v\t%v\n", decryptedString, datum.sectorId)
	}
	return 0
}

func main() {
	input := adv.GetInput("4", true, "\n", true)
	answer := NotDummies(input)
	fmt.Printf("Answer 1: %v\n\n", answer)
	answerTwo := Decrypt(input)
	fmt.Printf("Answer 2: %v\n", answerTwo)
}
