use std::collections::HashMap;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::io::prelude::*;

pub fn challenge1<R: Read>(reader: BufReader<R>) -> Result<()> {
    let mut twos = 0;
    let mut threes = 0;
    for line in reader.lines() {
        let mut counts = HashMap::new();
        for chr in line?.chars() {
            let cnt = counts.entry(chr).or_insert(0);
            *cnt += 1;
        }
        let mut inc_two = 0;
        let mut inc_three = 0;
        for count in counts.values() {
            if *count == 2 {
                inc_two = 1;
            } else if *count == 3 {
                inc_three = 1;
            }
        }
        twos += inc_two;
        threes += inc_three;
    }
    println!("Answer: {}", twos * threes);
    Ok(())
}

pub fn challenge2<R: Read>(reader: BufReader<R>) -> Result<()> {
    let mut known: Vec<String> = Vec::new();
    for line1 in reader.lines() {
        let line1 = line1?;
        'outer: for line2 in known.iter() {
            let mut diff_idx = None;
            for (idx, (c1, c2)) in line1.chars().zip(line2.chars()).enumerate() {
                if c1 == c2 {
                    continue;
                }
                if diff_idx.is_some() {
                    continue 'outer;
                }
                diff_idx = Some(idx);
            }
            if let Some(idx) = diff_idx {
                println!("Answer: {}{}", &line1[..idx], &line1[idx+1..]);
                return Ok(());
            }
        }
        known.push(line1);
    }
    Ok(())
}
