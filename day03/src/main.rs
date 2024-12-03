use regex::Regex;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

/**
"Our computers are having issues, so I have no idea if we have any Chief Historians in stock! You're
welcome to check the warehouse, though," says the mildly flustered shopkeeper at the North Pole
Toboggan Rental Shop. The Historians head out to take a look.

The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"

The computer appears to be trying to run a program, but its memory (your puzzle input) is corrupted.
All of the instructions have been jumbled up!

It seems like the goal of the program is just to multiply some numbers. It does that with
instructions like mul(X,Y), where X and Y are each 1-3 digit numbers. For instance, mul(44,46)
multiplies 44 by 46 to get a result of 2024. Similarly, mul(123,4) would multiply 123 by 4.

However, because the program's memory has been corrupted, there are also many invalid characters
that should be ignored, even if they look like part of a mul instruction. Sequences like
mul(4*, mul(6,9!, ?(12,34), or mul ( 2 , 4 ) do nothing.

For example, consider the following section of corrupted memory:

xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))

Only the four highlighted sections are real mul instructions. Adding up the result of each
instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).

Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the
results of the multiplications?
*/
struct Multiplier {
    a: i16,
    b: i16,
}

impl Multiplier {
    fn product(&self) -> i32 {
        self.a as i32 * self.b as i32
    }
}

fn main() {
    match parse_multipliers("input.txt") {
        Ok(multipliers) => {
            let sum: i32 = multipliers.iter().map(Multiplier::product).sum();
            println!("Sum of products: {sum}");
        }
        Err(e) => {
            eprintln!("error parsing multipliers: {e}")
        }
    }
}

fn parse_multipliers(filename: &str) -> io::Result<Vec<Multiplier>> {
    let r: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap(); // learn about custom error types
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    let mut results = Vec::<Multiplier>::new();

    for line in buf.lines() {
        let line = line?;
        for (_, [a, b]) in r.captures_iter(line.as_str()).map(|c| c.extract()) {
            let a: i16 = a.parse().unwrap(); // learn about custom error types
            let b: i16 = b.parse().unwrap(); // learn about custom error types
            results.push(Multiplier { a, b });
        }
    }
    Ok(results)
}
