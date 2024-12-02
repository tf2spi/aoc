package aoc2024
import "core:sort"
import "core:fmt"
import "core:strings"
import "core:strconv"
import "core:os"

SAMPLE1 :: `
3   4
4   3
2   5
1   3
3   9
3   3
`

SAMPLE2 :: `
3   4
4   3
2   5
1   3
3   9
3   3
`

day1 :: proc(part: int) -> bool {
	data := transmute(string)(os.read_entire_file("day1.txt") or_return)
	line1 := make([dynamic]int)
	line2 := make([dynamic]int)
	defer {
		delete(data)
		delete(line1)
		delete(line2)
	}
	data_iterator := data
	for {
		line, ok := strings.split_lines_iterator(&data_iterator) 
		if !ok { break }
		if len(line) == 0 { continue }
		fields := strings.split(line, "   ", context.temp_allocator)

		n1, n2 := strconv.atoi(fields[0]), strconv.atoi(fields[1])
		append(&line1, n1)
		append(&line2, n2)
	}
	sort.quick_sort(line1[:])
	sort.quick_sort(line2[:])

	if line1[0] < line2[0] {
		line1, line2 = line2, line1
	}
	if part == 1 {
		distance := 0
		for i in 0..<len(line1) {
			n1, n2 := line1[i], line2[i]
			distance += abs(n2 - n1)
		}
		fmt.println("Hello ", distance)
	} else {
		similarity := 0
		i := 0
		for i < len(line1) {
			j := 0
			n := line1[i]
			for j < len(line2) {
				if line2[j] == n { similarity += n }
				j += 1
			}
			i += 1
		}
		fmt.println("Hello ", similarity)
	}
	return true
}
