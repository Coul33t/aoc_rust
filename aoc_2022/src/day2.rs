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
    winning.insert("X", "C");
    winning.insert("Y", "A");
    winning.insert("Z", "B");

    let mut points = HashMap::<&str, i32>::new();
    points.insert("WIN", 6);
    points.insert("TIE", 3);
    points.insert("X", 1);
    points.insert("Y", 2);
    points.insert("Z", 3);

    day2_1(&data, &winning, &points);
    day2_2(&data, &winning, &points, &must_choose);
}

fn day2_1(data: &Vec<String>, winning: &HashMap<&str, &str>, points: &HashMap::<&str, i32>) {
    let mut score: i32 = 0;

    for line in data.iter() {
        let splitted: Vec<String> = line.split_whitespace().map(str::to_string).collect();
        let imp_choice: String = splitted[0].to_string();
        let player_choice: String = splitted[1].to_string();

        if let Some(point) = points.get(&player_choice as &str) {
            score += point;
        }

        else {
            println!("NOT FOUND: {}", player_choice);
        }

        if let Some(val) = winning.get(&player_choice as &str) {
            if val == &imp_choice.as_str() {
                score += points.get("WIN").unwrap(); 
            }

            else if imp_choice.as_bytes()[0] == player_choice.as_bytes()[0] - 23 {
                score += points.get("TIE").unwrap();
            }
        }    
    }

    println!("Score: {}", score);
}

fn day2_2(data: &Vec<String>, winning: &HashMap<&str, &str>, points: &HashMap::<&str, i32>) {
    let mut score: i32 = 0;

    for line in data.iter() {
        let splitted: Vec<String> = line.split_whitespace().map(str::to_string).collect();
        let imp_choice: String = splitted[0].to_string();
        let player_choice: String = splitted[1].to_string();

        // Lose
        if player_choice == "X" {
            player_choice = winning.get(imp_choice).unwrap();
        }

        // Tie

        else if player_choice == "Y" {

        } 

        else {

        }

        if let Some(point) = points.get(&player_choice as &str) {
            score += point;
        }

        else {
            println!("NOT FOUND: {}", player_choice);
        }

        if let Some(val) = winning.get(&player_choice as &str) {
            if val == &imp_choice.as_str() {
                score += points.get("WIN").unwrap(); 
            }

            else if imp_choice.as_bytes()[0] == player_choice.as_bytes()[0] - 23 {
                score += points.get("TIE").unwrap();
            }
        }    
    }

    println!("Score: {}", score);
}