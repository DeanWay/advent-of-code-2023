use std::io::{stdin, BufRead};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[derive(Clone, Debug)]
struct Race {
    race_duration: u64,
    record_distance: u64,
}

impl Race {
    fn concat(&self, other: &Self) -> Self {
        let race_duration = self.race_duration.to_string() + &other.race_duration.to_string();
        let record_distance = self.record_distance.to_string() + &other.record_distance.to_string();

        Race {
            race_duration: race_duration.parse().unwrap(),
            record_distance: record_distance.parse().unwrap(),
        }
    }
}

type Input = Vec<Race>;

fn parse_input(input: impl BufRead) -> Input {
    let mut lines = input.lines().map(|line| line.unwrap()).map(|line| {
        line.split_whitespace()
            .skip(1)
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    });
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();
    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race {
            race_duration: time,
            record_distance: distance,
        })
        .collect()
}

fn part_1(input: &Input) -> u64 {
    input
        .iter()
        .map(|race| ways_to_beat_record(race) as u64)
        .product()
}

fn ways_to_beat_record(race: &Race) -> usize {
    (1..=race.race_duration)
        .map(|hold_time| {
            let remaining_time = race.race_duration - hold_time;
            hold_time * remaining_time
        })
        .filter(|distance_traveled| *distance_traveled > race.record_distance)
        .count()
}

fn part_2(input: &Input) -> u64 {
    let input = input
        .into_iter()
        .cloned()
        .reduce(|acc, other| acc.concat(&other))
        .unwrap();
    part_1(&vec![input])
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 288);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 71503);
    }
}
