use std::cmp::{max, min};
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;



fn main() {
    let mut reader = BufReader::new(
        File::open("input.txt")
        // File::open("test.txt")
            .expect("Error opening file"));


    let mut buf = String::new();


    let mut overlaps: HashSet<(i32, i32)> = HashSet::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();


    while reader.read_line(&mut buf).expect("Error reading line") != 0 {
        let mut points = buf.split("->");
        let p1_vec: Vec<i32> = points.next().unwrap().trim().split(",").map(|el| el.parse::<i32>().unwrap()).collect();
        let p2_vec: Vec<i32> = points.next().unwrap().trim().split(",").map(|el| el.parse::<i32>().unwrap()).collect();

        let p1 = (p1_vec[0], p1_vec[1]);
        let p2 = (p2_vec[0], p2_vec[1]);

        if p1.0 == p2.0 {
            for i in min(p1.1, p2.1)..=max(p1.1, p2.1){
                let point = (p1.0, i);
                if visited.contains(&point){
                    overlaps.insert(point);
                } else {
                    visited.insert(point);
                }
            }
        }
        else if p1.1 == p2.1 {
            for i in min(p1.0, p2.0)..=max(p1.0, p2.0){
                let point = (i, p1.1);
                if visited.contains(&point){
                    overlaps.insert(point);
                } else {
                    visited.insert(point);
                }
            }
        }
        else {
            let x_dir = if p1.0 < p2.0 {1} else {-1};
            let y_dir = if p1.1 < p2.1 {1} else {-1};

            let mut x = p1.0;
            let mut y = p1.1;
            while x != (p2.0 + x_dir) {
                let point = (x, y);
                if visited.contains(&point){
                    overlaps.insert(point);
                } else {
                    visited.insert(point);
                }

                x += x_dir;
                y += y_dir;
            }

        }

        buf.clear();
    }
    println!("Number of dangerous points\n(Stage 2): {}", overlaps.len());
}
