use std::collections::{HashMap, HashSet};
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
        .zip(vec2.clone())
        .map(|(one, two)| one.abs_diff(two))
        .sum();

    let sim_score = similarity_score(vec1, vec2);

    println!("Total difference: {total_distance}");
    println!("Similarity score: {sim_score}");
}

fn similarity_score(vec1: Vec<u32>, vec2: Vec<u32>) -> u32 {
    let unique_in_one: HashSet<u32> = HashSet::from_iter(vec1.iter().cloned());
    let occurrences_in_two: HashMap<u32, u32> =
        vec2.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(*c).or_insert(0) += 1;
            acc
        });

    unique_in_one
        .iter()
        .map(|num| *num * occurrences_in_two.get(num).copied().unwrap_or(0))
        .sum()
}
