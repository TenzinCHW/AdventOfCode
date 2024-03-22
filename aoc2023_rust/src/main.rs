mod day1;
//mod day2;
//mod day3;
//mod day4;
//mod day5;
//mod day6;
//mod day7;
//mod day8;
//mod day9;
mod day10;
//mod day11;
//mod day15;


fn main() {
    //let hi: &str = "heeeey";
    //let hii: &str = &hi[0..1];
    //let meow: &str = "grr";
    //let hello = "".to_owned() + hi + meow; //format!("{hii} 5");
    //println!("{hello}, world!");

    //for c in meow.chars().rev() {
    //    println!("{c}");
    //}
    //println!("{}", type_of(meow.chars().rev()));

    //println!("{}", '4'.to_digit(10).expect("just no"));

    //let mut v: Vec<i32> = Vec::new();
    //let a = v.pop();
    //match a {
    //    Some(item) => {
    //        println!("Found something {item}");
    //    },
    //    _ => {
    //        println!("rekt");
    //    }
    //};

    //// Day 1
    //let input_file = "../data/day1.txt";
    //let ans = match day1::day1_p1(input_file) {
    //    Ok(m) => m,
    //    _ => panic!("GG rekt")
    //};
    //println!("{ans}");
    //let ans = day1::day1_p2(input_file).unwrap();
    //println!("{ans}");

    //// Day 2
    //let input_file = "../data/day2.txt";
    //let ans = day2::day2_p2(input_file).unwrap();
    //println!("{ans}");

    //// Day 3
    //let input_file = "../data/day3.txt";
    //let ans = day3::day3_p1(input_file).unwrap();
    //println!("{ans}");

    //// Day 4
    //let input_file = "../data/day4.txt";
    //let ans = day4::day4_p2(input_file).unwrap();
    //println!("{ans}");

    //// Day 5
    //let input_file = "../data/day5.txt";
    //let ans = day5::day5_p2(input_file).unwrap();
    //println!("{ans}");

    //// Day 6
    //let input_file = "../data/day6.txt";
    //let ans = day6::day6_p2(input_file).unwrap();
    //println!("{ans}");

    //// Day 7
    //let input_file = "../data/day7.txt";
    //let ans = day7::day7_p2(input_file).unwrap();
    //println!("{ans}");

    //// Day 8
    //let input_file = "../data/day8.txt";
    //let ans = day8::day8_p2(input_file).unwrap();
    //println!("{ans}");

    //// Day 9
    //let input_file = "../data/day9.txt";
    //let ans = day9::day9_p2(input_file).unwrap();
    //println!("{ans}");

    //// Day 10
    let input_file = "../data/ex_day10.txt";
    let ans = day10::day10_p1(input_file).unwrap();
    println!("{ans}");

    //// Day 11
    //let input_file = "../data/day11.txt";
    //let ans = day11::day11_p2(input_file).unwrap();
    //println!("{ans}");

    //// Day 15
    //let input_file = "../data/day15.txt";
    //let ans = day15::day15_p2(input_file).unwrap();
    //println!("{ans}");
}
