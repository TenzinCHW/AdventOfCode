package main

import (
    "fmt"
    "os"
    "log"
    "bufio"
    "strings"
    "strconv"
    "sort"
)


type IDRange struct {
    Low int
    High int
}


type ByRange []IDRange

func (a ByRange) Len() int {
    return len(a)
}

func (a ByRange) Swap(i, j int) {
    a[i], a[j] = a[j], a[i]
}

func (a ByRange) Less(i, j int) bool {
    return a[i].Low < a[j].Low || (a[i].Low == a[j].Low && a[i].High < a[j].High)
}


func IDInRange(id int, r IDRange) bool {
    return (r.Low <= id) && (id <= r.High)
}


func ExtractFreshRangeAndIDs(filepath string) ([]int, []IDRange) {
    file, err := os.Open(filepath)
    if err != nil {
        log.Fatalf("Error opening file: %v", err)
    }
    defer file.Close() // Ensure the file is closed after use

    scanner := bufio.NewScanner(file)
    scan_range := true
    ranges := []IDRange {}
    ids := []int {}
    for scanner.Scan() {
        line := scanner.Text()
        if line == "" {
            scan_range = false
            continue
        }
        if scan_range {
            lohi := strings.Split(line, "-")
            lo, err := strconv.Atoi(lohi[0])
            if err != nil {
                panic("Whelp couldn't convert low of range to int.")
            }
            hi, err := strconv.Atoi(lohi[1])
            if err != nil {
                panic("Whelp couldn't convert high of range to int.")
            }
            ranges = append(ranges, IDRange{lo, hi})
        } else {
            id, err := strconv.Atoi(line)
            if err != nil {
                panic("Whelp couldn't convert id to int.")
            }
            ids = append(ids, id)
        }
    }
    return ids, ranges
}


func CountFreshIngredientIDs(filepath string) int {
    ids, ranges := ExtractFreshRangeAndIDs(filepath)
    num_fresh := 0
    for i := 0; i < len(ids); i++ {
        for j := 0; j < len(ranges); j++ {
            //fmt.Println(lows[j], highs[j], ids[i])
            if IDInRange(ids[i], ranges[j]) {
                num_fresh++
                break
            }
        }
    }
    return num_fresh
}


func Unique(sorted_arr []int) []int {
    if len(sorted_arr) == 0 {
        return []int {}
    }

    cur := sorted_arr[0];
    unique_arr := []int {cur}
    for i := 1; i < len(sorted_arr); i++ {
        if cur != sorted_arr[i] {
            cur = sorted_arr[i]
            unique_arr = append(unique_arr, cur)
        }
    }
    return unique_arr
}


func CombineIDRange(range1 IDRange, range2 IDRange) IDRange {
    return IDRange{min(range1.Low, range2.Low), max(range1.High, range2.High)}
}


func CountFreshIDRange(filepath string) int {
    _, ranges := ExtractFreshRangeAndIDs(filepath)

    sort.Sort(ByRange(ranges))
    minimal_ranges := []IDRange {}
    for i := 0; i < len(ranges); i++ {
        min_range_len := len(minimal_ranges)
        if min_range_len != 0 && minimal_ranges[min_range_len-1].High >= ranges[i].High {
            continue
        }
        low := ranges[i].Low
        high := ranges[i].High
        for j := i + 1; j < len(ranges); j++ {
            if ranges[j].Low <= high {
                high = max(high, ranges[j].High)
            }
        }
        minimal_ranges = append(minimal_ranges, IDRange{low, high})
    }

    num_fresh_ids := 0
    for i := 0; i < len(minimal_ranges); i++ {
        r := minimal_ranges[i]
        num_fresh_ids += r.High - r.Low + 1
    }
    return num_fresh_ids

    // Nope this doesn't work
    //var all_fresh_ids []int
    //for i := 0; i < len(lows); i++ {
    //    for id := lows[i]; id <= highs[i]; id++ {
    //        all_fresh_ids = append(all_fresh_ids, id)
    //    }
    //}

    //sort.Ints(all_fresh_ids)
    //return len(Unique(all_fresh_ids))
}


func main() {
    //const filepath = "data/ex_day5.txt"
    const filepath = "data/day5.txt"

    num_fresh_ids := CountFreshIngredientIDs(filepath)
    fmt.Println(num_fresh_ids)
    num_fresh_id_range := CountFreshIDRange(filepath)
    fmt.Println(num_fresh_id_range)
}
