use array2d::Array2D;

use crate::prelude::*;
use std::fmt::Debug;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Energized = bool;

struct Floor {
    floor: Array2D<(Tile, Energized, Vec<Direction>)>,
}

impl Floor {
    fn parse(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| (Tile::from(c), false, Vec::new()))
                    .collect()
            })
            .collect::<Vec<_>>();

        Self {
            floor: Array2D::from_rows(&rows).unwrap(),
        }
    }

    fn walk_start(&mut self) {
        self.walk(Position(0, 0), Direction::Right)
    }

    fn walk(&mut self, pos: Position, dir: Direction) {
        let tile = self.floor.get_mut(pos.0, pos.1);
        if tile.is_none() {
            //reached end of floor
            return;
        }
        let (tile, energized, walked_dirs) = tile.unwrap();
        if walked_dirs.contains(&dir) {
            //already walked this way
            return;
        }

        println!("{pos:?}, Tile: {tile:?} {dir:?}");

        walked_dirs.push(dir);
        *energized = true;

        let new_dirs = tile.walk(pos, dir);
        new_dirs.into_iter().for_each(|(pos, dir)| {
            self.walk(pos, dir);
        });
    }

    fn get_energized(&self) -> usize {
        self.floor
            .rows_iter()
            .flatten()
            .filter(|(_, energized, _)| *energized)
            .count()
    }
}

pub fn cal_energized_tiles(input: &str) -> Result<usize> {
    let mut floor = Floor::parse(input);
    floor.walk_start();
    let energized = floor.get_energized();
    Ok(energized)
}
