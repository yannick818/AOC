use std::{collections::{HashSet, HashMap}, fmt::Debug};

use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = 
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

#[test]
fn test_sum_of_paths_2() {
    assert_eq!(374, cal_sum_of_paths(INPUT, 2).unwrap());
}

#[test]
fn test_sum_of_paths_10() {
    assert_eq!(1030, cal_sum_of_paths(INPUT, 10).unwrap());
}

#[test]
fn test_sum_of_paths_100() {
    assert_eq!(8410, cal_sum_of_paths(INPUT, 100).unwrap());
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Element {
    Empty, 
    Galaxy,
}

impl From<char> for Element {
    fn from(c: char) -> Self {
        match c {
            '.' => Element::Empty,
            '#' => Element::Galaxy,
            _ => panic!("invalid char"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    row: usize,
    col: usize,

}

impl Coordinate {
    fn distance(&self, other: &Self) -> usize {
        let dx = self.row as isize - other.row as isize;
        let dy = self.col as isize - other.col as isize;
        (dx.abs() + dy.abs()) as usize
    }
}

type Start = Coordinate;
struct Universe {
    row_len: usize,
    col_len: usize,
    galaxies: HashMap<Start, Coordinate>,
}

impl Universe {
    fn expand(&mut self, expansion_rate: usize) {
        (0..self.row_len).for_each(|row_idx| {
            let is_empty = !self.galaxies.iter()
            .any(|(start_coord, _)| start_coord.row == row_idx);

            if is_empty {
                self.galaxies.iter_mut()
                .filter(|(start, _)| start.row > row_idx)
                .for_each(|(_, moved_coord)| moved_coord.row += expansion_rate - 1);
            }
        });
        
        (0..self.col_len).for_each(|col_idx| {
            let is_empty = !self.galaxies.iter()
            .any(|(start_coord, _)| start_coord.col == col_idx);

            if is_empty {
                self.galaxies.iter_mut()
                .filter(|(start, _)| start.col > col_idx)
                .for_each(|(_, moved_coord)| moved_coord.col += expansion_rate - 1);
            }
        });

    }

    fn get_combinations(&self) -> HashSet<(Coordinate, Coordinate)> {
        let new_locations = self.galaxies.values().cloned().collect::<HashSet<_>>();
        let (combinations, _) = self.galaxies
        .values()
        .fold((HashSet::new(), new_locations), |(mut combinations, mut galaxies), galaxy| {
            galaxies.remove(galaxy);
            galaxies.iter()
            .for_each(|other_galaxy| {
                combinations.insert((*galaxy, *other_galaxy));
            });
            (combinations, galaxies)
        });
        combinations
    }
}

impl Debug for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut galaxies = self.galaxies.iter().collect::<Vec<_>>();
        galaxies.sort_by_key(|(start, _)| start.row);

        for (start, coord) in galaxies {
            writeln!(f, "({}, {}) -> ({}, {})", start.row, start.col, coord.row, coord.col)?;
        };
        Ok(())
    }
}

impl From<&str> for Universe {
    fn from(input: &str) -> Self {
       let galaxies = input.lines()
       .enumerate() 
       .flat_map(|(col_idx, line)| {
            line.chars()
            .enumerate()
            .map(move |(row_idx, c)| {
                let coord = Coordinate {
                    row: row_idx,
                    col: col_idx,
                };
                (Element::from(c), coord)
            })
       })
        .filter(|(element, _)| *element == Element::Galaxy)
        .map(|(_, coord)| (coord, coord))
        .collect::<HashMap<_, _>>();

        Self {
            row_len: input.lines().count(),
            col_len: input.lines().next().unwrap().len(),
            galaxies,
        }
    }
}


pub fn cal_sum_of_paths(input: &str, expansion_rate: usize) -> Result<usize> {
    let mut universe = Universe::from(input);

    // println!("{:?}", universe);

    universe.expand(expansion_rate);

    // println!("{:?}", universe);
    
    let sum = universe.get_combinations().iter()
    .map(|(galaxy, other_galaxy)| galaxy.distance(other_galaxy))
    .sum();
    Ok(sum)
}