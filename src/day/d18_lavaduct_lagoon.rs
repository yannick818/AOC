//TODO Implement D18
#![allow(unused)]

use std::{collections::HashMap, fmt::Debug};

use array2d::Array2D;

use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

#[test]
fn test_trench_colume() {
    assert_eq!(62, cal_trench_volume(INPUT).unwrap());
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("unknown direction {}", value),
        }
    }
}

type Color = String;

struct DigStep {
    direction: Direction,
    steps: usize,
    color: Color,
}

impl DigStep {
    fn parse(input: &str) -> Vec<Self> {
        input
            .lines()
            .map(|line| {
                let mut split = line.split(' ');
                let direction = Direction::from(split.next().unwrap());
                let steps = split.next().unwrap().parse().unwrap();
                let color = split.next().unwrap().to_owned();

                Self {
                    direction,
                    steps,
                    color,
                }
            })
            .collect()
    }
}

type Row = usize;
type Column = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position(Row, Column);

impl Position {
    fn step(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.0 -= 1,
            Direction::Down => self.0 += 1,
            Direction::Left => self.1 -= 1,
            Direction::Right => self.1 += 1,
        }
    }
}

type Digged = bool;
struct Trench {
    trench: Array2D<(Digged, Option<Color>)>,
}

impl Trench {
    fn dig(steps: &[DigStep]) -> Self {
        // dig using a map, so no final size is needed
        let mut pos = Position(0, 0);
        let mut trench_map: HashMap<Position, Color> = HashMap::new();
        for step in steps {
            for _ in 0..step.steps {
                pos.step(step.direction);
                trench_map.insert(pos, step.color.clone());
            }
        }

        // find max size
        let rows = trench_map
            .keys()
            .max_by(|a, b| a.0.cmp(&b.0))
            .map(|pos| pos.0)
            .unwrap();

        let columns = trench_map
            .keys()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|pos| pos.1)
            .unwrap();

        let vec = vec![vec![(false, None); columns+1]; rows+1];
        let mut trench = Array2D::from_rows(&vec).unwrap();

        // fill with values from map
        for (pos, color) in trench_map.into_iter() {
            *trench.get_mut(pos.0, pos.1).unwrap() = (true, Some(color));
        }

        Self { trench }
    }

    fn dig_interior(&mut self) {
        todo!("idk how to find inner trench. visit neighbor positions seems fittable?");
        // for row in 0..self.trench.column_len() {
        //     let mut inner = false;
        //     let mut on_trench = false;
        //     for col in 0..self.trench.row_len() {
        //         let (digged, _) = self.trench.get_mut(row, col).unwrap();
        //         if *digged {
        //             on_trench = true;
        //         }
        //         if !on_trench {}
        //         if inner {
        //             *digged = true;
        //         }
        //     }
        // }
        // todo!()
    }

    fn size(&self) -> usize {
        self.trench
            .elements_row_major_iter()
            .filter(|(digged, _)| *digged)
            .count()
    }
}

impl Debug for Trench {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = self
            .trench
            .rows_iter()
            .map(|row| {
                row.map(|(digged, _)| if *digged { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>();

        for row in rows {
            writeln!(f, "{}", row)?;
        }

        Ok(())
    }
}

pub fn cal_trench_volume(input: &str) -> Result<usize> {
    let steps = DigStep::parse(input);
    let mut trench = Trench::dig(&steps);

    println!("{:?}", trench);

    trench.dig_interior();

    println!("{:?}", trench);

    let size = trench.size();
    Ok(size)
}
