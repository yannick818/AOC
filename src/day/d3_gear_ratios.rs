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

#[derive(Debug)]
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

    fn touches_symbol(&self, symbols: &[Symbol], input: &str) -> bool {
        let x_range = (self.x.saturating_sub(1))..=(self.x+1);
        let y_range = (self.y.start.saturating_sub(1))..(self.y.end+1);

        let is_touching = symbols.iter().any(|symbol| {
            x_range.contains(&symbol.x) && y_range.contains(&symbol.y)
        });

        println!("{} {:#?}", is_touching, self);
        // Print Area for visualisation
        let area = input.lines().enumerate()
        .filter_map(|(x, line)| {
            x_range.contains(&x).then_some(line)
        })
        .map(|line| {
            &line[y_range.clone()]
        })
        .collect::<Vec<_>>();
        
        eprintln!("{} {:#?}", is_touching, area);

        is_touching
        
    }
}

struct Symbol{
    x: usize,
    y: usize,
}


pub fn cal_gear_ratio(input: &str) -> Result<u32> {
    
    let (numbers, symbols) = transform_input(input);

    let sum = numbers.into_iter().filter_map(|number| {
        number.touches_symbol(&symbols, input).then_some(number.value)
    })
    .sum();

    Ok(sum)
}

fn transform_input(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    input.lines().map(|line| line.trim()).enumerate().for_each(|(x, line)| {
        // println!("Line: {line}");
        let mut num_buffer = String::new();
        line.chars().enumerate().for_each(|(y, char)| {
            let mut save_num = || {

                if !num_buffer.is_empty() {
                    let value = num_buffer.parse::<u32>().unwrap();
                    let len = num_buffer.len();
                    let number = Number::new(value, x, (y-len)..(y));
                    numbers.push(number);
                    num_buffer.clear();
                }
            };
            match (char, char.is_numeric()) {
                ('.', _)  => {
                    save_num();
                },
                (digit, true) => {
                    num_buffer.push(digit);
                },
                (_symbol, false) => {
                    symbols.push(Symbol{x, y});
                    save_num();
                },
            }
        });
    });

    (numbers, symbols)
}