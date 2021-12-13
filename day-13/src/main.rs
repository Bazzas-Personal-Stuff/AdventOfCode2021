use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let reader = BufReader::new(
        // File::open("test.txt")
            File::open("input.txt")
            .expect("Error opening file"));

    let mut x_bounds = 0;
    let mut y_bounds = 0;
    let mut grid_set: HashSet<(i32, i32)> = HashSet::new();
    let mut fold_count = 1;

    let mut is_defining = true;
    for line in reader.lines() {
        let line_str = line.unwrap();
        if line_str.len() == 0 {
            is_defining = false;
            continue;
        }

        if is_defining {
            let mut split = line_str.split(",");
            let x_val: i32 = split.next().unwrap().parse().unwrap();
            let y_val: i32 = split.next().unwrap().parse().unwrap();
            if x_val > x_bounds {
                x_bounds = x_val;
            }
            if y_val > y_bounds{
                y_bounds = y_val;
            }
            grid_set.insert((x_val, y_val));
        }
        else {
            let split: Vec<&str> = line_str.split_whitespace().collect();
            let mut instr_string = split.get(2).unwrap().split("=");
            let axis = instr_string.next().unwrap();
            let fold_pos: i32 = instr_string.next().unwrap().parse().unwrap();
            if axis == "x" {
                x_bounds = fold_pos;
            } else {
                y_bounds = fold_pos;
            }

            let mut to_remove: Vec<(i32, i32)> = Vec::new();
            let mut to_insert: Vec<(i32, i32)> = Vec::new();

            for dot in grid_set.iter() {
                if axis == "x" {
                    let x_pos = dot.0;
                    if x_pos > fold_pos {
                        to_remove.push(*dot);
                        let folded_x = x_bounds - (x_pos - fold_pos);
                        to_insert.push((folded_x, dot.1));
                    }
                } else {
                    let y_pos = dot.1;
                    if y_pos > fold_pos {
                        to_remove.push(*dot);
                        let folded_y = y_bounds - (y_pos - fold_pos);
                        to_insert.push((dot.0, folded_y));
                    }
                }
            }

            for el in to_remove {
                grid_set.remove(&el);
            }
            for el in to_insert.into_iter() {
                grid_set.insert(el);
            }
            println!("fold {}: {}", fold_count, grid_set.len());
            fold_count += 1;
        }


    }

    let mut grid: Vec<Vec<bool>> = Vec::with_capacity(y_bounds as usize + 1);
    for _ in 0..=y_bounds {
        let mut row: Vec<bool> = Vec::with_capacity(x_bounds as usize + 1);
        for _ in 0..=x_bounds{
            row.push(false);
        }
        grid.push(row);
    }

    for coords in grid_set{
        let row = coords.1 as usize;
        let col = coords.0 as usize;
        grid[row][col] = true;
    }

    for row in grid {
        for el in row {
            print!("{}", if el {"█"} else {" "});
        }
        println!();
    }
}
