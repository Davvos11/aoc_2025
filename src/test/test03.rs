use crate::day::Day;
use crate::day::day03::Day03;

fn part1(input: &str, expected: &str) {
    let result = Day03::new(input.to_string()).part1();
    assert_eq!(result, expected, "Expected {}, got {}", expected, result)
}

fn part2(input: &str, expected: &str) {
    let result = Day03::new(input.to_string()).part2();
    assert_eq!(result, expected, "Expected {}, got {}", expected, result)
}

#[test]
fn day03_part1_01() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    part1(input, "357")
}

#[test]
fn day03_part1_02() {
    let input = "987654321111111";
    part1(input, "98")
}

#[test]
fn day03_part1_03() {
    let input = "811111111111119";
    part1(input, "89")
}

#[test]
fn day03_part1_04() {
    let input = "234234234234278";
    part1(input, "78")
}

#[test]
fn day03_part1_05() {
    let input = "818181911112111";
    part1(input, "92")
}

#[test]
fn day03_part2_01() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    part2(input, "3121910778619")
}

#[test]
fn day03_part2_02() {
    let input = "987654321111111";
    part2(input, "987654321111")
}

#[test]
fn day03_part2_03() {
    let input = "811111111111119";
    part2(input, "811111111119")
}
#[test]
fn day03_part2_04() {
    let input = "234234234234278";
    part2(input, "434234234278")
}

#[test]
fn day03_part2_05() {
    let input = "818181911112111";
    part2(input, "888911112111")
}

