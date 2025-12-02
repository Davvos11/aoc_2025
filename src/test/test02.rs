use crate::day::Day;
use crate::day::day02::Day02;

fn part1(input: &str, expected: &str) {
    let result = Day02::new(input.to_string()).part1();
    assert_eq!(result, expected)
}

fn part2(input: &str, expected: &str) {
    let result = Day02::new(input.to_string()).part2();
    assert_eq!(result, expected)
}

#[test]
fn day02_part1_01() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    part1(input, "1227775554")
}

#[test]
fn day02_part2_01() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    part2(input, "4174379265")
}

#[test]
fn day02_part2_02() {
    let input = "95-115";
    part2(input, "210")
}

#[test]
fn day02_part2_03() {
    let input = "998-1011";
    part2(input, "2009")
}
