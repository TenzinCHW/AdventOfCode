// Read in the file as a grid (again)
// process starting from S and write a function to move in the 4 directions
// enumerate all possible paths (should be a loop)
// Use this procedure to generate a graph
// Each node has a dist from start, max over all nodes in the graph is the answer

use std::fs;
use std::io;
use crate::day1::to_u32;
use std::cmp::min;


struct TreeNode {
    y: u32,
    x: u32,
}


fn grid_val(grid: &Vec<String>, i: usize, j:usize) -> char {
    grid[i].chars().nth(j).expect("welp")
}


fn find_s(grid: &Vec<String>) -> (usize, usize) {
    for i in 0..grid.len()-1 {
        for j in 0..grid[0].len()-1 {
            if grid_val(grid, i, j) == 'S' {
                return (i, j)
            }
        }
    }
    return (0, 0)
}


fn replace_char_at(s: &String, idx: usize, c: char) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    chars[idx] = c;
    chars.iter().collect::<String>()
}


fn loop_pipes(mut grid: Vec<String>, start_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut pipes: Vec<(usize, usize)> = Vec::new();
    let max_y: usize = grid.len();
    let max_x: usize = grid[0].len();
    let (mut y, mut x): (usize, usize) = start_pos;
    let mut cur = grid_val(&grid, y, x);
    while pipes.is_empty() || cur != 'S' {
        println!("{}", grid[0]);
        if y > 0 {
            let up: char = grid_val(&grid, y-1, x);
            if (cur == 'S' || cur == '|' || cur == 'J' || cur == 'L') && (up == '|' || up == '7' || up == 'F') {
                (y, x) = (y-1, x);
                cur = grid_val(&grid, y, x);
                grid[x] = replace_char_at(&grid[x], y, '*');
                pipes.push((y, x));
                continue;
            }
        }
        if y < max_y-1 {
            let down: char = grid_val(&grid, y+1, x);
            if (cur == 'S' || cur == '|' || cur == 'F' || cur == '7') && (down == '|' || down == 'L' || down == 'J') {
                (y, x) = (y+1, x);
                cur = grid_val(&grid, y, x);
                grid[x] = replace_char_at(&grid[x], y, '*');
                pipes.push((y, x));
                continue;
            }
        }
        if x > 0 {
            let left: char = grid_val(&grid, y, x-1);
            if (cur == 'S' || cur == '-' || cur == '7' || cur == 'J') && (left == '-' || left == 'F' || left == 'L') {
                (y, x) = (y, x-1);
                cur = grid_val(&grid, y, x);
                grid[x] = replace_char_at(&grid[x], y, '*');
                pipes.push((y, x));
                continue;
            }
        }
        if x < max_x {
            let right: char = grid_val(&grid, y, x+1);
            if (cur == 'S' || cur == '-' || cur == 'F' || cur == 'L') && (right == '-' || right == '7' || right == 'J') {
                (y, x) = (y, x+1);
                cur = grid_val(&grid, y, x);
                grid[x] = replace_char_at(&grid[x], y, '*');
                pipes.push((y, x));
                continue;
            }
        }
    }
    pipes
    //pipes.iter().enumerate().filter(|(i, _)| *i != pipes.len()-1).map(|(_, x)| *x).collect()
}


pub fn day10_p1(input_file: &str) -> Result<u32, io::Error>{
    let lines: Vec<String> = fs::read_to_string(input_file)?.lines().map(String::from).collect::<Vec<String>>();
    let s_loc: (usize, usize) = find_s(&lines);
    let pipes = loop_pipes(lines, s_loc);
    println!("{}", pipes.len());
    let max_dist_pos: (usize, usize) = pipes
        .iter()
        .enumerate()
        .map(|(i, x)| (min(i+1, pipes.len()-i), x))
        .max_by(|(i, _), (j, _)| i.cmp(j))
        .map(|(_, &x)| x)
        .expect("meow");
    let manhattan_from_max: usize = s_loc.0.abs_diff(max_dist_pos.0) + s_loc.1.abs_diff(max_dist_pos.1);
    Ok(to_u32(manhattan_from_max))
}
