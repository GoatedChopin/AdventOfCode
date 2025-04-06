package adv

import (
	"os"
	"strings"
)

func ReadFile(path string) string {
	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	content := string(data)
	return content
}

func SplitOn(s string, splitter string) []string {
	return strings.Split(s, splitter)
}

func Strip(s string) string {
	return strings.TrimSpace(s)
}

func GetInput(dayNum string, split bool, splitter string, stripLines bool) []string {
	filePath := ".." + string(os.PathSeparator) + "inputs" + string(os.PathSeparator) + dayNum + ".txt"
	fileString := ReadFile(filePath)
	if split {
		out := SplitOn(fileString, splitter)
		if stripLines {
			for i, line := range out {
				out[i] = Strip(line)
			}
		}
		return out
	}
	basic := []string{fileString}
	return basic
}
