use std::collections::HashMap;
use std::env;
use std::io::{BufReader, Result};
use std::fs::File;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;

fn main() -> Result<()> {
    let arg = read_argument();
    let challenges = init_challenges();
    let (input, challenge) = challenges.get(&arg).expect("Unexpected argument. Expected: day[1-25]_[1-2]");
    let file = File::open(format!("input/{}", input))?;
    let buf_reader = BufReader::new(file);
    challenge(buf_reader)
}

type InputProcessor = fn(BufReader<File>) -> Result<()>;

fn init_challenges() -> HashMap<String, (String, InputProcessor)> {
    let mut c: HashMap<String, (String, InputProcessor)> = HashMap::new();
    c.insert("day1_1".to_owned(), ("day1.txt".to_owned(), day1::challenge1));
    c.insert("day1_2".to_owned(), ("day1.txt".to_owned(), day1::challenge2));
    c.insert("day2_1".to_owned(), ("day2.txt".to_owned(), day2::challenge1));
    c.insert("day2_2".to_owned(), ("day2.txt".to_owned(), day2::challenge2));
    c.insert("day3_1".to_owned(), ("day3.txt".to_owned(), day3::challenge1));
    c.insert("day3_2".to_owned(), ("day3.txt".to_owned(), day3::challenge2));
    c.insert("day4_1".to_owned(), ("day4.txt".to_owned(), day4::challenge1));
    c.insert("day4_2".to_owned(), ("day4.txt".to_owned(), day4::challenge2));
    c.insert("day5_1".to_owned(), ("day5.txt".to_owned(), day5::challenge1));
    c.insert("day5_2".to_owned(), ("day5.txt".to_owned(), day5::challenge2));
    c.insert("day6_1".to_owned(), ("day6.txt".to_owned(), day6::challenge1));
    c.insert("day6_2".to_owned(), ("day6.txt".to_owned(), day6::challenge2));
    c.insert("day7_1".to_owned(), ("day7.txt".to_owned(), day7::challenge1));
    c.insert("day7_2".to_owned(), ("day7.txt".to_owned(), day7::challenge2));
    c.insert("day8_1".to_owned(), ("day8.txt".to_owned(), day8::challenge1));
    c.insert("day8_2".to_owned(), ("day8.txt".to_owned(), day8::challenge2));
    c.insert("day9_1".to_owned(), ("day9_1.txt".to_owned(), day9::challenge));
    c.insert("day9_2".to_owned(), ("day9_2.txt".to_owned(), day9::challenge));
    c.insert("day10_1".to_owned(), ("day10.txt".to_owned(), day10::challenge1));
    c.insert("day10_2".to_owned(), ("day10.txt".to_owned(), day10::challenge2));
    c.insert("day11_1".to_owned(), ("day11.txt".to_owned(), day11::challenge1));
    c.insert("day11_2".to_owned(), ("day11.txt".to_owned(), day11::challenge2));
    c.insert("day12_1".to_owned(), ("day12.txt".to_owned(), day12::challenge1));
    c.insert("day12_2".to_owned(), ("day12.txt".to_owned(), day12::challenge2));
    c.insert("day13_1".to_owned(), ("day13.txt".to_owned(), day13::challenge1));
    c.insert("day13_2".to_owned(), ("day13.txt".to_owned(), day13::challenge2));
    c
}

fn read_argument() -> String {
    env::args().nth(1).expect("Missing argument. Expected: day[1-25]_[1-2]")
}
