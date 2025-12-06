use crate::day::Day;

pub struct Day06 {
    input: String,
}

impl Day for Day06 {
    type Part1 = u64;
    type Part2 = u64;

    fn new(input: String) -> Self {
        Self { input }
    }

    fn part1(&mut self) -> Self::Part1 {
        compute(&parse_part1(&self.input))
    }
    fn part2(&mut self) -> Self::Part2 {
        compute(&parse_part2(&self.input))
    }
}

fn parse_operators(input: &str) -> Vec<(char, Vec<u64>)> {
    input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|split| split.chars().next().unwrap())
        .map(|operation| (operation, Vec::new()))
        .collect()
}

fn parse_part1(input: &str) -> Vec<(char, Vec<u64>)> {
    let mut result = parse_operators(input);
    let count = input.lines().count();
    for line in input.lines().take(count - 1) {
        line.split_whitespace()
            .map(|str| str.parse().unwrap())
            .enumerate()
            .for_each(|(i, num)| result[i].1.push(num))
    }
    result
}

fn parse_part2(input: &str) -> Vec<(char, Vec<u64>)> {
    let mut result = parse_operators(input);
    let length = input.lines().next().unwrap().len();
    let mut j = 0;
    for i in 0..length {
        let mut number = 0;
        for line in input.lines() {
            if let Some(digit) = line.chars().nth(i).unwrap().to_digit(10) {
                number = number * 10 + digit as u64;
            }
        }
        if number != 0 {
            result[j].1.push(number);
        } else {
            j += 1;
        }
    }
    result
}

fn compute(input: &[(char, Vec<u64>)]) -> u64 {
    input
        .iter()
        .map(|(op, numbers)| match *op {
            '+' => numbers.iter().sum::<u64>(),
            '*' => numbers.iter().product(),
            _ => unreachable!(),
        })
        .sum()
}
