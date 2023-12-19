use array2d::Array2D;

use crate::prelude::*;
use std::{fmt::Debug, collections::HashSet};

#[allow(dead_code)]
const INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

#[test]
fn test_energized_tiles() {
    assert_eq!(46, cal_energized_tiles(INPUT).unwrap());
}

#[test]
fn test_max_energized_tiles() {
    assert_eq!(51, cal_max_energized_tiles(INPUT).unwrap());
}

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    ForwardMirror,
    BackwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '/' => Tile::ForwardMirror,
            '\\' => Tile::BackwardMirror,
            '|' => Tile::VerticalSplitter,
            '-' => Tile::HorizontalSplitter,
            _ => panic!("unknown symbol {}", c),
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => f.write_str("."),
            Tile::ForwardMirror => f.write_str("/"),
            Tile::BackwardMirror => f.write_str("\\"),
            Tile::VerticalSplitter => f.write_str("|"),
            Tile::HorizontalSplitter => f.write_str("-"),
        }
    }
}

type Row = usize;
type Column = usize;
#[derive(Debug)]
struct Position(Row, Column);

impl Position {
    fn next(&self, dir: Direction) -> Option<Self> {
        if self.0 == 0 && dir == Direction::Up || self.1 == 0 && dir == Direction::Left {
            return None;
        }
        let next = match dir {
            Direction::Up => Self(self.0 - 1, self.1),
            Direction::Down => Self(self.0 + 1, self.1),
            Direction::Left => Self(self.0, self.1 - 1),
            Direction::Right => Self(self.0, self.1 + 1),
        };
        Some(next)
    }
}

impl Tile {
    fn walk(&self, pos: Position, dir: Direction) -> Vec<(Position, Direction)> {
        let new_dirs = match (self, dir) {
            (Tile::Empty, dir) => vec![dir],
            // /
            (Tile::ForwardMirror, Direction::Up) => vec![Direction::Right],
            (Tile::ForwardMirror, Direction::Down) => vec![Direction::Left],
            (Tile::ForwardMirror, Direction::Left) => vec![Direction::Down],
            (Tile::ForwardMirror, Direction::Right) => vec![Direction::Up],
            // \
            (Tile::BackwardMirror, Direction::Up) => vec![Direction::Left],
            (Tile::BackwardMirror, Direction::Down) => vec![Direction::Right],
            (Tile::BackwardMirror, Direction::Left) => vec![Direction::Up],
            (Tile::BackwardMirror, Direction::Right) => vec![Direction::Down],
            // |
            (Tile::VerticalSplitter, Direction::Left) => vec![Direction::Up, Direction::Down],
            (Tile::VerticalSplitter, Direction::Right) => vec![Direction::Up, Direction::Down],
            (Tile::VerticalSplitter, dir) => vec![dir],
            // -
            (Tile::HorizontalSplitter, Direction::Up) => vec![Direction::Left, Direction::Right],
            (Tile::HorizontalSplitter, Direction::Down) => vec![Direction::Left, Direction::Right],
            (Tile::HorizontalSplitter, dir) => vec![dir],
        };

        new_dirs
            .into_iter()
            .filter_map(|dir| {
                let next_pos = pos.next(dir);
                next_pos.map(|pos| (pos, dir))
            })
            .collect()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Energized = bool;

#[derive(Clone)]
struct Floor {
    floor: Array2D<(Tile, Energized, HashSet<Direction>)>,
}

impl Floor {
    fn parse(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| (Tile::from(c), false, HashSet::new()))
                    .collect()
            })
            .collect::<Vec<_>>();

        Self {
            floor: Array2D::from_rows(&rows).unwrap(),
        }
    }

    fn walk_start(&mut self, pos: Position, dir: Direction) {
        let mut queue = vec![(pos, dir)];
        while let Some((pos, dir)) = queue.pop() {
            let mut new_dirs = self.walk_step(pos, dir);
            queue.append(&mut new_dirs);
        }
    }

    fn walk_step(&mut self, pos: Position, dir: Direction) -> Vec<(Position, Direction)> {
        let tile = self.floor.get_mut(pos.0, pos.1);
        if tile.is_none() {
            //reached end of floor
            return Vec::new();
        }
        let (tile, energized, walked_dirs) = tile.unwrap();
        if walked_dirs.contains(&dir) {
            //already walked this way
            return Vec::new();
        }

        // println!("{pos:?}, Tile: {tile:?} {dir:?}");

        walked_dirs.insert(dir);
        *energized = true;

        tile.walk(pos, dir)
    }

    fn get_energized(&self) -> usize {
        self.floor
            .rows_iter()
            .flatten()
            .filter(|(_, energized, _)| *energized)
            .count()
    }

    fn find_max(&self) -> usize {
        let row_cnt = self.floor.column_len();
        let col_cnt = self.floor.row_len();

        let mut first_row = (0..col_cnt)
        .map(|i| {
            (Position(0, i), Direction::Down)
        })
        .collect::<Vec<_>>();

        let mut last_row = (0..col_cnt)
        .map(|i| {
            (Position(row_cnt-1, i), Direction::Up)
        })
        .collect::<Vec<_>>();

        let mut first_col = (0..row_cnt)
        .map(|i| {
            (Position(i, 0), Direction::Right)
        })
        .collect::<Vec<_>>();

        let mut last_col = (0..row_cnt)
        .map(|i| {
            (Position(i, col_cnt-1), Direction::Left)
        })
        .collect::<Vec<_>>();


        let mut start = Vec::new();
        start.append(&mut first_row);
        start.append(&mut last_row);
        start.append(&mut first_col);
        start.append(&mut last_col);
        
        let max = start.into_iter()
        .map(|(pos, dir)| {
            let mut floor = self.clone();
            floor.walk_start(pos, dir);
            floor.get_energized()
        })
        .max();

        max.unwrap()
    }
}

pub fn cal_energized_tiles(input: &str) -> Result<usize> {
    let mut floor = Floor::parse(input);
    floor.walk_start(Position(0,0), Direction::Right);
    let energized = floor.get_energized();
    Ok(energized)
}

pub fn cal_max_energized_tiles(input: &str) -> Result<usize> {
    let floor = Floor::parse(input);
    let max = floor.find_max();
    Ok(max)
}
