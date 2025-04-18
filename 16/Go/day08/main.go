package main

import "fmt"

type vector2 struct {
	x int
	y int
}

func Move(v vector2, d vector2) vector2 {
	return vector2{v.x + d.x, v.y + d.y}
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

func FollowInstructions(lines []string) int {
	return 0
}

func main() {
	fmt.Printf("Starting day 8")

}
