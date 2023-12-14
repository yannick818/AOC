use array2d::Array2D;

use crate::prelude::*;

#[test]
fn test_reflection_code() {
    let input = "#.##..##.
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

    assert_eq!(405, cal_reflection_code(input).unwrap());
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

    fn find_mirror(&self) -> Mirror {
        let horizontal_mirror = self.tiles.rows_iter().enumerate().find_map(|(i, _)| {
            let rows = self.tiles.as_rows();
            let (left, right) = rows.split_at(i);
            let comp = right.iter().zip(left.iter().rev()).collect::<Vec<_>>();
            let symmetric = comp.iter().all(|(r, l)| r == l);
            if symmetric && !comp.is_empty() {
                Some(Mirror::Horizontal(i))
            } else {
                None
            }
        });

        if let Some(mirror) = horizontal_mirror {
            return mirror;
        }

        let vertical_mirror = self.tiles.columns_iter().enumerate().find_map(|(i, _)| {
            let columns = self.tiles.as_columns();
            let (top, bottom) = columns.split_at(i);
            let comp = bottom.iter().zip(top.iter().rev()).collect::<Vec<_>>();
            let symmetric = comp.iter().all(|(b, t)| b == t);
            if symmetric && !comp.is_empty() {
                Some(Mirror::Vertical(i))
            } else {
                None
            }
        });

        vertical_mirror.unwrap()
    }
}

pub fn cal_reflection_code(input: &str) -> Result<usize> {
    let patterns = Pattern::parse(input);
    let code = patterns.iter().map(|p| p.find_mirror().value()).sum();
    Ok(code)
}
