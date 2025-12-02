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
        let result = self
            .input
            .trim()
            .split(',')
            .map(string_to_range)
            .map(|range| {
                range
                    .into_iter()
                    .filter(is_repeating_multiple)
                    .sum::<usize>()
            })
            .sum::<usize>();
        format!("{result}")
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

fn is_repeating_multiple(input: &usize) -> bool {
    let digits = input.ilog10() + 1;
    'a: for size in 1..digits {
        if !digits.is_multiple_of(size) {
            continue;
        }

        let mut current = *input;
        let mut previous = None;
        for set in (0..(digits / size)).rev() {
            let divisor = 10_usize.pow(set * size);
            let new = current / divisor;
            if let Some(previous) = previous
                && new != previous
            {
                continue 'a;
            }
            previous = Some(new);
            current = current % divisor;
        }
        return true;
    }
    false
}
