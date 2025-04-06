package main

import (
	"fmt"
	"math"
	"strconv"

	"github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

var CardinalDirections = map[int][]int{
	0:   {0, 1},  // North
	90:  {1, 0},  // East
	180: {0, -1}, // South
	270: {-1, 0}, // West
}

func Turn(start int, degrees int) int {
	start += degrees
	for start < 0 {
		start += 360
	}
	for start > 270 {
		start -= 360
	}
	return start
}

func TaxiCabDistance(x int, y int, xp int, yp int) int {
	return int(math.Abs(float64(x-xp))) + int(math.Abs(float64(y-yp)))
}

func Walk(input []string, part uint) int {
	visited := map[[2]int]bool{
		{0, 0}: true,
	}
	dir := 0
	position := []int{0, 0}
	for i := range input {
		turn := string(input[i][0])
		if turn == "R" {
			dir = Turn(dir, 90)
		} else if turn == "L" {
			dir = Turn(dir, -90)
		} else {
			panic("Unhandled turn")
		}
		vector := CardinalDirections[dir]
		magnitude, err := strconv.Atoi(input[i][1:])
		if err != nil {
			panic(err)
		}
		fmt.Printf("%v + %v (%v -> %v)", position, vector, input[i], magnitude)
		for step := 0; step < magnitude; step++ {
			for pi, dx := range vector {
				position[pi] += dx
			}
			if visited[[2]int{position[0], position[1]}] && part == 2 {
				return TaxiCabDistance(0, 0, position[0], position[1])
			}
			visited[[2]int{position[0], position[1]}] = true
		}
		fmt.Printf(" = %v\t%v\n", position, dir)
	}
	fmt.Printf("%v, %v\n", position, TaxiCabDistance(0, 0, position[0], position[1]))
	return TaxiCabDistance(0, 0, position[0], position[1])
}

func main() {
	input := adv.GetInput("1", true, ",", true)
	answer := Walk(input, 1)
	fmt.Printf("Answer: %v\n\n", answer)
	answerTwo := Walk(input, 2)
	fmt.Printf("Answer 2: %v\n", answerTwo)
}
