use crate::day::Day;
use crate::day::day05::Day05;

fn part1(input: &str, expected: <Day05 as Day>::Part1) {
    let result = Day05::new(input.to_string()).part1();
    assert_eq!(result, expected, "Expected {}, got {}", expected, result)
}

fn part2(input: &str, expected: <Day05 as Day>::Part2) {
    let result = Day05::new(input.to_string()).part2();
    assert_eq!(result, expected, "Expected {}, got {}", expected, result)
}

#[test]
fn day05_part1_01() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    part1(input, 3)
}

#[test]
fn day05_part2_01() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    part2(input, 14)
}

#[test]
fn day05_part2_02() {
    let input = "3-5
10-14
16-20
12-18
2-6

1";
    // 2, 3, 4, 5, 6, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
    part2(input, 16)
}
