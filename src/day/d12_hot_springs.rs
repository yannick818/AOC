// TODO remove these
// #![allow(dead_code)]
// #![allow(unused)]

use std::collections::VecDeque;

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

#[derive(Debug, PartialEq, Eq)]
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

struct Group {
    springs: Vec<Spring>,
}

impl Group {

}

struct Record {
    springs: Vec<Spring>,
    group_sizes: Vec<usize>,
}

impl Record {
    fn parse(input: &str) -> Vec<Record> {
        todo!()
    }

    fn split_first(springs: &[Spring]) -> (Vec<Spring>, Vec<Spring>) {
        todo!()
    }

    fn get_possibilities(springs: &[Spring], group_size: usize) -> Vec<Vec<Spring>> {
        todo!()
    }

    fn all_fits(springs: &[Spring], group_size: VecDeque<usize>) -> Vec<Vec<Spring>> {
        let possibilities = Self::get_possibilities(springs, group_size[0]);
        todo!()
    }

    fn different_arrangements(&self) -> usize {
        let mut groups = VecDeque::from(self.group_sizes.clone());
        while let Some(group_size) = groups.pop_front() {
            let fits = Self::all_fits(&self.springs, group_size);
            
        } 
        for &group_size in self.damaged_group_size.iter() {
            let fits = Self::all_fits(&self.springs, group_size);
            for 


        }



        //^\.*(\?*#{0,1})\.+(\?*#{0,1})\.+(\?*#{0,3})\.*$
        todo!("should be solveable with regex?");
        // for group_len in &self.damaged_group_size {
        //     // self.spring_groups.iter()
        //     // .fi
        //     for group in &self.spring_groups {
        //         if group.could_contain(*group_len) {
        //             todo!()
        //         }
        //     }
        // }
    }
}

pub fn cal_arrangement_sum(input: &str) -> Result<usize> {
    let records = Record::parse(input);

    let sum = records.iter().map(|r| r.different_arrangements()).sum();

    Ok(sum)
}
