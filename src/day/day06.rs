use crate::day::Day;

pub struct Day06 {
    columns: Vec<(char, Vec<u64>)>,
}

impl Day for Day06 {
    type Part1 = u64;
    type Part2 = u64;

    fn new(input: String) -> Self {
        Self {
            columns: parse_columns(&input),
        }
    }

    fn part1(&mut self) -> Self::Part1 {
        self.columns
            .iter()
            .map(|(op, numbers)| match *op {
                '+' => numbers.iter().sum::<u64>(),
                '*' => numbers.iter().product(),
                _ => unreachable!(),
            })
            .sum()
    }
    fn part2(&mut self) -> Self::Part2 {
        todo!()
    }
}

fn parse_columns(input: &str) -> Vec<(char, Vec<u64>)> {
    let mut result: Vec<_> = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|split| split.chars().next().unwrap())
        .map(|operation| (operation, Vec::new()))
        .collect();

    let count = input.lines().count();
    for line in input.lines().take(count - 1) {
        line.split_whitespace()
            .map(|str| str.parse().unwrap())
            .enumerate()
            .for_each(|(i, num)| result[i].1.push(num))
    }

    result
}
