// cur = 0
// for ascii in code
//      cur += ascii
//      cur *= 17
//      cur %= 256
//  return cur

use std::fs;
use std::io;
use std::iter::zip;
use crate::day1::{to_usize, to_u32};


fn hash(to_hash: &str) -> u32 {
    let ascii_vals: Vec<u8> = to_hash.chars().map( |x| x as u8 ).collect::<Vec<u8>>();
    let mut cur: u32 = 0;
    for val in ascii_vals {
        cur += u32::from(val);
        cur *= 17;
        cur %= 256;
    }
    cur
}


fn assembly_power(lens_assembly: &Vec<(&str, u32)>, assembly_box_id: u32) -> u32 {
    if lens_assembly.is_empty() {
        return 0;
    }
    let mut pow: u32 = 0;
    for (j, (_, lens_pow)) in lens_assembly.iter().enumerate() {
        let lp = (assembly_box_id+1) * (to_u32(j)+1) * lens_pow;
        pow += lp;
    }
    pow
}


pub fn day15_p1(input_file: &str) -> Result<u32, io::Error>{
    let line: &String = &fs::read_to_string(input_file).unwrap().lines().map(String::from).collect::<Vec<String>>()[0];
    let steps = line.split(',').collect::<Vec<&str>>();
    let hash_values = steps.iter().map( |x| hash(*x));
    Ok(hash_values.sum())
}


pub fn day15_p2(input_file: &str) -> Result<u32, io::Error>{
    let line: &String = &fs::read_to_string(input_file).unwrap().lines().map(String::from).collect::<Vec<String>>()[0];
    let steps = line.split(',').collect::<Vec<&str>>();
    let mut lenses: Vec<Vec<(&str, u32)>> = Vec::new();
    for _ in 0..256 {
        let lens_assembly: Vec<(&str, u32)> = Vec::new();
        lenses.push(lens_assembly);
    }
    for step in steps {
        if step.contains('-') {
            let name: &str = step.strip_suffix("-").expect("GG malformed remove step");
            let h = hash(name);
            lenses[to_usize(h)] = lenses[to_usize(h)].iter().cloned().filter(|x| x.0 != name).collect::<Vec<(&str, u32)>>();
        } else if step.contains("=") {
            let n_lp: Vec<&str> = step.split('=').collect::<Vec<&str>>();
            let (name, lens_pow): (&str, u32) = (n_lp[0], n_lp[1].parse::<u32>().expect("GG malformed replace step"));
            let h = hash(name);
            if lenses[to_usize(h)].iter().any( |x| x.0 == name) {
                let replace_index: Vec<usize>= lenses[to_usize(h)].iter().enumerate().filter( |(_, x)| x.0 == name).map( |(i, _)| i).collect();
                assert_eq!(replace_index.len(), 1);
                lenses[to_usize(h)][replace_index[0]] = (name, lens_pow);
            } else {
                lenses[to_usize(h)].push((name, lens_pow));
            }
        } else {
            panic!("GG this step is malformed.");
        }
    }

    let total: u32 = lenses.iter().enumerate().map( |(i, x)| assembly_power(x, to_u32(i))).sum();
    Ok(total)
}
