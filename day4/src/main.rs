use std::io::{stdin, BufRead};

use regex::Regex;

fn main() {
    let cards = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&cards));
}

#[derive(Debug, PartialEq, Eq)]
struct ScratchCard {
    wining_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}
impl ScratchCard {
    pub fn card_winning_numbers(&self) -> Vec<u32> {
        self.card_numbers
            .iter()
            .filter(|card_number| self.wining_numbers.contains(card_number))
            .map(|card_number| *card_number)
            .collect()
    }
}

fn parse_input(input: impl BufRead) -> Vec<ScratchCard> {
    let ws = Regex::new(r"\s+").unwrap();
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (_, numbers) = line.split_once(':').unwrap();
            let (winning_numbers, card_numbers) = numbers.split_once('|').unwrap();
            let parse_numbers = |numbers: &str| -> Vec<u32> {
                ws.split(numbers.trim())
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect()
            };
            ScratchCard {
                wining_numbers: parse_numbers(winning_numbers),
                card_numbers: parse_numbers(card_numbers),
            }
        })
        .collect()
}

fn part_1(scratch_cards: &[ScratchCard]) -> u64 {
    scratch_cards
        .iter()
        .map(|card| card.card_winning_numbers().len())
        .map(
            |card_total_wining_numbers| match card_total_wining_numbers {
                0 => 0,
                _ => 2u64.pow(card_total_wining_numbers as u32 - 1),
            },
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");
    #[test]
    fn test_parse_input_example() {
        let cards = parse_input(EXAMPLE.as_bytes());
        assert_eq!(
            cards[0],
            ScratchCard {
                wining_numbers: vec![41, 48, 83, 86, 17],
                card_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
            }
        );
    }

    #[test]
    fn test_part_1_example() {
        let cards = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&cards), 13);
    }
}
