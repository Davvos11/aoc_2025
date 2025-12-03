use std::fmt::Display;
use std::time::Instant;

pub mod day01;
pub mod day02;
pub mod day03;

pub trait Day {
    type Part1;
    type Part2;

    fn new(input: String) -> Self
    where
        Self: Sized;
    fn part1(&self) -> Self::Part1;
    fn part2(&self) -> Self::Part2;
}

pub trait PrintableDay {
    fn run(&self);
}

impl<A, B, T> PrintableDay for T
where
    A: Display,
    B: Display,
    T: Day<Part1 = A, Part2 = B>,
{
    fn run(&self) {
        let t = Instant::now();
        let part1 = self.part1();
        println!("Part 1: {}\nTook {:3.2?}", part1, t.elapsed());
        let t = Instant::now();
        let part2 = self.part2();
        println!("Part 2: {}\nTook {:3.2?}", part2, t.elapsed());
    }
}

pub type DayConstructor = fn(String) -> Box<dyn PrintableDay>;

#[rustfmt::skip]
pub fn days() -> Vec<DayConstructor> {
    vec![
        |input| Box::new(day01::Day01::new(input)),
        |input| Box::new(day02::Day02::new(input)),
        |input| Box::new(day03::Day03::new(input)),
    ]
}
