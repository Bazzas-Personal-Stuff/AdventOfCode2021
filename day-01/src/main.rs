use std::io;

// This is a rewrite after discussing performance optimisations with others.
// My original solution can still be found in main_old.rs
fn main(){
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut arr: [i32; 3] = [0; 3];
    let mut count = 0;
    let mut oldest = 0;

    for i in 0..3{
        stdin.read_line(&mut buf).expect("Read error");
        arr[i] = buf.trim().parse::<i32>().unwrap();
        buf.clear();
    }

    while stdin.read_line(&mut buf).unwrap() != 0 {
        let new_num = buf.trim().parse::<i32>().unwrap();
        buf.clear();

        if new_num > arr[oldest] {
            count += 1;
        }

        arr[oldest] = new_num;
        oldest = (oldest + 1) % 3;
    }

    println!("There are {} measurements larger than the previous", count);
}