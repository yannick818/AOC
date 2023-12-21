use std::{collections::HashMap, fmt::Debug};

// TODO Day 12.2 Speed improvement
#[allow(unused_imports)]
use rayon::prelude::*;

use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

#[test]
fn test_arrangement_sum() {
    assert_eq!(21, cal_arrangement_sum(INPUT).unwrap());
}

#[test]
fn test_arrangement_sum_check_last() {
    let input = ".#.?.#.# 1,1,1";
    assert_eq!(1, cal_arrangement_sum(input).unwrap());
}

#[test]
fn test_arrangement_sum_folded() {
    assert_eq!(525152, cal_arrangement_sum_folded(INPUT).unwrap());
}

#[test]
#[should_panic]
fn test_slices() {
    let vec = vec![1, 2, 3];
    let slice = &vec[..];
    assert_eq!(slice.len(), 3);
    assert_eq!(&slice[..3], slice); //works

    let slice_large = &vec[..5]; //panics
    assert_eq!(slice_large.len(), 3);
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("invalid char: {}", c),
        }
    }
}

impl Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operational => write!(f, "."),
            Self::Damaged => write!(f, "#"),
            Self::Unknown => write!(f, "?"),
        }
    }
}

struct Record {
    springs: Vec<Spring>,
    group_sizes: Vec<usize>,
}

impl Record {
    fn parse(input: &str, factor: usize) -> Vec<Record> {
        input
            .lines()
            .map(|line| {
                let (springs, sizes) = line.split_once(' ').unwrap();
                let org_springs = springs.chars().map(Spring::from).collect::<Vec<_>>();
                let org_sizes = sizes
                    .split(',')
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<_>>();

                let mut springs = Vec::new();
                let mut sizes = Vec::new();
                for _ in 0..factor {
                    springs.append(&mut org_springs.clone());
                    springs.push(Spring::Unknown);
                    sizes.append(&mut org_sizes.clone());
                }
                springs.pop();

                Self {
                    springs,
                    group_sizes: sizes,
                }
            })
            .collect()
    }

    // crazy... https://github.com/Domyy95/Challenges/blob/master/2023-12-Advent-of-code/12.py this runs in just a blink
    // @cache stores input and outputs and reuses them
    // without a cache this function needs multible hours
    fn count_fits(
        springs: &[Spring],
        sizes: &[usize],
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        // println!("find {:?} in {:?}", sizes, springs);
        match (springs.is_empty(), sizes.is_empty()) {
            (false, false) => {}
            (true, true) => return 1,
            (true, false) => return 0,
            (false, true) => {
                if springs.contains(&Spring::Damaged) {
                    return 0;
                } else {
                    return 1;
                }
            }
        }

        if let Some(count) = cache.get(&(springs.len(), sizes.len())) {
            return *count;
        }

        let mut sum = 0;
        let first = springs.first().unwrap();

        if *first != Spring::Damaged {
            // treat first spring as functional
            sum += Self::count_fits(&springs[1..], sizes, cache);
        }
        if *first != Spring::Operational {
            // try to fit group at start
            let group_len = sizes[0];
            if springs.len() >= group_len {
                let relevant = &springs[..group_len];
                let next = springs.get(group_len);
                let relevant_ok = !relevant.contains(&Spring::Operational);
                let next_ok = match next {
                    None => true,
                    Some(next_spring) => next_spring != &Spring::Damaged,
                };
                if relevant_ok && next_ok {
                    let is_last = next.is_none();
                    let start_index = if is_last { group_len } else { group_len + 1 };
                    sum += Self::count_fits(&springs[start_index..], &sizes[1..], cache);
                }
            }
        }
        cache.insert((springs.len(), sizes.len()), sum);
        sum
    }

    #[allow(clippy::let_and_return)]
    fn different_arrangements(&self) -> usize {
        let cnt = Self::count_fits(&self.springs, &self.group_sizes, &mut HashMap::new());
        // let groups = format!("{:?}", self.group_sizes);
        // let springs = self
        //     .springs
        //     .iter()
        //     .map(|spring| match spring {
        //         Spring::Operational => '.',
        //         Spring::Damaged => '#',
        //         Spring::Unknown => '?',
        //     })
        //     .collect::<String>();
        // println!("{:>20} fits in {:>20} {} ways", groups, springs, cnt);
        cnt
    }
}

//https://www.reddit.com/r/adventofcode/comments/18gozoj/2023_day_12_part_1_rust_i_have_no_idea_where_to/
pub fn cal_arrangement_sum(input: &str) -> Result<usize> {
    let records = Record::parse(input, 1);
    let sum = records.iter().map(|r| r.different_arrangements()).sum();
    Ok(sum)
}

pub fn cal_arrangement_sum_folded(input: &str) -> Result<usize> {
    let records = Record::parse(input, 5);
    let sum = records
        .iter()
        .map(|record| record.different_arrangements())
        .sum();
    Ok(sum)
}
