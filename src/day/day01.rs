use crate::day::Day;

pub struct Day01 {
    input: String,
}

impl Day for Day01 {
    type Part1 = u32;
    type Part2 = u64;

    fn new(input: String) -> Self {
        Self { input }
    }

    fn part1(&self) -> Self::Part1 {
        let numbers = self.input.lines().map(|l| rotation_to_number(l.trim()));
        let mut current = 50;
        let mut count = 0;
        for number in numbers {
            current = (current + number).rem_euclid(100);
            if current == 0 {
                count += 1;
            }
        }
        count
    }
    fn part2(&self) -> Self::Part2 {
        let numbers = self.input.lines().map(|l| rotation_to_number(l.trim()));
        let mut current = 50;
        let mut count = 0;
        for number in numbers {
            let new = current + number;
            let rotations = new as f64 / 100.;
            let mut zeroes = if rotations <= 0. {
                (rotations - 0.01).floor().abs() as u64
            } else if rotations >= 1. {
                rotations.floor() as u64
            } else {
                0
            };
            if new < 0 && zeroes > 0 && current == 0 {
                zeroes -= 1;
            }
            count += zeroes;
            current = new.rem_euclid(100);
        }
        count
    }
}

fn rotation_to_number(input: &str) -> i64 {
    let number = &input[1..].parse::<i64>().unwrap();
    match &input[0..1] {
        "L" => -number,
        "R" => *number,
        _ => unreachable!(),
    }
}
