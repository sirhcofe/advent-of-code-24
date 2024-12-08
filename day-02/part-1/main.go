package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	safe_reports := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		split := strings.Split(scanner.Text(), " ")
		numbers := convertToNumArray(split)
		safe_reports += solve(numbers)
	}

	fmt.Println("Total safe reports: ", safe_reports)
}

func convertToNumArray(split []string) []int {
	var numberSlice []int
	for _, str := range split {
		number, err := strconv.Atoi(str)
		if err != nil {
			fmt.Println("Error converting string to float:", err)
			os.Exit(1)
		}
		numberSlice = append(numberSlice, number)
	}
	return numberSlice
}

func solve(numbers []int) int {
	if len(numbers) < 2 {
		return 1
	}

	firstDelta := numbers[1] - numbers[0]
	if absInt(firstDelta) < 1 || absInt(firstDelta) > 3 {
		return 0
	}

	order := firstDelta

	for i := 1; i < len(numbers); i++ {
		delta := numbers[i] - numbers[i-1]
		if absInt(delta) < 1 || absInt(delta) > 3 {
			return 0
		}
		if !checkOrder(delta, order) {
			return 0
		}
	}
	return 1
}

func absInt(number int) int {
	if number < 0 {
		return number * -1
	}
	return number
}

func checkOrder(delta int, order int) bool {
	if order > 0 {
		return delta > 0
	}
	return delta < 0
}
