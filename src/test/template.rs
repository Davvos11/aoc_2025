use crate::day::Day;
use crate::day::dayXX::DayXX;

fn part1(input: &str, expected: &str) {
    let result = DayXX::new(input.to_string()).part1();
    assert_eq!(result, expected)
}

fn part2(input: &str, expected: &str) {
    let result = DayXX::new(input.to_string()).part2();
    assert_eq!(result, expected)
}

#[test]
fn dayXX_part1_01() {
    let input = "";
    part1(input, "")
}
