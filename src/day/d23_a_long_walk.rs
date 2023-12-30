use array2d::Array2D;
use enum_iterator::Sequence;

use crate::prelude::*;

#[test]
fn test_longest_hike() {
    let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    assert_eq!(cal_longest_hike(input).unwrap(), 94);
}

type Walked = bool;

#[derive(Sequence, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path(Walked),
    Forest,
    Slope(Walked, Direction),
}

#[derive(PartialEq, Eq, Clone, Copy)]
// row, col starting on top left
struct Position(usize, usize);

impl Position {
    fn walk(&self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::Up => {
                if self.0 == 0 {
                    None
                } else {
                    Some(Position(self.0 - 1, self.1))
                }
            }
            Direction::Down => Some(Position(self.0 + 1, self.1)),
            Direction::Left => {
                if self.1 == 0 {
                    None
                } else {
                    Some(Position(self.0, self.1 - 1))
                }
            }
            Direction::Right => Some(Position(self.0, self.1 + 1)),
        }
    }
}

#[derive(Clone)]
struct Trail {
    trail: Array2D<Tile>,
    pos: Position,
    end: Position,
    path_len: usize,
}

enum WalkResult {
    End(usize),
    Walked(Vec<Trail>),
}

impl Trail {
    fn new(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Tile::Forest,
                        '.' => Tile::Path(false),
                        '>' => Tile::Slope(false, Direction::Right),
                        '<' => Tile::Slope(false, Direction::Left),
                        '^' => Tile::Slope(false, Direction::Up),
                        'v' => Tile::Slope(false, Direction::Down),
                        _ => panic!("invalid input"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let start = rows[0]
            .iter()
            .position(|&tile| tile == Tile::Path(false))
            .unwrap();
        let end = rows[rows.len() - 1]
            .iter()
            .position(|&tile| tile == Tile::Path(false))
            .unwrap();

        Self {
            trail: Array2D::from_rows(&rows).unwrap(),
            pos: Position(0, start),
            end: Position(rows.len() - 1, end),
            path_len: 0,
        }
    }

    fn get_hike_len(self) -> Vec<usize> {
        let mut path_lens = Vec::new();
        self.hike(&mut path_lens)
    }

    fn hike(&self, path_lens: &mut Vec<usize>) -> Vec<usize> {
        match self.walk() {
            WalkResult::End(path_len) => path_lens.push(path_len),
            WalkResult::Walked(paths) => {
                for path in paths {
                    path.hike(path_lens);
                }
            }
        }
        path_lens.clone()
    }

    fn walk(&self) -> WalkResult {
        if self.pos == self.end {
            return WalkResult::End(self.path_len);
        }

        let new_trails = enum_iterator::all::<Direction>()
            .filter_map(|dir| self.pos.walk(dir).map(|pos| (dir, pos)))
            .filter_map(|(walk_dir, pos)| {
                let tile = self.trail.get(pos.0, pos.1).unwrap();
                match tile {
                    Tile::Forest => None,
                    Tile::Path(true) => None,
                    Tile::Path(false) => {
                        let mut new_trail = self.clone();
                        new_trail.pos = pos;
                        new_trail.path_len += 1;
                        *new_trail.trail.get_mut(pos.0, pos.1).unwrap() = Tile::Path(true);
                        Some(new_trail)
                    }
                    Tile::Slope(true, _) => None,
                    Tile::Slope(false, slope_dir) => {
                        if walk_dir == *slope_dir {
                            let mut new_trail = self.clone();
                            new_trail.pos = pos;
                            new_trail.path_len += 1;
                            *new_trail.trail.get_mut(pos.0, pos.1).unwrap() =
                                Tile::Slope(true, *slope_dir);
                            Some(new_trail)
                        } else {
                            None
                        }
                    }
                }
            })
            .collect();

        WalkResult::Walked(new_trails)
    }
}

pub fn cal_longest_hike(input: &str) -> Result<usize> {
    let trail = Trail::new(input);
    let longest = trail.get_hike_len().into_iter().max().unwrap();
    Ok(longest)
}
