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

--- Part Two ---

As you scan through the corrupted memory, you notice that some of the conditional statements are also still intact. If you handle some of the uncorrupted conditional statements in the program, you might be able to get an even more accurate result.

There are two new instructions you'll need to handle:

    The do() instruction enables future mul instructions.
    The don't() instruction disables future mul instructions.

Only the most recent do() or don't() instruction applies. At the beginning of the program, mul instructions are enabled.

For example:

xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))

This corrupted memory is similar to the example from before, but this time the mul(5,5) and mul(11,8) instructions are disabled because there is a don't() instruction before them. The other mul instructions function normally, including the one at the end that gets re-enabled by a do() instruction.

This time, the sum of the results is 48 (2*4 + 8*5).

Handle the new instructions; what do you get if you add up all of the results of just the enabled multiplications?

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
    let r: Regex = Regex::new(
        r"(?<do>do)\(\)|(?<dont>don't)\(\)|(?<mul>mul)\((?<mul_a>\d{1,3}),(?<mul_b>\d{1,3})\)",
    )
    .unwrap(); // learn about custom error types
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    let mut results = Vec::<Multiplier>::new();
    let mut should_add: bool = true;

    for line in buf.lines() {
        let line = line?;
        let captures_iter = r.captures_iter(line.as_str());
        for capture in captures_iter {
            if capture.name("do").is_some() {
                should_add = true;
            } else if capture.name("dont").is_some() {
                should_add = false;
            } else if capture.name("mul").is_some() {
                // nest to avoid logging "unknown" when in "don't" mode:
                if should_add {
                    let a: i16 = capture.name("mul_a").unwrap().as_str().parse().unwrap();
                    let b: i16 = capture.name("mul_b").unwrap().as_str().parse().unwrap();
                    results.push(Multiplier { a, b });
                }
            } else {
                eprintln!("unknown capture group");
            }
        }
    }
    Ok(results)
}
