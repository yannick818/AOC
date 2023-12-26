use array2d::Array2D;

use crate::prelude::*;

#[test]
fn test_plot_count() {
    let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    assert_eq!(cal_plot_count(input, 6).unwrap(), 16);
}

#[derive(Clone, Copy)]
struct Position(usize, usize);
type Seated = bool;

#[derive(Clone, Copy)]
enum Field {
    Rock,
    Garden(Seated),
    Start(Seated),
}

struct Map {
    fields: Array2D<Field>,
}

impl Map {
    fn step(&mut self, steps: usize) {
        for _ in 0..steps {
            // find positions of occupied seats
            let seats = self
                .fields
                .rows_iter()
                .enumerate()
                .flat_map(|(row_no, row)| {
                    row.enumerate()
                        .filter_map(move |(col_no, field)| match field {
                            Field::Garden(true) => Some(Position(row_no, col_no)),
                            Field::Start(true) => Some(Position(row_no, col_no)),
                            _ => None,
                        })
                })
                .collect::<Vec<_>>();

            // clear all old seats
            for Position(row, col) in seats.iter() {
                let field = self.fields.get_mut(*row, *col).unwrap();
                match field {
                    Field::Garden(seat) => *seat = false,
                    Field::Start(seat) => *seat = false,
                    _ => (),
                }
            }

            // place new seats
            seats
                .into_iter()
                .flat_map(|Position(row, col)| {
                    let mut seats = Vec::new();
                    if let Some(top_row) = row.checked_sub(1) {
                        seats.push(Position(top_row, col));
                    }
                    seats.push(Position(row + 1, col));
                    if let Some(left_col) = col.checked_sub(1) {
                        seats.push(Position(row, left_col));
                    }
                    seats.push(Position(row, col + 1));
                    seats
                })
                .for_each(|Position(row, col)| {
                    if let Some(field) = self.fields.get_mut(row, col) {
                        match field {
                            Field::Garden(seat) => *seat = true,
                            Field::Start(seat) => *seat = true,
                            _ => (),
                        }
                    }
                })
        }
    }

    fn count_seated(&self) -> usize {
        self.fields.elements_row_major_iter()
        .filter(|field| matches!(field, Field::Garden(true) | Field::Start(true)))
        .count()
    }

    fn new(input: &str) -> Self {
        let rows = input.lines()
        .map(|line| {
            line.chars()
            .map(|c| match c {
                '#' => Field::Rock,
                '.' => Field::Garden(false),
                'S' => Field::Start(true),
                _ => panic!("invalid input {}", c),
            })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

        Self {
            fields: Array2D::from_rows(&rows).unwrap(),
        }
    }
}

pub fn cal_plot_count(input: &str, steps: usize) -> Result<usize> {
    let mut map = Map::new(input);
    map.step(steps);
    Ok(map.count_seated())
}
