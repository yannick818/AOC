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

struct Platrom {
    platform: Array2D<Element>,
}

impl Platrom {
    fn tilt_up(&mut self) {
        let new_cols = self
            .platform
            .as_columns()
            .into_iter()
            .map(|col| {
                let mut new_col = col.split(|&e| e == Element::FixRock)
                    .map(|rollables| rollables.to_vec())
                    .flat_map(|mut rollables| {
                        rollables.sort_by(|a, b| {
                            match (a, b) {
                                (Element::Empty, Element::RoundRock) => Ordering::Greater,
                                (Element::RoundRock, Element::Empty) => Ordering::Less,
                                // TODO here could be a panic on FixedRock
                                _ => Ordering::Equal,
                            }
                        });
                        rollables.push(Element::FixRock);
                        rollables
                    })
                    .collect::<Vec<_>>();
                //last # is not needed
                new_col.pop();
                new_col
            })
            .collect::<Vec<_>>();

        self.platform = Array2D::from_columns(&new_cols).unwrap();
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
    platform.tilt_up();
    println!("{:?}", platform);
    let load = platform.cal_load();
    Ok(load)
}
