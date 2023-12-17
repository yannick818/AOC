mod day;
mod prelude;

use crate::day::*;

use crate::prelude::*;

#[allow(unused_imports)]
use std::time::Instant;
use std::{fs::File, io::Read};

fn main() -> Result<()> {

    let input = read_file("input/1.txt")?;
    println!("Day 1.1: {}", d1_trebuchet::cal_trebuchet(&input)?);
    println!("Day 1.2: {}", d1_trebuchet::cal_trebuchet_str(&input)?);

    let input = read_file("input/2.txt")?;
    println!("Day 2.1: {}", d2_cube_conundrum::cal_cubes(&input)?);
    println!("Day 2.2: {}", d2_cube_conundrum::cal_cubes2(&input)?);

    let input = read_file("input/3.txt")?;
    println!("Day 3.1: {}", d3_gear_ratios::cal_gear_ratio(&input)?);
    println!("Day 3.2: {}", d3_gear_ratios::cal_gear_ratio2(&input)?);

    let input = read_file("input/4.txt")?;
    println!("Day 4.1: {}", d4_scratchcards::cal_card_points(&input)?);
    println!("Day 4.2: {}", d4_scratchcards::count_cards(&input)?);

    let input = read_file("input/5.txt")?;
    println!("Day 5.1: {}", d5_fertilizer::cal_lowest_location(&input)?);
    //HACK do a little runner, so that the rest is not blocked; maybe meassure all days
    //30s with par_iter, 115s without 
    // let start = Instant::now();
    // println!("Day 5.2: {}", d5_fertiliter::cal_lowest_loc_ranges(&input)?);
    // let end = Instant::now();
    // println!("time: {:?}", end.duration_since(start));

    let input = read_file("input/6.txt")?;
    println!("Day 6.1: {}", d6_wait_for_it::cal_ways_to_win(&input)?);
    println!("Day 6.2: {}", d6_wait_for_it::cal_ways_to_win2(&input)?);


    let input = read_file("input/7.txt")?;
    println!("Day 7.1: {}", d7_camel_cards::cal_winning_points(&input, false)?); 
    println!("Day 7.2: {}", d7_camel_cards::cal_winning_points(&input, true)?); 

    let input = read_file("input/8.txt")?;
    println!("Day 8.1: {}", d8_haunted_wasteland::cal_steps(&input)?); 
    println!("Day 8.2: {}", d8_haunted_wasteland::cal_steps_simultanious(&input)?); 

    let input = read_file("input/9.txt")?;
    println!("Day 9.1: {}", d9_mirage_maintenance::cal_next_steps(&input)?); 
    println!("Day 9.2: {}", d9_mirage_maintenance::cal_prev_steps(&input)?); 

    let input = read_file("input/10.txt")?;
    println!("Day 10.1: {}", d10_pipe_maze::cal_maze_distance(&input)?); 
    println!("Day 10.2: {}", d10_pipe_maze::cal_enclosed_tiles(&input)?); 

    let input = read_file("input/11.txt")?;
    println!("Day 11.1: {}", d11_cosmic_expansion::cal_sum_of_paths(&input, 1)?); 
    println!("Day 11.2: {}", d11_cosmic_expansion::cal_sum_of_paths(&input, 1_000_000)?); 

    // let input = read_file("input/12.txt")?;
    // println!("Day 12.1: {}", d12_hot_springs::cal_arrangement_sum(&input)?); 
    // println!("Day 12.2: {}", d11_cosmic_expansion::cal_sum_of_paths(&input, 1_000_000)?); 

    let input = read_file("input/13.txt")?;
    println!("Day 13.1: {}", d13_point_of_incidence::cal_reflection_code(&input)?); 
    println!("Day 13.2: {}", d13_point_of_incidence::cal_reflection_code2(&input)?); 

    let input = read_file("input/14.txt")?;
    println!("Day 14.1: {}", d14_parabolic_reflector_dish::cal_total_load(&input)?); 
    println!("Day 14.2: {}", d14_parabolic_reflector_dish::cal_load_after(&input, 1_000_000_000)?); 

    Ok(())
}

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

#[test]
fn test_website() {
    let path = "https://adventofcode.com/2023/day/3/input";
    let input = read_website(path).unwrap();
    // login needed...
    println!("{}", input);
}

#[allow(dead_code)]
fn read_website(url: &str) -> Result<String> {
    let response = reqwest::blocking::get(url)?;
    let input = response.text()?;
    Ok(input)
}

