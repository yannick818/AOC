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
    let sum = input.lines().try_fold(0_u32, |mut sum, line| {
        sum += cal_line(line)?;
        Ok::<u32, AocError>(sum)
    })?;
    Ok(sum)
}

fn cal_line(line: &str) -> Result<u32> {
    let numbers = line
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    let first = numbers.first().ok_or("no first")?.ok_or("no parsable digit")?.to_string();
    let last = numbers.last().ok_or("no last")?.ok_or("no parsable digit")?.to_string();
    let sum = first + &last;

    Ok(sum.parse::<u32>()?)
}
