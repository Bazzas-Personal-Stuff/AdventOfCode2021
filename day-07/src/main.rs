use std::io::{BufReader, Read};
use std::fs::File;

fn main() {
    let mut reader = BufReader::new(
        // File::open("test.txt")
            File::open("input.txt")
            .expect("Error opening file"));


    let mut buf = String::new();
    reader.read_to_string(&mut buf).expect("Error reading file");


    let mut inputs: Vec<i64> = buf
        .trim()
        .split(",")
        .map(|el| {
            let int = el.parse::<i64>().expect("Parse error");
            // sum += int;
            return int;

        })
        .collect();

    inputs.sort();


    // stage 1
    let mut fuel_median: i64 = 0;
    let median = inputs[inputs.len()/2];

    for crab in inputs.iter() {
        fuel_median += (median - crab).abs();
    }

    // stage 2
    let mut best_fuel = i64::max_value();
    let mut best_point = 0;
    for i in inputs[0]..inputs[inputs.len()-1] {
        let mut cur_fuel = 0;

        for crab in inputs.iter() {
            let n = (crab - i).abs();
            cur_fuel += (n * (n + 1)) / 2;
        }

        if cur_fuel < best_fuel {
            best_fuel = cur_fuel;
            best_point = i;
        }
        else {
            break;
        }
    }

    println!("Stage 1: best point: {}, fuel: {}", median, fuel_median);
    println!("Stage 2: best point: {}, fuel: {}", best_point, best_fuel);
}
