pub mod day01;
pub mod day02;

pub trait Day {
    fn new(input: String) -> Self
    where
        Self: Sized;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

pub type DayConstructor = fn(String) -> Box<dyn Day>;

#[rustfmt::skip]
pub fn days() -> Vec<DayConstructor> {
    vec![
        |input| Box::new(day01::Day01::new(input)),
        |input| Box::new(day02::Day02::new(input)),
    ]
}
