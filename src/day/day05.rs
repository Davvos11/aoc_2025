use crate::day::Day;
use crate::utils::parse::string_to_range;
use std::ops::RangeInclusive;

pub struct Day05 {
    ranges: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

impl Day for Day05 {
    type Part1 = usize;
    type Part2 = u64;

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
        self.ingredients.iter()
            .filter(|i| self.is_fresh(**i))
            .count()
    }
    fn part2(&mut self) -> Self::Part2 {
        todo!()
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
}