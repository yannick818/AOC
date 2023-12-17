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
    /// checks if a group of cnt damaged pipes would fit in this group
    fn could_contain(&self, cnt: usize) -> bool {
        let len = self.springs.len();
        let damaged_cnt = self
            .springs
            .iter()
            .filter(|&s| *s == Spring::Damaged)
            .count();
        len >= cnt && damaged_cnt >= cnt
    }

    fn reduce(&mut self, dmg_cnt: usize) -> Self {
        todo!()
    }
}

struct Record {
    spring_groups: Vec<Group>,
    damaged_group_size: Vec<usize>,
    damaged_spring_count: usize,
}

impl Record {
    fn parse(input: &str) -> Vec<Record> {
        todo!()
    }

    fn reduce(&mut self) -> bool {
        for damaged_size in self.damaged_group_size.iter() {
            let fittable_group = self
                .spring_groups
                .iter_mut()
                .filter(|g| g.could_contain(*damaged_size))
                .collect::<Vec<_>>();

            let only_hit = fittable_group.len() == 1;
            if only_hit {
                // let mut group = fittable_group[0];
                // let new_groups = group.reduce(*damaged_size);
                // self.spring_groups.extend(new_groups);
                return true;
            }
        }
        todo!()
    }

    fn different_arrangements(&self) -> usize {
        //^\.*(\?*#{0,1})\.+(\?*#{0,1})\.+(\?*#{0,3})\.*$
        todo!("should be solveable with regex?");
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

#[allow(dead_code)]
pub fn cal_arrangement_sum(input: &str) -> Result<usize> {
    let records = Record::parse(input);

    let sum = records.iter().map(|r| r.different_arrangements()).sum();

    Ok(sum)
}
