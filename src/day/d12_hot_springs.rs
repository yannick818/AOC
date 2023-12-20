use std::{collections::VecDeque, fmt::Debug};

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

    fn get_remaining(springs: &[Spring], group_size: usize, remaining: usize) -> Vec<Vec<Spring>> {
        // println!("find {} in {:?}", group_size, springs);
        let mut possibilities = Vec::new();
        if springs.len() < group_size {
            return possibilities;
        }
        for pos in 0..=(springs.len() - group_size) {
            // println!("{}", pos);
            // previous cannot have a damaged spring
            let mut previous = springs.iter().take(pos);
            let prev_ok = previous.all(|&spring| spring != Spring::Damaged);
            if !prev_ok {
                // println!("prev {}", pos);
                break;
            }

            let mut relevant = springs.iter().skip(pos).take(group_size);
            let next = springs.get(pos + group_size);
            let rest = springs.iter().skip(pos + group_size + 1);

            let relevant_ok = relevant.all(|&spring| spring != Spring::Operational);
            let next_ok = match next {
                Some(&spring) => spring != Spring::Damaged,
                None => true,
            };
            // rest cannot have too much damaged springs left
            let rest_ok = remaining >= rest.clone().filter(|&&spring| spring == Spring::Damaged).count();
            
            // rest should have at enough ? and #
            let rest_cnt = rest.clone().filter(|&&spring| spring != Spring::Operational).count();
            if rest_cnt < remaining {
                // println!("rest {}", pos);
                break;
            }

            if prev_ok && relevant_ok && next_ok && rest_ok {
                possibilities.push(rest.cloned().collect());
            }
        }
        possibilities
    }

    fn count_fits(springs: &[Spring], mut group_size: VecDeque<usize>) -> usize {
        // println!("find {:?} in {:?}", group_size, springs);
        if group_size.is_empty() {
            return 1;
        }
        let size = group_size.pop_front().unwrap();
        let remaining = group_size.iter().sum();
        let remaining_springs = Self::get_remaining(springs, size, remaining);
        remaining_springs
            .into_iter()
            .map(|remaining| Self::count_fits(&remaining, group_size.clone()))
            .sum()
    }

    #[allow(clippy::let_and_return)]
    fn different_arrangements(&self) -> usize {
        let groups = VecDeque::from(self.group_sizes.clone());
        // println!("test {:?} with {:?}", self.springs, self.group_sizes);
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

//https://www.reddit.com/r/adventofcode/comments/18gozoj/2023_day_12_part_1_rust_i_have_no_idea_where_to/
pub fn cal_arrangement_sum(input: &str) -> Result<usize> {
    let records = Record::parse(input, 1);
    let sum = records.iter().map(|r| r.different_arrangements()).sum();
    Ok(sum)
}

#[allow(dead_code)]
pub fn cal_arrangement_sum_folded(input: &str) -> Result<usize> {
    let records = Record::parse(input, 5);
    let len = records.len();
    let mut cnt = 0;
    let mut sum = 0;
    for record in records.iter() {
        let poss = record.different_arrangements();
        sum += poss;
        cnt += 1;
        println!("{}/{}: {}", cnt, len, poss);    
    }
    Ok(sum)
}
