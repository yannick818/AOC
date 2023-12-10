mod day;
mod prelude;

use crate::day::d1_trebuchet::*;
use crate::day::d2_cube_conundrum::*;
use crate::day::d3_gear_ratios::*;
use crate::day::d4_scratchcards::*;
use crate::day::d5_fertilizer::*;

use crate::prelude::*;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::{fs::File, io::Read};

fn main() -> Result<()> {
    // trace_signal();

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
    println!("Day 5.2: {}", cal_lowest_loc_ranges(&input)?);

    Ok(())
}

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}


fn trace_signal() {
    let term = Arc::new(AtomicBool::new(false));
    // HACK SIGSTOP cant be registered, since its forbidden
    signal_hook::flag::register(signal_hook::consts::SIGSTOP, term.clone()).unwrap();

    thread::spawn(move || {
        while !term.load(Ordering::Relaxed) {
            // Do some time-limited stuff here
            // (if this could block forever, then there's no guarantee the signal will have any
            // effect).
            eprintln!("SIGSTOP occurred");
        }
    });
}
