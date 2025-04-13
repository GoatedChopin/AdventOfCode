package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

func Triangles(input []string) int {
	valid := 0
	for _, line := range input {
		sides := []int{0, 0, 0}
		for i, numString := range strings.Split(strings.ReplaceAll(line, "  ", " "), " ") {
			num, err := strconv.Atoi(strings.TrimSpace(numString))
			if err != nil {
				fmt.Printf("Failed on %v -> %v", line, numString)
				panic("BAD INT")
			}
			sides[i] = num
		}
		sort.Ints(sides)
		if sides[0]+sides[1] > sides[2] {
			valid++
		}
	}
	return valid
}

func VertTriangles(input []string) int {
	valid := 0
	triangles := [][]int{
		{0, 0, 0},
		{0, 0, 0},
		{0, 0, 0},
	}
	for li, line := range input {
		for i, numString := range strings.Split(strings.ReplaceAll(line, "  ", " "), " ") {
			num, err := strconv.Atoi(strings.TrimSpace(numString))
			if err != nil {
				fmt.Printf("Failed on %v -> %v", line, numString)
				panic("BAD INT")
			}
			fmt.Printf("%v\n\n", triangles)
			triangles[i][li%3] = num
		}
		if li%3 == 2 {
			for tri := range 3 {
				sort.Ints(triangles[tri])
				fmt.Printf("%v\n\n\n", triangles[tri])
				if triangles[tri][0]+triangles[tri][1] > triangles[tri][2] {
					valid++
				}
			}
		}
	}
	return valid
}

func main() {
	input := adv.GetInput("3", true, "\n", true)
	answer := Triangles(input)
	fmt.Printf("Answer 1: %v\n\n", answer)
	answerTwo := VertTriangles(input)
	fmt.Printf("Answer 2: %v\n", answerTwo)
}
