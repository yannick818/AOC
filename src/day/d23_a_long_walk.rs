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

    fn get_hike_len(mut self, ignore_slope: bool) -> Vec<usize> {
        *self
            .trail
            .get_mut(self.walk_pos.0, self.walk_pos.1)
            .unwrap() = Tile::Path(true);
        self.hike(ignore_slope)
    }

    fn hike(self, ignore_slope: bool) -> Vec<usize> {
        let mut path_lens = Vec::new();
        let mut queue = vec![self];
        while let Some(trail) = queue.pop() {
            println!("queue len {}", queue.len());
            println!("results len {}", path_lens.len());
            match trail.walk(ignore_slope) {
                WalkResult::End(path_len) => path_lens.push(path_len),
                WalkResult::Walked(mut paths) => queue.append(&mut paths),
            }
        }
        path_lens
    }

    fn walk(self, ignore_slope: bool) -> WalkResult {
        static mut GOAL: usize = 0;
        static mut DEAD_END: usize = 0;
        // println!("{:?}", self);
        let mut queue = vec![self];
        while let Some(trail) = queue.pop() {
            if trail.walk_pos == trail.end {
                unsafe {
                    GOAL += 1;
                }
                let ratio = unsafe { GOAL as f64 / DEAD_END as f64 };
                println!("ratio {}", ratio);
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
                0 => {
                    // println!("DEAD END");
                    // println!("{:?}", self);
                    unsafe {
                        DEAD_END += 1;
                    }
                    let ratio = unsafe { GOAL as f64 / DEAD_END as f64 };
                    println!("ratio {}", ratio);
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

    fn walk_dead_end(mut self) -> Option<Self> {
        let mut walked = false;
        let mut queue = vec![self.dead_pos];
        while let Some(pos) = queue.pop() {
            self.dead_pos = pos;
            // println!("{:?}", self);
            if self.dead_pos == self.end {
                // println!("DEAD END");
                // println!("{:?}", self);
                return None;
            }
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
                        self.dead_pos = pos;
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
}

pub fn cal_longest_hike(input: &str) -> Result<usize> {
    let trail = Trail::new(input);
    let hikes = trail.get_hike_len(false);
    let longest = hikes.into_iter().max().unwrap();
    Ok(longest)
}

pub fn cal_longest_hike_noslope(input: &str) -> Result<usize> {
    let trail = Trail::new(input);
    let hikes = trail.get_hike_len(true);
    let longest = hikes.into_iter().max().unwrap();
    Ok(longest)
}
