use std::fmt::Debug;

use array2d::Array2D;
use enum_iterator::Sequence;

use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = "#.#####################
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

#[test]
fn test_longest_hike() {
    assert_eq!(cal_longest_hike(INPUT).unwrap(), 94);
}

#[test]
fn test_longest_hike_noslope() {
    assert_eq!(cal_longest_hike_noslope(INPUT).unwrap(), 154);
}

#[test]
fn test_4way() {
    let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.#...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#######.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    assert_eq!(cal_longest_hike_noslope(input).unwrap(), 118);
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
    walk_pos: Position,
    dead_pos: Position,
    end: Position,
    path_len: usize,
}

impl Debug for Trail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.trail.rows_iter() {
            for tile in row {
                match tile {
                    Tile::Path(false) => write!(f, ".")?,
                    Tile::Path(true) => write!(f, "O")?,
                    Tile::Forest => write!(f, "#")?,
                    Tile::Slope(true, _) => write!(f, "O")?,
                    Tile::Slope(false, dir) => match dir {
                        Direction::Up => write!(f, "^")?,
                        Direction::Down => write!(f, "v")?,
                        Direction::Left => write!(f, "<")?,
                        Direction::Right => write!(f, ">")?,
                    },
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
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
            walk_pos: Position(0, start),
            end: Position(rows.len() - 1, end),
            path_len: 0,
            dead_pos: Position(0, start),
        }
    }

    fn get_max_possible_hike_len(&self) -> usize {
        let free_tiles = self
            .trail
            .elements_row_major_iter()
            .filter(|tile| matches!(tile, Tile::Path(false) | Tile::Slope(false, _)))
            .count();
        free_tiles + self.path_len
    }

    fn get_hike_len(mut self, ignore_slope: bool) -> usize {
        *self
            .trail
            .get_mut(self.walk_pos.0, self.walk_pos.1)
            .unwrap() = Tile::Path(true);
        *self.trail.get_mut(self.end.0, self.end.1).unwrap() = Tile::Path(true);
        // self.path_len += 1;
        self.hike(ignore_slope)
    }

    fn hike(self, ignore_slope: bool) -> usize {
        let mut max_path = 0;
        let mut queue = vec![self];
        while let Some(trail) = queue.pop() {
            
            if trail.get_max_possible_hike_len() <= max_path {
                continue;
            }

            match trail.walk(ignore_slope) {
                WalkResult::End(path_len) => {
                    max_path = max_path.max(path_len);
                }
                WalkResult::Walked(mut paths) => queue.append(&mut paths),
            }
        }
        max_path
    }

    fn walk(self, ignore_slope: bool) -> WalkResult {
        let mut queue = vec![self];
        while let Some(trail) = queue.pop() {
            // println!("{:?}", trail);
            if trail.walk_pos == trail.end {
                return WalkResult::End(trail.path_len);
            }

            let mut new_trails = enum_iterator::all::<Direction>()
                .filter_map(|dir| trail.walk_pos.walk(dir).map(|pos| (dir, pos)))
                .filter_map(|(walk_dir, pos)| {
                    let tile = trail.trail.get(pos.0, pos.1).unwrap();
                    match tile {
                        Tile::Forest => None,
                        Tile::Path(true) => None,
                        Tile::Path(false) => {
                            let mut new_trail = trail.clone();
                            *new_trail.trail.get_mut(pos.0, pos.1).unwrap() = Tile::Path(true);
                            new_trail.dead_pos = new_trail.walk_pos;
                            new_trail.walk_pos = pos;
                            new_trail.path_len += 1;
                            Some(new_trail)
                        }
                        Tile::Slope(true, _) => None,
                        Tile::Slope(false, slope_dir) => {
                            if walk_dir == *slope_dir || ignore_slope {
                                let mut new_trail = trail.clone();
                                *new_trail.trail.get_mut(pos.0, pos.1).unwrap() =
                                    Tile::Slope(true, *slope_dir);
                                new_trail.dead_pos = new_trail.walk_pos;
                                new_trail.walk_pos = pos;
                                new_trail.path_len += 1;
                                Some(new_trail)
                            } else {
                                None
                            }
                        }
                    }
                })
                .collect::<Vec<_>>();

            match new_trails.len() {
                // dead end
                0 => {
                    return WalkResult::Walked(Vec::new());
                }
                // finish iterating simple path
                1 => {
                    queue.push(new_trails.pop().unwrap());
                }
                // split path
                _ => {
                    let new_trails = new_trails
                        .into_iter()
                        .filter_map(|trail| trail.walk_dead_end())
                        .filter_map(|trail| trail.walk_good_end())
                        .collect::<Vec<_>>();
                    if new_trails.len() == 1 {
                        queue.push(new_trails.into_iter().next().unwrap());
                    } else {
                        return WalkResult::Walked(new_trails);
                    }
                }
            }
        }
        unreachable!()
    }

