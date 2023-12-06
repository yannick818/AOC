use std::{usize, ops::Range};

use crate::prelude::*;


#[test]
fn test_gear_ratios() {
    let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

    assert_eq!(4361, cal_gear_ratio(input).unwrap());
}

struct Number {
    value: u32,
    x: usize,
    y: Range<usize>,
}

impl Number {
    fn new(value: u32, x: usize, y: Range<usize>) -> Self {
        Self {
            value,
            x,
            y,
        }
    }

    fn touches_symbol(&self, symbols: &[Symbol]) -> bool {
        let x_range = (self.x.saturating_sub(1))..=(self.x+1);
        let y_range = (self.y.start.saturating_sub(1))..=(self.y.end+1);

        symbols.iter().any(|symbol| {
            matches!((x_range.contains(&symbol.x), y_range.contains(&symbol.y)), (true, true))
        })
        
    }
}

struct Symbol{
    x: usize,
    y: usize,
}


pub fn cal_gear_ratio(input: &str) -> Result<u32> {
    
    let (numbers, symbols) = transform_input(input)?;

    let sum = numbers.into_iter().filter_map(|number| {
        if number.touches_symbol(&symbols) {
            Some(number.value)
        } else {
            None
        }
    })
    .sum();

    Ok(sum)
}

fn transform_input(input: &str) -> Result<(Vec<Number>, Vec<Symbol>)> {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    input.lines().enumerate().for_each(|(x, line)| {
        let elements = line.split('.').filter(|str| !str.is_empty());
        for element in elements {
            match element.parse::<u32>() {
                Ok(value) => {
                    let str = value.to_string();
                    let pos = line.find(&str).unwrap();
                    let len = str.len();
                    let number = Number::new(value, x, pos..(pos+len));
                    numbers.push(number);
                }
                Err(_) => {
                    let y = line.find(element).unwrap();
                    symbols.push(Symbol{x, y});
                }
            }
        }
    });

    Ok((numbers, symbols))
}