use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    let games = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&games));
    println!("part 1: {}", part_2(&games));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    reveals: Vec<HashMap<Color, u32>>,
}

fn parse_input(input: impl BufRead) -> Vec<Game> {
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_game(&line))
        .collect()
}

fn parse_game(line: &str) -> Game {
    let (game_str, reveals_str) = line.split_once(":").unwrap();
    let id = game_str
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();

    let reveals = reveals_str
        .split(";")
        .map(|reveal| reveal.split(",").map(parse_color_reveal).collect())
        .collect();
    Game { id, reveals }
}

fn parse_color_reveal(s: &str) -> (Color, u32) {
    let (number, color) = s.trim().split_once(" ").unwrap();
    let color = match color {
        "blue" => Color::Blue,
        "green" => Color::Green,
        "red" => Color::Red,
        _ => panic!("unknown color {}", color),
    };
    let number = number.parse().unwrap();
    (color, number)
}

fn part_1(games: &[Game]) -> u32 {
    let bag: HashMap<_, _> = [(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)].into();
    games
        .iter()
        .filter(|game| game_is_possible_given_bag(&bag, game))
        .map(|game| game.id)
        .sum()
}

fn part_2(games: &[Game]) -> u32 {
    let power_of_bag = |bag: HashMap<_, u32>| bag.values().product::<u32>();
    games
        .iter()
        .map(smallest_possible_bag)
        .map(power_of_bag)
        .sum()
}

fn game_is_possible_given_bag(bag: &HashMap<Color, u32>, game: &Game) -> bool {
    game.reveals
        .iter()
        .flat_map(|reveal| reveal.iter())
        .all(|(color, number)| number <= bag.get(color).unwrap_or(&0))
}

fn smallest_possible_bag(game: &Game) -> HashMap<Color, u32> {
    let mut bag = HashMap::<Color, u32>::new();
    let all_reveals = game.reveals.iter().flat_map(|reveal| reveal.iter());
    for (color, number) in all_reveals {
        let current_max = bag.entry(*color).or_insert(*number);
        if *current_max < *number {
            *current_max = *number;
        }
    }
    bag
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_parse_input_example() {
        let games = parse_input(EXAMPLE.as_bytes());
        assert_eq!(
            games[0],
            Game {
                id: 1,
                reveals: vec![
                    [(Color::Blue, 3), (Color::Red, 4)].into(),
                    [(Color::Red, 1), (Color::Green, 2), (Color::Blue, 6)].into(),
                    [(Color::Green, 2)].into()
                ]
            },
        )
    }

    #[test]
    fn test_part_1_example() {
        let games = parse_input(EXAMPLE.as_bytes());
        let result = part_1(&games);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_2_example() {
        let games = parse_input(EXAMPLE.as_bytes());
        let result = part_2(&games);
        assert_eq!(result, 2286);
    }
}
