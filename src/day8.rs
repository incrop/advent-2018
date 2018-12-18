use std::io::{BufReader, Read, Result};

struct Tree {
    children: Vec<Tree>,
    metadata: Vec<usize>,
}

pub fn challenge1<R: Read>(reader: BufReader<R>) -> Result<()> {
    let tree = read_tree(reader);
    println!("Answer: {}", sum_metadata(&tree));
    Ok(())
}

fn read_tree<R: Read>(mut reader: BufReader<R>) -> Tree {
    let mut line = String::new();
    reader.read_to_string(&mut line).unwrap();
    tree_from_digits(&mut line.split(' ').map(|d| d.trim().parse().unwrap()))
}

fn tree_from_digits<I: Iterator<Item=usize>>(digits: &mut I) -> Tree {
    let c_len = digits.next().unwrap();
    let m_len = digits.next().unwrap();
    let mut children = Vec::new();
    for _ in 0..c_len {
        children.push(tree_from_digits(digits));
    }
    let mut metadata = Vec::new();
    for _ in 0..m_len {
        metadata.push(digits.next().unwrap());
    }
    Tree {children, metadata}
}

fn sum_metadata(tree: &Tree) -> usize {
    let c_sum: usize = tree.children.iter().map(sum_metadata).sum();
    let m_sum: usize = tree.metadata.iter().sum();
    c_sum + m_sum
}

pub fn challenge2<R: Read>(reader: BufReader<R>) -> Result<()> {
    let tree = read_tree(reader);
    println!("Answer: {}", node_value(&tree));
    Ok(())
}

fn node_value(tree: &Tree) -> usize {
    if tree.children.is_empty() {
        return tree.metadata.iter().sum();
    }
    tree.metadata.iter()
        .map(|idx| tree.children
            .get(idx - 1)
            .map(node_value)
            .unwrap_or(0))
        .sum()
}
