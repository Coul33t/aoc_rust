use std::fs;
use std::path::Path;
use std::env;

pub fn print_cwd() {
    println!("Current working directory: {}", env::current_dir().expect("Idk").display());
}

fn read_file(path: &Path) -> String {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    return contents;
}

fn split_string(s: &str, separator: &str) -> Vec<String> {
    let splitted: Vec<String>;

    match separator.as_ref() {
        "line"  => splitted = s.lines().map(str::to_string).collect(),
        "space" => splitted = s.split_whitespace().map(str::to_string).collect(),
        _       => splitted = s.split(&separator).map(str::to_string).collect()
    }
    return splitted;
}

pub fn read_and_split(path: &Path, separator: &str) -> Vec<String> {
    let contents = read_file(path);
    let splitted = split_string(&contents, separator);
    return splitted;
}