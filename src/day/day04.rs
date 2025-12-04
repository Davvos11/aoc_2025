use crate::day::Day;
use crate::utils::grid::Grid;
use crate::utils::grid_map::GridMap;
use rustc_hash::FxHashMap;

pub struct Day04 {
    grid: GridMap<char>,
}

impl Day for Day04 {
    type Part1 = u64;
    type Part2 = usize;

    fn new(input: String) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars())
            .enumerate()
            .flat_map(|(y, row)| row.enumerate().map(move |(x, val)| ((x, y), val)))
            .filter(|&(_, val)| val == '@')
            .collect::<FxHashMap<_, _>>()
            .into();
        Self { grid }
    }

    fn part1(&mut self) -> Self::Part1 {
        self.find_removable().count() as u64
    }
    fn part2(&mut self) -> Self::Part2 {
        let mut total = 0;
        loop {
            let removable = self.find_removable().collect::<Vec<_>>();
            for (x, y, _) in &removable {
                self.grid.remove(*x, *y);
            }

            let removed = removable.len();
            total += removed;
            if removed == 0 {
                return total;
            }
        }
    }
}

impl Day04 {
    fn find_removable(&self) -> impl Iterator<Item = (usize, usize, char)> {
        self.grid
            .iterate()
            .filter(|(_, _, value)| *value == '@')
            .filter(|(x, y, _)| {
                self.grid
                    .get_adjacent(*x, *y).len()
                    < 4
            })
    }
}
