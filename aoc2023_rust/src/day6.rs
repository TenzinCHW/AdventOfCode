// v = h
// d = (t - h) * v = h * (t - h)
// d is quadratic in h
// d = h * (t - h)
// where d is the record r, we need to win by 1
// -h^2 + th = r + 1
// h^2 - th + r + 1 = 0
// h_0 = (t - sqrt(t^2 - 4(r+1))) / 2
// h_1 = (t + sqrt(t^2 - 4(r+1))) / 2
// Hold for between ceil(h_0) and floor(h1)

use std::fs;
use std::io;
use std::iter::zip;


fn str2u64_arr(line: String, header: &str, on_error: &str) -> Vec<u64> {
    let values: Vec<u64> = line
        .split_whitespace()
        .filter(|x| *x != header)
        .map(|x| x.parse::<u64>().expect(on_error))
        .collect();
    values
}


fn process_inputs_1(lines: Vec<String>) -> (Vec<u64>, Vec<u64>) {
    let timings: Vec<u64> = str2u64_arr(lines[0].to_string(), "Time:", "Timings not in the right format");
    let record_distances: Vec<u64> = str2u64_arr(lines[1].to_string(), "Distance:", "Distances not in the right format");
    (timings, record_distances)
}


fn solve_hold_time(timing: u64, distance: u64) -> (u64, u64) {
    // h_0 = (t +- sqrt(t^2 - 4(r+1))) / 2
    let t: f64 = timing as f64;
    let r: f64 = distance as f64;
    let h_0 = (t - (t.powi(2) - 4. * (r + 1.)).sqrt()) / 2.;
    let h_1 = (t + (t.powi(2) - 4. * (r + 1.)).sqrt()) / 2.;
    (h_0.ceil() as u64, h_1.floor() as u64)
}


pub fn day6_p1(input_file: &str) -> Result<u64, io::Error> {
    let lines: Vec<String> = fs::read_to_string(input_file)?.lines().map(String::from).collect();
    let (timings, record_distances) = process_inputs_1(lines);
    let mut prod: u64 = 1;
    for (timing, record_distance) in zip(timings, record_distances) {
        let (h_0, h_1): (u64, u64) = solve_hold_time(timing, record_distance);
        println!("{h_0} {h_1}");
        println!("{}", h_1 - h_0 + 1);
        prod *= h_1 - h_0 + 1;
    }
    Ok(prod)
}


fn str2u64_stripspace(line: String, header: &str) -> u64 {
    let value_str: Vec<&str> = line
        .split_whitespace()
        .filter(|x| *x != header)
        .collect();
    let mut value_full: String = "".to_owned();
    for val in value_str {
        value_full.push_str(val);
    }
    value_full.parse::<u64>().expect("Should work")
}


fn process_inputs_2(lines: Vec<String>) -> (u64, u64) {
    let timing: u64 = str2u64_stripspace(lines[0].to_string(), "Time:");
    let distance: u64 = str2u64_stripspace(lines[1].to_string(), "Distance:");
    (timing, distance)
}

pub fn day6_p2(input_file: &str) -> Result<u64, io::Error> {
    let lines: Vec<String> = fs::read_to_string(input_file)?.lines().map(String::from).collect();
    let (timing, distance): (u64, u64) = process_inputs_2(lines);
    let (h0, h1) = solve_hold_time(timing, distance);
    Ok(h1 - h0 + 1)
}
