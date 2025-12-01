use crate::day::Day;

pub struct Day01 {
    input: String,
}

impl Day for Day01 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part1(&self) -> String {
        let numbers = self.input.lines().map(|l| rotation_to_number(l.trim()));
        let mut current = 50;
        let mut count = 0;
        for number in numbers {
            current = (current + number).rem_euclid(100);
            if current == 0 {
                count += 1;
            }
        }
        count.to_string()
    }
    fn part2(&self) -> String {
        todo!()
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
