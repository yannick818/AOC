use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

#[test]
fn test_cal_hash_sum() {
    assert_eq!(1320, cal_hash_sum(INPUT).unwrap());
}

#[test]
fn test_focus_power() {
    assert_eq!(145, cal_focus_power(INPUT).unwrap());
}

#[derive(Clone)]
enum Operation {
    Remove,
    FocalLen(u8),
}

#[derive(Clone)]
struct Step {
    input: String,
    label: String,
    operation: Operation,
}

impl Step {
    fn parse(input: &str) -> Vec<Self> {
        input
            .split(',')
            .map(|s| {
                if s.contains('-') {
                    let label = s.split('-').next().unwrap();
                    Self {
                        input: s.to_owned(),
                        label: label.to_owned(),
                        operation: Operation::Remove,
                    }
                } else {
                    let (label, focal) = s.split_once('=').unwrap();
                    Self {
                        input: s.to_owned(),
                        label: label.to_owned(),
                        operation: Operation::FocalLen(focal.parse().unwrap()),
                    }
                }
            })
            .collect()
    }

    fn hash(input: &str) -> u8 {
        input
            .chars()
            .map(|c| {
                let utf8 = c as u32;
                //utf8 == ascii for this input
                utf8 as u8
            })
            .fold(0_usize, |hash, ascii| {
                let hash = hash + ascii as usize;
                let hash = hash * 17;
                hash % 256
            }) as u8
    }

    fn full_hash(&self) -> u8 {
        Self::hash(&self.input)
    }

    fn label_hash(&self) -> u8 {
        Self::hash(&self.label)
    }
}

pub fn cal_hash_sum(input: &str) -> Result<usize> {
    let steps = Step::parse(input);
    let sum = steps.iter().map(|s| s.full_hash() as usize).sum();
    Ok(sum)
}

pub fn cal_focus_power(input: &str) -> Result<usize> {
    let steps = Step::parse(input);
    //+1 because 0..256
    let mut boxes: Vec<Vec<Step>> = vec![Vec::new(); u8::MAX as usize + 1];
    for step in steps {
        let hash = step.label_hash();
        let mut old_box = boxes[hash as usize].clone();
        let new_box = match step.operation {
            Operation::Remove => old_box
                .into_iter()
                .filter(|s| s.label != step.label)
                .collect(),
            Operation::FocalLen(len) => {
                let lens = old_box.iter_mut().find(|s| s.label == step.label);
                if let Some(mut step) = lens {
                    step.operation = Operation::FocalLen(len);
                } else {
                    old_box.push(step);
                }
                old_box
            }
        };
        boxes[hash as usize] = new_box;
    }

    let result = boxes
        .iter()
        .enumerate()
        .flat_map(|(box_no, boxx)| {
            boxx.iter().enumerate().map(move |(slot_no, slot)| {
                let focal_len = match slot.operation {
                    Operation::FocalLen(len) => len,
                    _ => panic!("remove operations not allowed"),
                };
                (box_no + 1) * (slot_no + 1) * focal_len as usize
            })
        })
        .sum();
    Ok(result)
}
