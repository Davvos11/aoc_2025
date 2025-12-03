use crate::day::Day;
use crate::day::day01::Day01;

fn part1(input: &str, expected: <Day01 as Day>::Part1) {
    let result = Day01::new(input.to_string()).part1();
    assert_eq!(result, expected, "Expected {}, got {}", expected, result)
}

fn part2(input: &str, expected: <Day01 as Day>::Part2) {
    let result = Day01::new(input.to_string()).part2();
    assert_eq!(result, expected, "Expected {}, got {}", expected, result)
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
    part1(input, 3)
}

#[test]
fn day01_part2_01() {
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
    part2(input, 6)
}

#[test]
fn day01_part2_02() {
    let input = "R1000";
    part2(input, 10)
}

#[test]
fn day01_part2_03() {
    let input = "L1000";
    part2(input, 10)
}

#[test]
fn day01_part2_04() {
    let input = "L50
L101
R2
L1";
    // 50 - 50 = 0 (1)
    // 0 - 101 = -101 -> 99 (2)
    // 99 + 2 = 101 -> 1 (3)
    // 1 - 1 = 0 (4)
    part2(input, 4)
}

#[test]
fn day01_part2_05() {
    let input = "L50
R101
L2
R1";
    // 50 - 50 = 0 (1)
    // 0 + 101 = 101 -> 1 (2)
    // 101 - 2 = 99 (3)
    // 99 + 1 = 100 -> 0 (4)
    part2(input, 4)
}
