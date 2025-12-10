package main

import (
    "fmt"
    "math"
    "os"
    "log"
    "bufio"
)


func power(base int, exponent int) int {
    return int(math.Pow(float64(base), float64(exponent)))
}


func GetFirstBiggestDigitIndex(bank string) int {
    nums := []rune(bank)
    max_val := nums[0]
    for i := 0; i < len(nums); i++ {
        if nums[i] > max_val {
            max_val = nums[i]
        }
    }
    for i := 0; i < len(bank); i++ {
        if nums[i] == max_val {
            return i
        }
    }
    return 0
}


func GetMaxJoltage(bank string, num_battery int) int {
    // recursive function to compute the max of all
    first_max_val_index := GetFirstBiggestDigitIndex(bank[:len(bank) - num_battery + 1])
    max_val := float64([]rune(bank)[first_max_val_index] - 48)
    max_val = max_val * math.Pow10(int(num_battery) - 1)
    int_max_val := int(max_val)
    if num_battery == 1 {
        return int_max_val
    }
    int_max_val += GetMaxJoltage(bank[first_max_val_index+1:], num_battery - 1)
    return int_max_val
}


func GetAllMaxJoltage(filepath string, num_battery int) []int {
    file, err := os.Open(filepath)
    if err != nil {
        log.Fatalf("Error opening file: %v", err)
    }
    defer file.Close() // Ensure the file is closed after use


    scanner := bufio.NewScanner(file)
    var max_joltages []int
    for scanner.Scan() {
        bank := scanner.Text()
        max_joltage := GetMaxJoltage(bank, num_battery)
        max_joltages = append(max_joltages, max_joltage)
    }
    return max_joltages
}


func sum(arr []int) int {
    acc := 0
    for _, i := range arr {
        acc += i
    }
    return acc
}


func main() {
    //const filepath = "data/ex_day3.txt"
    const filepath = "data/day3.txt"

    num_battery := 12
    max_joltages := GetAllMaxJoltage(filepath, num_battery)
    max_joltage_sum := sum(max_joltages)
    fmt.Println(max_joltage_sum)
    //a := "hihi"
    //b := []rune(a)[:2]
    //c := []rune(a)[2:]
    //fmt.Printf("b is %v and c is %v\n", b, c)
    ////fmt.Println(slices.Equal(b, c))
    //d := "hi,hi"
    //e := strings.Split(d, ",")
    //fmt.Printf("%v, %v\n", e[0], e[1])
}
