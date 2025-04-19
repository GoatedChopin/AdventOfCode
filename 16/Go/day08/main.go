package main

import (
	"fmt"
	"strconv"
	"strings"

	adv "github.com/GoatedChopin/AdventOfCode/16/Go/util"
)

type vector2 struct {
	x int
	y int
}

func Move(v vector2, d vector2) vector2 {
	return vector2{v.x + d.x, v.y + d.y}
}

func PrintScreen(screen [][]bool) {
	for r, row := range screen {
		for c := range row {
			if screen[r][c] {
				fmt.Print("1")
			} else {
				fmt.Print(" ")
			}
		}
		fmt.Print("\n")
	}
	fmt.Print("\n")
}

func Clamp(v vector2, width int, height int) vector2 {
	for v.x < 0 {
		v.x += height
	}
	for v.x >= height {
		v.x -= height
	}
	for v.y < 0 {
		v.y += width
	}
	for v.y >= width {
		v.y -= width
	}
	return v
}

func NewScreen(width int, height int) [][]bool {
	screen := make([][]bool, height)
	for i := range height {
		screen[i] = make([]bool, width)
	}
	return screen
}

func Rect(a int, b int, s [][]bool) {
	for ia := range a {
		for ib := range b {
			s[ib][ia] = true
		}
	}
}

func RotateRow(row int, by int, s [][]bool) {
	nums := s[row]
	if by < 0 || len(nums) == 0 {
		return
	}
	r := len(nums) - by%len(nums)
	nums = append(nums[r:], nums[:r]...)
	copy(s[row], nums)
}

func RotateCol(col int, by int, s [][]bool) {
	if by < 0 || len(s) == 0 || len(s[0]) == 0 {
		return
	}
	rows := len(s)
	nums := make([]bool, rows)
	for i := range nums {
		nums[i] = s[i][col]
	}
	r := len(nums) - by%len(nums)
	nums = append(nums[r:], nums[:r]...)
	for i := range nums {
		s[i][col] = nums[i]
	}
}

func ParseInstruction(line string) (string, int, int) {
	chunks := strings.Split(line, " ")
	if len(chunks) == 2 {
		dimensions := strings.Split(chunks[1], "x")
		x, err := strconv.Atoi(dimensions[0])
		if err != nil {
			panic("Bad X")
		}
		y, err := strconv.Atoi(dimensions[1])
		if err != nil {
			panic("Bad Y")
		}
		return "rect", x, y
	} else {
		dir := chunks[1]
		by, err := strconv.Atoi(chunks[4])
		if err != nil {
			panic("Bad by value")
		}
		place, err := strconv.Atoi(strings.Split(chunks[2], "=")[1])
		if err != nil {
			panic("Bad place value")
		}
		return dir, place, by
	}
}

func FollowInstructions(lines []string) int {
	screen := NewScreen(50, 6)
	for _, line := range lines {
		instruction, a, b := ParseInstruction(line)
		if instruction == "rect" {
			Rect(a, b, screen)
		} else if instruction == "column" {
			RotateCol(a, b, screen)
		} else if instruction == "row" {
			RotateRow(a, b, screen)
		} else {
			fmt.Printf("%v\n", line)
			panic("Unknown instruction")
		}
		PrintScreen(screen)
	}
	lit := 0
	for r, row := range screen {
		for c := range row {
			if screen[r][c] {
				lit++
			}
		}
	}
	return lit
}

func main() {
	fmt.Printf("Starting day 8\n")
	inputs := adv.GetInput("8", true, "\n", true)
	part1 := FollowInstructions(inputs)
	fmt.Printf("%v\n", part1)
}
