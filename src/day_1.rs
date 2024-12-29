use std::fs;
use std::collections::HashMap;

pub fn solve_1() -> String {
    let mut col1: Vec<u32> = Vec::new();
    let mut col2: Vec<u32> = Vec::new();

    let content = fs::read_to_string("./input/day_1_puzzle_1.txt").expect("Should have been able to read the file");

    let mut content: Vec<Vec<String>> = content.split("\n")
                                        .map(|x| x.split_whitespace().map(str::to_string).collect())
                                        .collect();

    // I can't figure out a nicer way than to make content mutable and pop
    content.pop();

    for line in &content[..]{
        col1.push(line[0].parse().unwrap());
        col2.push(line[1].parse().unwrap());
    }

    col1.sort();
    col2.sort();

    let mut sum = 0;
    for values in col1.into_iter().zip(col2.into_iter()) {
        sum += if values.0 > values.1 {values.0 - values.1} else {values.1 - values.0};
    }

    sum.to_string()
}

pub fn solve_2() -> String {
    let mut col1: Vec<u32> = Vec::new();
    let mut col2: HashMap<u32, u32> = HashMap::new();

    let content = fs::read_to_string("./input/day_1_puzzle_1.txt").expect("Should have been able to read the file");

    let mut content: Vec<Vec<String>> = content.split("\n")
                                        .map(|x| x.split_whitespace().map(str::to_string).collect())
                                        .collect();

    // I can't figure out a nicer way than to make content mutable and pop
    content.pop();

    for line in &content[..]{
        col1.push(line[0].parse().unwrap());
        let count = col2.entry(line[1].parse().unwrap()).or_insert(0);
        *count += 1;
    }

    let mut sim_score = 0;

    for loc in col1 {
        sim_score += loc * col2.get(&loc).copied().unwrap_or(0);
    }

    sim_score.to_string()
}