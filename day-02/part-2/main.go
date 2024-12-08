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
		numberSlice := convertToNumArray(split)
		temp := solve(numberSlice, 0)
		// safe_reports += solve(numberSlice)
		fmt.Println("Line: ", scanner.Text(), " result: ", temp)
		safe_reports += temp
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

func solve(numberSlice []int, iteration int) int {
	if len(numberSlice) < 2 {
		return 1
	}

	problem := 0
	order := numberSlice[1] - numberSlice[0]

	for i := 1; i < len(numberSlice); i++ {
		delta := numberSlice[i] - numberSlice[i-1]
		if absInt(delta) < 1 || absInt(delta) > 3 {
			problem = 1
		} else if !checkOrder(delta, order) {
			problem = 1
		}
		if problem != 0 {
			if iteration == 0 {
				return problemDampener(numberSlice)
			} else {
				return 0
			}
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

func problemDampener(numberSlice []int) int {
	for i := 0; i < len(numberSlice); i++ {
		newSlice := make([]int, 0, len(numberSlice)-1)
		newSlice = append(newSlice, numberSlice[:i]...)
		newSlice = append(newSlice, numberSlice[i+1:]...)
		if solve(newSlice, 1) == 1 {
			return 1
		}
	}
	return 0
}
