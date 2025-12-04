use crate::day::Day;

pub struct Day03 {
    input: String,
}

impl Day for Day03 {
    type Part1 = u64;
    type Part2 = u64;

    fn new(input: String) -> Self {
        Self { input }
    }

    fn part1(&mut self) -> Self::Part1 {
        self.solve(2)
    }
    fn part2(&mut self) -> Self::Part2 {
        self.solve(12)
    }
}

impl Day03 {
    fn solve(&self, digits: usize) -> u64 {
        self.input
            .lines()
            .map(|bank| {
                let batteries = bank
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect::<Vec<u64>>();
                find_joltage(&batteries, 0, digits)
            })
            .sum()
    }
}

fn find_joltage(batteries: &[u64], current: u64, digits: usize) -> u64 {
    let candidates = &batteries[0..(batteries.len() - digits + 1)];
    let (max_i, max) = candidates
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, battery)| battery)
        .unwrap();
    let new = current * 10 + max;
    if batteries.len() > 1 && digits > 1 {
        find_joltage(&batteries[max_i + 1..], new, digits - 1)
    } else {
        new
    }
}
