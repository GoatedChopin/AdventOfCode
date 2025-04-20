package main

import (
	"container/list"
	"fmt"
	"strconv"
)

func parity(row, col, favNumber int) bool {
	calc := row*row + 3*row + 2*row*col + col + col*col
	sum := calc + favNumber
	binary := strconv.FormatInt(int64(sum), 2)
	ones := 0
	for _, c := range binary {
		if c == '1' {
			ones++
		}
	}
	return ones%2 == 0
}

func isWall(row, col, favNumber int) bool {
	return !parity(row, col, favNumber)
}

func CreateMaze(rows, cols, favNumber int) [][]bool {
	maze := make([][]bool, rows)
	for r := range rows {
		maze[r] = make([]bool, cols)
		for c := range cols {
			maze[r][c] = isWall(r, c, favNumber)
		}
	}
	return maze
}

func VisualizeMaze(grid [][]bool) {
	if len(grid) == 0 || len(grid[0]) == 0 {
		fmt.Println("Empty grid")
		return
	}

	rows := len(grid)
	cols := len(grid[0])

	// Print column headers
	fmt.Print("   ")
	for col := 0; col < cols; col++ {
		fmt.Printf("%d", col%10) // Wraps at 10 for wider grids
	}
	fmt.Println()

	// Print each row with row number
	for row := 0; row < rows; row++ {
		fmt.Printf("%2d ", row)
		for col := 0; col < cols; col++ {
			if grid[row][col] {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}

func EncodePosition(row, col int) string {
	return strconv.Itoa(row) + "," + strconv.Itoa(col)
}

type Position struct {
	row   int
	col   int
	steps int
}

func ValidMoves(position Position, visited *map[string]bool, maze *[][]bool) []Position {
	out := make([]Position, 0)
	for _, rd := range []int{-1, 0, 1} {
		for _, cd := range []int{-1, 0, 1} {
			if rd+cd == 1 || rd+cd == -1 {
				newRow, newCol := position.row+rd, position.col+cd
				if newRow < 0 || newCol < 0 {
					continue
				}
				newPosition := Position{newRow, newCol, position.steps + 1}
				_, been := (*visited)[EncodePosition(newRow, newCol)]
				if !been && !(*maze)[newRow][newCol] {
					out = append(out, newPosition)
				}
			}
		}
	}
	return out
}

func BFS(sr, sc, er, ec int, maze [][]bool) int {
	visited := make(map[string]bool)
	visited[EncodePosition(sr, sc)] = true
	queue := list.New()
	queue.PushBack(Position{sr, sc, 0})
	for queue.Len() > 0 {
		front := queue.Front()
		position := queue.Remove(front).(Position)
		if position.row == er && position.col == ec {
			return position.steps
		}
		validMoves := ValidMoves(position, &visited, &maze)
		for _, move := range validMoves {
			visited[EncodePosition(move.row, move.col)] = true
			queue.PushBack(move)
		}
	}
	return -1
}

func MaxSteps(sr, sc, steps int, maze [][]bool) int {
	visited := make(map[string]bool)
	visited[EncodePosition(sr, sc)] = true
	queue := list.New()
	queue.PushBack(Position{sr, sc, 0})
	for queue.Len() > 0 {
		front := queue.Front()
		position := queue.Remove(front).(Position)
		if position.steps == steps {
			continue
		}
		validMoves := ValidMoves(position, &visited, &maze)
		for _, move := range validMoves {
			visited[EncodePosition(move.row, move.col)] = true
			queue.PushBack(move)
		}
	}
	return len(visited)
}

func ShortestPath(sr, sc, er, ec, favNumber int) int {
	maxRow, maxCol := 0, 0
	if er > sr {
		maxRow = er
	} else {
		maxRow = sr
	}
	maxRow *= 2
	if ec > sc {
		maxCol = er
	} else {
		maxCol = sr
	}
	maxCol *= 2
	maze := CreateMaze(maxRow, maxCol, favNumber)
	return BFS(sr, sc, er, ec, maze)
}

func MaxLocations(sr, sc, steps, favNumber int) int {
	maze := CreateMaze(sr+(steps*2), sc+(steps*2), favNumber)
	return MaxSteps(sr, sc, steps, maze)
}

func main() {
	fmt.Print("Starting day 12\n")
	// 1350
	part1 := ShortestPath(1, 1, 31, 39, 1350)
	fmt.Printf("Part 1: %v\n", part1)
	part2 := MaxLocations(1, 1, 50, 1350)
	fmt.Printf("Part 2: %v\n", part2)
}
