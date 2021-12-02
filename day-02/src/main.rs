use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let mut pos: [i64; 2] = [0; 2];

    let reader = BufReader::new(
        File::open("input.txt")
            .expect("Error opening file"));

    for line in reader.lines() {
        let line_str = line.unwrap();
        let split: Vec<&str> = line_str.split_whitespace().collect();
        let dist = split[1].parse::<i64>().unwrap();
        match split[0] {
            "forward" => pos[0] += dist,
            "backward" => pos[0] -= dist,
            "up" => pos[1] -= dist,
            "down" => pos[1] += dist,
            _ => println!("Unexpected instruction")
        }
    }
    println!("Multiplication result: {}", pos[0] * pos[1]);
}
