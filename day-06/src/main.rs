use std::io::{BufReader, BufRead};
use std::fs::File;

fn simulate(fish_arr: &mut [u64; 9]){
    let to_spawn = fish_arr[0];
    for i in 1..9{
        fish_arr[i-1] = fish_arr[i];
    }
    fish_arr[6] += to_spawn;
    fish_arr[8] = to_spawn;
}

fn main() {
    let mut reader = BufReader::new(
        // File::open("test.txt")
            File::open("input.txt")
            .expect("Error opening file"));


    let mut fish: [u64; 9] = [0; 9];

    let mut buf = String::new();
    reader.read_line(&mut buf).expect("Error reading line");
    for num in buf.trim().split(",").map(|el| el.parse::<usize>().unwrap()) {
        fish[num] += 1;
    }


    for _i in 0..80 {
        simulate(&mut fish);
    }
    println!("Fish count after 80 days (Stage 1): {}", fish.iter().fold(0, |a, b| a + b));

    for _i in 80..256{
        simulate(&mut fish);
    }

    println!("Final count after 256 days (Stage 2): {}", fish.iter().fold(0, |a, b| a + b));
}
