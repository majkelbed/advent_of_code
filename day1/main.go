package day1

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover.
// On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

// For example:

// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

// Consider your entire calibration document. What is the sum of all of the calibration values?

// --- Part Two ---
// Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

// Equipped with this new information, you now need to find the real first and last digit on each line. For example:

// two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen
// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

const (
	ONE   = "one"
	TWO   = "two"
	THREE = "three"
	FOUR  = "four"
	FIVE  = "five"
	SIX   = "six"
	SEVEN = "seven"
	EIGHT = "eight"
	NINE  = "nine"
)

func is_digit(ch byte) bool {
	return ch >= '0' && ch <= '9'
}

func calibration_value(in string) int {
	input := strings.ReplaceAll(strings.ReplaceAll(strings.ReplaceAll(strings.ReplaceAll(strings.ReplaceAll(strings.ReplaceAll(strings.ReplaceAll(strings.ReplaceAll(strings.ReplaceAll(in, ONE, "1"), TWO, "2"), THREE, "3"), FOUR, "4"), FIVE, "5"), SIX, "6"), SEVEN, "7"), EIGHT, "8"), NINE, "9")
	sum := 0
	lines := strings.Split(input, "\n")

	for i := 0; i < len(lines); i++ {
		line := lines[i]
		line_len := len(line)

		if line_len == 0 {
			continue
		}

		var first, last byte

		for j := 0; j < line_len; j++ {
			ch := line[j]
			rch := line[line_len-j-1]

			if is_digit(ch) && first == 0 {
				first = ch
			}

			if is_digit(rch) && last == 0 {
				last = rch
			}

			if first != 0 && last != 0 {
				break
			}
		}

		num, err := strconv.Atoi(string(first) + string(last))

		if err != nil {
			panic(fmt.Errorf("Error parsing %b, %b, %w, %d", first, last, err, i))
		}

		sum += num
	}

	return sum
}

func Main() {
	input, err := os.ReadFile("day1/input.txt")

	if err != nil {
		panic(err)
	}

	fmt.Printf("Result is: %d", calibration_value(string(input)))
}
