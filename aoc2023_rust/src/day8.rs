// Need a way to repeat instructions

use std::fs;
use std::io;
use std::collections::HashMap;
use crate::day1::{to_u32, to_usize, to_u64};


fn process_inputs(lines: Vec<String>) -> (Vec<u32>, Vec<(u32, u32)>) {
    let instructions: Vec<u32> = lines[0]
        .chars()
        .map(|x| if x == 'L' { 0 } else { 1 })
        .collect();
    let mut mappings: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines[1..lines.len()].iter() {
        if *line != "" {
            let key: &str = &line[0..3];
            let value: (&str, &str) = (&line[7..10], &line[12..15]);
            mappings.insert(key, value);
        }
    }
    let mut sorted_keys: Vec<&str> = mappings.keys().map(|x| *x).collect();
    sorted_keys.sort();
    let mut key_index_map: HashMap<&str, u32> = HashMap::new();
    for (i, k) in sorted_keys.iter().enumerate() {
        key_index_map.insert(k, to_u32(i));
    }
    let mut vec_mappings: Vec<(u32, u32)> = Vec::new();
    for key in sorted_keys.iter() {
        let val: (&str, &str) = mappings[key];
        println!("mapping {} to {} {}", key, val.0, val.1);
        let map_indices: (u32, u32) = (key_index_map[val.0], key_index_map[val.1]);
        vec_mappings.push(map_indices);
    }
    (instructions, vec_mappings)
}


fn next_location(instructions: &Vec<u32>, mappings: &Vec<(u32, u32)>, cur_instr: usize, loc_ind: usize, num_instr: usize) -> u32{
    let mapped: (u32, u32) = mappings[loc_ind];
    let loc: u32 = match instructions[cur_instr % num_instr] {
        0 => mapped.0,
        1 => mapped.1,
        _ => panic!("Welp"),
    };
    loc
}


fn follow_instructions(instructions: Vec<u32>, mappings: Vec<(u32, u32)>) -> u32 {
    let target_location = to_u32(mappings.len()) - 1;
    let num_instr: usize = instructions.len();
    let mut cur_instr: usize = 0;
    let mut loc_ind: usize = 0;
    while next_location(&instructions, &mappings, cur_instr, loc_ind, num_instr) != target_location {
        loc_ind = to_usize(next_location(&instructions, &mappings, cur_instr, loc_ind, num_instr));
        cur_instr += 1;
    }
    to_u32(cur_instr + 1)
}


pub fn day8_p1(input_file: &str) -> Result<u32, io::Error> {
    let lines: Vec<String> = fs::read_to_string(input_file)?.lines().map(String::from).collect::<Vec<String>>();
    let (instructions, mappings): (Vec<u32>, Vec<(u32, u32)>) = process_inputs(lines);
    let num_instr: u32 = follow_instructions(instructions, mappings);
    Ok(num_instr)
}


fn process_inputs_2(lines: &Vec<String>) -> (Vec<u32>, HashMap<&str, (&str, &str)>) {
    let instructions: Vec<u32> = lines[0]
        .chars()
        .map(|x| if x == 'L' { 0 } else { 1 })
        .collect();
    let mut mappings: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines[1..lines.len()].iter() {
        if *line != "" {
            let key: &str = &line[0..3];
            let value: (&str, &str) = (&line[7..10], &line[12..15]);
            mappings.insert(key, value);
        }
    }
    (instructions, mappings)
}


fn next_location_2<'a>(instructions: &'a Vec<u32>, mappings: &'a HashMap<&'a str, (&'a str, &'a str)>, cur_instr: usize, loc_ind: &str, num_instr: usize) -> &'a str {
    let mapped: (&str, &str) = mappings[loc_ind];
    let loc = match instructions[cur_instr % num_instr] {
        0 => mapped.0,
        1 => mapped.1,
        _ => panic!("Welp"),
    };
    //let mapped: Vec<(&str, &str)> = loc_ind
    //    .iter()
    //    .map(|x| mappings[x])
    //    .collect();
    //let loc: Vec<&str> = mapped
    //    .iter()
    //    .map(|x| if instructions[cur_instr % num_instr] == 0 {x.0} else {x.1})
    //    .collect();
    loc
}


fn gcd(a: u64, b: u64) -> u64 {
    if a == b {
        return a;
    }
    let mut ap = a;
    let mut bp = b;
    if bp > ap {
        let temp = ap;
        ap = bp;
        bp = temp;
    }
    while bp > 0 {
        let temp = ap;
        ap = bp;
        bp = temp % bp;
    }
    ap
}


fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}


fn vec_lcm(nums: Vec<u64>) -> u64 {
    if nums.len() == 2 {
        return lcm(nums[0], nums[1]);
    }
    let mut ans: u64 = lcm(nums[0], nums[1]);
    for i in 2..nums.len() {
        ans = lcm(ans, nums[i]);
    }
    ans
}


fn follow_instructions_2(instructions: Vec<u32>, mappings: HashMap<&str, (&str, &str)>) -> u64 {
    let num_instr: usize = instructions.len();
    let loc_inds: Vec<&str> = mappings
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| *x)
        .collect();
    let mut num_instructions_needed: Vec<u64> = Vec::new();
    for loc in loc_inds {
        let mut loc_ind = loc;
        let mut cur_instr: usize = 0;
        while !next_location_2(&instructions, &mappings, cur_instr, loc_ind, num_instr).ends_with('Z') {
            loc_ind = next_location_2(&instructions, &mappings, cur_instr, loc_ind, num_instr);
            cur_instr += 1;
        }
        num_instructions_needed.push(to_u64(cur_instr + 1));
    }
    vec_lcm(num_instructions_needed)
}

 
pub fn day8_p2(input_file: &str) -> Result<u64, io::Error> {
    let lines: Vec<String> = fs::read_to_string(input_file)?.lines().map(String::from).collect::<Vec<String>>();
    let (instructions, mappings): (Vec<u32>, HashMap<&str, (&str, &str)>) = process_inputs_2(&lines);
    let num_instr: u64 = follow_instructions_2(instructions, mappings);
    Ok(num_instr)
}
