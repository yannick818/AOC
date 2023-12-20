use std::{collections::VecDeque, fmt::Debug};

use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

#[test]
fn test_cal_arrangement_sum() {
    assert_eq!(21, cal_arrangement_sum(INPUT).unwrap());
}

#[test]
fn test_cal_arrangement_sum_check_last
() {
    let input = ".#.?.#.# 1,1,1";
    assert_eq!(1, cal_arrangement_sum(input).unwrap());
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
    fn parse(input: &str) -> Vec<Record> {
        input
            .lines()
            .map(|line| {
                let (springs, sizes) = line.split_once(' ').unwrap();
                let springs = springs.chars().map(Spring::from).collect();
                let group_sizes = sizes.split(',').map(|num| num.parse().unwrap()).collect();
                Self {
                    springs,
                    group_sizes,
                }
            })
            .collect()
    }

    fn get_remaining(springs: &[Spring], group_size: usize, last: bool) -> Vec<Vec<Spring>> {
        // println!("find {} in {:?}", group_size, springs);
        let mut possibilities = Vec::new();
        if springs.len() < group_size {
            return possibilities;
        }
        for pos in 0..=(springs.len() - group_size) {
            let mut previous = springs.iter().take(pos);
            let mut relevant = springs.iter().skip(pos).take(group_size);
            let next = springs.get(pos + group_size);
            let mut rest = springs.iter().skip(pos + group_size + 1);


            let prev_ok = previous.all(|&spring| spring != Spring::Damaged);
            let relevant_ok = relevant.all(|&spring| spring != Spring::Operational);
            let next_ok = match next {
                Some(&spring) => spring != Spring::Damaged,
                None => true,
            };
            let rest_ok = !last || rest.all(|&spring| spring != Spring::Damaged);

            if prev_ok && relevant_ok && next_ok && rest_ok {
                let remaining = springs
                    .iter()
                    .cloned()
                    // +1 bc the next spring is the placeholder between groups
                    .skip(pos + group_size + 1)
                    .collect::<Vec<_>>();
                possibilities.push(remaining);
            }
        }
        possibilities
    }

    fn count_fits(springs: &[Spring], mut group_size: VecDeque<usize>) -> usize {
        // println!("find {:?} in {:?}", group_size, springs);
        if group_size.is_empty() {
            return 1;
        }
        let last = group_size.len() == 1;
        let size = group_size.pop_front().unwrap();
        let remaining_springs = Self::get_remaining(springs, size, last);
        remaining_springs
            .into_iter()
            .map(|remaining| Self::count_fits(&remaining, group_size.clone()))
            .sum()
    }

    #[allow(clippy::let_and_return)]
    fn different_arrangements(&self) -> usize {
        let groups = VecDeque::from(self.group_sizes.clone());
        let cnt = Self::count_fits(&self.springs, groups);
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

pub fn cal_arrangement_sum(input: &str) -> Result<usize> {
    let records = Record::parse(input);

    let sum = records.iter().map(|r| r.different_arrangements()).sum();

    Ok(sum)
}
