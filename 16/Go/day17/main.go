package main

import (
	"container/list"
	"crypto/md5"
	"encoding/hex"
	"fmt"
)

type Vec struct {
	x     int
	y     int
	label rune
}

func move(v Vec, d Vec) Vec {
	return Vec{v.x + d.x, v.y + d.y, d.label}
}

func inBounds(v Vec) bool {
	if 0 <= v.x && v.x < 4 && 0 <= v.y && v.y < 4 {
		return true
	}
	return false
}

type Path struct {
	fullPath string // passcode + directions
	position Vec
}

func Unlocked(fullPath string, position Vec) []Vec {
	sum := md5.Sum([]byte(fullPath))
	hash := hex.EncodeToString(sum[:])
	unlockChars := map[byte]bool{'b': true, 'c': true, 'd': true, 'e': true, 'f': true}
	dirs := []Vec{{0, -1, 'U'}, {0, 1, 'D'}, {-1, 0, 'L'}, {1, 0, 'R'}}
	var out []Vec
	for i := 0; i < 4; i++ {
		if unlockChars[hash[i]] {
			newPos := Vec{position.x + dirs[i].x, position.y + dirs[i].y, dirs[i].label}
			if 0 <= newPos.x && newPos.x < 4 && 0 <= newPos.y && newPos.y < 4 {
				out = append(out, dirs[i])
			}
		}
	}
	return out
}

func Pathfind(s string) string {
	queue := list.New()
	queue.PushBack(Path{s, Vec{0, 0, ' '}})
	for queue.Len() > 0 {
		front := queue.Front()
		path := queue.Remove(front).(Path)
		if path.position.x == 3 && path.position.y == 3 {
			return path.fullPath[len(s):]
		}
		for _, dir := range Unlocked(path.fullPath, path.position) {
			queue.PushBack(Path{path.fullPath + string(dir.label), Vec{path.position.x + dir.x, path.position.y + dir.y, dir.label}})
		}
	}
	return ""
}

func LongestPathfind(s string) int {
	max := 0
	queue := list.New()
	queue.PushBack(Path{s, Vec{0, 0, ' '}})
	for queue.Len() > 0 {
		front := queue.Front()
		path := queue.Remove(front).(Path)
		if path.position.x == 3 && path.position.y == 3 {
			if newMax := len(path.fullPath[len(s):]); newMax > max {
				fmt.Printf("New longest: %v\n", newMax)
				max = newMax
				continue
			}
		}
		for _, dir := range Unlocked(path.fullPath, path.position) {
			queue.PushBack(Path{path.fullPath + string(dir.label), Vec{path.position.x + dir.x, path.position.y + dir.y, dir.label}})
		}
	}
	return max
}

func main() {
	fmt.Print("Starting day 15\n")
	input := "udskfozm"
	part1 := LongestPathfind(input)
	fmt.Printf("Part 1: %v", part1)
}
