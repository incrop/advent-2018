use std::collections::HashSet;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;

type Polymer = Vec<char>;
type Polymery = [char];

pub fn challenge1<R: Read>(reader: BufReader<R>) -> Result<()> {
    let vec = read_polymer(reader);
    println!("Answer: {}", vec.len());
    Ok(())
}

fn read_polymer<R: Read>(reader: BufReader<R>) -> Polymer {
    let mut poly = Vec::new();
    for curr in reader.bytes() {
        let curr = curr.unwrap() as char;
        if !curr.is_ascii_alphabetic() {
            continue;
        }
        if let Some(last) = poly.pop() {
            if curr.eq_ignore_ascii_case(&last) && curr != last {
                continue;
            }
            poly.push(last);
        }
        poly.push(curr);
    }
    poly
}

pub fn challenge2<R: Read>(reader: BufReader<R>) -> Result<()> {
    let orig = read_polymer(reader);
    let mut best = orig.clone();
    for kind in distinct_kinds(&orig) {
        let upd = remove_kind(kind, &orig);
        if upd.len() < best.len() {
            best = upd;
        }
    }
    println!("Answer: {}", best.len());
    Ok(())
}

fn distinct_kinds(polymer: &Polymery) -> HashSet<char> {
    let mut kinds = HashSet::new();
    for unit in polymer {
        kinds.insert(unit.to_ascii_lowercase());
    }
    kinds
}

fn remove_kind(kind: char, orig: &Polymery) -> Polymer {
    let mut poly: Polymer = Vec::new();
    for curr in orig {
        if kind.eq_ignore_ascii_case(curr) {
            continue;
        }
        if let Some(last) = poly.pop() {
            if last.eq_ignore_ascii_case(curr) && *curr != last {
                continue;
            }
            poly.push(last);
        }
        poly.push(*curr);
    }
    poly
}
