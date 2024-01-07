// Recursive function to take differences between each value of input
// base case: all 0, return last value of input
// recursive case: return last value plus recursion on difference

use std::fs;
use std::io;


fn get_first(items: Vec<i32>) -> i32 {
    *items.first().unwrap()
}


fn get_last(items: Vec<i32>) -> i32 {
    *items.last().unwrap()
}


fn extrapolate(observations: Vec<i32>, mult: i32,
               take_fn: &dyn Fn(Vec<i32>) -> i32) -> i32 {
    let mut diffs: Vec<i32> = Vec::new();
    for (i, obs) in observations[0..observations.len()-1].iter().enumerate() {
        diffs.push(observations[i+1] - obs);
    }
    if *diffs.iter().min().unwrap() == *diffs.iter().max().unwrap() &&
        *diffs.iter().min().unwrap() == 0 {
        return take_fn(observations)
    } else {
        return take_fn(observations) + mult * extrapolate(diffs, mult, take_fn);
    }
}


fn predict(observations: Vec<i32>) -> i32 {
    extrapolate(observations, 1, &get_last)
}


fn unpredict(observations: Vec<i32>) -> i32 {
    extrapolate(observations, -1, &get_first)
}


fn parse_as_i32_array(line: Vec<&str>) -> Vec<i32> {
    line.iter().map(|x| x.parse::<i32>().expect("Welp found a non-u32")).collect::<Vec<i32>>()
}


pub fn day9_p1(input_file: &str) -> Result<i32, io::Error> {
    let mut total: i32 = 0;
    for line in fs::read_to_string(input_file)?.lines() {
        let line_array: Vec<&str> = line.split_whitespace().collect();
        let observations: Vec<i32> = parse_as_i32_array(line_array);
        let prediction: i32 = predict(observations);
        total += prediction;
    }
    Ok(total)
}


pub fn day9_p2(input_file: &str) -> Result<i32, io::Error> {
    let mut total: i32 = 0;
    for line in fs::read_to_string(input_file)?.lines() {
        let line_array: Vec<&str> = line.split_whitespace().collect();
        let observations: Vec<i32> = parse_as_i32_array(line_array);
        let prediction: i32 = unpredict(observations);
        total += prediction;
    }
    Ok(total)
}
