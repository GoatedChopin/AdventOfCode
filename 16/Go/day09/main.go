package main

import (
	"fmt"
	"strconv"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

func Decompress(s string) string {
	d := ""
	duping := false
	dupBuff := ""
	skipTo := 0
	for i, c := range strings.Split(s, "") {
		if skipTo > i {
			continue
		}
		if c == "(" {
			dupBuff = ""
			duping = true
		} else if duping && c != ")" {
			dupBuff += c
		} else if duping && c == ")" {
			dims := strings.Split(dupBuff, "x")
			numChars, err := strconv.Atoi(dims[0])
			if err != nil {
				panic("Bad numChars")
			}
			numRepeats, err := strconv.Atoi(dims[1])
			if err != nil {
				panic("Bad numRepeats")
			}
			for range numRepeats {
				d += s[i+1 : i+numChars+1]
			}
			skipTo = i + numChars + 1
			duping = false
		} else {
			d += c
		}
	}
	return d
}

func DecompressV2(s string) int {
	d := 0
	duping := false
	dupBuff := ""
	skipTo := 0
	for i, c := range strings.Split(s, "") {
		if skipTo > i {
			continue
		}
		if c == "(" {
			dupBuff = ""
			duping = true
		} else if duping && c != ")" {
			dupBuff += c
		} else if duping && c == ")" {
			dims := strings.Split(dupBuff, "x")
			numChars, err := strconv.Atoi(dims[0])
			if err != nil {
				panic("Bad numChars")
			}
			numRepeats, err := strconv.Atoi(dims[1])
			if err != nil {
				panic("Bad numRepeats")
			}
			for range numRepeats {
				d += DecompressV2(s[i+1 : i+numChars+1])
			}
			skipTo = i + numChars + 1
			duping = false
		} else {
			d++
		}
	}
	return d
}

func main() {
	fmt.Printf("Starting day 9\n")
	input := adv.GetInput("9", false, "", true)
	part1 := Decompress(input[0])
	fmt.Printf("%v\n", len(part1))
	part2 := DecompressV2(input[0])
	fmt.Printf("%v\n", part2)
}
