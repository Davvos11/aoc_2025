use crate::day::Day;
use crate::day::dayXX::DayXX;

fn part1(input: &str, expected: <DayXX as Day>::Part1) {
    let result = DayXX::new(input.to_string()).part1();
    assert_eq!(result, expected, "Expected {}, got {}", expected, result)
}

fn part2(input: &str, expected: <DayXX as Day>::Part2) {
    let result = DayXX::new(input.to_string()).part2();
    assert_eq!(result, expected, "Expected {}, got {}", expected, result)
}

#[test]
fn dayXX_part1_01() {
    let input = "";
    part1(input, 0)
}
