package main

import (
    "fmt"
    "os"
    "log"
    "bufio"
    "strings"
    "strconv"
)


type extract_fn func(string) ([][] int, []string)


func ExtractNumberGrid(filepath string) (number_columns [][]int, operations []string) {
    file, err := os.Open(filepath)
    if err != nil {
        log.Fatalf("Error opening file: %v", err)
    }
    defer file.Close() // Ensure the file is closed after use

    scanner := bufio.NewScanner(file)
    scan_numbers := true
    for scanner.Scan() {
        line := scanner.Text()
        if strings.Contains(line, "+") || strings.Contains(line, "*") {
            scan_numbers = false
            // split line and apply operation
            ops := strings.Split(line, " ")
            for i := 0; i < len(ops); i++ {
                if ops[i] == "+" || ops[i] == "*" {
                    operations = append(operations, ops[i])
                }
            }
        }
        if scan_numbers {
            nums := strings.Split(line, " ")
            num_col := 0
            for i := 0; i < len(nums); i++ {
                if nums[i] != "" {
                    num_col++
                }
            }
            if len(number_columns) != num_col {
                number_columns = make([][]int, num_col)
            }
            j := 0
            for i := 0; i < len(nums); i++ {
                if nums[i] != "" {
                    num, err := strconv.Atoi(nums[i])
                    if err != nil {
                        panic("Whelp couldn't convert num to int.")
                    }
                    number_columns[j] = append(number_columns[j], num)
                    j++
                }
            }
        }
    }
    return number_columns, operations
}


func ApplyNumColOperator(filepath string, number_grid_fn extract_fn) int {
    number_columns, operations := number_grid_fn(filepath)
    num_col := len(number_columns)
    fmt.Println("Number of columns ", len(number_columns), "number of rows ", len(number_columns[0]), "number of operations ", len(operations))
    res := 0
    var r int
    for i := 0; i < num_col; i++ {
        if operations[i] == "+" {
            r = 0
        } else if operations[i] == "*" {
            r = 1
        } else {
            fmt.Printf("Operation that failed: %v", operations[i])
            panic("Operation not recognised.")
        }
        //fmt.Printf("Op %v ", operations[i])
        for j := 0; j < len(number_columns[i]); j++ {
            num := number_columns[i][j]
            if operations[i] == "+" {
                r += num
            } else if operations[i] == "*" {
                r *= num
            } else {
                fmt.Printf("Operation that failed: %v", operations[i])
                panic("Operation not recognised but should have failed earlier.")
            }
        }
        //fmt.Printf("Got %v using operation %v\n", r, operations[i])
        res += r
    }
    return res
}


func ExtractDigitGrid(filepath string) (number_columns [][]int, operations []string) {
    file, err := os.Open(filepath)
    if err != nil {
        log.Fatalf("Error opening file: %v", err)
    }
    defer file.Close() // Ensure the file is closed after use

    scanner := bufio.NewScanner(file)
    var digit_lines []string
    longest_line := 0
    for scanner.Scan() {
        line := scanner.Text()
        longest_line = max(longest_line, len(line))
        if strings.Contains(line, "+") || strings.Contains(line, "*") {
            // split line and apply operation
            ops := strings.Split(line, " ")
            for i := 0; i < len(ops); i++ {
                if ops[i] == "+" || ops[i] == "*" {
                    operations = append(operations, ops[i])
                }
            }
        } else {
            // put into digit_lines
            digit_lines = append(digit_lines, line)
        }
    }

    num_rows := len(digit_lines)
    cur_col := make([]int, 0, num_rows)
    for j := 0; j < longest_line; j++ {
        // for each column, loop through each row to get its 
        cur_num := ""
        for i := 0; i < num_rows; i++ {
            var dig string
            if j >= len(digit_lines[i]) {
                dig = " "
            } else {
                //fmt.Println("j is ", j, "line length is ", len(digit_lines[i]))
                dig = string(digit_lines[i][j])
            }
            if dig != " " {
                cur_num += dig
            }
        }
        // if all zeros, copy cur_col into a new slice and append to number_columns and make a new slice for cur_col
        if cur_num == "" {
            dst := make([]int, len(cur_col))
            copy(dst, cur_col)
            number_columns = append(number_columns, dst)
            cur_col = nil
        } else {
            // add atoi of cur_num to cur_col
            num, err := strconv.Atoi(cur_num)
            if err != nil {
                panic("Whelp unable to convert number to valid int.")
            }
            //fmt.Println("Found num ", cur_num)
            cur_col = append(cur_col, num)
        }
    }
    number_columns = append(number_columns, cur_col)
    fmt.Println("Actual columns found ", len(number_columns))
    return number_columns, operations
}


func main() {
    //const filepath = "data/ex_day6.txt"
    const filepath = "data/day6.txt"

    //a := ApplyNumColOperator(filepath, ExtractNumberGrid)
    a := ApplyNumColOperator(filepath, ExtractDigitGrid)
    fmt.Println(a)
}
