use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;
use std::str::FromStr;

struct Board {
    // Row-major order:
    // Row_idx = idx / 5
    // Col_idx = idx % 5

    // map value -> board idx
    positions: HashMap<i32, i32>,
    sum: i32,
    marked: i32,
    row_count: [i32; 5],
    col_count: [i32; 5],
    // diag_count: [i32; 2],
    completed: bool
}

fn solve_board(board: &Board, last_ball: &i32) -> i32 {
    println!("Sum of unmarked numbers: {}\t\tLast ball: {}", board.sum, last_ball);
    println!("Winning board: {:#?}", board.positions);
    return board.sum * last_ball;
}


fn play_game(call_order: &Vec<i32>, mut boards: &mut Vec<Board>) -> Option<i32> {
    for ball in call_order{
        for mut board in boards.iter_mut(){
            let idx = match board.positions.get(ball) {
                None => continue,
                Some(val) => *val as usize,
            };

            // The cell exists on the board
            board.marked += 1;
            board.sum -= ball;

            // Update win condition trackers
            let row_idx = idx / 5;
            let col_idx = idx % 5;
            board.row_count[row_idx] += 1;
            board.col_count[col_idx] += 1;
            // if row_idx == col_idx {
            //     board.diag_count[0] += 1;
            // }
            // if row_idx == 4 - col_idx {
            //     board.diag_count[1] += 1;
            // }

            // check win conditions
            for row_markers in board.row_count.iter(){
                if row_markers == &5{
                    return Some(solve_board(&board, ball));
                }
            }
            for col_markers in board.col_count.iter(){
                if col_markers == &5 {
                    return Some(solve_board(&board, ball));
                }
            }
            // for diag_markers in board.diag_count.iter(){
            //     if diag_markers == &5 {
            //         return Some(solve_board(&board, ball));
            //     }
            // }

        }
    }


    return None;
}



fn main() {
    let mut reader = BufReader::new(
        File::open("input.txt")
        // File::open("test.txt")
            .expect("Error reading file"));

    let mut buf = String::new();
    reader.read_line(&mut buf);

    let call_order: Vec<i32> = buf.trim().split(",").map(|el| i32::from_str(el).unwrap()).collect();
    buf.clear();

    println!("{:?}", call_order);


    let mut boards: Vec<Board> = Vec::new();

    // Loop guard automatically removes the blank lines
    while reader.read_line(&mut buf).expect("Error reading lines") != 0 {
        buf.clear();

        for _ in 0..5{
            reader.read_line(&mut buf);
        }
        let board_iter = buf.trim().split_whitespace().map(|el| i32::from_str(el).unwrap());

        let mut sum = 0;
        let mut idx = 0;
        let mut pos_map: HashMap<i32, i32> = HashMap::new();
        for el in board_iter{
            sum += el;
            pos_map.insert(el, idx);
            idx += 1;
        }

        let mut this_board = Board {
            positions: pos_map,
            sum,
            marked: 0,
            row_count: [0; 5],
            col_count: [0; 5],
            // diag_count: [0; 2],
            completed: false
        };
        boards.push(this_board);
        buf.clear();
    }

    println!("Stage 1: {}", play_game(&call_order, &mut boards).unwrap_or(-1))
}
