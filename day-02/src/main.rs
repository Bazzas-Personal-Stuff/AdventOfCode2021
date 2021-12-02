use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

struct Transform {
    x: i64,
    y: i64,
    aim: i64
}

fn stage_1(){
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
            "up" => transform.y -= dist,
            "down" => transform.y += dist,
            _ => println!("Unexpected instruction")
        }
    }
    println!("Stage 1 result: {}", transform.x * transform.y);
}


fn stage_2(){
    let mut transform = Transform { x: 0, y: 0, aim: 0 };

    let reader = BufReader::new(
        File::open("input.txt")
            .expect("Error opening file"));

    for line in reader.lines() {
        let line_str = line.unwrap();
        let split: Vec<&str> = line_str.split_whitespace().collect();
        let dist = split[1].parse::<i64>().unwrap();
        match split[0] {
            "forward" =>  {
                transform.x += dist;
                transform.y += transform.aim * dist;
            },
            "up" => transform.aim -= dist,
            "down" => transform.aim += dist,
            _ => println!("Unexpected instruction")
        }
    }

    println!("Stage 2 result: {}", transform.x * transform.y);
}

fn main() {


    stage_1();
    stage_2();
}

