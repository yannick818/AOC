mod day;
mod prelude;

use crate::day::d1_trebuchet::*;
use crate::day::d2_cube_conundrum::*;
use crate::day::d3_gear_ratios::*;
use crate::day::d4_scratchcards::*;

use crate::prelude::*;

use std::{fs::File, io::Read};

fn main() -> Result<()> {
    let input = read_file("input/1.txt")?;
    println!("Day 1.1: {}", cal_trebuchet(&input)?);
    println!("Day 1.2: {}", cal_trebuchet_str(&input)?);

    let input = read_file("input/2.txt")?;
    println!("Day 2.1: {}", cal_cubes(&input)?);
    println!("Day 2.2: {}", cal_cubes2(&input)?);

    let input = read_file("input/3.txt")?;
    println!("Day 3.1: {}", cal_gear_ratio(&input)?);
    // println!("Day 2.2: {}", cal_cubes2(&input)?);
    
    let input = read_file("input/4.txt")?;
    println!("Day 4.1: {}",cal_card_points(&input)?);
    println!("Day 4.2: {}", count_cards(&input)?);
    
    Ok(())
}

fn read_file(path: &str) -> Result<String> {

    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}