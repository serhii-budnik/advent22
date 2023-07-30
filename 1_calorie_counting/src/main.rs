use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt")
        .expect("Can not read input file");

    let mut calories = elfs_calories(input);
    let top_elfs = top_n_elfs(&mut calories, 3);

    println!("Top 3 elfs: {:?}", top_elfs);
    println!("{}", top_elfs.iter().sum::<u32>());
}

fn elfs_calories(input: String) -> Vec<u32> {
    let mut calories = Vec::new();
    let mut current_elf = 0;

    for line in input.lines() {
        if line.is_empty() {
            calories.push(current_elf);
            current_elf = 0;
            continue;
        }

        current_elf += line.parse::<u32>().unwrap();
    }

    calories
}

fn top_n_elfs(calories: &mut Vec<u32>, n: usize) -> &[u32] {
    calories.sort();
    &calories[calories.len() - n..]
}
