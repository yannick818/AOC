use std::collections::{HashMap, HashSet};

use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

#[test]
fn test_plot_count() {
    assert_eq!(cal_plot_count(INPUT, 6).unwrap(), 16);
}

#[test]
#[allow(unreachable_code)]
fn test_plot_count_extending() {
    assert_eq!(cal_plot_count_extending(INPUT, 6).unwrap(), 16);
    assert_eq!(cal_plot_count_extending(INPUT, 10).unwrap(), 50);
    assert_eq!(cal_plot_count_extending(INPUT, 50).unwrap(), 1594);
    todo!("takes too long");
    assert_eq!(cal_plot_count_extending(INPUT, 100).unwrap(), 6536);
    assert_eq!(cal_plot_count_extending(INPUT, 500).unwrap(), 167004);
    assert_eq!(cal_plot_count_extending(INPUT, 1000).unwrap(), 668697);
    assert_eq!(cal_plot_count_extending(INPUT, 5000).unwrap(), 16733044);
}

#[test]
#[allow(clippy::identity_op)]
fn test_modulo() {
    assert_eq!(-1 % 10, -1);
    assert_eq!((-1_isize).rem_euclid(10), 9);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize);

#[derive(Clone, Copy)]
enum Field {
    Rock,
    Garden,
    Start,
}

struct Map {
    fields: HashMap<Position, (HashSet<Position>, Field)>,
    init_rows: isize,
    init_cols: isize,
}

impl Map {
    fn step(&mut self, steps: usize, extending: bool) {
        for _ in 0..steps {
            // find positions of occupied seats and reset them
            let seats = self
                .fields
                .values_mut()
                .flat_map(|(set, _)| set.drain())
                .collect::<Vec<_>>();

            // place new seats
            seats
                .into_iter()
                .flat_map(|Position(row, col)| {
                    [
                        Position(row - 1, col),
                        Position(row + 1, col),
                        Position(row, col - 1),
                        Position(row, col + 1),
                    ]
                })
                .for_each(|pos| {
                    if let Some((set, field)) = self.fields.get_mut(&pos) {
                        match field {
                            Field::Garden => set.insert(pos),
                            Field::Start => set.insert(pos),
                            _ => true,
                        };
                    } else if extending {
                        // % performs remainder, not modulo
                        let init_pos = Position(
                            pos.0.rem_euclid(self.init_rows),
                            pos.1.rem_euclid(self.init_cols),
                        );
                        let (set, field) = self.fields.get_mut(&init_pos).unwrap();
                        match field {
                            Field::Garden => set.insert(pos),
                            Field::Start => set.insert(pos),
                            _ => true,
                        };
                    }
                })
        }
    }

    fn count_seated(&self) -> usize {
        self.fields.values().map(|(set, _)| set.len()).sum()
    }

    fn new(input: &str) -> Self {
        let fields = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().map(move |(col, c)| {
                    let pos = Position(row as isize, col as isize);

                    let typ = match c {
                        '#' => Field::Rock,
                        '.' => Field::Garden,
                        'S' => Field::Start,
                        _ => panic!("invalid input {}", c),
                    };

                    let set = match typ {
                        Field::Start => {
                            let mut set = HashSet::new();
                            set.insert(pos);
                            set
                        }
                        _ => HashSet::new(),
                    };

                    (pos, (set, typ))
                })
            })
            .collect();

        Self {
            fields,
            init_cols: input.lines().next().unwrap().len() as isize,
            init_rows: input.lines().count() as isize,
        }
    }
}

pub fn cal_plot_count(input: &str, steps: usize) -> Result<usize> {
    let mut map = Map::new(input);
    map.step(steps, false);
    Ok(map.count_seated())
}

// TODO impl Day 21.2 faster
#[allow(dead_code)]
pub fn cal_plot_count_extending(input: &str, steps: usize) -> Result<usize> {
    let mut map = Map::new(input);
    map.step(steps, true);
    let seated = map.count_seated();
    println!("seated: {} for {} steps", seated, steps);
    Ok(seated)
}


