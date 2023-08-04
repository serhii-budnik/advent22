use std::{fs, ops::RangeInclusive};

fn main() {
    let input = fs::read_to_string("./input.txt")
        .expect("Can not read input file");

    let mut count = 0;

    for line in input.lines() {
        let mut ranges = line.split(',');

        let first_range = parse_string_to_range(ranges.next().unwrap());
        let second_range = parse_string_to_range(ranges.next().unwrap());

        if first_range.is_overlap(&second_range) {
            count += 1;
            continue;
        }

        if second_range.is_overlap(&first_range) {
            count += 1;
            continue;
        }
    }

    println!("count: {}", count);
}

fn parse_string_to_range(input: &str) -> RangeInclusive<u32> {
    let mut res = input.split('-');
    let beginning: u32 = res.next().unwrap().parse().unwrap();
    let ending: u32 = res.next().unwrap().parse().unwrap();

    beginning..=ending
}

trait Subset {
    fn is_subset(&self, other: &Self) -> bool;
}

trait Overlap {
    fn is_overlap(&self, other: &Self) -> bool;
}

impl Subset for RangeInclusive<u32> {
    fn is_subset(&self, other: &Self) -> bool {
        self.start() >= other.start() && self.end() <= other.end()
    }
}

impl Overlap for RangeInclusive<u32> {
    fn is_overlap(&self, other: &Self) -> bool {
        self.contains(&other.start()) || self.contains(&other.end())
    }
}
