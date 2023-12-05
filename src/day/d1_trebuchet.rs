use crate::prelude::*;

#[test]
fn test_trebuchet() {
    let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    assert_eq!(142, cal_trebuchet(input).unwrap());
}

pub fn cal_trebuchet(input: &str) -> Result<u32> {
    let sum = input.split('\n').map(cal_line).sum();
    Ok(sum)
}

fn cal_line(line: &str) -> u32 {
    let numbers = line
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let first = numbers.first().unwrap().to_string();
    let last = numbers.last().unwrap().to_string();
    let sum = first + &last;

    sum.parse::<u32>().unwrap()
}
