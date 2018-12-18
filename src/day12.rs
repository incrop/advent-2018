extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::io::{BufReader, Read, Result};
use std::io::prelude::*;
use std::iter::repeat;

use self::lazy_static::lazy_static;
use self::regex::Regex;

struct State {
    shift: i64,
    pots: Vec<bool>
}

pub fn challenge1<R: Read>(reader: BufReader<R>) -> Result<()> {
    let mut lines = reader.lines();
    let mut state = read_state(&lines.next().unwrap()?);
    lines.next();
    let rules = read_rules(lines);
    print_state(&state);
    for _ in 0..20 {
        evolve(&rules, &mut state);
        print_state(&state);
    }
    println!("Answer: {}", sum_indices(&state));
    Ok(())
}

pub fn challenge2<R: Read>(reader: BufReader<R>) -> Result<()> {
    let mut lines = reader.lines();
    let mut state = read_state(&lines.next().unwrap()?);
    lines.next();
    let rules = read_rules(lines);
    for i in 0u64..50_000_000_000 {
        if i % 100_000_000 == 0 {
            println!("After {}: {}", i, sum_indices(&state));
        }
        evolve(&rules, &mut state);
    }
    println!("Answer: {}", sum_indices(&state));
    Ok(())
}

fn read_state(line: &str) -> State {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"initial state: (?P<pots>[.#]+)").unwrap();
    }
    let wrap = "....";
    let content = RE.captures(&line).unwrap()
        .name("pots").unwrap()
        .as_str();
    State {
        shift: wrap.len() as i64,
        pots: wrap.chars()
            .chain(content.chars())
            .chain(wrap.chars())
            .map(pot_to_bool)
            .collect(),
    }
}

fn pot_to_bool(c: char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        c => panic!("Unexpected char {}", c),
    }
}

type Rules = HashMap<Vec<bool>, bool>;

fn read_rules<L: Iterator<Item=Result<String>>>(lines: L) -> Rules {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<from>[.#]{5}) => (?P<to>[.#])").unwrap();
    }
    let mut rules = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        let cap = RE.captures(&line).unwrap();
        let from = cap.name("from").unwrap()
            .as_str()
            .chars()
            .map(pot_to_bool)
            .collect();
        let to = cap.name("to").unwrap()
            .as_str()
            .chars()
            .map(pot_to_bool)
            .next().unwrap();
        rules.insert(from, to);
    }
    rules
}

fn evolve(rules: &Rules, state: &mut State) {
    let first = state.pots.iter().position(|p| *p).unwrap();
    let last = state.pots.iter().rev().position(|p| *p).unwrap();
    let next_pots = state.pots[first-4..state.pots.len()-last+4]
        .windows(5)
        .map(|w| *rules.get(w).unwrap_or(&false));
    state.shift += 6 - first as i64;
    state.pots = repeat(false).take(4)
        .chain(next_pots)
        .chain(repeat(false).take(4))
        .collect();
}

fn print_state(state: &State) {
    for _ in 0..40-state.shift {
        print!(" ");
    }
    for p in state.pots.iter() {
        if *p {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();
}

fn sum_indices(state: &State) -> i64 {
    let mut sum = 0;
    for (idx, p) in state.pots.iter().enumerate() {
        if *p {
            sum += idx as i64 - state.shift;
        }
    }
    sum
}
