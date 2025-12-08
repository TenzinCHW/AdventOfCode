package main

import (
    "fmt"
    "strconv"
    "os"
    "log"
    "bufio"
)


func Turn(instruction string) int {
    dir := string(instruction[0])
    num_click, err := strconv.Atoi(instruction[1:])
    if (err != nil) {
        panic(err)
    }
    switch dir {
    case "L":
        return -num_click
    case "R":
        return num_click
    default:
        panic("GG")
    }
}


func CountZeroAfterTurn(filepath string) int {
    file, err := os.Open(filepath)
    if err != nil {
        log.Fatalf("Error opening file: %v", err)
    }
    defer file.Close() // Ensure the file is closed after use

    state := 50
    num_zero := 0
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        state += Turn(scanner.Text())
        state %= 100
        if state == 0 {
            num_zero++
        }
    }
    return num_zero
}


func CountZeroAfterAnyClick(filepath string) int {
    file, err := os.Open(filepath)
    if err != nil {
        log.Fatalf("Error opening file: %v", err)
    }
    defer file.Close() // Ensure the file is closed after use

    state := 50
    num_zero := 0
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        num_click := Turn(scanner.Text())
        rem_click := num_click % 100
        num_cycle := (num_click - rem_click) / 100
        if num_cycle < 0 {
            num_cycle = -num_cycle
        }
        num_zero += num_cycle
        new_state := state + rem_click
        //fmt.Println(state, rem_click, new_state)
        if new_state < 0 {
            new_state = 100 + new_state
            if state != 0 {
                num_zero += 1
            }
        } else if new_state >= 100 || new_state == 0 {
            num_zero += 1
        }
        state = new_state % 100
        //fmt.Println(state, num_zero)
    }
    return num_zero
}


func main() {
    //const filepath = "data/ex_day1.txt"
    const filepath = "data/day1.txt"

    //count := CountZeroAfterTurn(filepath)
    count := CountZeroAfterAnyClick(filepath)
    fmt.Println(count)
}
