mod password_entry;

use std::fs::File;
use std::io::{BufRead, BufReader};
use password_entry::PasswordEntry;

fn main() {
    let lines: Vec<String> = read_lines_from_file("Input.txt");
    let entries: Vec<PasswordEntry> = lines.iter().map(|line| PasswordEntry::from_string(line.to_owned())).collect();

    solve_part_one(&entries);
    solve_part_two(&entries);
}

fn read_lines_from_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap_or_else(|err| panic!("Failed to open file: {}", err));

    return BufReader::new(file)
        .lines()
        .map(|result| result.unwrap_or_else(|err| panic!("Failed to read line: {}", err)))
        .collect();
}

fn solve_part_one(entries: &Vec<PasswordEntry>) {
    println!("--- Part One ---");

    let results: Vec<&PasswordEntry> = entries.iter().filter(|entry| {
        let occurrences: i32 = entry.content.matches(entry.character).count() as i32;
        occurrences <= entry.max_occurrences && occurrences >= entry.min_occurrences
    }).collect();

    println!("Found {} results", results.len());
}

fn solve_part_two(entries: &Vec<PasswordEntry>) {
    println!("--- Part Two ---");

    let results: Vec<&PasswordEntry> = entries.iter().filter(|entry| {
        let first_char: char = entry.content.chars().nth(entry.min_occurrences as usize - 1).unwrap();
        let second_char: char = entry.content.chars().nth(entry.max_occurrences as usize - 1).unwrap();

        if first_char == second_char { false } else {
            first_char == entry.character || second_char == entry.character
        }
    }).collect();

    println!("Found {} results", results.len());
}
