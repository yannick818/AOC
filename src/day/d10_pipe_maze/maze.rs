use crate::prelude::*;

use core::panic;
use std::{collections::HashMap, fmt::Debug};

use enum_iterator::Sequence;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub row: i64,
    pub col: i64,
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
pub enum Location {
    Inner,
    Outter,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TileType {
    Pipe(Pipe),
    Ground(Option<Location>),
    Start(Option<Pipe>),
}

impl TileType {
    fn walk(&self, walking_dir: &Direction) -> Result<Direction> {
        match (self, walking_dir) {
            (TileType::Pipe(Pipe::Vertical), Direction::North) => Ok(Direction::North),
            (TileType::Pipe(Pipe::Vertical), Direction::South) => Ok(Direction::South),
            (TileType::Pipe(Pipe::Horizontal), Direction::West) => Ok(Direction::West),
            (TileType::Pipe(Pipe::Horizontal), Direction::East) => Ok(Direction::East),
            (TileType::Pipe(Pipe::NorthEast), Direction::South) => Ok(Direction::East),
            (TileType::Pipe(Pipe::NorthEast), Direction::West) => Ok(Direction::North),
            (TileType::Pipe(Pipe::NorthWest), Direction::South) => Ok(Direction::West),
            (TileType::Pipe(Pipe::NorthWest), Direction::East) => Ok(Direction::North),
            (TileType::Pipe(Pipe::SouthEast), Direction::North) => Ok(Direction::East),
            (TileType::Pipe(Pipe::SouthEast), Direction::West) => Ok(Direction::South),
            (TileType::Pipe(Pipe::SouthWest), Direction::North) => Ok(Direction::West),
            (TileType::Pipe(Pipe::SouthWest), Direction::East) => Ok(Direction::South),
            (TileType::Ground(_), _) => Err("On Ground".into()),
            (TileType::Start(_), dir) => Ok(*dir),
            (tile, dir) => Err(format!("Pipe blocked walkin {:?} to {:?}", dir, tile).into()),
        }
    }

    fn is_uptile(&self) -> bool {
        matches!(
            self,
            TileType::Pipe(Pipe::NorthEast)
                | TileType::Pipe(Pipe::SouthEast)
                | TileType::Start(Some(Pipe::NorthEast))
                | TileType::Start(Some(Pipe::SouthEast))
        )
    }

    fn is_downtile(&self) -> bool {
        matches!(
            self,
            TileType::Pipe(Pipe::NorthWest)
                | TileType::Pipe(Pipe::SouthWest)
                | TileType::Start(Some(Pipe::NorthWest))
                | TileType::Start(Some(Pipe::SouthWest))
        )
    }

    fn get_pipe(&self) -> Pipe {
        match self {
            TileType::Pipe(pipe) => *pipe,
            TileType::Start(Some(pipe)) => *pipe,
            _ => panic!("Not a pipe: {:?}", self),
        }
    }

    pub fn is_start(&self) -> bool {
        matches!(self, TileType::Start(_))
    }
}

impl From<char> for TileType {
    fn from(value: char) -> Self {
        match value {
            '-' => TileType::Pipe(Pipe::Horizontal),
            '|' => TileType::Pipe(Pipe::Vertical),
            'L' => TileType::Pipe(Pipe::NorthEast),
            'J' => TileType::Pipe(Pipe::NorthWest),
            'F' => TileType::Pipe(Pipe::SouthEast),
            '7' => TileType::Pipe(Pipe::SouthWest),
            '.' => TileType::Ground(None),
            'S' => TileType::Start(None),
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
    pub maze: Vec<Vec<Tile>>,
    pub start: Tile,
}

impl Debug for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let maze = self
            .maze
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| match tile.typ {
                        TileType::Pipe(Pipe::Vertical) => '|',
                        TileType::Pipe(Pipe::Horizontal) => '-',
                        TileType::Pipe(Pipe::NorthEast) => 'L',
                        TileType::Pipe(Pipe::NorthWest) => 'J',
                        TileType::Pipe(Pipe::SouthEast) => 'F',
                        TileType::Pipe(Pipe::SouthWest) => '7',
                        TileType::Ground(None) => '.',
                        TileType::Ground(Some(Location::Outter)) => 'O',
                        TileType::Ground(Some(Location::Inner)) => 'I',
                        TileType::Start(_) => 'S',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", maze)
    }
}

impl Maze {
    pub fn cleanup(&mut self, main_pipe: &HashMap<Position, Tile>) {
        self.maze
            .iter_mut()
            .flatten()
            .filter(|tile| !main_pipe.contains_key(&tile.pos))
            .for_each(|tile| {
                tile.typ = TileType::Ground(None);
            });
    }

    pub fn get_tile(&mut self, pos: &Position) -> Option<&mut Tile> {
        self.maze
            .get_mut(pos.col as usize)
            .and_then(|row| row.get_mut(pos.row as usize))
    }

    ///Maze should be cleaned up before calling this
    ///returns the number of inner tiles
    pub fn define_ground(&mut self) -> usize {
        self.maze.iter_mut().for_each(|row| {
            row.iter_mut()
                .fold((0, None), |(vertical_cnt, up_tile), tile| match tile.typ {
                    TileType::Ground(_) => {
                        let is_outer = vertical_cnt % 2 == 0;
                        let cur_loc = if is_outer {
                            Location::Outter
                        } else {
                            Location::Inner
                        };
                        tile.typ = TileType::Ground(Some(cur_loc));
                        (vertical_cnt, None)
                    }
                    TileType::Pipe(Pipe::Vertical) => (vertical_cnt + 1, None),
                    up_tile if up_tile.is_uptile() => {
                        
                        (vertical_cnt, Some(up_tile.get_pipe()))

                    },
                    down_tile if down_tile.is_downtile() => {
                        match (up_tile.unwrap(), down_tile.get_pipe()) {
                            (Pipe::NorthEast, Pipe::NorthWest) => (vertical_cnt, None),
                            (Pipe::NorthEast, Pipe::SouthWest) => (vertical_cnt + 1, None),
                            (Pipe::SouthEast, Pipe::SouthWest) => (vertical_cnt, None),
                            (Pipe::SouthEast, Pipe::NorthWest) => (vertical_cnt + 1, None),
                            _ => panic!("Invalid pipe: {:?} {:?}", up_tile, down_tile),
                        }
                    }
                    _ => (vertical_cnt, up_tile),
                });
        });

        self.maze
            .iter()
            .flatten()
            .filter(|tile| matches!(tile.typ, TileType::Ground(Some(Location::Inner))))
            .count()
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

                            let start = if tile.typ == TileType::Start(None) {
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
