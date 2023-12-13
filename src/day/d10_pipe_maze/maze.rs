use crate::prelude::*;

use core::panic;
use std::collections::HashMap;

use enum_iterator::Sequence;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position(i64, i64);

impl Position {
    fn walk(&self, dir: &Direction) -> Position {
        // upmost left is (0,0)
        match dir {
            Direction::North => Position(self.0, self.1 - 1),
            Direction::South => Position(self.0, self.1 + 1),
            Direction::East => Position(self.0 + 1, self.1),
            Direction::West => Position(self.0 - 1, self.1),
        }
    }
}
#[derive(Debug, Clone, Copy, Sequence)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TileType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
}

impl TileType {
    fn walk(&self, walking_dir: &Direction) -> Result<Direction> {
        match (self, walking_dir) {
            (TileType::Vertical, Direction::North) => Ok(Direction::North),
            (TileType::Vertical, Direction::South) => Ok(Direction::South),
            (TileType::Horizontal, Direction::West) => Ok(Direction::West),
            (TileType::Horizontal, Direction::East) => Ok(Direction::East),
            (TileType::NorthEast, Direction::South) => Ok(Direction::East),
            (TileType::NorthEast, Direction::West) => Ok(Direction::North),
            (TileType::NorthWest, Direction::South) => Ok(Direction::West),
            (TileType::NorthWest, Direction::East) => Ok(Direction::North),
            (TileType::SouthEast, Direction::North) => Ok(Direction::East),
            (TileType::SouthEast, Direction::West) => Ok(Direction::South),
            (TileType::SouthWest, Direction::North) => Ok(Direction::West),
            (TileType::SouthWest, Direction::East) => Ok(Direction::South),
            (TileType::Ground, _) => Err("On Ground".into()),
            (TileType::Start, dir) => Ok(*dir),
            (tile, dir) => Err(format!("Pipe blocked walkin {:?} to {:?}", dir, tile).into()),
        }
    }
}

impl From<char> for TileType {
    fn from(value: char) -> Self {
        match value {
            '-' => TileType::Horizontal,
            '|' => TileType::Vertical,
            '.' => TileType::Ground,
            'L' => TileType::NorthEast,
            'J' => TileType::NorthWest,
            'F' => TileType::SouthEast,
            '7' => TileType::SouthWest,
            'S' => TileType::Start,
            _ => panic!("Unknown tile type: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub typ: TileType,
    pub pos: Position,
}

impl Tile {
    pub fn walk(&self, walking_dir: &Direction) -> Result<(Direction, Position)> {
        let direction = self.typ.walk(walking_dir)?;
        let pos = self.pos.walk(&direction);
        Ok((direction, pos))
    }
}

pub struct Maze {
    pub maze: HashMap<Position, Tile>,
    pub start: Tile,
}

impl Maze {
    pub fn cleanup(&mut self, main_pipe: &HashMap<Position, Tile>) {
        self.maze
            .iter_mut()
            .filter_map(|(pos, tile)| {
                if main_pipe.contains_key(pos) {
                    None
                } else {
                    Some(tile)
                }
            })
            .for_each(|tile| {
                tile.typ = TileType::Ground;
            });
    }
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let (maze, start) = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, tile)| Tile {
                    typ: tile.into(),
                    pos: Position(x as i64, y as i64),
                })
            })
            .fold((HashMap::new(), None), |(mut map, start), tile| {
                map.insert(tile.pos, tile);
                let start = if tile.typ == TileType::Start {
                    if start.is_some() {
                        panic!("Multiple start tiles");
                    }
                    Some(tile)
                } else {
                    start
                };
                (map, start)
            });

        Self {
            maze,
            start: start.unwrap(),
        }
    }
}
