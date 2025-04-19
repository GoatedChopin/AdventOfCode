package adv

func Abs(i int) int {
	if i < 0 {
		return -i
	}
	return i
}

func Factorial(n int) int {
	out := 1
	for i := n; i > 0; i-- {
		out *= i
	}
	return out
}
