use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

#[test]
fn test_cal_hash_sum() {
    assert_eq!(1320, cal_hash_sum(INPUT).unwrap());
}

struct Step {
    input: String,
}

impl Step {
    fn parse(input: &str) -> Vec<Self> {
        input
            .split(',')
            .map(|s| Step {
                input: s.to_string(),
            })
            .collect()
    }

    fn hash(&self) -> usize {
        self.input
            .chars()
            .map(|c| {
                let utf8 = c as u32;
                //utf8 == ascii for this input
                utf8 as u8
            })
            .fold(0, |hash, ascii| {
                let hash = hash + ascii as usize;
                let hash = hash * 17;
                hash % 256
            })
    }
}

pub fn cal_hash_sum(input: &str) -> Result<usize> {
    let steps = Step::parse(input);
    let sum = steps.iter().map(|s| s.hash()).sum();
    Ok(sum)
}
