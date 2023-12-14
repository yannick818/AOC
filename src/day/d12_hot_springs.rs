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
    Unknown
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("invalid char: {}", c)
        }
    }
}

struct Group {
    springs: Vec<Spring>,
}

impl Group {
    /// checks if a group of cnt damaged pipes would fit in this group
    fn could_contain(&self, cnt: usize) -> bool {
        let len = self.springs.len();
        let damaged_cnt = self.springs.iter().filter(|&s| *s == Spring::Damaged).count();
        len >= cnt && damaged_cnt >= cnt
    }
}

struct Record {
    spring_groups: Vec<Group>,
    damaged_group_size: Vec<usize>,
}

impl Record {
    fn parse(input: &str) -> Vec<Record> {
        todo!()
    }

    fn different_arrangements(&self) -> usize {
        todo!("should ne solveable with regex?");
        for group_len in &self.damaged_group_size {
            // self.spring_groups.iter()
            // .fi
            for group in &self.spring_groups {
                if group.could_contain(*group_len) {
                    todo!()
                }
            }
        }
        todo!()
    }
}


pub fn cal_arrangement_sum(input: &str) -> Result<usize> {
    let records = Record::parse(input);

    let sum = records.iter().map(|r| r.different_arrangements()).sum();

    Ok(sum)
}