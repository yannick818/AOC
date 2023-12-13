use core::panic;
use std::{collections::HashMap, rc::Rc};

use enum_iterator::Sequence;

use crate::prelude::*;

#[allow(dead_code)]
#[test]
fn test_maze_distance() {
    let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    assert_eq!(4, cal_maze_distance(input).unwrap());
}

#[test]
fn test_maze_distance2() {
    let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    assert_eq!(8, cal_maze_distance(input).unwrap());
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position(i64, i64);

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
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TileType {
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
struct Tile {
    typ: TileType,
    pos: Position,
}

impl Tile {
    fn walk(&self, from: &Direction) -> Result<(Direction, Position)> {
        let direction = self.typ.walk(from)?;
        let pos = self.pos.walk(&direction);
        Ok((direction, pos))
    }
}

struct Maze {
    maze: HashMap<Position, Tile>,
    start: Tile,
}

struct MazeRunner {
    maze: Rc<Maze>,
    pos: Tile,
    walking_dir: Direction,
    started: bool,
}

impl MazeRunner {
    fn new(maze: Rc<Maze>, start: Tile, direction: Direction) -> Self {
        Self {
            maze,
            pos: start,
            walking_dir: direction,
            started: false,
        }
    }
}

impl Iterator for MazeRunner {
    type Item = (Tile, Direction);

    fn next(&mut self) -> Option<(Tile, Direction)> {
        println!("pos: {:?} ({:?})", self.pos, self.walking_dir);

        if self.started && self.pos.typ == TileType::Start {
            return None;
        }
        self.started = true;

        let new_step = self.pos.walk(&self.walking_dir).map(|(new_dir, new_pos)| {
            let new_tile = self.maze.maze.get(&new_pos);
            (new_tile, new_dir)
        });

        match new_step {
            Ok((Some(new_tile), new_dir)) => {
                self.pos = *new_tile;
                self.walking_dir = new_dir;
                Some((*new_tile, new_dir))
            }
            //walked out of maze
            Ok((None, _)) => None,
            //hit obstacle
            Err(_) => None,
        }
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

pub fn cal_maze_distance(input: &str) -> Result<usize> {
    let maze = Rc::new(Maze::from(input));
    let runner =
        enum_iterator::all::<Direction>().map(|dir| MazeRunner::new(maze.clone(), maze.start, dir));

    let len = runner
        .filter_map(|runner| {
            match runner.enumerate().last() {
                Some((iteration, (last, _dir))) if last.typ == TileType::Start => Some(iteration+1),
                _ => None,
            }
        })
        .min()
        .unwrap();

    Ok(len / 2)
}
