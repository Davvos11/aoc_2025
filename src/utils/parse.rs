use std::ops::RangeInclusive;

pub fn string_to_range(input: &str) -> RangeInclusive<usize> {
    let mut strings = input.split('-');
    let start = strings.next().unwrap().parse().unwrap();
    let end = strings.next().unwrap().parse().unwrap();

    start..=end
}
