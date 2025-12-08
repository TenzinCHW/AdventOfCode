package main

import (
    "fmt"
    "strconv"
    "os"
    "log"
    "bufio"
    "strings"
    "slices"
)


func IDIsInvalid(id int) bool {
    // write check for repeat
    str_id := strconv.Itoa(id)
    half_id_len := len(str_id) / 2
    return slices.Equal([]rune(str_id)[:half_id_len], []rune(str_id)[half_id_len:])
}


func IDAllSliceEqual(str_id string, slice_size int) bool {
    // check all i consecutive values
    id_len := len(str_id)
    num_slice := id_len / slice_size
    for j := 1; j < num_slice; j++ {
        if str_id[:slice_size] != str_id[slice_size*j:slice_size*(j+1)] {
            return false
        }
    }
    return true
}


func IDIsVeryInvalid(id int) bool {
    str_id := strconv.Itoa(id)
    id_len := len(str_id)
    for i := 1; i < id_len; i++ {
        if id_len % i == 0 {
            if IDAllSliceEqual(str_id, i) {
                return true
            }
        }
    }
    return false
}


func GetInvalidIDsInRange(start int, end int, check_invalid_id func(int) bool) []int {
    // loop over range
    var invalid_ids []int
    for i := start; i < end + 1; i++ {
        if check_invalid_id(i) {
            invalid_ids = append(invalid_ids, i)
        }
    }
    return invalid_ids
}


func GetInvalidIDsInFile(filepath string, check_invalid_id func(int) bool) []int {
    file, err := os.Open(filepath)
    if err != nil {
        log.Fatalf("Error opening file: %v", err)
    }
    defer file.Close() // Ensure the file is closed after use

    scanner := bufio.NewScanner(file)
    var invalid_ids []int
    for scanner.Scan() {
        // split text by comma
        ranges := strings.Split(scanner.Text(), ",")
        // split each element by hyphen
        for _, se := range ranges {
            startend := strings.Split(se, "-")
            start, err := strconv.Atoi(startend[0])
            if err != nil {
                panic("Whelp couldn't convert start range to int.")
            }
            end, err := strconv.Atoi(startend[1])
            if err != nil {
                panic("Whelp couldn't convert end range to int.")
            }
            for _, iid := range GetInvalidIDsInRange(start, end, check_invalid_id) {
                invalid_ids = append(invalid_ids, iid)
            }
        }
    }
    return invalid_ids
}


func sum(arr []int) int {
    acc := 0
    for _, i := range arr {
        acc += i
    }
    return acc
}


func main() {
    //const filepath = "data/ex_day2.txt"
    const filepath = "data/day2.txt"

    //invalid_ids := GetInvalidIDsInFile(filepath, IDIsInvalid)
    invalid_ids := GetInvalidIDsInFile(filepath, IDIsVeryInvalid)
    invalid_sum := sum(invalid_ids)
    fmt.Println(invalid_sum)
    //a := "hihi"
    //b := []rune(a)[:2]
    //c := []rune(a)[2:]
    //fmt.Printf("b is %v and c is %v\n", b, c)
    ////fmt.Println(slices.Equal(b, c))
    //d := "hi,hi"
    //e := strings.Split(d, ",")
    //fmt.Printf("%v, %v\n", e[0], e[1])
}
