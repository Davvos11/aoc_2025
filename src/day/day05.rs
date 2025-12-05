use crate::day::Day;
use crate::utils::parse::string_to_range;
use std::ops::RangeInclusive;

pub struct Day05 {
    ranges: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

impl Day for Day05 {
    type Part1 = usize;
    type Part2 = usize;

    fn new(input: String) -> Self {
        let mut split = input.split("\n\n");
        let ranges_input = split.next().unwrap();
        let ingredients_input = split.next().unwrap();

        let ranges = ranges_input.lines().map(string_to_range).collect();
        let ingredients = ingredients_input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();

        Self {
            ranges,
            ingredients,
        }
    }

    fn part1(&mut self) -> Self::Part1 {
        self.ingredients
            .iter()
            .filter(|i| self.is_fresh(**i))
            .count()
    }
    fn part2(&mut self) -> Self::Part2 {
        loop {
            self.sort_ranges();
            let (changed, merged_ranges) = merge_all(&self.ranges);
            if !changed {
                break;
            }
            self.ranges = merged_ranges;
        }
        self.ranges.iter().map(|r| r.clone().count()).sum()
    }
}

impl Day05 {
    fn is_fresh(&self, ingredient: usize) -> bool {
        for range in &self.ranges {
            if range.contains(&ingredient) {
                return true;
            }
        }
        false
    }

    fn sort_ranges(&mut self) {
        self.ranges
            .sort_by_key(|range| (*range.start(), *range.end()));
    }
}

fn try_merge(
    a: &RangeInclusive<usize>,
    b: &RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
    if *a.end() >= *b.start() {
        let end = usize::max(*a.end(), *b.end());
        Some(*a.start()..=end)
    } else {
        None
    }
}

fn merge_all(ranges: &[RangeInclusive<usize>]) -> (bool, Vec<RangeInclusive<usize>>) {
    let mut merged_ranges = Vec::with_capacity(ranges.len());
    let mut changed = false;

    let mut i = 0;
    while i < ranges.len() {
        let a = &ranges[i];
        let b = &ranges[i + 1];
        if let Some(merged) = try_merge(a, b) {
            merged_ranges.push(merged);
            changed = true;
            i += 2;
        } else {
            merged_ranges.push(a.clone());
            i += 1;
        }

        if i == ranges.len() - 1 {
            merged_ranges.push(ranges[i].clone());
            i += 1;
        }
    }

    (changed, merged_ranges)
}
