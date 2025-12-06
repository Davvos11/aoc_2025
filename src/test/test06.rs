use crate::day::Day;
use crate::day::day06::Day06;

fn part1(input: &str, expected: <Day06 as Day>::Part1) {
    let result = Day06::new(input.to_string()).part1();
    assert_eq!(result, expected, "Expected {}, got {}", expected, result)
}

fn part2(input: &str, expected: <Day06 as Day>::Part2) {
    let result = Day06::new(input.to_string()).part2();
    assert_eq!(result, expected, "Expected {}, got {}", expected, result)
}

#[test]
fn day06_part1_01() {
    let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
    part1(input, 4277556)
}

#[test]
fn day06_part2_01() {
    let input = "123 328  51 64.
 45 64  387 23.
  6 98  215 314
*   +   *   +  ";
    part2(input, 3263827)
}
