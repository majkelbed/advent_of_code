package day1

import "testing"

func Test1(t *testing.T) {
	result := calibration_value("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen")

	if result != 281 {
		t.Errorf("Failed to calculate proper result, %d", result)
	}
}
