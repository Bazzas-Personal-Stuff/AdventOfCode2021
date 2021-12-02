use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Transform {
    x: i64,
    y: i64,
    aim: i64
}

fn main() {

    let mut transform = Transform { x: 0, y: 0, aim: 0 };

    let reader = BufReader::new(
        File::open("input.txt")
            .expect("Error opening file"));

    for line in reader.lines() {
        let line_str = line.unwrap();
        let split: Vec<&str> = line_str.split_whitespace().collect();
        let dist = split[1].parse::<i64>().unwrap();
        match split[0] {
            "forward" =>  transform.x += dist,
            "backward" => transform.x -= dist,
            "up" => transform.y -= dist,
            "down" => transform.y += dist,
            _ => println!("Unexpected instruction")
        }
    }
    println!("Multiplication result: {}", transform.x * transform.y);
}
