//TODO Implement D17
#![allow(dead_code, unused)]

use array2d::Array2D;

use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

#[test]
fn test_min_heat_loss() {
    assert_eq!(102, cal_minimum_heat_loss(INPUT).unwrap());
}

struct Map {
    map: Array2D<usize>,
}

impl Map {
    fn parse(input: &str) -> Self {
        todo!("idk how to find the minimum... iterating over all possibilities cant be the solution")
    }
}

pub fn cal_minimum_heat_loss(input: &str) -> Result<usize> {
    let map = Map::parse(input);
    todo!()
}