use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::{BinaryHeap, VecDeque, HashSet};
use std::cmp::Reverse;

fn main() {
    let  reader = BufReader::new(
        // File::open("test.txt")
            File::open("input.txt")
            .expect("Error opening file"));

    let mut heightmap: Vec<Vec<u8>> = Vec::new();
    for line in reader.lines(){
        heightmap.push(line.unwrap().chars().map(|el| el.to_digit(10).unwrap() as u8).collect::<Vec<u8>>());
    }

    let mut risk_sum: u64 = 0;
    let mut basin_heap = BinaryHeap::new();
    basin_heap.push(Reverse(0 as u64));
    basin_heap.push(Reverse(0 as u64));
    basin_heap.push(Reverse(0 as u64));

    let max_row = heightmap.len();
    let max_col = heightmap[0].len();
    for r in 0..max_row {
        for c in 0..max_col{
            let cur_height = heightmap[r][c];
            let mut is_low_point = true;

            // check left
            if r > 0 {
                if cur_height >= heightmap[r-1][c] {
                    is_low_point = false;
                }
            }
            // check right
            if r < max_row - 1 {
                if cur_height >= heightmap[r+1][c] {
                    is_low_point = false;
                }
            }
            // check up
            if c > 0 {
                if cur_height >= heightmap[r][c-1] {
                    is_low_point = false;
                }
            }
            // check down
            if c < max_col - 1 {
                if cur_height >= heightmap[r][c+1] {
                    is_low_point = false;
                }
            }

            if is_low_point {
                risk_sum += 1 + cur_height as u64;

                // BFS
                let mut bfs_queue: VecDeque<(usize, usize)> = VecDeque::new();
                let mut visited: HashSet<(usize, usize)> = HashSet::new();

                bfs_queue.push_back((r, c));
                visited.insert((r, c));

                let mut basin_size = 1;
                while !bfs_queue.is_empty() {
                    let (cur_r, cur_c) = bfs_queue.pop_front().unwrap();
                    let bfs_height = heightmap[cur_r][cur_c];
                    if cur_r > 0 {
                        if !visited.contains(&(cur_r - 1, cur_c))
                            && heightmap[cur_r - 1][cur_c] != 9
                            && heightmap[cur_r - 1][cur_c] > bfs_height {

                            basin_size += 1;
                            bfs_queue.push_back((cur_r-1, cur_c));
                            visited.insert((cur_r-1, cur_c));
                        }
                    }

                    if cur_r < max_row - 1 {
                        if !visited.contains(&(cur_r + 1, cur_c))
                            && heightmap[cur_r + 1][cur_c] != 9
                            && heightmap[cur_r + 1][cur_c] > bfs_height {

                            basin_size += 1;
                            bfs_queue.push_back((cur_r+1, cur_c));
                            visited.insert((cur_r+1, cur_c));
                        }
                    }

                    if cur_c > 0 {
                        if !visited.contains(&(cur_r, cur_c - 1))
                            && heightmap[cur_r][cur_c - 1] != 9
                            && heightmap[cur_r][cur_c - 1] > bfs_height {

                            basin_size += 1;
                            bfs_queue.push_back((cur_r, cur_c - 1));
                            visited.insert((cur_r, cur_c - 1));
                        }
                    }

                    if  cur_c < max_col - 1 {
                        if !visited.contains(&(cur_r, cur_c + 1))
                            && heightmap[cur_r][cur_c + 1] != 9
                            && heightmap[cur_r][cur_c + 1] > bfs_height {

                            basin_size += 1;
                            bfs_queue.push_back((cur_r, cur_c + 1));
                            visited.insert((cur_r, cur_c + 1));
                        }
                    }

                }

                if basin_heap.peek().unwrap().0 < basin_size{
                    basin_heap.pop();
                    basin_heap.push(Reverse(basin_size));
                }
            }
        }
    }

    let mut basin_mult = 1;
    for el in basin_heap.iter() {
        basin_mult *= el.0;
    }

    println!{"Risk sum (Stage 1): {}", risk_sum};
    println!{"{:?}", basin_heap.iter()}
    println!{"Basin total (Stage 2): {}", basin_mult};
}
