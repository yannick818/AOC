use std::{collections::HashSet, fmt::Display};

use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

#[test]
fn test_loose_bricks() {
    assert_eq!(cal_loose_bricks(INPUT).unwrap(), 5);
}

#[test]
fn test_fallings_bricks() {
    assert_eq!(cal_falling_bricks(INPUT).unwrap(), 7);
}

type BrickId = usize;
type Position = (usize, usize, usize);

struct Brick {
    start: Position,
    end: Position,
}

struct Pile {
    bricks: Vec<Brick>,
    pile: Vec<Vec<Vec<Option<BrickId>>>>,
    // under, over
    touching: Vec<(HashSet<BrickId>, HashSet<BrickId>)>,
}

impl Display for Pile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for xview in self.pile.iter() {
            for line in xview.iter() {
                for zview in line.iter() {
                    if let Some(id) = zview {
                        output.push_str(&format!("{}", id));
                    } else {
                        output.push('_');
                    }
                }
                output.push('\n');
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

impl Pile {
    fn new(input: &str) -> Self {
        let mut bricks = input
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('~').unwrap();
                let mut startpos = start.split(',');
                let x = startpos.next().unwrap().parse().unwrap();
                let y = startpos.next().unwrap().parse().unwrap();
                let z = startpos.next().unwrap().parse().unwrap();
                let start = (x, y, z);
                let mut endpos = end.split(',');
                let x = endpos.next().unwrap().parse().unwrap();
                let y = endpos.next().unwrap().parse().unwrap();
                let z = endpos.next().unwrap().parse().unwrap();
                let end = (x, y, z);
                Brick { start, end }
            })
            .collect::<Vec<_>>();
        bricks.sort_by_key(|brick| brick.start.2);

        let max_x = bricks
            .iter()
            .map(|brick| brick.start.0.max(brick.end.0))
            .max()
            .unwrap();
        let max_y = bricks
            .iter()
            .map(|brick| brick.start.1.max(brick.end.1))
            .max()
            .unwrap();
        let max_height = bricks
            .iter()
            .map(|brick| (brick.start.2..brick.end.2).len() + 1)
            .sum();

        let touching = bricks
            .iter()
            .map(|_| (HashSet::new(), HashSet::new()))
            .collect::<Vec<_>>();

        let pile = vec![vec![vec![None; max_height]; max_y + 1]; max_x + 1];
        Self {
            bricks,
            pile,
            touching,
        }
    }

    fn fall(&mut self) {
        let mut highest_tile = 0_usize;

        for (id, brick) in self.bricks.iter().enumerate() {
            //3D Array slice isnt implemented on InclusiveRange
            let xrange = brick.start.0..brick.end.0 + 1;
            let yrange = brick.start.1..brick.end.1 + 1;
            let zrange = brick.start.2..brick.end.2 + 1;

            let mut hit = None;
            for z in (0..=highest_tile).rev() {
                hit = None;
                let xpiles = &self.pile[xrange.clone()];
                for xpile in xpiles {
                    let ypiles = &xpile[yrange.clone()];
                    for ypile in ypiles {
                        if let Some(under_id) = ypile[z] {
                            // println!("{} hits {} at {}", id, under_id, z);
                            self.touching.get_mut(under_id).unwrap().1.insert(id);
                            self.touching.get_mut(id).unwrap().0.insert(under_id);
                            hit = Some(z);
                        }
                    }
                }
                // obove iteration has to finish to set under and over
                if hit.is_some() {
                    break;
                }
            }

            highest_tile = if let Some(hit) = hit {
                let z_offset = hit + 1;
                let zlen = zrange.len();
                for xpile in &mut self.pile[xrange.clone()] {
                    for ypile in &mut xpile[yrange.clone()] {
                        for z in ypile.iter_mut().skip(z_offset).take(zlen) {
                            *z = Some(id);
                        }
                    }
                }
                highest_tile.max(z_offset + zlen)
            } else {
                // if no hit then it falls on ground
                let zlen = zrange.len();
                for xpile in &mut self.pile[xrange.clone()] {
                    for ypile in &mut xpile[yrange.clone()] {
                        for z in ypile.iter_mut().take(zlen) {
                            *z = Some(id);
                        }
                    }
                }
                highest_tile.max(zlen)
            };

            // println!("{}", self);
        }
    }

    fn get_loose(&self) -> Vec<BrickId> {
        let mut loose = Vec::new();
        for (id, (_under, over)) in self.touching.iter().enumerate() {
            let all_stable = over.iter().all(|over_id| {
                let mut other_under = self.touching.get(*over_id).unwrap().0.clone();
                other_under.remove(&id);
                !other_under.is_empty()
            });
            if all_stable {
                loose.push(id);
            }
        }
        loose
    }

    fn get_falling(&self) -> Vec<usize> {
        self.touching
            .iter()
            .enumerate()
            .map(|(id, _)| {
                let falling = self.cal_falling(id, &mut HashSet::new());
                // println!("brick {} falls {}", id, falling);
                falling
            })
            .collect()
    }

    // HACK this would be easy to cache
    fn cal_falling(&self, id: BrickId, falling: &mut HashSet<BrickId>) -> usize {
        falling.insert(id);
        let over = &self.touching.get(id).unwrap().1;
        let mut new_fallings = Vec::new();
        for overid in over {
            let other_under = &self.touching.get(*overid).unwrap().0;
            let is_falling = other_under.difference(falling).count() == 0;
            if is_falling {
                falling.insert(*overid);
                new_fallings.push(*overid);
            }
        }
        for overid in new_fallings {
            self.cal_falling(overid, falling); 
        }
        let falling_wo_self = falling.len() - 1;
        // println!("  brick {} falls {}", id, falling_wo_self);
        falling_wo_self
    }
}

pub fn cal_loose_bricks(input: &str) -> Result<usize> {
    let mut pile = Pile::new(input);
    pile.fall();
    let loose = pile.get_loose();
    Ok(loose.len())
}

pub fn cal_falling_bricks(input: &str) -> Result<usize> {
    let mut pile = Pile::new(input);
    pile.fall();
    let falling = pile.get_falling();
    Ok(falling.iter().sum())
}
