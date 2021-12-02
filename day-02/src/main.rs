use std::fs::File;
use std::io::{BufRead, BufReader};

struct Transform {
    x: i64,
    y: i64,
    aim: i64
}

fn main() {
    let mut stage_1 = Transform { x: 0, y: 0, aim: 0 };
    let mut stage_2 = Transform { x: 0, y: 0, aim: 0 };

    let reader = BufReader::new(
        File::open("input.txt")
            .expect("Error opening file"));
    let lines = reader.lines();
    for line in lines{
        let line_str = line.unwrap();
        let instruction: Vec<&str> = line_str.split_whitespace().collect();
        
        let dist = instruction[1].parse::<i64>().unwrap();
        match instruction[0] {
            "forward" =>  {
                stage_1.x += dist;
                stage_2.x += dist;
                stage_2.y += stage_2.aim * dist;
            },
            "up" => {
                stage_1.y -= dist;
                stage_2.aim -= dist;
            },
            "down" => {
                stage_1.y += dist;
                stage_2.aim += dist;
            },
            _ => println!("Unexpected instruction")
        }
    }


    println!("Stage 1 result: {}", stage_1.x * stage_1.y);
    println!("Stage 2 result: {}", stage_2.x * stage_2.y);
}

