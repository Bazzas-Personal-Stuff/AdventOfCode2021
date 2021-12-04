use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;
use std::str::FromStr;

struct Board {
    // Row-major order:
    // Row_idx = idx / 5
    // Col_idx = idx % 5

    // map value -> board idx
    solved: bool,
    positions: HashMap<i32, i32>,
    sum: i32,
    marked: i32,
    row_count: [i32; 5],
    col_count: [i32; 5],
}


fn play_game(call_order: &Vec<i32>, boards: &mut Vec<Board>) -> (Option<i32>, Option<i32>) {
    let mut first_score: Option<i32> = None;
    let mut last_score: Option<i32> = None;

    for ball in call_order{
        for mut board in boards.iter_mut(){
            if board.solved {
                continue
            };

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

            // check win conditions
            if board.row_count[row_idx] == 5 ||
                board.col_count[col_idx] == 5 {

                board.solved = true;
                let score = board.sum * ball;
                if first_score.is_none(){
                    first_score = Some(score);
                }
                last_score = Some(score);
            }
        }
    }

    return (first_score, last_score);
}



fn main() {
    let mut reader = BufReader::new(
        File::open("input.txt")
        // File::open("test.txt")
            .expect("Error reading file"));

    let mut buf = String::new();
    reader.read_line(&mut buf).expect("Error reading call order");

    let call_order: Vec<i32> = buf.trim().split(",").map(|el| i32::from_str(el).unwrap()).collect();
    buf.clear();

    let mut boards: Vec<Board> = Vec::new();

    // Loop guard automatically removes the blank lines
    while reader.read_line(&mut buf).expect("Error reading lines") != 0 {
        buf.clear();

        for _ in 0..5{
            reader.read_line(&mut buf).expect("Error reading bingo card");
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

        let this_board = Board {
            solved: false,
            positions: pos_map,
            sum,
            marked: 0,
            row_count: [0; 5],
            col_count: [0; 5],
        };
        boards.push(this_board);
        buf.clear();
    }

    let scores = play_game(&call_order, &mut boards);
    println!("Stage 1: {}\nStage 2: {}", scores.0.unwrap(), scores.1.unwrap())
}
