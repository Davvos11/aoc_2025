use crate::day::Day;
use crate::utils::grid::Grid;

pub struct Day04 {
    input: String,
    grid: Grid<char>,
}

impl Day for Day04 {
    type Part1 = u64;
    type Part2 = u64;

    fn new(input: String) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<_>>>()
            .into();
        Self { input, grid }
    }

    fn part1(&self) -> Self::Part1 {
        self.grid
            .iterate()
            .filter(|(_, _, value)| **value == '@')
            .filter(|(x, y, _)| {
                self.grid
                    .get_adjacent(*x, *y)
                    .iter()
                    .filter(|value| ***value == '@')
                    .count()
                    < 4
            })
            .count() as u64
    }
    fn part2(&self) -> Self::Part2 {
        todo!()
    }
}
