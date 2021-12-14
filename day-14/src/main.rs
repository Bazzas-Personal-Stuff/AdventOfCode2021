use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut reader = BufReader::new(
        // File::open("test.txt")
            File::open("input.txt")
            .expect("Error opening file"));


    let mut pairs: HashMap<String, char> = HashMap::new();
    let mut pair_count: HashMap<String, i64> = HashMap::new();

    let mut template = String::new();
    let mut char_count: HashMap<char, i64> = HashMap::new();

    reader.read_line(&mut template).expect("Error reading template");
    reader.read_line(&mut template).expect("Error discarding empty line");

    for line_result in reader.lines(){
        let line = line_result.unwrap();
        let pair_split: Vec<String> = line.split(" -> ").map(|el| el.to_string()).collect();
        pairs.insert(pair_split[0].clone(), pair_split[1].chars().next().unwrap());
    }

    let polymer: Vec<char> = template.trim().chars().collect();

    // Get count for initial template
    for c in polymer.windows(2) {

        let mut lookup_string = String::new();
        lookup_string.push(c[0]);
        lookup_string.push(c[1]);

        *pair_count.entry(lookup_string).or_insert(0) += 1;
    }
    for c in polymer {
        *char_count.entry(c).or_insert(0) += 1;
    }


    for _i in 0..40 {
        // Each cycle, every instance of a pair gets broken up, and for each of them,
        // two new pairs are created.
        let mut count_diffs: HashMap<String, i64> = HashMap::new();
        for (lookup_string, n_pairs) in pair_count.iter() {
            if n_pairs <= &0 {
                continue;
            }
            let mut lookup_iter = lookup_string.chars();
            let first_char = lookup_iter.next().unwrap();
            let second_char = lookup_iter.next().unwrap();

            // Get the character to make the two new polymers
            let add_char = pairs[lookup_string].clone();

            // Create new polymers
            let mut a_polymer = String::new();
            let mut b_polymer = String::new();
            a_polymer.push(first_char);
            a_polymer.push(add_char.clone());

            b_polymer.push(add_char.clone());
            b_polymer.push(second_char);

            // Keep track of character count
            *char_count.entry(add_char).or_insert(0) += n_pairs;

            *count_diffs.entry(lookup_string.clone()).or_insert(0) -= n_pairs;
            *count_diffs.entry(a_polymer).or_insert(0) += n_pairs;
            *count_diffs.entry(b_polymer).or_insert(0) += n_pairs;

        }

        // Update actual map
        for (pair_str, pair_diff) in count_diffs {
            *pair_count.entry(pair_str).or_insert(0) += pair_diff;
        }
    }

    // Get min and max characters
    let mut min_count = &i64::MAX;
    let mut max_count = &0;
    for val in char_count.values() {
        if val < min_count {
            min_count = val;
        }
        if val > max_count {
            max_count = val;
        }
    }

    println!("{}", max_count - min_count);
}
