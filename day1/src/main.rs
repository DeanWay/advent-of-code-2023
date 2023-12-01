use std::io::{stdin, BufRead};

fn main() {
    let input = parse_input(stdin().lock());
    println!("solution 1: {}", solution_1(&input));
    println!("solution 2: {}", solution_2(&input));
}

fn parse_input(input: impl BufRead) -> Vec<String> {
    input.lines().map(|line| line.unwrap()).collect()
}

fn solution_1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            let first_digit = line.chars().filter(|c| c.is_ascii_digit()).next().unwrap();
            let last_digit = line.chars().filter(|c| c.is_ascii_digit()).last().unwrap();
            let num_string: String = [first_digit, last_digit].iter().collect();
            num_string.parse::<u32>().unwrap()
        })
        .sum()
}

fn solution_2(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            let line_digits: Vec<u32> = iter_substrings(line)
                .filter_map(|substr| starts_with_digit(substr))
                .collect();
            line_digits.first().unwrap() * 10 + line_digits.last().unwrap()
        })
        .sum()
}

fn iter_substrings(s: &str) -> impl Iterator<Item = &str> {
    (0..s.len()).map(|offset| &s[offset..])
}

fn starts_with_digit(s: &str) -> Option<u32> {
    let matched = match s {
        s if s.starts_with("one") => Some(1),
        s if s.starts_with("two") => Some(2),
        s if s.starts_with("three") => Some(3),
        s if s.starts_with("four") => Some(4),
        s if s.starts_with("five") => Some(5),
        s if s.starts_with("six") => Some(6),
        s if s.starts_with("seven") => Some(7),
        s if s.starts_with("eight") => Some(8),
        s if s.starts_with("nine") => Some(9),
        _ => None,
    };
    if matched.is_some() {
        return matched;
    }
    s.chars().next().and_then(|c| c.to_digit(10))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn test_solution_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(solution_1(&input), 142);
    }

    #[test]
    fn test_solution_2_example() {
        let input = parse_input(EXAMPLE2.as_bytes());
        assert_eq!(solution_2(&input), 281);
    }
}
