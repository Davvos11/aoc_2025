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
}
