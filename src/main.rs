mod day;
mod prelude;

use crate::day::d1_trebuchet::*;
use crate::day::d2_cube_conundrum::*;
use crate::day::d3_gear_ratios::*;
use crate::day::d4_scratchcards::*;
use crate::day::d5_fertilizer::*;
use crate::day::d6_wait_for_it::*;
use crate::day::d7_camel_cards::*;

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
    println!("Day 3.2: {}", cal_gear_ratio2(&input)?);

    let input = read_file("input/4.txt")?;
    println!("Day 4.1: {}", cal_card_points(&input)?);
    println!("Day 4.2: {}", count_cards(&input)?);

    let input = read_file("input/5.txt")?;
    println!("Day 5.1: {}", cal_lowest_location(&input)?);
    //is slow...
    // println!("Day 5.2: {}", cal_lowest_loc_ranges(&input)?);

    let input = read_file("input/6.txt")?;
    println!("Day 6.1: {}", cal_ways_to_win(&input)?);
    println!("Day 6.2: {}", cal_ways_to_win2(&input)?);

    let input = read_file("input/7.txt")?;
    println!("Day 7.1: {}", cal_winning_points(&input)?);
    // println!("Day 6.2: {}", cal_ways_to_win2(&input)?);

    Ok(())
}

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

