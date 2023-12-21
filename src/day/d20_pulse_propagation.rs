use crate::prelude::*;

#[test]
fn test_pulses() {
    let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    assert_eq!(cal_pulses(input).unwrap(), 32000000);
}

#[test]
fn test_pulses2() {
    let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    assert_eq!(cal_pulses(input).unwrap(), 11687500);
}

pub fn cal_pulses(input: &str) -> Result<usize> {
    todo!()
}
