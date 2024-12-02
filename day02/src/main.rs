use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

/**
The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a list of numbers called levels that are separated by spaces. For example:

7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9

This example data contains six reports each containing five levels.

The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:

    The levels are either all increasing or all decreasing.
    Any two adjacent levels differ by at least one and at most three.

In the example above, the reports can be found safe or unsafe by checking those rules:

    7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
    1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
    9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
    1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
    8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
    1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.

So, in this example, 2 reports are safe.

Analyze the unusual data from the engineers. How many reports are safe?
*/
const ABS_DIFF: u8 = 3;
fn main() {
    let file = File::open("input.txt").expect("File should be openable.");
    let reader = BufReader::new(file);
    let mut num_safe_reports: u32 = 0;
    'report: for line in reader.lines() {
        let line = line.unwrap();
        let report = line
            .split_whitespace()
            .map(|num| num.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let mut report = report.iter();

        let first_level: &u8 = report.next().expect("Report should not be empty");
        let mut prev_level: &u8 = report.next().expect("Report should have two entries");
        let expected_ordering = first_level.cmp(prev_level);
        if expected_ordering == Ordering::Equal || first_level.abs_diff(*prev_level) > ABS_DIFF {
            continue;
        }

        while let Some(current_level) = report.next() {
            let ordering = prev_level.cmp(&current_level);
            if ordering != expected_ordering || prev_level.abs_diff(current_level.clone()) > ABS_DIFF {
                continue 'report;
            } else {
                prev_level = current_level;
            }
        }
        num_safe_reports += 1;
    }

    println!("Safe reports: {num_safe_reports}");
}
