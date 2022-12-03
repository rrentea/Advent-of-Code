use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_lines() -> Vec<String> {
    let file = File::open("input/input").expect("No such file");
    let reader = BufReader::new(file);
    reader.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_score(a: char, b: char) -> u32{
    match (a, b) {
        ('A', 'X') => return 0 + 3,
        ('A', 'Y') => return 3 + 1,
        ('A', 'Z') => return 6 + 2,
        ('B', 'X') => return 0 + 1,
        ('B', 'Y') => return 3 + 2,
        ('B', 'Z') => return 6 + 3,
        ('C', 'X') => return 0 + 2,
        ('C', 'Y') => return 3 + 3,
        ('C', 'Z') => return 6 + 1,
        _ => {
            println!("Invalid input!");
            return 0;
        },
    }
}

fn main() {
    let mut total_score: u32 = 0;
    for line in get_lines() {
        let vec = line.split(" ").collect::<Vec<&str>>();
        let a = vec[0].chars().nth(0).unwrap();
        let b = vec[1].chars().nth(0).unwrap();
        total_score += get_score(a, b);
    }
    println!("{}", total_score);
}
