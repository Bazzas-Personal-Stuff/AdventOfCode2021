use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::BTreeMap;

fn main() {
    let reader = BufReader::new(
        // File::open("test.txt")
            File::open("input.txt")
            .expect("Error opening file"));

    let mut corrupted_count = 0;

    let mut right_scores: BTreeMap<char, i32> = BTreeMap::new();
    right_scores.insert(')', 3);
    right_scores.insert(']', 57);
    right_scores.insert('}', 1197);
    right_scores.insert('>', 25137);

    let mut left_scores: BTreeMap<char, u64> = BTreeMap::new();
    left_scores.insert('(', 1);
    left_scores.insert('[', 2);
    left_scores.insert('{', 3);
    left_scores.insert('<', 4);
    let mut auto_scores: Vec<u64> = Vec::new();

    for line in reader.lines(){
        let line_str = line.unwrap();

        let mut chunk_stack: Vec<char> = Vec::new();
        let mut corrupted = false;
        for symbol in line_str.chars() {
            // Stage 1
            let value = right_scores.get(&symbol);
            if value.is_some(){
                let match_char = match symbol {
                   ')' => '(',
                   ']' => '[',
                   '}' => '{',
                   '>' => '<',
                    _ => '_'
                };
                if chunk_stack.last() == Some(&match_char){
                    chunk_stack.pop();
                }
                else {
                    corrupted_count += value.unwrap();
                    corrupted = true;
                    break;
                }
            }
            else {
                chunk_stack.push(symbol);
            }
        }
        if corrupted{
            continue;
        }

        // Stage 2
        let mut autocomplete_score: u64 = 0;
        while !chunk_stack.is_empty(){
            autocomplete_score *= 5;
            autocomplete_score += left_scores[&chunk_stack.pop().unwrap()];
        }

        if autocomplete_score != 0 {
            auto_scores.push(autocomplete_score);
        }
    }

    auto_scores.sort();
    let middle_score = auto_scores[auto_scores.len()/2];
    println!("Corrupted line count: {}", corrupted_count);
    println!("Best autocomplete score: {}", middle_score);
}
