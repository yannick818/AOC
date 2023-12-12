
use std::collections::VecDeque;

use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

#[test]
fn test_next_steps() {
    assert_eq!(114, cal_next_steps(INPUT).unwrap());
}

#[test]
fn test_prev_steps() {
    assert_eq!(2, cal_prev_steps(INPUT).unwrap());
}

type Value = i64;

#[derive(Debug, Clone)]
struct Row(VecDeque<Value>);

struct History {
    history: Vec<Row>,
}

impl History {

    fn new(row: Row) -> Self {
        let mut history = vec![row];
        while !history.last().unwrap().is_zero() {
            history.push(history.last().unwrap().cal_next());
        }

        Self { history }
    }

    fn predict_next(&mut self) -> Value {
        let zeros = self.history.last_mut().unwrap();
        zeros.0.push_back(0);

        self.history.iter_mut()
        .rev()
        .skip(1)
        .fold(0_i64, |step, row| {
           let last = row.0.back().unwrap();
           let new = last + step;
            row.0.push_back(new); 
            new
        })
    }

    fn predict_prev(&mut self) -> Value {
        let zeros = self.history.last_mut().unwrap();
        zeros.0.push_back(0);

        self.history.iter_mut()
        .rev()
        .skip(1)
        .fold(0_i64, |step, row| {
           let first = row.0.front().unwrap();
           let new = first - step;
            row.0.push_front(new); 
            new
        })
    }

    fn get_last(&self) -> Value {
        let first_row = self.history.first().unwrap();
        *first_row.0.back().unwrap()
    }

    fn get_first(&self) -> Value {
        let first_row = self.history.first().unwrap();
        *first_row.0.front().unwrap()
    }
}
impl Row {

    fn cal_next(&self) -> Row {
        self.0.iter()
        .zip(self.0.iter().skip(1))
        .fold(Row(VecDeque::new()), |mut row, (v1, v2)| {
            row.0.push_back(v2 - v1);
            row
        })
    }

    fn is_zero(&self) -> bool {
       self.0.iter().all(|&v| v == 0) 
    }
}

impl From<&str> for Row {
    fn from(input: &str) -> Self {
        let numbers = input.split(' ').map(|s| s.parse().unwrap()).collect();
        Self (numbers) 
    }
}

fn parse_history(input: &str) -> Vec<History> {
    input.lines().map(|line| {
        let element = Row::from(line);
        History::new(element)
    })
    .collect()
}

pub fn cal_next_steps(input: &str) -> Result<Value> {
    let mut histories = parse_history(input);

    histories.iter_mut().for_each(|history| {
        history.predict_next();
    });

    let sum = histories.iter().map(|history| {
        history.get_last()
    }).sum::<Value>();

    Ok(sum)
}

pub fn cal_prev_steps(input: &str) -> Result<Value> {
    let mut histories = parse_history(input);

    histories.iter_mut().for_each(|history| {
        history.predict_prev();
    });

    let sum = histories.iter().map(|history| {
        history.get_first()
    }).sum::<Value>();

    Ok(sum)
}
