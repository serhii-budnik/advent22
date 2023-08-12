use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt")
        .expect("Can not read input file");

    for line in input.lines() {
        let res1 = length_of_first_uniq_set_of_chars(&line, 4);
        println!("res 1 = {:?}", res1);
    }

    println!("\n\n-----\n\n");

    for line in input.lines() {
        let res2 = length_of_first_uniq_set_of_chars(&line, 14);
        println!("res 2 = {:?}", res2);
    }
}

fn length_of_first_uniq_set_of_chars(line: &str, n_of_uniq_set: usize) -> usize {
    let mut start = 0;
    let mut map = HashMap::new();

    for (index, char) in line.chars().enumerate() {
        if let Some(&char_index) = map.get(&char) { 
            if char_index >= start {
                start = char_index + 1;
            }
        }

        if (index - start) >= n_of_uniq_set - 1 { return index + 1 }

        map.insert(char, index);
    } 

    line.len()
}
