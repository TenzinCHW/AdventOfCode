use std::fs;
use std::io;


fn first_digit(s: &str) -> u32 {
    for c in s.chars() {
        if c.is_digit(10) {
            return c.to_digit(10).expect("There are no digits in this line {s}.");
        }
    }
    0
}


pub fn day1_p1(input_file: &str) -> Result<u32, io::Error> {
    let mut total = 0;
    for line in fs::read_to_string(input_file)?.lines() {
        let first: u32 = first_digit(line) * 10;
        let last: u32 = first_digit(line.chars().rev().collect::<String>().as_str());
        total += first + last;
    }
    Ok(total)
}


pub fn to_u32(u: usize) -> u32 {
    u32::try_from(u).unwrap()
}


pub fn to_u64(u: usize) -> u64 {
    u64::try_from(u).unwrap()
}


pub fn to_usize(u: u32) -> usize {
    usize::try_from(u).unwrap()
}


fn all_equal(v: Vec<u32>) -> bool {
    v.iter().min().unwrap() == v.iter().max().unwrap()
}


fn first_last_digit_or_spelt_number(s: &str, possible_words: [&str; 10]) -> (u32, u32) {
    // store the index of the digit or first char of word in a vec, store its value in a
    // parallel vec, pick the value with the lowest index in first vec
    let mut indices: Vec<usize> = Vec::new();
    let mut values: Vec<u32> = Vec::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_digit(10) {
            indices.push(i);
            values.push(c.to_digit(10).expect("{c} should be a digit."));
        }
    }
    for (i, word) in possible_words.iter().enumerate() {
        let v: Vec<_> = s.match_indices(word).collect();
        for (ind, _) in v {
            indices.push(ind);
            values.push(to_u32(i));
        }
    }
    let index_min = indices
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index);
    let index_max = indices
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index);
    (values[index_min.unwrap()], values[index_max.unwrap()])
}


pub fn day1_p2(input_file: &str) -> Result<u32, io::Error> {
    let possible_words: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut total = 0;
    for line in fs::read_to_string(input_file)?.lines() {
        let (first, last): (u32, u32) = first_last_digit_or_spelt_number(line, possible_words);
        total += first *10 + last;
    }
    Ok(total)
}

