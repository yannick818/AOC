use std::collections::HashMap;

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
fn test_plot_count_extending() {
    assert_eq!(cal_plot_count_extending(INPUT, 6).unwrap(), 16);
    assert_eq!(cal_plot_count_extending(INPUT, 10).unwrap(), 50);
    assert_eq!(cal_plot_count_extending(INPUT, 50).unwrap(), 1594);
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
type Seated = bool;

#[derive(Clone, Copy)]
enum Field {
    Rock,
    Garden(Seated),
    Start(Seated),
}

struct Map {
    fields: HashMap<Position, Field>,
    init_rows: isize,
    init_cols: isize,
}

impl Map {
    fn step(&mut self, steps: usize, extending: bool) {
        for _ in 0..steps {
            // find positions of occupied seats and reset them
            let seats = self
                .fields
                .iter_mut()
                .filter_map(|(pos, field)| match field {
                    Field::Garden(seated) => {
                        if *seated {
                            *seated = false;
                            Some(*pos)
                        } else {
                            None
                        }
                    }
                    Field::Start(seated) => {
                        if *seated {
                            *seated = false;
                            Some(*pos)
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
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
                    // self.fields.
                    if let Some(field) = self.fields.get_mut(&pos) {
                        match field {
                            Field::Garden(seat) => *seat = true,
                            Field::Start(seat) => *seat = true,
                            _ => (),
                        }
                    } else if extending {
                        // % performs remainder, not modulo
                        let init_pos = Position(pos.0.rem_euclid(self.init_rows), pos.1.rem_euclid(self.init_cols));
                        let mut field = *self.fields.get(&init_pos).unwrap();
                        match &mut field {
                            Field::Garden(seat) => *seat = true,
                            Field::Start(seat) => *seat = true,
                            _ => (),
                        }
                        self.fields.insert(pos, field);
                    }
                })
        }
    }

    fn count_seated(&self) -> usize {
        self.fields
            .iter()
            .filter(|(_, field)| matches!(field, Field::Garden(true) | Field::Start(true)))
            .count()
    }

    fn new(input: &str) -> Self {
        let fields = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().map(move |(col, c)| match c {
                    '#' => (Position(row as isize, col as isize), Field::Rock),
                    '.' => (Position(row as isize, col as isize), Field::Garden(false)),
                    'S' => (Position(row as isize, col as isize), Field::Start(true)),
                    _ => panic!("invalid input {}", c),
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

pub fn cal_plot_count_extending(input: &str, steps: usize) -> Result<usize> {
    let mut map = Map::new(input);
    map.step(steps, true);
    let seated = map.count_seated();
    println!("seated: {} for {} steps", seated, steps);
    Ok(seated)
}


