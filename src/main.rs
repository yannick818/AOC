mod day;
mod prelude;

use crate::day::*;

use crate::prelude::*;

#[allow(unused_imports)]
use std::time::Instant;
use std::{fs::File, io::Read};

macro_rules! measure {
    ($title:expr, $func:expr) => {{
        let start = Instant::now();
        let result = $func;
        let duration = start.elapsed();
        let time = if duration.as_secs() / 60 > 1 {
            format!("{} min", duration.as_secs())
        } else if duration.as_secs() > 1 {
            format!("{} s  ", duration.as_secs())
        } else if duration.as_millis() > 1 {
            format!("{} ms ", duration.as_millis())
        } else if duration.as_micros() > 1 {
            format!("{} us ", duration.as_micros())
        } else {
            format!("{} ns ", duration.as_nanos())
        };
        println!("Day {:>4} in {:>8}: {}", $title, time, result);
    }};
}

fn main() -> Result<()> {

    let input = read_file("input/1.txt")?;
    measure!("1.1", d1_trebuchet::cal_trebuchet(&input)?);
    measure!("1.2", d1_trebuchet::cal_trebuchet_str(&input)?);

    let input = read_file("input/2.txt")?;
    measure!("2.1", d2_cube_conundrum::cal_cubes(&input)?);
    measure!("2.2", d2_cube_conundrum::cal_cubes2(&input)?);

    let input = read_file("input/3.txt")?;
    measure!("3.1", d3_gear_ratios::cal_gear_ratio(&input)?);
    measure!("3.2", d3_gear_ratios::cal_gear_ratio2(&input)?);

    let input = read_file("input/4.txt")?;
    measure!("4.1", d4_scratchcards::cal_card_points(&input)?);
    measure!("4.2", d4_scratchcards::count_cards(&input)?);

    let input = read_file("input/5.txt")?;
    measure!("5.1", d5_fertilizer::cal_lowest_location(&input)?);
    //30s with par_iter, 115s without on MacAir M1
    // measure!("5.2", d5_fertilizer::cal_lowest_loc_ranges(&input)?);

    let input = read_file("input/6.txt")?;
    measure!("6.1", d6_wait_for_it::cal_ways_to_win(&input)?);
    measure!("6.2", d6_wait_for_it::cal_ways_to_win2(&input)?);

    let input = read_file("input/7.txt")?;
    measure!("7.1", d7_camel_cards::cal_winning_points(&input, false)?); 
    measure!("7.2", d7_camel_cards::cal_winning_points(&input, true)?); 

    let input = read_file("input/8.txt")?;
    measure!("8.1", d8_haunted_wasteland::cal_steps(&input)?); 
    measure!("8.2", d8_haunted_wasteland::cal_steps_simultanious(&input)?); 

    let input = read_file("input/9.txt")?;
    measure!("9.1", d9_mirage_maintenance::cal_next_steps(&input)?); 
    measure!("9.2", d9_mirage_maintenance::cal_prev_steps(&input)?); 

    let input = read_file("input/10.txt")?;
    measure!("10.1", d10_pipe_maze::cal_maze_distance(&input)?); 
    measure!("10.2", d10_pipe_maze::cal_enclosed_tiles(&input)?); 

    let input = read_file("input/11.txt")?;
    measure!("11.1", d11_cosmic_expansion::cal_sum_of_paths(&input, 1)?); 
    measure!("11.2", d11_cosmic_expansion::cal_sum_of_paths(&input, 1_000_000)?); 

    let input = read_file("input/12.txt")?;
    measure!("12.1", d12_hot_springs::cal_arrangement_sum(&input)?); 
    // to slow
    // measure!("12.2", d12_hot_springs::cal_arrangement_sum_folded(&input)?); 

    let input = read_file("input/13.txt")?;
    measure!("13.1", d13_point_of_incidence::cal_reflection_code(&input)?); 
    measure!("13.2", d13_point_of_incidence::cal_reflection_code2(&input)?); 

    let input = read_file("input/14.txt")?;
    measure!("14.1", d14_parabolic_reflector_dish::cal_total_load(&input)?); 
    measure!("14.2", d14_parabolic_reflector_dish::cal_load_after(&input, 1_000_000_000)?); 

    let input = read_file("input/15.txt")?;
    measure!("15.1", d15_lens_library::cal_hash_sum(&input)?); 
    measure!("15.2", d15_lens_library::cal_focus_power(&input)?); 

    let input = read_file("input/16.txt")?;
    measure!("16.1", d16_the_floor_will_be_lava::cal_energized_tiles(&input)?); 
    measure!("16.2", d16_the_floor_will_be_lava::cal_max_energized_tiles(&input)?); 

    // let input = read_file("input/18.txt")?;
    // println!("Day 18.1: {}", d18_lavaduct_lagoon::cal_trench_volume(&input)?); 
    // println!("Day 18.2: {}", d16_the_floor_will_be_lava::cal_max_energized_tiles(&input)?); 

    let input = read_file("input/19.txt")?;
    measure!("19.1", d19_aplenty::cal_sum_accepted(&input)?); 
    // measure!("19.2", d19_aplenty::cal_all_possibilities(&input)?); 

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

