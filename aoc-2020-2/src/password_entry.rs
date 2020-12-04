#[derive(Debug)]
pub struct PasswordEntry {
    pub max_occurrences: i32,
    pub min_occurrences: i32,
    pub character: char,
    pub content: String,
}

impl PasswordEntry {
    pub fn from_string(string: String) -> PasswordEntry {
        let initial_split = string.split(" ").collect::<Vec<&str>>();
        let min: i32 = initial_split[0].split("-").collect::<Vec<&str>>()[0].parse().unwrap();
        let max: i32 = initial_split[0].split("-").collect::<Vec<&str>>()[1].parse().unwrap();
        let char: char = initial_split[1].chars().nth(0).unwrap();
        let content_text: String = String::from(initial_split[2]);

        return PasswordEntry {
            max_occurrences: max,
            min_occurrences: min,
            character: char,
            content: content_text,
        };
    }
}