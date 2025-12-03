use crate::day::Day;

pub struct Day03 {
    input: String,
}

impl Day for Day03 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part1(&self) -> String {
        let result = self
            .input
            .lines()
            .map(|bank| {
                let batteries = bank
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>();
                let mut max = 0;
                for i in 0..batteries.len() {
                    for j in i + 1..batteries.len() {
                        let candidate = batteries[i] * 10 + batteries[j];
                        if candidate > max {
                            max = candidate;
                        }
                    }
                }
                max
            })
            .sum::<u32>();

        result.to_string()
    }
    fn part2(&self) -> String {
        let result = self
            .input
            .lines()
            .map(|bank| {
                let batteries = bank
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect::<Vec<u64>>();
                find_joltage(&batteries, 0, 12)
            })
            .sum::<u64>();

        result.to_string()
    }
}

fn find_joltage(batteries: &[u64], current: u64, digits: usize) -> u64 {
    let mut max = 0;
    let mut max_i = 0;
    let candidates = &batteries[0..(batteries.len() - digits + 1)];
    for (i, battery) in candidates.iter().enumerate() {
        let candidate = current * 10 + battery;
        if candidate > max {
            max = candidate;
            max_i = i;
        }
    }
    if batteries.len() > 1 && digits > 1 {
        find_joltage(&batteries[max_i + 1..], max, digits - 1)
    } else {
        max
    }
}
