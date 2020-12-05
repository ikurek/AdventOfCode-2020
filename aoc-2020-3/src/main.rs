use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    let lines: Vec<String> = read_lines_from_file("Input.txt");
    let map_height = lines.len();
    // Tobogan goes one down and 3 right, so this has to be
    // at least three times larger than height
    // And 7 for second part
    let desired_map_width = map_height * 7;
    let map: Vec<Vec<char>> = generate_complete_map(&lines, &map_height, &desired_map_width);

    //    for row in map {
    //         for char in row {
    //             print!("{}", char);
    //         }
    //         println!();
    //     }

    solve_part_one(&map);
    solve_part_two(&map);
}

fn read_lines_from_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap_or_else(|err| panic!("Failed to open file: {}", err));

    return BufReader::new(file)
        .lines()
        .map(|result| result.unwrap_or_else(|err| panic!("Failed to read line: {}", err)))
        .collect();
}

fn generate_complete_map(lines: &Vec<String>, map_height: &usize, map_width: &usize) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::new();

    for i in 0..*map_height {
        let mut expanded_map_row: Vec<char> = Vec::new();
        while expanded_map_row.len() <= *map_width {
            lines[i].chars().for_each(|char| expanded_map_row.push(char))
        }
        output.push(expanded_map_row);
    }

    return output;
}

fn solve_part_one(map: &Vec<Vec<char>>) {
    println!("--- Part One ---");
    println!("Map size is {}x{}", map.len(), map[0].len());

    let result = solve_for_path_modifier(map, 3, 1);

    println!("Made {} steps, found {} dots and {} hashes", result.0, result.1, result.2);
}

fn solve_part_two(map: &Vec<Vec<char>>) {
    println!("--- Part Two ---");
    println!("Map size is {}x{}", map.len(), map[0].len());

    let modifiers: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let results: Vec<(&(usize, usize), (i32, i32, i32))> = modifiers.iter().map(|modifier| (modifier, solve_for_path_modifier(map, modifier.0, modifier.1))).collect();

    let mut multiplied_results: i64 = 1;

    for result in results {
        multiplied_results = multiplied_results * result.1.2 as i64;
        println!("Modifier {:?}, made {} steps, found {} dots and {} hashes", result.0, result.1.0, result.1.1, result.1.2);
    }

    println!("Multiplied hash count: {}", multiplied_results);
}

fn solve_for_path_modifier(map: &Vec<Vec<char>>, x_mod: usize, y_mod: usize) -> (i32, i32, i32) {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut steps: i32 = 0;
    let mut dot_count: i32 = 0;
    let mut hash_count: i32 = 0;

    while y < map.len() {
        match map[y][x] {
            '.' => { dot_count = dot_count + 1 }
            '#' => { hash_count = hash_count + 1 }
            _ => { panic!("Unrecognised character in map!") }
        }
        x = x + x_mod;
        y = y + y_mod;
        steps = steps + 1;
    }
    return (steps, dot_count, hash_count);
}