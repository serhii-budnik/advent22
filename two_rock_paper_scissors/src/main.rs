use std::fs;

#[derive(Debug, PartialEq, Clone)]
enum Shape { 
    Rock,
    Paper,
    Scissors,
}

enum Rating {
    Won,
    Lost,
    Draw,
}

impl Rating {
    fn points(&self) -> u32 {
        match self {
            Self::Won => 6,
            Self::Draw => 3,
            Self::Lost => 0,
        }
    }

    fn decode(ch: char) -> Self {
        match ch {
            'X' => Self::Lost,
            'Y' => Self::Draw,
            'Z' => Self::Won,
            other => panic!("unexpected input {}", other),
        }
    }
}

impl Shape {
    fn decode(ch: char) -> Self {
        match ch {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            other => panic!("unexpected input {}", other),
        }
    }

    fn calc(&self, other: &Self) -> Rating {
        if self == other { return Rating::Draw }

        let for_win = Self::shape_to_win(other);

        if self == &for_win { Rating::Won } else { Rating::Lost }
    }

    fn shape_to_win(shape: &Shape) -> Shape {
        match shape {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn shape_to_lose(shape: &Shape) -> Shape {
        match shape {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn shape_points(&self) -> u32 {
        match &self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}


fn main() {
    let input = fs::read_to_string("./input.txt")
        .expect("Can not read input file");

    let mut points: u32 = 0;

    for line in input.lines() {
        let mut chars = line.chars();

        let opponent = Shape::decode(chars.next().unwrap());
        let expected_result = Rating::decode(chars.last().unwrap());

        let me = match expected_result {
            Rating::Won => Shape::shape_to_win(&opponent),
            Rating::Draw => opponent.clone(),
            Rating::Lost => Shape::shape_to_lose(&opponent),
        };

        let res = me.calc(&opponent);
        points += res.points() + me.shape_points();
    }

    println!("{}", points);
}
