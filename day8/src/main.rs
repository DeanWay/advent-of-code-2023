use std::{
    collections::HashMap,
    io::{stdin, BufRead},
    iter::successors,
};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Input = CamelMap;

fn parse_input(input: impl BufRead) -> Input {
    let mut lines = input.lines().map(|line| line.unwrap());
    let first_line = lines.next().unwrap();

    let steps = first_line
        .trim()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("unexpected direction"),
        })
        .collect();
    // blank line
    lines.next().unwrap();

    let graph = lines
        .map(|line| {
            let (place, paths) = line.split_once(" = ").unwrap();
            let (left_path, right_path) = paths.split_once(", ").unwrap();
            let left_path: String = left_path.chars().skip(1).collect();
            let right_path: String = right_path.chars().take(right_path.len() - 1).collect();
            (place.to_owned(), (left_path, right_path))
        })
        .collect();
    CamelMap { steps, graph }
}

fn part_1(input: &Input) -> usize {
    let mut directions = input.steps.iter().cycle();
    let next_location = |current_location: &&str| -> Option<&str> {
        let direction = directions.next().unwrap();
        let (left_path, right_path) = input.graph.get(*current_location).unwrap();
        match direction {
            Direction::Left => Some(left_path),
            Direction::Right => Some(right_path),
        }
    };
    successors(Some("AAA"), next_location)
        .take_while(|location| *location != "ZZZ")
        .count()
}

fn part_2(input: &Input) -> usize {
    let mut directions = input.steps.iter().cycle();
    let starting_positions: Vec<&str> = input
        .graph
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|key| key.as_str())
        .collect();
    let next_location = |current_locations: &Vec<&str>| -> Option<Vec<&str>> {
        let direction = directions.next().unwrap();
        Some(
            current_locations
                .iter()
                .map(|current_location| {
                    let (left_path, right_path) = input.graph.get(*current_location).unwrap();
                    match direction {
                        Direction::Left => left_path,
                        Direction::Right => right_path,
                    }
                })
                .map(|s| s.as_str())
                .collect(),
        )
    };
    successors(Some(starting_positions), next_location)
        .take_while(|locations| !locations.iter().all(|location| location.ends_with('Z')))
        .count()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct CamelMap {
    steps: Vec<Direction>,
    graph: HashMap<String, (String, String)>,
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");
    const PART_2_EXAMPLE: &str = include_str!("../part_2_example.txt");

    #[test]
    fn test_parse_input_example2() {
        let input = parse_input(EXAMPLE2.as_bytes());
        use Direction::*;
        assert_eq!(input.steps.as_slice(), [Left, Left, Right]);
        assert_eq!(
            input.graph,
            [
                ("AAA".into(), ("BBB".into(), "BBB".into())),
                ("BBB".into(), ("AAA".into(), "ZZZ".into())),
                ("ZZZ".into(), ("ZZZ".into(), "ZZZ".into())),
            ]
            .into()
        );
    }

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn test_part_1_example2() {
        let input = parse_input(EXAMPLE2.as_bytes());
        assert_eq!(part_1(&input), 6);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(PART_2_EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 6);
    }
}
