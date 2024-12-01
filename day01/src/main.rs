use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Should have been able to open the file");
    let reader = BufReader::new(file);

    let mut vec1: Vec<u32> = Vec::new();
    let mut vec2: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let parts: Vec<u32> = line
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();

        vec1.push(parts[0]);
        vec2.push(parts[1]);
    }

    vec1.sort();
    vec2.sort();

    let total_distance: u32 = vec1
        .iter()
        .zip(vec2)
        .map(|(one, two)| one.abs_diff(two))
        .sum();

    println!("Total difference: {total_distance}");
}
