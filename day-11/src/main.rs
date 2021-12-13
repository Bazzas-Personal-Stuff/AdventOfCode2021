use std::io::{BufReader, BufRead};
use std::fs::File;
use std::cmp::{min, max};

fn main() {
    let mut reader = BufReader::new(
        // File::open("test.txt")
            File::open("input.txt")
            .expect("Error opening file"));
    let mut buf = String::new();


    let mut grid = [[0u8;10]; 10];

    let mut flash_count: u64 = 0;

    // Construct grid
    for row in 0..10 {
        reader.read_line(&mut buf).expect("Couldn't read row");
        let mut chars = buf.chars();
        for col in 0..10 {
            let val = chars.next().unwrap().to_digit(10).unwrap() as u8;
            grid[row][col] = val;
        }
        buf.clear();
    }


    // Stage 1
    for _step in 0..100 {
        let (flashes, _) = step(&mut grid);
        flash_count += flashes;
    }
    println!("{}", flash_count);

    let mut is_synced = false;
    let mut step_count = 99;
    while !is_synced {
        step_count += 1;
        let (_, synced) = step(&mut grid);
        is_synced = synced;
    }
    println!("{}", step_count);
}


fn step(grid: &mut [[u8;10];10]) -> (u64, bool){
    for row in 0..10 {
        for col in 0..10 {
            check_flash(row, col, grid);
        }
    }

    let mut is_synced = true;
    let mut flash_count: u64 = 0;
    let first_energy = grid[0][0];
    // Collect flash count and reset
    for row in 0..10 {
        for col in 0..10 {
            let energy = grid[row][col];
            if energy != first_energy {
                is_synced = false;
            }
            if energy > 9 {
                grid[row][col] = 0;
                flash_count += 1;
            }
        }
    }
    return (flash_count, is_synced);
}

fn check_flash(row: usize, col: usize, grid: &mut [[u8;10];10]){
    grid[row][col] += 1;
    if grid[row][col] == 10 {
        let min_row = if row == 0 {0} else {row - 1};
        let max_row = if row == 9 {9} else {row + 1};
        let min_col = if col == 0 {0} else {col - 1};
        let max_col = if col == 9 {9} else {col + 1};

        for r in min_row ..= max_row {
            for c in min_col ..= max_col {
                if r == row && c == col {
                    continue;
                }
                check_flash(r, c, grid);
            }
        }
    }
}
