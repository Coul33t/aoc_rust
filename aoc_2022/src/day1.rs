use crate::tools::read_and_split;
use std::path::Path;

pub fn day1() {
    let path = Path::new("src/data/day1.txt");
    let data = read_and_split(path, "line");
    day1_1(&data);
    day1_2(&data);
}

fn day1_1(data: &Vec<String>) {
    println!("Day 1, first part:");

    let mut cal: i32 = 0;
    let mut max_cal: i32 = 0;

    for line in data.iter() {
        if line.eq("") {
            
            if cal > max_cal {
                max_cal = cal;
            }
            
            cal = 0;
        }

        else {
            cal += line.parse::<i32>().unwrap();
        }
    }

    println!("Max calories: {}", max_cal);
}

fn day1_2(data: &Vec<String>) {
    println!("Day 1, part 2:");
    
    let mut top_3_cal = vec![0, 0, 0];
    let mut cal: i32 = 0;

    for line in data.iter() {
        if line.eq("") {

            let min_cal = top_3_cal.iter().min().unwrap();

            println!("Current cal: {} vs Min cal: {}", cal, min_cal);
            println!("Vector: {:?}", top_3_cal);

            if cal > *min_cal {
                let mut min_idx: usize = 0;

                for (index, &x) in top_3_cal.iter().enumerate() {
                    if &x == min_cal {
                        min_idx = index;
                    }
                }

                top_3_cal[min_idx] = cal;
            }

            cal = 0;
        }

        else {
            cal += line.parse::<i32>().unwrap();
        }
    }

    println!("Top 3 cumulated max calories: {}", top_3_cal.iter().sum::<i32>());
}