use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {

    let mut stage1: [i32; 12] = [0; 12];
    let mut num_vec: Vec<u32> = Vec::new();

    let reader = BufReader::new(
        File::open("input.txt")
            .expect("Error reading file"));

    for line in reader.lines(){
        let mut i = 0;
        let line_str = line.unwrap();
        num_vec.push(u32::from_str_radix(&line_str, 2).unwrap()); // binary string to uint

        for char in line_str.chars(){
            match char {
                '0' => stage1[i] -= 1,
                '1' => stage1[i] += 1,
                _ => println!("Unexpected char in input")
            }
            i += 1;
        }
    }


    // Stage 1
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for num in stage1.iter(){
        let bit = if num > &0 {1} else {0};
        gamma = gamma << 1;
        gamma += bit;

        epsilon = epsilon << 1;
        epsilon += bit ^ 1;

    }
    println!("Gamma: {}\t\tEpsilon: {}", gamma, epsilon);
    println!("Power Consumption (stage 1): {}", gamma * epsilon);


    // Stage 2
    let mut max_vec = num_vec.to_vec();
    let mut min_vec = num_vec;

    for i in 0..stage1.len(){
        let offset = stage1.len() as u32 - 1 - i as u32;
        let mask = 1 << offset;

        if max_vec.len() > 1 {
           let (max_vec_0, max_vec_1): (Vec<u32>, Vec<u32>) = max_vec
               .into_iter()
               .partition(|el| (*el & mask) >> offset == 0);

            if max_vec_1.len() >= max_vec_0.len(){
                max_vec = max_vec_1;
            }
            else {
                max_vec = max_vec_0;
            }
        }

        if min_vec.len() > 1 {
            let (min_vec_0, min_vec_1): (Vec<u32>, Vec<u32>) = min_vec
                .into_iter()
                .partition(|el| (*el & mask) >> offset == 0);

            if min_vec_0.len() <= min_vec_1.len(){
                min_vec = min_vec_0;
            }
            else {
                min_vec = min_vec_1;
            }
        }

    }

    println!("\nOxygen: {}\t\tCO2: {}", max_vec[0], min_vec[0]);
    println!("Life support (stage 2): {}", max_vec[0] * min_vec[0]);
}


