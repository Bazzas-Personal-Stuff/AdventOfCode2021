use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let mut reader = BufReader::new(
        // File::open("test.txt")
            File::open("input.txt")
            .expect("Error opening file"));

    let mut lookup: Vec<bool> = Vec::with_capacity(512);
    let mut buf = String::new();

    reader.read_line(&mut buf).expect("Error reading lookup table");
    for c in buf.trim().chars(){
        match c {
            '#' => lookup.push(true),
            '.' => lookup.push(false),
            _ => panic!("Unexpected character in algorithm"),
        }
    }


    reader.read_line(&mut buf).expect("Clearing next line");

    let mut grid: HashSet<(i64, i64)> = HashSet::new();
    let mut grid_buf: HashSet<(i64, i64)> = HashSet::new();
    let mut checked: HashSet<(i64, i64)> = HashSet::new();

    let mut y_idx = 0;
    for line in reader.lines(){
        let mut x_idx = 0;
        for c in line.unwrap().chars(){
            if c == '#' {
                grid.insert((x_idx, y_idx));
            }
            x_idx += 1;
        }
        y_idx += 1;
    }


    for i in 0..50 {
        // Does the current (read) grid store inverted values?
        let stores_inverted = i % 2 == 1;

        for (x_cur, y_cur) in grid.iter() {
            // Going to check all adjacent pixels to each tracked pixel (on, or off if inverted)
            for y in y_cur-1 ..= y_cur+1 {
                for x in x_cur-1 ..= x_cur+1 {
                    if checked.contains(&(x, y)){
                        continue;
                    }
                    checked.insert((x, y));


                    let mut lookup_idx = 0;
                    let mut bit_offset = 8;

                    // Get value for this pixel
                    for y_check in y-1 ..= y+1 {
                        for x_check in x-1 ..= x+1 {
                            let is_bit_set = grid.contains(&(x_check, y_check)) ^ stores_inverted;
                            if is_bit_set {
                                lookup_idx |= 1 << bit_offset;
                            }
                            bit_offset -= 1;
                        }
                    }

                    // Store the value in the grid buffer, inverting if we have to
                    if lookup[lookup_idx] ^ !stores_inverted {
                        grid_buf.insert((x, y));
                    }
                }
            }
        }

        // Swap buffers
        let temp_grid = grid;
        grid = grid_buf;
        grid_buf = temp_grid;
        grid_buf.clear();
        checked.clear();
    }

    println!("Number of lights on: {}", grid.len());
}
