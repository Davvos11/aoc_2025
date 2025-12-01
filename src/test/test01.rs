use crate::day::Day;
use crate::day::day01::Day01;

fn part1(input: &str, expected: &str) {
    let result = Day01::new(input.to_string()).part1();
    assert_eq!(result, expected)
}

fn part2(input: &str, expected: &str) {
    let result = Day01::new(input.to_string()).part2();
    assert_eq!(result, expected)
}

#[test]
fn day01_part1_01() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    part1(input, "3")
}
