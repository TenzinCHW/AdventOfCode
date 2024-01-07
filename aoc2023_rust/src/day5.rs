// Keep each map in a vector
// Define 1 function that maps input to output given a map
// iterate through the maps to get the final output for each seed
// select the minimum

use std::fs;
use std::io;
use std::cmp::{min, max};
use std::collections::HashMap;
use std::process;


fn in_range(start: u64, interval: u64, val: u64) -> bool {
    let end: u64 = start + interval;
    start <= val && val < end
}


fn do_map(maps: HashMap<(u64, u64), u64>, val: u64) -> u64 {
    for (src_interval, dst) in maps.iter() {
        let (src, interval): (u64, u64) = (src_interval.0, src_interval.1);
        if in_range(src, interval, val) {
            let diff: u64 = val - src;
            return dst + diff
        }
    }
    val
}


fn process_seeds(line: &str) -> Vec<u64> {
    let seeds: Vec<u64> = line
        .split_whitespace()
        .enumerate()
        .filter(|(i, _)| *i > 0)
        .map(|(_, x)| x.parse::<u64>().unwrap())
        .collect();
    seeds
}


fn process_map_line(line: &str) -> ((u64, u64), u64) {
    let parsed: Vec<u64> = line
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let (dst, src, interval): (u64, u64, u64) = (parsed[0], parsed[1], parsed[2]);
    ((src, interval), dst)
}


fn process_map_line_2(line: &str) -> (Range, Range) {
    let parsed: Vec<u64> = line
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let (dst, src, interval): (u64, u64, u64) = (parsed[0], parsed[1], parsed[2]);
    (Range {
        start: src,
        end: src + interval,
    },
    Range {
        start: dst,
        end: dst + interval,
    })
}


pub fn day5_p1(input_file: &str) -> Result<u64, io::Error> {
    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Vec<HashMap<(u64, u64), u64>> = Vec::new();
    let mut proc_map = false;
    let mut cur_map: HashMap<(u64, u64), u64> = HashMap::new();
    for line in fs::read_to_string(input_file)?.lines() {
        if line.contains("seeds") {
            seeds = process_seeds(line);
        } else if line.contains("map") {
            cur_map = HashMap::new();
            proc_map = true;
        } else if proc_map && line != "" {
            let (src_interval, dst):
                ((u64, u64), u64) = process_map_line(line);
            cur_map.insert(src_interval, dst);
        } else if !cur_map.is_empty() {
            maps.push(cur_map.clone());
        }
    }
    // No last newline so we gotta manually do this for the last map
    maps.push(cur_map.clone());

    let mut locations: Vec<u64> = Vec::new();
    for seed in seeds {
        let mut out: u64 = seed;
        for m in maps.iter() {
            out = do_map(m.clone(), out);
        }
        locations.push(out);
    }
    Ok(*locations.iter().min().unwrap())
}


fn process_seeds_2(line: &str) -> Vec<Range> {
    let raw_seed_vals: Vec<u64> = process_seeds(line);
    let rsv: Vec<u64> = raw_seed_vals.clone();
    let seed_ranges: Vec<Range> =
        raw_seed_vals.into_iter()
        .step_by(2)
        .zip(rsv
             .into_iter()
             .skip(1)
             .step_by(2))
        .map(|x| Range {
            start: x.0,
            end: x.0 + x.1,
        })
        .collect::<Vec<_>>();
    seed_ranges
}


#[derive(Copy, Clone, Eq, PartialOrd, Ord, Hash)]
struct Range {
    start: u64,
    end: u64,
}


impl PartialEq for Range {
    fn eq(&self, other: &Range) -> bool {
        self.start == other.start && self.end == other.end
    }
}


impl Range {
    fn overlaps(&self, input_range: &Range) -> bool {
        !(self.end <= input_range.start || input_range.end <= self.start)
    }

    fn fully_overlaps(&self, input_range: &Range) -> bool {
        self.start == input_range.start && self.end == input_range.end
    }

