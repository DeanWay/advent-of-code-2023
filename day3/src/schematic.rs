use std::collections::HashMap;

pub type EntityId = u32;

#[derive(Debug)]
pub struct Schematic {
    entities: HashMap<EntityId, Entity>,
    position_to_entity: HashMap<Position, EntityId>,
}

impl Schematic {
    pub fn get_entities(&self) -> &HashMap<EntityId, Entity> {
        &self.entities
    }

    pub fn get_entity_at_position(&self, pos: &Position) -> Option<&Entity> {
        self.position_to_entity
            .get(pos)
            .and_then(|id| self.entities.get(id))
    }

    pub fn get_entity_positions<'a>(
        &'a self,
        entity_id: &'a EntityId,
    ) -> impl Iterator<Item = Position> + 'a {
        self.position_to_entity
            .iter()
            .filter(move |(_, id)| *id == entity_id)
            .map(|(pos, _)| *pos)
    }
}

impl From<Grid> for Schematic {
    fn from(grid: Grid) -> Self {
        let mut current_id = 0;
        let mut next_id = || {
            current_id += 1;
            current_id
        };
        let mut entities = HashMap::new();
        let mut position_to_entity = HashMap::new();

        for r in 0..grid.len() {
            let mut num_str = String::new();
            for c in 0..grid[r].len() {
                match &grid[r][c] {
                    Cell::Digit(d) => {
                        num_str.push(*d);
                    }
                    _ => {
                        if !num_str.is_empty() {
                            let number = num_str.parse::<u64>().unwrap();
                            let entity = Entity {
                                id: next_id(),
                                value: EntityValue::Number(number),
                            };
                            for num_c in c - num_str.len()..c {
                                position_to_entity
                                    .insert((r as i32, num_c as i32).into(), entity.id);
                            }
                            entities.insert(entity.id, entity);
                            num_str = String::new();
                        }
                    }
                }
                match &grid[r][c] {
                    Cell::Symbol(s) => {
                        let entity = Entity {
                            id: next_id(),
                            value: EntityValue::Symbol(*s),
                        };
                        position_to_entity.insert((r as i32, c as i32).into(), entity.id);
                        entities.insert(entity.id, entity);
                    }
                    _ => {}
                }
            }
            if !num_str.is_empty() {
                let number = num_str.parse::<u64>().unwrap();
                let entity = Entity {
                    id: next_id(),
                    value: EntityValue::Number(number),
                };
                let c = grid[r].len();
                for num_c in c - num_str.len()..c {
                    position_to_entity.insert((r as i32, num_c as i32).into(), entity.id);
                }
                entities.insert(entity.id, entity);
            }
        }

        Schematic {
            entities,
            position_to_entity,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Entity {
    pub id: EntityId,
    pub value: EntityValue,
}

impl Entity {
    pub fn is_number(&self) -> bool {
        matches!(self.value, EntityValue::Number(_))
    }
    pub fn is_symbol(&self) -> bool {
        matches!(self.value, EntityValue::Symbol(_))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EntityValue {
    Number(u64),
    Symbol(char),
}

#[derive(Debug)]
pub enum Cell {
    Digit(char),
    Symbol(char),
    Empty,
}

pub type Grid = Vec<Vec<Cell>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    row: i32,
    col: i32,
}

impl Position {
    pub fn adjacent(&self) -> Vec<Position> {
        let deltas = [-1, 0, 1];
        let rows = deltas.iter().map(|d| self.row + d);
        let cols = || deltas.iter().map(|d| self.col + d);
        rows.flat_map(move |row| cols().map(move |col| (row, col).into()))
            .filter(|pos| pos != self)
            .collect()
    }
}

impl From<(i32, i32)> for Position {
    fn from(value: (i32, i32)) -> Self {
        Self {
            row: value.0,
            col: value.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_adjacent() {
        let p = Position { row: 0, col: 0 };
        assert_eq!(
            p.adjacent(),
            vec![
                Position { row: -1, col: -1 },
                Position { row: -1, col: 0 },
                Position { row: -1, col: 1 },
                Position { row: 0, col: -1 },
                Position { row: 0, col: 1 },
                Position { row: 1, col: -1 },
                Position { row: 1, col: 0 },
                Position { row: 1, col: 1 },
            ]
        );
    }
}
