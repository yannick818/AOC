use array2d::Array2D;

use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

#[test]
fn test_reflection_code() {
    assert_eq!(405, cal_reflection_code(INPUT).unwrap());
}

#[test]
fn test_reflection_code2() {
    assert_eq!(400, cal_reflection_code2(INPUT).unwrap());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Ash,
            '#' => Tile::Rock,
            _ => panic!("invalid char: {}", c),
        }
    }
}

enum Mirror {
    Horizontal(usize),
    Vertical(usize),
}

impl Mirror {
    fn value(&self) -> usize {
        match self {
            Mirror::Horizontal(v) => *v * 100,
            Mirror::Vertical(v) => *v,
        }
    }
}

struct Pattern {
    tiles: Array2D<Tile>,
}

impl Pattern {
    fn parse(input: &str) -> Vec<Self> {
        input.split("\n\n").map(|input| {
            let patterns = input
                .lines()
                .map(|line| line.chars().map(Tile::from).collect())
                .collect::<Vec<Vec<Tile>>>();

            Self {
                tiles: Array2D::from_rows(&patterns).unwrap(),
            }
        })
        .collect()
    }
    
    fn find_clean(&self) -> Mirror {
        self.find_mirror(0)
    }

    fn find_smudge(&self) -> Mirror {
        self.find_mirror(1)
    }

    fn find_mirror(&self, smuge_cnt: usize) -> Mirror {

        let horizontal_mirrot = self.find_horizonal().into_iter()
        .find_map(|reflection| {
            if reflection.smudge_count == smuge_cnt {
                Some(reflection.mirror)
            } else {
                None
            }
        });

        if let Some(mirror) = horizontal_mirrot {
            return mirror;
        }

        let vertical_mirror = self.find_vertical().into_iter()
        .find_map(|reflection| {
            if reflection.smudge_count == smuge_cnt {
                Some(reflection.mirror)
            } else {
                None
            }
        });

        vertical_mirror.unwrap()
    }

    fn find_horizonal(&self) -> Vec<Reflection> {

        self.tiles.rows_iter().enumerate().filter_map(|(i, _)| {
            let rows = self.tiles.as_rows();
            let (left, right) = rows.split_at(i);
            let comp = right.iter().zip(left.iter().rev()).collect::<Vec<_>>();
            let smudges = comp.iter()
            .flat_map(|(rhs, lhs)| {
                rhs.iter().zip(lhs.iter())
            })
            .fold(0, |smudges, (rhs, lhs)| {
                if rhs == lhs {
                    smudges
                } else {
                    smudges + 1
                }
            });

            if !comp.is_empty() {
                Some(Reflection{ mirror: Mirror::Horizontal(i), smudge_count: smudges })
            } else {
                None
            }
        })
        .collect()
    }
    
    fn find_vertical(&self) -> Vec<Reflection> {

        self.tiles.columns_iter().enumerate().filter_map(|(i, _)| {
            let col = self.tiles.as_columns();
            let (top, bottom) = col.split_at(i);
            let comp = bottom.iter().zip(top.iter().rev()).collect::<Vec<_>>();
            let smudges = comp.iter()
            .flat_map(|(rhs, lhs)| {
                rhs.iter().zip(lhs.iter())
            })
            .fold(0, |smudges, (rhs, lhs)| {
                if rhs == lhs {
                    smudges
                } else {
                    smudges + 1
                }
            });

            if !comp.is_empty() {
                Some(Reflection{ mirror: Mirror::Vertical(i), smudge_count: smudges })
            } else {
                None
            }
        })
        .collect()
    }
}

struct Reflection {
    mirror: Mirror,
    smudge_count: usize,
}

pub fn cal_reflection_code(input: &str) -> Result<usize> {
    let patterns = Pattern::parse(input);
    let code = patterns.iter().map(|p| p.find_clean().value()).sum();
    Ok(code)
}

pub fn cal_reflection_code2(input: &str) -> Result<usize> {
    let patterns = Pattern::parse(input);
    let code = patterns.iter().map(|p| p.find_smudge().value()).sum();
    Ok(code)
}
