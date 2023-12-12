
use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

#[test]
fn test_next_steps() {
    assert_eq!(114, cal_next_steps(INPUT).unwrap());
}

type Value = i64;

#[derive(Debug, Clone)]
struct Row(Vec<Value>);

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
        let last = self.history.last_mut().unwrap();
        last.0.push(0);

        self.history.iter_mut()
        .rev()
        .skip(1)
        .fold(0_i64, |step, row| {
           let last = row.0.last().unwrap();
           let new = last + step;
            row.0.push(new); 
            new
        })

    }

    fn get_last(&self) -> Value {
        let first_row = self.history.first().unwrap();
        *first_row.0.last().unwrap()
    }
}
impl Row {

    fn cal_next(&self) -> Row {
        self.0.iter()
        .zip(self.0.iter().skip(1))
        .fold(Row(vec![]), |mut row, (v1, v2)| {
            row.0.push(v2 - v1);
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

pub fn cal_next_steps(input: &str) -> Result<Value> {
    let mut histories = input.lines().map(|line| {
        let element = Row::from(line);
        History::new(element)
    })
    .collect::<Vec<_>>();

    histories.iter_mut().for_each(|history| {
        history.predict_next();
    });

    let sum = histories.iter().map(|history| {
        history.get_last()
    }).sum::<Value>();

    Ok(sum)
}

