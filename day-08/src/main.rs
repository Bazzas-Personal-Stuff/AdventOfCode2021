use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let reader = BufReader::new(
        // File::open("test.txt")
            File::open("input.txt")
            .expect("Error opening file"));

    let mut stage1 = 0;
    let mut stage2: u64 = 0;


    for line in reader.lines() {
        let line_str = line.unwrap();
        let in_out_split: Vec<&str> = line_str.split("|").collect();
        let mut in_displays: Vec<&str> = in_out_split[0].split_whitespace().collect();
        let out_displays: Vec<&str> = in_out_split[1].split_whitespace().collect();

        // Stage 2
        in_displays.sort_by(|&a, &b| a.len().cmp(&b.len()));
        let in_masks: Vec<u8> = in_displays.iter().map(|&display| {
            let mut mask: u8 = 0;
            for segment in display.bytes() {
                let seg_offset = segment - 97;
                mask = 1 << seg_offset | mask;
            }
            return mask;
        }
        ).collect();

        let five_segs = &in_masks[3..=5];
        let six_segs = &in_masks[6..=8];

        let mut num_map: HashMap<u8, char> = HashMap::new();
        num_map.insert(in_masks[0], '1');
        num_map.insert(in_masks[1], '7');
        num_map.insert(in_masks[2], '4');
        num_map.insert(in_masks[9], '8');

        for display in five_segs {
            if (display & in_masks[2]).count_ones() == 2 {
                num_map.insert(*display, '2');
            }
            else if (display & in_masks[0]).count_ones() == 2 {
                num_map.insert(*display, '3');
            }
            else {
                num_map.insert(*display, '5');
            }
        }

        for display in six_segs {
            if (display & in_masks[2]).count_ones() == 4 {
                num_map.insert(*display, '9');
            }
            else if (display & in_masks[0]).count_ones() == 2 {
                num_map.insert(*display, '0');
            }
            else {
                num_map.insert(*display, '6');
            }
        }


        let mut out_decimal: String = String::new();
        for display in out_displays {

            // Stage 1
            if display.len() == 2 ||
                display.len() == 3 ||
                display.len() == 4 ||
                display.len() == 7 {

                stage1 += 1;
            }


            // Stage 2
            let mut num_lookup: u8 = 0;
            for segment in display.as_bytes() {
                let seg_offset = segment - 97;
                num_lookup = 1 << seg_offset | num_lookup;
            }
            out_decimal.push(num_map[&num_lookup])

        }

        stage2 += out_decimal.parse::<u64>().unwrap();
    }

    println!("Stage 1: {}", stage1);
    println!("Stage 2: {}", stage2);
}
