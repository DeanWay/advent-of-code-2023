use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn part_1(input: &[Play]) -> u64 {
    input
        .iter()
        .sorted_by(|a, b| {
            a.hand
                .hand_type()
                .cmp(&b.hand.hand_type())
                .then_with(|| a.hand.cards.cmp(&b.hand.cards))
        })
        .zip(1..)
        .map(|(hand, rank)| hand.bid as u64 * rank as u64)
        .sum()
}

fn part_2(input: &[Play]) -> u64 {
    let mapped_input: Vec<Play> = input
        .iter()
        .map(|play| Play {
            hand: play.hand.clone().jacks_into_jokers(),
            bid: play.bid,
        })
        .collect();
    part_1(&mapped_input)
}

#[derive(Debug)]
struct Play {
    hand: Hand,
    bid: u32,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<CardVal>,
}

impl Hand {
    fn jacks_into_jokers(self) -> Self {
        Self {
            cards: self
                .cards
                .into_iter()
                .map(|card| match card {
                    CardVal::Jack => CardVal::Joker,
                    c => c,
                })
                .collect(),
        }
    }

    fn card_counts(&self) -> HashMap<CardVal, usize> {
        let mut counts = HashMap::new();
        for card in self.cards.iter() {
            let entry = counts.entry(card.clone()).or_insert(0);
            *entry += 1;
        }
        counts
    }

    fn hand_type(&self) -> HandType {
        assert_eq!(self.cards.len(), 5);

        let mut card_counts = self.card_counts();
        let count_jokers = card_counts.remove(&CardVal::Joker).unwrap_or(0);
        let mut groups: Vec<usize> = card_counts.into_values().sorted().rev().collect();
        if groups.is_empty() {
            return HandType::FiveOfAKind;
        }
        groups[0] += count_jokers;

        match groups.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::Pair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("unexpected hand {:?}", self),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
enum CardVal {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse_input(input: impl BufRead) -> Vec<Play> {
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            Play {
                hand: Hand {
                    cards: cards.chars().map(parse_card_val).collect(),
                },
                bid: bid.parse().unwrap(),
            }
        })
        .collect()
}

fn parse_card_val(c: char) -> CardVal {
    match c {
        'A' => CardVal::Ace,
        'K' => CardVal::King,
        'Q' => CardVal::Queen,
        'J' => CardVal::Jack,
        'T' => CardVal::Ten,
        '9' => CardVal::Nine,
        '8' => CardVal::Eight,
        '7' => CardVal::Seven,
        '6' => CardVal::Six,
        '5' => CardVal::Five,
        '4' => CardVal::Four,
        '3' => CardVal::Three,
        '2' => CardVal::Two,
        c => panic!("unexpected card value {c}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 6440);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 5905);
    }
}
