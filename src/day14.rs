use std::io::{BufReader, Read, Result};

pub fn challenge1<R: Read>(mut reader: BufReader<R>) -> Result<()> {
    let mut line = String::new();
    reader.read_to_string(&mut line).unwrap();
    let threshold: usize = line.trim().parse().unwrap();
    let mut recipies: Vec<u8> = vec![3, 7];
    let mut e1 = 0;
    let mut e2 = 1;
    while recipies.len() < threshold + 10 {
        let sum = recipies[e1] + recipies[e2];
        if sum < 10 {
            recipies.push(sum);
        } else {
            recipies.push(1);
            recipies.push(sum % 10);
        }
        e1 = (e1 + 1 + recipies[e1] as usize) % recipies.len();
        e2 = (e2 + 1 + recipies[e2] as usize) % recipies.len();
    }
    let result: Vec<String> = recipies[threshold..threshold+10].iter().map(|r| r.to_string()).collect();
    println!("Answer: {}", result.join(""));
    Ok(())
}

pub fn challenge2<R: Read>(mut reader: BufReader<R>) -> Result<()> {
    let mut line = String::new();
    reader.read_to_string(&mut line).unwrap();
    let sequence = parse_sequence(&line);
    println!("Answer: {}", find_sequence_idx(&sequence));
    Ok(())
}

fn find_sequence_idx(sequence: &[u8]) -> usize {
    let mut recipies: Vec<u8> = vec![3, 7];
    let mut e1 = 0;
    let mut e2 = 1;
    let mut idx = 0;
    loop {
        while recipies.len() < idx + sequence.len() {
            let sum = recipies[e1] + recipies[e2];
            if sum < 10 {
                recipies.push(sum);
            } else {
                recipies.push(1);
                recipies.push(sum % 10);
            }
            e1 = (e1 + 1 + recipies[e1] as usize) % recipies.len();
            e2 = (e2 + 1 + recipies[e2] as usize) % recipies.len();
        }
        if recipies[idx..idx + sequence.len()] == *sequence {
            return idx;
        }
        idx += 1;
    }
}

fn parse_sequence(line: &str) -> Vec<u8> {
    line.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}
