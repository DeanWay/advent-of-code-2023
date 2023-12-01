use std::io::{stdin, BufRead};

fn main() {
    let input = parse_input(stdin().lock());
    println!("solution 1: {}", solution_1(input));
}

fn parse_input(input: impl BufRead) -> Vec<String> {
    input.lines().map(|line| line.unwrap()).collect()
}

fn solution_1(input: Vec<String>) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_solution_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(solution_1(input), 142);
    }
}
