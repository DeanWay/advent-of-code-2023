use std::io::{stdin, BufRead};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Input = Vec<Vec<i32>>;

fn parse_input(input: impl BufRead) -> Input {
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.split(' ').map(|num| num.parse().unwrap()).collect())
        .collect()
}

fn part_1(input: &Input) -> i64 {
    input.iter().map(|report| get_next_value(report)).sum()
}

fn part_2(input: &Input) -> i64 {
    input.iter().map(|report| get_prev_value(report)).sum()
}

fn get_next_value(report: &[i32]) -> i64 {
    if report.len() == 0 {
        return 0;
    }
    if all_same(report) {
        return report[0] as i64;
    }
    return report[report.len() - 1] as i64 + get_next_value(&differences(report));
}

fn get_prev_value(report: &[i32]) -> i64 {
    if report.len() == 0 {
        return 0;
    }
    if all_same(report) {
        return report[0] as i64;
    }
    return report[0] as i64 - get_prev_value(&differences(report));
}

fn all_same(nums: &[i32]) -> bool {
    nums.windows(2).all(|w| w[0] == w[1])
}

fn differences(nums: &[i32]) -> Vec<i32> {
    nums.windows(2).map(|w| w[1] - w[0]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 114);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 2);
    }
}
