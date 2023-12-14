use std::collections::HashSet;

use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = "...#......
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
fn test_sum_of_paths() {
    assert_eq!(374, cal_sum_of_paths(INPUT).unwrap());
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
    x: usize,
    y: usize,

}

impl Coordinate {
    fn distance(&self, other: &Self) -> usize {
        let dx = self.x as isize - other.x as isize;
        let dy = self.y as isize - other.y as isize;
        (dx.abs() + dy.abs()) as usize
    }
}

struct Universe {
    elements: Vec<Vec<Element>>,
}

impl Universe {
    fn expand(&mut self) {
        let old_universe = self.elements.clone();
        // double empty rows
        old_universe.iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&e| e == Element::Empty))
        .rev() //Backwards to dont mess up indices
        .for_each(|(index, empty_row)| {
            self.elements.insert(index, vec![Element::Empty; empty_row.len()]);
        });
        
        // double empty columns
        let col_len = self.elements[0].len();
        (0..col_len)
        .map(|col_idx| {
            old_universe.iter()
            .map(move |row| row[col_idx])
        })
        .enumerate()
        .filter_map(|(col_idx, mut col)| {
            let is_empty = col.all(|e| e == Element::Empty);
            if is_empty {
                Some(col_idx)
            } else {
                None
            }
        })
        .rev() //Backwards to dont mess up indices
        .for_each(|empty_col_idx| {
            self.elements.iter_mut().for_each(|row| row.insert(empty_col_idx, Element::Empty));
        });

    }

    fn get_galaxies(&self) -> HashSet<Coordinate> {
        self.elements.iter()
        .enumerate()
        .fold(HashSet::new(), |mut galaxies, (y, row)| {
            row.iter()
            .enumerate()
            .filter(|(_, &e)| e == Element::Galaxy)
            .for_each(|(x, _)| {
                galaxies.insert(Coordinate {x, y});
            });
            galaxies
        })
    }
}

impl From<&str> for Universe {
    fn from(s: &str) -> Self {
        let elements = s.lines()
        .map(|line| {
            line.chars()
            .map(Element::from)
            .collect()
        })
        .collect();

        Self {
            elements
        }
    }
}

fn get_combinations(galaxies: &HashSet<Coordinate>) -> HashSet<(Coordinate, Coordinate)> {
    let (combinations, _) = galaxies.iter()
    .fold((HashSet::new(), galaxies.clone()), |(mut comb, mut galaxies), galaxy| {
        galaxies.remove(galaxy);
        galaxies.iter()
        .for_each(|other_galaxy| {
            comb.insert((*galaxy, *other_galaxy));
        });
        (comb, galaxies)
    });
    combinations
}

pub fn cal_sum_of_paths(input: &str) -> Result<usize> {
    let mut universe = Universe::from(input);
    universe.expand();
    let galaxies = universe.get_galaxies();

    let sum = get_combinations(&galaxies).iter()
    .map(|(galaxy, other_galaxy)| galaxy.distance(other_galaxy))
    .sum();
    Ok(sum)
}