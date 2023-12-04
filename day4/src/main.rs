use std::{
    collections::VecDeque,
    io::{stdin, BufRead},
    rc::Rc,
};

use regex::Regex;

fn main() {
    let cards = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&cards));
    println!("part 2: {}", part_2(&cards));
}

#[derive(Debug, PartialEq, Eq)]
struct ScratchCard {
    wining_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
    card_total: usize,
}

impl ScratchCard {
    pub fn new(wining_numbers: Vec<u32>, card_numbers: Vec<u32>) -> Self {
        Self {
            card_total: Self::calculate_card_total(&wining_numbers, &card_numbers),
            wining_numbers,
            card_numbers,
        }
    }

    fn calculate_card_total(wining_numbers: &[u32], card_numbers: &[u32]) -> usize {
        card_numbers
            .iter()
            .filter(|card_number| wining_numbers.contains(card_number))
            .count()
    }

    pub fn card_total_winning_numbers(&self) -> usize {
        self.card_total
    }
}

fn parse_input(input: impl BufRead) -> Vec<Rc<ScratchCard>> {
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
            Rc::new(ScratchCard::new(
                parse_numbers(winning_numbers),
                parse_numbers(card_numbers),
            ))
        })
        .collect()
}

fn part_1(scratch_cards: &[Rc<ScratchCard>]) -> u64 {
    scratch_cards
        .iter()
        .map(|card| card.card_total_winning_numbers())
        .map(
            |card_total_wining_numbers| match card_total_wining_numbers {
                0 => 0,
                _ => 2u64.pow(card_total_wining_numbers as u32 - 1),
            },
        )
        .sum()
}

fn part_2(scratch_cards: &[Rc<ScratchCard>]) -> usize {
    let mut copies: VecDeque<Vec<Rc<ScratchCard>>> = scratch_cards
        .iter()
        .cloned()
        .map(|card| vec![card])
        .collect();
    let mut total = 0;
    for i in 0..scratch_cards.len() {
        let current_copies = copies
            .pop_front()
            .expect("copies should be same len as scratch_cards");
        for card in current_copies {
            total += 1;
            let n = card.card_total_winning_numbers();
            for j in 0..n {
                copies[j].push(scratch_cards[i + j + 1].clone());
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");
    #[test]
    fn test_parse_input_example() {
        let cards = parse_input(EXAMPLE.as_bytes());
        assert_eq!(&cards[0].wining_numbers, &[41, 48, 83, 86, 17]);
        assert_eq!(&cards[0].card_numbers, &[83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    fn test_part_1_example() {
        let cards = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&cards), 13);
    }

    #[test]
    fn test_part_2_example() {
        let cards = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&cards), 30);
    }
}
