// For each card, for each number you have and each winning number, compute length of intersection
// of the two sets
// 2^(n-1) is the score for that card where n is the length of intersection
use std::fs;
use std::io;
use crate::day1::{to_u32, to_usize};


fn clean_parse_u32(val: &str) -> u32 {
    val.trim_matches(' ').parse::<u32>().expect("Not actually an integer: {val}")
}


fn num_wins(raw_line: &str) -> u32 {
    let id_card: Vec<&str> = raw_line.split(':').collect();
    let id = id_card[0].parse::<u32>();
    let win_owned: Vec<&str> = id_card[1].split('|').collect();
    let win_nums: Vec<u32> = win_owned[0].split_whitespace().map( |x| clean_parse_u32(x)).collect();
    let owned_nums: Vec<u32> = win_owned[1].split_whitespace().map( |x| clean_parse_u32(x)).collect();
    let owned_wins: usize = owned_nums.iter().filter( |x| win_nums.contains(x)).count();
    to_u32(owned_wins)
}


fn card_score(raw_line: &str) -> u32 {
    let owned_wins: u32 = num_wins(raw_line);
    if owned_wins == 0 {
        return 0
    }
    u32::pow(2, owned_wins - 1)
}


pub fn day4_p1(input_file: &str) -> Result<u32, io::Error>{
    let mut score: u32 = 0;
    for line in fs::read_to_string(input_file)?.lines() {
        let s = card_score(line);
        score += s;
    }
    Ok(score)
}


pub fn day4_p2(input_file: &str) -> Result<u32, io::Error>{
    let mut card_counts: Vec<u32> = Vec::new();
    let lines: Vec<String> = fs::read_to_string(input_file)?.lines().map(String::from).collect::<Vec<String>>();
    for _ in lines.iter() {
        card_counts.push(1);
    }
    for (i, line) in lines.iter().enumerate() {
        let k: u32 = to_u32(i);
        let nw = num_wins(line);
        let nc = card_counts[i];
        for j in k+1..k+nw+1 {
            card_counts[to_usize(j)] += nc;
        }
    }
    let num_cards: u32 = card_counts.iter().sum();
    Ok(num_cards)
}
