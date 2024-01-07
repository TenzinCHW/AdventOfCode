use std::fs;
use std::io;
use crate::day1::{to_u32, to_usize};


#[derive(Copy, Clone)]
struct SchematicChar {
    c: char,
    x: u32,
}


fn char_is_symbol(sc: &SchematicChar) -> bool {
    !(sc.c.is_digit(10) || sc.c == '.')
}


fn char_is_digit(sc: &SchematicChar) -> bool {
    sc.c.is_digit(10)
}


fn replace_w_dots(line: &mut Vec<SchematicChar>, start: u32, end: u32) {
    for i in start..end {
        line[to_usize(i)] = SchematicChar {
            c: '.',
            x: line[to_usize(i)].x,
        }
    }
}


fn join_sc_vec(sc_vec: &Vec<SchematicChar>) -> String {
    let mut svec: Vec<char> = Vec::new();
    for sc in sc_vec {
        svec.push(sc.c);
    }
    svec.iter().collect()
}


fn search_contiguous_digits(line: &mut Vec<SchematicChar>, i: u32) -> u32 {
    let mut j = i;
    while j > 0 && char_is_digit(&line[to_usize(j)]) {
        j -= 1;
    }
    if !char_is_digit(&line[to_usize(j)]) {
        j += 1;
    }
    let start = j;
    j = i;
    while to_usize(j) < line.len() && char_is_digit(&line[to_usize(j)]){
        j += 1;
    }
    let end = j;
    let res = line[to_usize(start)..to_usize(end)].iter().map( |sc| sc.c).collect::<String>();
    replace_w_dots(line, start, end);
    res.parse::<u32>().unwrap()
}


pub fn day3_p1(input_file: &str) -> Result<u32, io::Error> {
    // verify input (all lines have same length), find coords of all digits and symbols
    // for each digit on a line, search symbol list for any where diff in x and y coords are both
    // at most 1. If it is, search for string of digits (left and right) on the same line and parse
    // it as a u32

    let lines: Vec<String> = fs::read_to_string(input_file).unwrap().lines().map(String::from).collect::<Vec<String>>();
    let lengths = lines.iter().map(|line| line.len()).collect::<Vec<usize>>();
    assert_eq!(lengths.iter().min().unwrap(), lengths.iter().max().unwrap());

    let mut sc_lines: Vec<Vec<SchematicChar>> = Vec::new();
    for line in lines {
        let sc_line = line.chars().enumerate().map(| (i, c) | SchematicChar {
            c: c,
            x: to_u32(i),
        }).collect::<Vec<SchematicChar>>();
        sc_lines.push(sc_line);
    }

    let mut prev_line: Vec<SchematicChar> = Vec::new();
    for i in 0..lengths[0]-1 {
        prev_line.push(SchematicChar {
            c: '.',
            x: to_u32(i),
        })
    }
    let mut total: u32 = 0;
    for sc_line in sc_lines {
        // fliter out non-digits, and non-symbols into separate vectors
        let mut sc_line_mut = sc_line.iter().copied().collect();
        let symbols = sc_line.iter().filter(|sc| !(sc.c.is_digit(10) || sc.c == '.'));
        for symbol in symbols {
            let mut start = 0;
            let mut end = to_u32(sc_line.len());
            if symbol.x > start && char_is_digit(&sc_line[to_usize(symbol.x-1)]) {
                // search left
                // convert &sc_line[i..symbol.x] to u32, add to total
                total += search_contiguous_digits(&mut sc_line_mut, symbol.x-1);
            }
            if symbol.x < end-2 && char_is_digit(&sc_line[to_usize(symbol.x+1)]) {
                // search right
                total += search_contiguous_digits(&mut sc_line_mut, symbol.x+1);
            }
            // search prev_line for digits
            if symbol.x > start {
                start = symbol.x-1;
            }
            if symbol.x+2 < end {
                end = symbol.x+2;
            }
            for i in start..end {
                if char_is_digit(&prev_line[to_usize(i)]) {
                    total += search_contiguous_digits(&mut prev_line, i);
                }
            }
        }

        let prev_line_symbols = prev_line.iter().filter(|sc| !(sc.c.is_digit(10) || sc.c == '.'));
        for symbol in prev_line_symbols {
            // search prev_line for symbols
            let mut start = 0;
            let mut end = to_u32(sc_line.len());
            if symbol.x > start {
                start = symbol.x-1;
            }
            if symbol.x+2 < end {
                end = symbol.x+2;
            }
            for i in start..end {
                if char_is_digit(&sc_line_mut[to_usize(i)])
                {
                    total += search_contiguous_digits(&mut sc_line_mut, i);
                }
            }
        }
        // assign this line to previous line
        prev_line = sc_line_mut;
    }
    Ok(total)
}
