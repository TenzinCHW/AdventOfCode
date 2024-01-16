// Keep a vec of true for each column
// Keep an empty vec of u32 for each expanded row index
// Keep an empty vec of coords of galaxies
// For each line
//     if all chars are ., push the index into row
//     For each column with a #, set the column index in the col vec to false
//     Add the (r, c) indices to the coords vec
// Create new vec of u32 for expanded column indices
// For each column that is true, add the index to this expanded column vec
// Create updated galaxy positions vec
// For each galaxy x and y, add 1 for each column that current x is bigger than column index and
// each row that is bigger than a row index
// Take combinations of all galaxies and compute the Manhattan distance between them

use std::fs;
use std::io;
use crate::day1::to_u64;

fn star_loc(line: &str, line_no: usize) -> Vec<(u64, u64)> {
    line
    .chars()
    .enumerate()
    .filter(|(_, x)| *x == '#')
    .map(|(j, _)| (to_u64(line_no), to_u64(j))) 
    .collect()
}


fn line_empty(line: &str) -> bool {
    line.chars().all(|x| x == '.')
}


fn num_empty_before(loc: u64, empty: &Vec<u64>) -> u64 {
    let l = empty.iter().filter(|i| **i < loc).map(|i| *i).collect::<Vec<u64>>().len();
    to_u64(l)
}


fn star_dist(s1: (u64, u64), s2: (u64, u64), empty_row: &Vec<u64>,
             empty_col: &Vec<u64>, dist_per_empty: u64) -> u64 {
    let s1y: u64 = s1.0 + num_empty_before(s1.0, &empty_row) * (dist_per_empty - 1);
    let s1x: u64 = s1.1 + num_empty_before(s1.1, &empty_col) * (dist_per_empty - 1);
    let s2y: u64 = s2.0 + num_empty_before(s2.0, &empty_row) * (dist_per_empty - 1);
    let s2x: u64 = s2.1 + num_empty_before(s2.1, &empty_col) * (dist_per_empty - 1);
    s1y.abs_diff(s2y) + s1x.abs_diff(s2x)
}


pub fn calc_star_dists(input_file: &str, dist_per_empty: u64) -> Result<u64, io::Error> {
    let mut stars: Vec<(u64, u64)> = Vec::new();
    let mut empty_row: Vec<u64> = Vec::new();
    for (i, line) in fs::read_to_string(input_file)?.lines().enumerate() {
        if line_empty(line) {
            empty_row.push(to_u64(i));
            continue;
        }
        let line_stars = star_loc(line, i);
        for ij in line_stars.iter() {
            stars.push(*ij);
        }
    }
    let non_empty_col: Vec<u64> = stars.iter().map(|(_, j)| *j).collect();
    let ncol: u64 = stars.iter().map(|(_, j)| *j).max().expect("No stars to consider");
    let empty_col: Vec<u64> = (0..ncol).filter(|i| !non_empty_col.contains(i)).collect();
    let mut star_dists: Vec<u64> = Vec::new();
    for i in 0..stars.len()-1 {
        for j in (i+1)..stars.len() {
            star_dists.push(star_dist(stars[i], stars[j], &empty_row, &empty_col, dist_per_empty));
        }
    }
    let dist: u64 = star_dists.iter().sum();
    Ok(dist)
}


pub fn day11_p1(input_file: &str) -> Result<u64, io::Error> {
    let dist: u64 = calc_star_dists(input_file, 2).expect("Something's wrong");
    Ok(dist)
}


pub fn day11_p2(input_file: &str) -> Result<u64, io::Error> {
    let dist: u64 = calc_star_dists(input_file, 1_000_000).expect("Something's wrong");
    Ok(dist)
}
