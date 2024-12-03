use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let (mut vec1, mut vec2) = read_input("input.txt").expect("Should have opened and parsed file");

    let distance: u32 = total_distance(&mut vec1, &mut vec2);
    let sim_score = similarity_score(&vec1, &vec2);

    println!("Total difference: {distance}");
    println!("Similarity score: {sim_score}");
}

fn read_input(path: &str) -> io::Result<(Vec<u32>, Vec<u32>)> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut vec1: Vec<u32> = Vec::new();
    let mut vec2: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<u32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<u32>().ok())
            .collect();

        vec1.push(parts[0]);
        vec2.push(parts[1]);
    }

    Ok((vec1, vec2))
}

fn total_distance(vec1: &mut [u32], vec2: &mut [u32]) -> u32 {
    vec1.sort();
    vec2.sort();

    vec1.iter()
        .zip(vec2)
        .map(|(one, two)| one.abs_diff(*two))
        .sum()
}

fn similarity_score(vec1: &[u32], vec2: &[u32]) -> u32 {
    let unique_in_one: HashSet<u32> = HashSet::from_iter(vec1.iter().cloned());
    let occurrences_in_two: HashMap<&u32, u32> = vec2.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    unique_in_one
        .iter()
        .map(|num| num * occurrences_in_two.get(num).unwrap_or(&0))
        .sum()
}
