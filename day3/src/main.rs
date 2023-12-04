use std::io::{stdin, BufRead};
mod schematic;
use schematic::{Cell, EntityValue, Schematic};

fn main() {
    let schematic = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&schematic));
}

fn parse_input(input: impl BufRead) -> Schematic {
    let grid: Vec<Vec<Cell>> = input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    c if c.is_ascii_digit() => Cell::Digit(c),
                    c if !c.is_ascii_alphanumeric() => Cell::Symbol(c),
                    _ => panic!("unexpected cell value {c}"),
                })
                .collect()
        })
        .collect();
    Schematic::from(grid)
}

fn part_1(schematic: &Schematic) -> u64 {
    schematic
        .get_entities()
        .values()
        .filter_map(|entity| match &entity.value {
            EntityValue::Number(num) => Some((entity.id, num)),
            _ => None,
        })
        .filter(|(entity_id, _)| {
            schematic
                .get_entity_positions(&entity_id)
                .flat_map(|pos| pos.adjacent())
                .any(|pos| match schematic.get_entity_at_position(&pos) {
                    Some(entity) if entity.is_symbol() => true,
                    _ => false,
                })
        })
        .map(|(_, num)| num)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_parse_input_example() {
        let schematic = parse_input(EXAMPLE.as_bytes());
        assert_eq!(
            schematic
                .get_entities()
                .values()
                .filter_map(|entity| match entity.value {
                    EntityValue::Number(number) => Some(number),
                    _ => None,
                })
                .collect::<HashSet<_>>(),
            [467, 114, 35, 633, 617, 58, 592, 755, 664, 598].into()
        );
        assert_eq!(
            schematic
                .get_entity_at_position(&(0, 0).into())
                .map(|entity| &entity.value),
            Some(&EntityValue::Number(467))
        );
        assert_eq!(
            schematic
                .get_entity_at_position(&(0, 0).into())
                .map(|entity| &entity.value),
            Some(&EntityValue::Number(467))
        );
        assert_eq!(
            schematic
                .get_entity_at_position(&(0, 0).into())
                .map(|entity| &entity.value),
            Some(&EntityValue::Number(467))
        );
        assert_eq!(
            schematic
                .get_entity_at_position(&(0, 0).into())
                .map(|entity| &entity.value),
            Some(&EntityValue::Number(467))
        );

        assert_eq!(schematic.get_entity_at_position(&(0, 3).into()), None);
    }

    #[test]
    fn test_part_1_example() {
        let schematic = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&schematic), 4361);
    }
}
