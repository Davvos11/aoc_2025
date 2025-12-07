use crate::day::Day;
use std::collections::HashSet;

pub struct Day07 {
    input: String,
}

impl Day for Day07 {
    type Part1 = u64;
    type Part2 = u64;

    fn new(input: String) -> Self {
        Self { input }
    }

    fn part1(&mut self) -> Self::Part1 {
        let rows: Vec<Vec<_>> = self
            .input
            .lines()
            .map(|l| {
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '^')
                    .map(|(i, _)| i)
                    .collect()
            })
            .collect();
        let start_x = self
            .input
            .lines()
            .next()
            .unwrap()
            .chars()
            .enumerate()
            .find(|(_, c)| *c == 'S')
            .map(|(i, _)| i)
            .unwrap();

        let mut beams = HashSet::from([start_x]);
        let mut splits = 0;
        for row in rows {
            let mut new_beams = HashSet::new();
            for beam in &beams {
                if row.contains(beam) {
                    new_beams.insert(beam - 1);
                    new_beams.insert(beam + 1);
                    splits += 1;
                } else {
                    new_beams.insert(*beam);
                }
            }
            beams = new_beams;
        }

        splits
    }
    fn part2(&mut self) -> Self::Part2 {
        todo!()
    }
}
