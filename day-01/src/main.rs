use std::io;
use std::io::prelude::*;

fn main(){
    let stdin = io::stdin();
    let mut array: [i32; 3] = [0; 3];

    for i in 0..3 {
        let mut buf = String::new();
        stdin.read_line(&mut buf).expect("first line read error");
        array[i] = buf.trim().parse::<i32>().expect("first line parse error");
    }

    let mut prev_sum = 0;
    for num in &array {
        prev_sum += num;
    }

    let mut count = 0;

    for line in stdin.lock().lines() {
        for i in 1..3 {
            array[i-1] = array[i];
        }
        let this_int = line.unwrap().parse::<i32>().expect("bad line");
        array[2] = this_int;

        let mut this_sum = 0;
        for num in &array{
            this_sum += num;
        }

        if this_sum > prev_sum {
            count += 1;
        }

        prev_sum = this_sum;
    }

    println!("There are {} measurements larger than the previous", count);
}
