use std::fs::File;
use std::io::{BufRead, BufReader};

// BRUTE FORCE IS THE ONLY SOLUTION YOU"LL EVER NEED
fn main() {
    let file = File::open("Input.txt").unwrap_or_else(|err| panic!("Failed to open file: {}", err));

    let lines: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|result| result.unwrap_or_else(|err| panic!("Failed to read line: {}", err)))
        .map(|line| {
            line.parse::<i32>()
                .unwrap_or_else(|err| panic!("Failed to parse line: {}, {}", line, err))
        })
        .collect();

    solve_part_one(&lines);
    solve_part_two(&lines);
}

fn solve_part_one(numbers: &Vec<i32>) {
    println!("--- Part One ---");
    for element_one in numbers.iter() {
        for element_two in numbers.iter() {
            if element_one + element_two == 2020 {
                println!(
                    "({} + {} = 2020, {} * {} = {})",
                    element_one,
                    element_two,
                    element_one,
                    element_two,
                    element_one * element_two
                );
                return;
            }
        }
    }
}

// This was genuinely the most fun with code I've had in a very long time
fn solve_part_two(numbers: &Vec<i32>) {
    println!("--- Part Two ---");
    for element_one in numbers.iter() {
        for element_two in numbers.iter() {
            for element_three in numbers.iter() {
                if element_one + element_two + element_three == 2020 {
                    println!(
                        "({} + {} + {} = 2020, {} * {} * {} = {})",
                        element_one,
                        element_two,
                        element_three,
                        element_one,
                        element_two,
                        element_three,
                        element_one * element_two * element_three
                    );
                    return;
                }
            }
        }
    }
}
