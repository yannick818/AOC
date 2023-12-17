use core::panic;
use std::{cmp::Ordering, fmt::Debug};

use array2d::Array2D;

use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

#[test]
fn test_total_load() {
    assert_eq!(136, cal_total_load(INPUT).unwrap());
}

#[test]
fn test_total_load2() {
    assert_eq!(64, cal_load_after(INPUT, 1_000_000_000).unwrap());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    Empty,
    RoundRock,
    FixRock,
}

impl From<char> for Element {
    fn from(c: char) -> Self {
        match c {
            '.' => Element::Empty,
            'O' => Element::RoundRock,
            '#' => Element::FixRock,
            _ => panic!("Invalid char: {}", c),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn sort(&self, a: &Element, b: &Element) -> Ordering {
        match (self, a, b) {
            (_, Element::FixRock, _) => panic!("Canbot sort on fixed rock"),
            (_, _, Element::FixRock) => panic!("Canbot sort on fixed rock"),
            (_, a, b) if a == b => Ordering::Equal,
            (Direction::Up, Element::RoundRock, _) => Ordering::Less,
            (Direction::Up, _, Element::RoundRock) => Ordering::Greater,
            (Direction::Down, Element::RoundRock, _) => Ordering::Greater,
            (Direction::Down, _, Element::RoundRock) => Ordering::Less,
            (Direction::Left, Element::RoundRock, _) => Ordering::Less,
            (Direction::Left, _, Element::RoundRock) => Ordering::Greater,
            (Direction::Right, Element::RoundRock, _) => Ordering::Greater,
            (Direction::Right, _, Element::RoundRock) => Ordering::Less,
            _ => panic!("Invalid sort: {:?} {:?} {:?}", self, a, b),
        }
    }

    fn needs_row(&self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Platrom {
    platform: Array2D<Element>,
}

impl Platrom {
    fn tilt(&mut self, direction: Direction) {
        let iter = if direction.needs_row() {
            self.platform.as_rows()
        } else {
            self.platform.as_columns()
        };
        let new_cols = iter
            .into_iter()
            .map(|col| {
                let mut new_col = col
                    .split(|&e| e == Element::FixRock)
                    .map(|rollables| rollables.to_vec())
                    .flat_map(|mut rollables| {
                        rollables.sort_by(|a, b| direction.sort(a, b));
                        rollables.push(Element::FixRock);
                        rollables
                    })
                    .collect::<Vec<_>>();
                //last # is not needed
                new_col.pop();
                new_col
            })
            .collect::<Vec<_>>();

        if direction.needs_row() {
            self.platform = Array2D::from_rows(&new_cols).unwrap();
        } else {
            self.platform = Array2D::from_columns(&new_cols).unwrap();
        }
    }

    fn cal_load(&self) -> usize {
        let row_len = self.platform.row_len();
        self.platform
            .rows_iter()
            .enumerate()
            .map(|(col_iter, row)| {
                row.filter(|&&e| e == Element::RoundRock).count() * (row_len - col_iter)
            })
            .sum()
    }
}

impl From<&str> for Platrom {
    fn from(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| line.chars().map(Element::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self {
            platform: Array2D::from_rows(&rows).unwrap(),
        }
    }
}

impl Debug for Platrom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.platform.rows_iter() {
            for &e in row {
                let c = match e {
                    Element::Empty => '.',
                    Element::RoundRock => 'O',
                    Element::FixRock => '#',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn cal_total_load(input: &str) -> Result<usize> {
    let mut platform = Platrom::from(input);
    println!("{:?}", platform);
    platform.tilt(Direction::Up);
    println!("{:?}", platform);
    let load = platform.cal_load();
    Ok(load)
}

pub fn cal_load_after(input: &str, rounds: usize) -> Result<usize> {
    let mut platform = Platrom::from(input);
    for i in 0..rounds {
        println!("Round {}", i);
        let start = platform.clone();
        // println!("{:?}", platform);
        platform.tilt(Direction::Up);
        // println!("{:?}", platform);
        platform.tilt(Direction::Left);
        // println!("{:?}", platform);
        platform.tilt(Direction::Down);
        // println!("{:?}", platform);
        platform.tilt(Direction::Right);
        // println!("{:?}", platform);
        // println!("-----------------------");
        if start == platform {
            break;
        }
    }
    let load = platform.cal_load();
    Ok(load)
}
