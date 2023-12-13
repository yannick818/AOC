use crate::prelude::*;

use core::panic;
use std::collections::HashMap;

use enum_iterator::Sequence;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    row: i64,
    col: i64,
}

impl Position {
    fn walk(&self, dir: &Direction) -> Position {
        // upmost left is (0,0)
        match dir {
            Direction::North => Self {
                row: self.row,
                col: self.col - 1,
            },
            Direction::South => Self {
                row: self.row,
                col: self.col + 1,
            },
            Direction::East => Self {
                row: self.row + 1,
                col: self.col,
            },
            Direction::West => Self {
                row: self.row - 1,
                col: self.col,
            },
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
    /// Col(Row(Tile)))
    maze: Vec<Vec<Tile>>,
    pub start: Tile,
}

impl Maze {
    pub fn cleanup(&mut self, main_pipe: &HashMap<Position, Tile>) {
        self.maze
            .iter_mut()
            .flatten()
            .filter(|tile| !main_pipe.contains_key(&tile.pos))
            .for_each(|tile| {
                tile.typ = TileType::Ground;
            });
    }

    pub fn get_tile(&self, pos: &Position) -> Option<&Tile> {
        self.maze
            .get(pos.col as usize)
            .and_then(|row| row.get(pos.row as usize))
    }
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let (maze, start) =
            value
                .lines()
                .enumerate()
                .fold((Vec::new(), None), |(mut col, start), (y, line)| {
                    let (row, start) = line.chars().enumerate().fold(
                        (Vec::new(), start),
                        move |(mut row, start), (x, tile)| {
                            let tile = Tile {
                                typ: tile.into(),
                                pos: Position {
                                    row: x as i64,
                                    col: y as i64,
                                },
                            };
                            row.push(tile);

                            let start = if tile.typ == TileType::Start {
                                if start.is_some() {
                                    panic!("Multiple start tiles");
                                }
                                Some(tile)
                            } else {
                                start
                            };

                            (row, start)
                        },
                    );
                    col.push(row);
                    (col, start)
                });

        Self {
            maze,
            start: start.unwrap(),
        }
    }
}
