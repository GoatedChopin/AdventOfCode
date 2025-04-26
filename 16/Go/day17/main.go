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
	label byte
}

type Path struct {
	path     []byte // just the directions, not the full passcode
	position Vec
}

func inBounds(v Vec) bool {
	return 0 <= v.x && v.x <= 3 && 0 <= v.y && v.y <= 3
}

var unlockChars = [256]bool{
	'b': true, 'c': true, 'd': true, 'e': true, 'f': true,
}

var dirs = [4]Vec{{0, -1, 'U'}, {0, 1, 'D'}, {-1, 0, 'L'}, {1, 0, 'R'}}

func Unlocked(passcode string, path []byte, position Vec) []Vec {
	buf := make([]byte, len(passcode)+len(path))
	copy(buf, passcode)
	copy(buf[len(passcode):], path)
	sum := md5.Sum(buf)
	hash := hex.EncodeToString(sum[:])

	var out []Vec
	for i := 0; i < 4; i++ {
		if unlockChars[hash[i]] {
			newPos := Vec{position.x + dirs[i].x, position.y + dirs[i].y, dirs[i].label}
			if inBounds(newPos) {
				out = append(out, dirs[i])
			}
		}
	}
	return out
}

func LongestPathfind(passcode string) int {
	max := 0
	queue := []Path{{[]byte{}, Vec{0, 0, 0}}}

	for len(queue) > 0 {
		path := queue[0]
		queue = queue[1:]

		if path.position.x == 3 && path.position.y == 3 {
			if newMax := len(path.path); newMax > max {
				max = newMax
			}
			continue
		}

		for _, dir := range Unlocked(passcode, path.path, path.position) {
			newPath := make([]byte, len(path.path)+1)
			copy(newPath, path.path)
			newPath[len(path.path)] = dir.label
			queue = append(queue, Path{newPath, Vec{path.position.x + dir.x, path.position.y + dir.y, dir.label}})
		}
	}
	return max
}

func Pathfind(s string) string {
	queue := list.New()
	queue.PushBack(Path{[]byte(s), Vec{0, 0, 0}})
	for queue.Len() > 0 {
		front := queue.Front()
		path := queue.Remove(front).(Path)
		if path.position.x == 3 && path.position.y == 3 {
			return string(path.path[len(s):])
		}
		for _, dir := range Unlocked(s, path.path, path.position) {
			queue.PushBack(Path{path.path, dir})
		}
	}
	return ""
}

func main() {
	fmt.Print("Starting day 17\n")
	input := "udskfozm"
	part1 := LongestPathfind(input)
	fmt.Printf("Part 1: %v\n", part1)
}
