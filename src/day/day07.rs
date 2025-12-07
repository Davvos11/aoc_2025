use crate::day::Day;
use std::collections::{HashMap, HashSet};

pub struct Day07 {
    rows: Vec<Vec<usize>>,
    start_x: usize,
}

impl Day for Day07 {
    type Part1 = u64;
    type Part2 = usize;

    fn new(input: String) -> Self {
        let rows: Vec<Vec<_>> = input
            .lines()
            .map(|l| {
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '^')
                    .map(|(i, _)| i)
                    .collect()
            })
            .collect();
        let start_x = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .enumerate()
            .find(|(_, c)| *c == 'S')
            .map(|(i, _)| i)
            .unwrap();
        Self { rows, start_x }
    }

    fn part1(&mut self) -> Self::Part1 {
        let mut beams = HashSet::from([self.start_x]);
        let mut splits = 0;
        for row in &self.rows {
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
        let mut beams = HashMap::from([(self.start_x, 1)]);
        for row in &self.rows {
            let mut new_beams = HashMap::new();
            for (beam, count) in beams {
                if row.contains(&beam) {
                    *new_beams.entry(beam - 1).or_default() += count;
                    *new_beams.entry(beam + 1).or_default() += count;
                } else {
                    *new_beams.entry(beam).or_default() += count;
                }
            }
            beams = new_beams;
        }

        beams.values().sum()
    }
}
