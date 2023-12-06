mod day;
mod prelude;

use crate::day::d1_trebuchet::*;
use crate::day::d2_cube_conundrum::*;
use crate::prelude::*;

use std::{fs::File, io::Read};

fn main() -> Result<()> {
    let input = read_file("input/1.txt")?;
    println!("Trebuchtet Ergebnis 1: {}", cal_trebuchet(&input)?);
    println!("Trebuchtet Ergebnis 2: {}", cal_trebuchet_str(&input)?);

    let input = read_file("input/2.txt")?;
    println!("Cube Ergebnis 1: {}", cal_cubes(&input)?);
    // println!("Cube Ergebnis 2: {}", cal_trebuchet_str(&input)?);

    Ok(())
}

fn read_file(path: &str) -> Result<String> {

    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}