use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let mut reader = BufReader::new(
        File::open("test.txt")
            // File::open("input.txt")
            .expect("Error opening file"));

}
