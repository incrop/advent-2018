use std::collections::HashSet;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::io::prelude::*;

pub fn challenge1<R: Read>(reader: BufReader<R>) -> Result<()> {
    let mut sum = 0i32;
    for line in reader.lines() {
        sum += line?.parse::<i32>().unwrap();
    }
    println!("Answer: {}", sum);
    Ok(())
}

pub fn challenge2<R: Read>(reader: BufReader<R>) -> Result<()> {
    let deltas: Vec<i32> = reader.lines()
            .map(|line| line.unwrap().parse::<i32>().unwrap())
            .collect();
    let mut known = HashSet::new();
    let mut freq = 0;
    known.insert(freq);
    loop {
        for delta in &deltas {
            freq += delta;
            if known.contains(&freq) {
                println!("Answer: {}", freq);
                return Ok(());
            }
            known.insert(freq);
        }
    }
}