    fn split_overlap_nonoverlap(&self, input_range: &Range) -> (Option<Range>, Vec<Range>){
        let mut non_overlapping_ranges: Vec<Range> = Vec::new();
        if self.overlaps(input_range) {
            let start = max(self.start, input_range.start);
            let end = min(self.end, input_range.end);
            let overlapping_range = Range {
                start: start,
                end: end,
            };
            if !self.fully_overlaps(input_range) {
                if input_range.start < self.start {
                    non_overlapping_ranges.push(
                        Range {
                            start: input_range.start,
                            end: self.start,
                        }
                    );
                }
                if input_range.end > self.end {
                    non_overlapping_ranges.push(
                        Range {
                            start: self.end,
                            end: input_range.end,
                        }
                    );
                }
            }
            return (Some(overlapping_range), non_overlapping_ranges)
        } else {
            non_overlapping_ranges.push(*input_range);
            return (None, non_overlapping_ranges)
        }
    }
}


fn map_overlapping_range(overlapped_input_range: &Range, domain_range: &Range, image_range: &Range) -> Range {
    let start = image_range.start + overlapped_input_range.start - domain_range.start;
    let end = image_range.start + overlapped_input_range.end - domain_range.start;
    Range {
        start: start,
        end: end,
    }
}


fn map_ranges(map: &HashMap<Range, Range>, input_ranges: Vec<Range>) -> Vec<Range> {
    // input_ranges is the vec of split inputs at the same level
    let mut out_ranges: Vec<Range> = Vec::new();
    for input_range in input_ranges.iter() {
        let ol_range = map.iter().filter(|(x, _)| x.overlaps(&input_range)).collect::<Vec<_>>();
        if ol_range.is_empty() {
            out_ranges.push(*input_range);
        } else {
            let mut remaining_input_ranges: Vec<Range> = Vec::from([*input_range]);
            for (domain_range, image_range) in ol_range {
                // Map the range, reassign the rest back to remaining_input_ranges
                let to_map: Vec<Range> = remaining_input_ranges
                    .clone()
                    .into_iter()
                    .filter(|x| domain_range.overlaps(x))
                    .collect();
                let mut unmapped: Vec<Range> = remaining_input_ranges
                    .clone()
                    .into_iter()
                    .filter(|x| !domain_range.overlaps(x))
                    .collect();
                for tm in to_map {
                    match domain_range.split_overlap_nonoverlap(&tm) {
                        (Some(m), unmapped_ranges) => {
                            let mapped_range = map_overlapping_range(&m, &domain_range, &image_range);
                            out_ranges.push(mapped_range);
                            for unmapped_range in unmapped_ranges {
                                unmapped.push(unmapped_range);
                            }
                        },
                        (None, unmapped_ranges) => {
                            for unmapped_range in unmapped_ranges {
                                unmapped.push(unmapped_range);
                            }
                        },
                    };
                }
                remaining_input_ranges = unmapped;
            }
            for unmapped_range in remaining_input_ranges {
                out_ranges.push(unmapped_range);
            }
        }
    }

    out_ranges
}


fn map_min(maps: &Vec<HashMap<Range, Range>>, val_range: Range) -> u64 {
    // Take val_range as input range
    // For each map, produce a set of output ranges
    // chain output ranges of each map as input ranges into next map
    let mut input_ranges: Vec<Range> = Vec::from([val_range]);
    for map in maps {
        input_ranges = map_ranges(map, input_ranges);
    }
    let res: u64 = input_ranges.iter().map(|x| x.start).min().expect("Not possible");
    res
}


pub fn day5_p2(input_file: &str) -> Result<u64, io::Error> {
    let mut seeds: Vec<Range> = Vec::new();
    let mut maps: Vec<HashMap<Range, Range>> = Vec::new();
    let mut proc_map = false;
    let mut cur_map: HashMap<Range, Range> = HashMap::new();
    for line in fs::read_to_string(input_file)?.lines() {
        if line.contains("seeds") {
            seeds = process_seeds_2(line);
        } else if line.contains("map") {
            cur_map = HashMap::new();
            proc_map = true;
        } else if proc_map && line != "" {
            let (src_interval, dst_interval):
                (Range, Range) = process_map_line_2(line);
            cur_map.insert(src_interval, dst_interval);
        } else if !cur_map.is_empty() {
            maps.push(cur_map.clone());
        }
    }
    // No last newline so we gotta manually do this for the last map
    maps.push(cur_map.clone());

    let smallest: u64 = seeds
        .iter()
        .map(|x| map_min(&maps, *x))
        .min()
        .unwrap();
    Ok(smallest)
}

