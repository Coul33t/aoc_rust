use crate::tools::read_and_split;
use std::path::Path;
use std::collections::HashMap;

pub fn day2() {
    let path = Path::new("src/data/day2.txt");
    let data = read_and_split(path, "line");

    // First win over second
    let mut winning = HashMap::<&str, &str>::new();

    winning.insert("A", "Z");
    winning.insert("B", "Y");
    winning.insert("C", "X");

    day2_1(&data, &winning);
    day2_2(&data, &winning);
}

fn day2_1(data: &Vec<String>, winning: &HashMap<&str, &str>) -> i32 {
    let mut score: i32 = 0;

    for line in data.iter() {
        let splitted: Vec<String> = line.split_whitespace().map(str::to_string).collect();
    }

    return score;
}

fn day2_2(data: &Vec<String>, winning: &HashMap<&str, &str>) {

}