use crate::day::Day;
use std::ops::RangeInclusive;

pub struct Day02 {
    input: String,
}

impl Day for Day02 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part1(&self) -> String {
        let result = self
            .input
            .trim()
            .split(',')
            .map(string_to_range)
            .map(|range| range.into_iter().filter(is_repeating).sum::<usize>())
            .sum::<usize>();
        format!("{result}")
    }
    fn part2(&self) -> String {
        todo!()
    }
}

fn string_to_range(input: &str) -> RangeInclusive<usize> {
    let mut strings = input.split('-');
    let start = strings.next().unwrap().parse().unwrap();
    let end = strings.next().unwrap().parse().unwrap();

    start..=end
}

fn is_repeating(input: &usize) -> bool {
    let digits = input.ilog10() + 1;
    let divisor = 10_usize.pow(digits / 2);
    let first = input / divisor;
    let second = input % divisor;
    first == second
}
