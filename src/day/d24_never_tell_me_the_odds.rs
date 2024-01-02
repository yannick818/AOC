use ndarray::prelude::*;
use ndarray_linalg::Solve;
use std::{fmt::Debug, ops::RangeInclusive};

use crate::prelude::*;

#[test]
fn test_future_intersections() {
    let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    let range = 7..=27;

    assert_eq!(cal_future_intersections(input, range).unwrap(), 2);
}

#[derive(Clone, Copy, Debug)]
struct Vec3(isize, isize, isize);

#[derive(Clone, Copy, Debug)]
struct Hail {
    pos: Vec3,
    vel: Vec3,
}

impl Hail {
    fn intersect(&self, other: &Self) -> Option<(f64, f64)> {
        // pos + r * vel = other.pos + s * other.vel
        // r * vel - s * other.vel = other.pos - pos
        // A * x = b
        let a = array![
            [self.vel.0 as f64, -other.vel.0 as f64],
            [self.vel.1 as f64, -other.vel.1 as f64]
        ];
        let b = array![
            other.pos.0 as f64 - self.pos.0 as f64,
            other.pos.1 as f64 - self.pos.1 as f64
        ];
        let intersection = match a.solve_into(b) {
            Ok(x) => {
                let r = x[0];
                let _s = x[1];
                let x = self.pos.0 as f64 + r * self.vel.0 as f64;
                let y = self.pos.1 as f64 + r * self.vel.1 as f64;
                // println!("intersection: ({}, {})", x, y);
                Some((x, y))
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                None
            }
        };

        intersection.filter(|(x, _y)| {
            // pos + r * vel = intersection
            // r = (intersection - pos) / vel
            let r = (x - self.pos.0 as f64) / self.vel.0 as f64;
            let s = (x - other.pos.0 as f64) / other.vel.0 as f64;
            // intersection in past
            r > 0.0 && s > 0.0
        })
    }
}

struct Storm {
    hails: Vec<Hail>,
}

impl Storm {
    fn new(input: &str) -> Self {
        let hails = input
            .lines()
            .map(|line| {
                let (pos, vel) = line.split_once(" @ ").unwrap();
                let pos = pos
                    .split(", ")
                    .map(|s| s.trim().parse::<isize>().unwrap())
                    .collect::<Vec<_>>();
                let vel = vel
                    .split(", ")
                    .map(|s| s.trim().parse::<isize>().unwrap())
                    .collect::<Vec<_>>();
                Hail {
                    pos: Vec3(pos[0], pos[1], pos[2]),
                    vel: Vec3(vel[0], vel[1], vel[2]),
                }
            })
            .collect();

        Self { hails }
    }

    fn count_intersections(&self, range: RangeInclusive<f64>) -> usize {
        let mut queue = self.hails.clone();
        let mut count = 0;
        while let Some(hail) = queue.pop() {
            for other in &queue {
                // println!("{:?} {:?}", hail, other);
                if let Some((r, s)) = hail.intersect(other) {
                    // println!("timestamp ({}, {})", r, s);
                    if range.contains(&r) && range.contains(&s) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

pub fn cal_future_intersections(input: &str, range: RangeInclusive<isize>) -> Result<usize> {
    let storm = Storm::new(input);
    let range = *range.start() as f64..=*range.end() as f64;
    let count = storm.count_intersections(range);
    Ok(count)
}
