use crate::day::Day;
use crate::utils::parse::string_to_range;

pub struct Day02 {
    input: String,
}

impl Day for Day02 {
    type Part1 = usize;
    type Part2 = usize;

    fn new(input: String) -> Self {
        Self { input }
    }

    fn part1(&mut self) -> Self::Part1 {
        self.input
            .trim()
            .split(',')
            .map(string_to_range)
            .map(|range| range.into_iter().filter(is_repeating).sum::<usize>())
            .sum()
    }
    fn part2(&mut self) -> Self::Part2 {
        self.input
            .trim()
            .split(',')
            .map(string_to_range)
            .map(|range| {
                range
                    .into_iter()
                    .filter(is_repeating_multiple)
                    .sum::<usize>()
            })
            .sum()
    }
}

fn is_repeating(input: &usize) -> bool {
    let digits = input.ilog10() + 1;
    let divisor = 10_usize.pow(digits / 2);
    let first = input / divisor;
    let second = input % divisor;
    first == second
}

#[allow(clippy::assign_op_pattern)]
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