    // none if end is blocked
    fn walk_dead_end(mut self) -> Option<Self> {
        let mut walked = false;
        let mut queue = vec![self.dead_pos];
        while let Some(pos) = queue.pop() {
            // println!("{:?}", self);
            if pos == self.end {
                // dead end cannot walk to end bc other paths might find it
                *self.trail.get_mut(pos.0, pos.1).unwrap() = Tile::Path(false);
                return Some(self);
            }
            self.dead_pos = pos;
            let possible_ways = enum_iterator::all::<Direction>()
                .filter_map(|dir| self.dead_pos.walk(dir))
                .filter(|pos| {
                    let tile = self.trail.get(pos.0, pos.1).unwrap();
                    match tile {
                        Tile::Forest => false,
                        Tile::Path(true) => false,
                        Tile::Path(false) => true,
                        Tile::Slope(true, _) => false,
                        Tile::Slope(false, _) => true,
                    }
                })
                .collect::<Vec<_>>();

            //check with cur pos O or #
            let cur_is_path = matches!(
                self.trail.get(self.dead_pos.0, self.dead_pos.1).unwrap(),
                Tile::Path(true) | Tile::Slope(true, _)
            );

            match (possible_ways.len(), cur_is_path) {
                (1, _) | (2, true) => {
                    for pos in possible_ways {
                        queue.push(pos);
                        *self.trail.get_mut(pos.0, pos.1).unwrap() = Tile::Forest;
                    }
                    walked = true;
                }
                (0, _) => {}
                _ => {
                    if walked {
                        // step back
                        let prev_tile = self
                            .trail
                            .get_mut(self.dead_pos.0, self.dead_pos.1)
                            .unwrap();
                        *prev_tile = Tile::Path(false);
                    }
                }
            }
        }
        if walked {
            Some(self)
        } else {
            None
        }
    }

    // none if end is blocked
    fn walk_good_end(mut self) -> Option<Self> {
        let mut queue = vec![self.end];
        while let Some(pos) = queue.pop() {
            self.end = pos;
            // println!("{:?}", self);
            let possible_ways = enum_iterator::all::<Direction>()
                .filter_map(|dir| self.end.walk(dir))
                .filter(|pos| {
                    let tile = self.trail.get(pos.0, pos.1);
                    if let Some(tile) = tile {
                        match tile {
                            Tile::Forest => false,
                            Tile::Path(true) => false,
                            Tile::Path(false) => true,
                            Tile::Slope(true, _) => false,
                            Tile::Slope(false, _) => true,
                        }
                    } else {
                        false
                    }
                })
                .collect::<Vec<_>>();

            match possible_ways.len() {
                0 => {
                    return None;
                }
                1 => {
                    self.path_len += 1;
                    let pos = possible_ways.into_iter().next().unwrap();
                    *self.trail.get_mut(pos.0, pos.1).unwrap() = Tile::Path(true);
                    queue.push(pos);
                }
                _ => {
                    *self.trail.get_mut(self.end.0, self.end.1).unwrap() = Tile::Path(false);
                    return Some(self);
                }
            }
        }
        unreachable!()
    }
}

pub fn cal_longest_hike(input: &str) -> Result<usize> {
    let trail = Trail::new(input);
    let longest = trail.get_hike_len(false);
    Ok(longest)
}

pub fn cal_longest_hike_noslope(input: &str) -> Result<usize> {
    let trail = Trail::new(input);
    let longest = trail.get_hike_len(true);
    Ok(longest)
}
