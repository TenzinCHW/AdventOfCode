package main

import (
    "fmt"
    "os"
    "log"
    "bufio"
)


func CountAdjacentRolls(roll_grid []string, i int, j int, blast_radius int) int {
    num_row := len(roll_grid)
    num_col := len(roll_grid[0])
    num_rolls := 0
    num_cells := 0
    for ii := max(0, i - blast_radius); ii < min(num_row, i + blast_radius + 1); ii++ {
        for jj := max(0, j - blast_radius); jj < min(num_col, j + blast_radius + 1); jj++ {
            if roll_grid[ii][jj] == '@' && !(i == ii && j == jj) {
                num_rolls += 1
            }
            num_cells++
        }
    }
    return num_rolls
}


func CountAllAdjacentRolls(roll_grid []string, blast_radius int) [][]int {
    // iterate over row and column
    // iterate over row and column of neighbours up to edge (0 and len(roll_grid) for vertical, 0 and len(roll_grid[0]) for horizontal)
    num_row := len(roll_grid)
    num_col := len(roll_grid[0])
    count_grid := make([][]int, num_row)
    for i := 0; i < num_row; i++ {
        count_grid[i] = make([]int, num_col)
    }
    for i := 0; i < num_row; i++ {
        for j := 0; j < num_col; j++ {
            if roll_grid[i][j] == '@' {
                count_grid[i][j] = CountAdjacentRolls(roll_grid, i, j, blast_radius)
            }
            //fmt.Print(count_grid[i][j], " ")
        }
        //fmt.Println()
    }
    return count_grid
}


func GetNumAccesibleRollsFromGrid(roll_grid []string, blast_radius int, max_neighbours int) ([]string, int) {
    neighbour_counts := CountAllAdjacentRolls(roll_grid, blast_radius)
    num_row := len(roll_grid)
    num_col := len(roll_grid[0])
    num_accessible := 0
    for i := 0; i < num_row; i++ {
        for j := 0; j < num_col; j++ {
            if roll_grid[i][j] == '@' && neighbour_counts[i][j] < max_neighbours {
                num_accessible += 1;
                row := []rune(roll_grid[i])
                row[j] = '.'
                roll_grid[i] = string(row)
            }
        }
    }
    return roll_grid, num_accessible
}


func GetNumAccesibleRollsFromFile(filepath string, blast_radius int, max_neighbours int, remove_rolls bool) int {
    file, err := os.Open(filepath)
    if err != nil {
        log.Fatalf("Error opening file: %v", err)
    }
    defer file.Close() // Ensure the file is closed after use

    scanner := bufio.NewScanner(file)
    var roll_grid []string
    for scanner.Scan() {
        roll_grid = append(roll_grid, scanner.Text())
    }

    roll_grid, num_accessible := GetNumAccesibleRollsFromGrid(roll_grid, blast_radius, max_neighbours)
    
    if remove_rolls {
        na := num_accessible
        for na != 0 {
            roll_grid, na = GetNumAccesibleRollsFromGrid(roll_grid, blast_radius, max_neighbours)
            num_accessible += na
        }
    }

    return num_accessible
}


func main() {
    //const filepath = "data/ex_day4.txt"
    const filepath = "data/day4.txt"

    blast_radius := 1
    max_neighbours := 4
    remove_rolls := true
    num_accessible := GetNumAccesibleRollsFromFile(filepath, blast_radius, max_neighbours, remove_rolls)
    fmt.Println(num_accessible)
}
