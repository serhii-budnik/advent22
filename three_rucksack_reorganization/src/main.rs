use regex::Regex;
use std::fs;

fn main() {
    let mut sum: u32 = 0;

    let input = fs::read_to_string("./input.txt")
        .expect("Can not read input file");


    let lines: Vec<&str> = input.lines().collect();

    for chunk in lines.chunks(3) {
        let first = find_all_shared_items(&chunk[0], &chunk[1]);
        let second = find_all_shared_items(&chunk[0], &chunk[2]);

        let shared_item = find_shared_item(&first, &second);
        let priority = get_priority(shared_item);

        println!("{}", priority);

        sum += priority;

    }

    println!("Sum: {}", sum);
}

fn find_all_shared_items(items: &str, items2: &str) -> String {
    let reg = Regex::new(&format!("[{}]", items)).unwrap();
    let result: String = reg.find_iter(items2).map(|m| m.as_str()).collect();

    result
}

fn find_shared_item(items: &str, items2: &str) -> char {
    let reg = Regex::new(&format!("[{}]", items)).unwrap();
    let caps = reg.captures(items2).unwrap();

    let res = caps.get(0).unwrap().as_str().chars().next().unwrap();
    
    res
}

fn get_priority(ch: char) -> u32 {
    let num: u32 = ch.into();

    let subtract = match num {
        (97..=122) => 96,
        (65..=90) => 38,
        e => todo!("unexpected char {}", e),
    };

    num - subtract
}
