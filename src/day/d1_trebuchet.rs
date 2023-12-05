use enum_iterator::{all, Sequence};

use crate::prelude::*;

#[test]
fn test_trebuchet() {
    let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    assert_eq!(142, cal_trebuchet(input).unwrap());
}

#[test]
fn test_trebuchet_2() {
    let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

    assert_eq!(281, cal_trebuchet_str(input).unwrap());
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

    let first = numbers
        .first()
        .ok_or("no first")?
        .ok_or("no parsable digit")?
        .to_string();
    let last = numbers
        .last()
        .ok_or("no last")?
        .ok_or("no parsable digit")?
        .to_string();
    let sum = first + &last;

    Ok(sum.parse::<u32>()?)
}

pub fn cal_trebuchet_str(input: &str) -> Result<u32> {
    let sum = input.lines().try_fold(0_u32, |mut sum, line| {
        sum += cal_line_str(line)?;
        Ok::<u32, AocError>(sum)
    })?;
    Ok(sum)
}

fn cal_line_str(line: &str) -> Result<u32> {

    let first = find(line, true)?.to_string();
    let last = find(line, false)?.to_string();
    let sum = first + &last;

    Ok(sum.parse::<u32>()?)
}

#[derive(Sequence)]
enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,

    Ione,
    Itwo,
    Ithree,
    Ifour,
    Ifive,
    Isix,
    Iseven,
    Ieight,
    Inine,
}

impl Number {
    fn as_int(&self) -> u8 {
        match self {
            Number::One | Number::Ione => 1,
            Number::Two | Number::Itwo => 2,
            Number::Three | Number::Ithree => 3,
            Number::Four | Number::Ifour => 4,
            Number::Five | Number::Ifive => 5,
            Number::Six | Number::Isix => 6,
            Number::Seven | Number::Iseven => 7,
            Number::Eight | Number::Ieight => 8,
            Number::Nine | Number::Inine => 9,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Number::One => "one",
            Number::Two => "two",
            Number::Three => "three",
            Number::Four => "four",
            Number::Five => "five",
            Number::Six => "six",
            Number::Seven => "seven",
            Number::Eight => "eight",
            Number::Nine => "nine",
            Number::Ione => "1",
            Number::Itwo => "2",
            Number::Ithree => "3",
            Number::Ifour => "4",
            Number::Ifive => "5",
            Number::Isix => "6",
            Number::Iseven => "7",
            Number::Ieight => "8",
            Number::Inine => "9",
        }
    }
}

fn find(line: &str, first: bool) -> Result<u8> {
    let res = if first {
        all::<Number>()
            .filter_map(|n| line.find(n.as_str()).map(|pos| (pos, n)))
            .min_by(|(minpos, _), (pos, _)| minpos.cmp(pos))
    } else {
        all::<Number>()
            .filter_map(|n| line.rfind(n.as_str()).map(|pos| (pos, n)))
            .max_by(|(maxpos, _), (pos, _)| maxpos.cmp(pos))
    };

    match res {
        Some((_, number)) => Ok(number.as_int()),
        None => Err("no number found".into()),
    }
}
