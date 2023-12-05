mod day;
mod prelude;

use crate::day::d1_trebuchet::*;
use crate::prelude::*;

use std::{fs::File, io::Read};

fn main() -> Result<()> {
    let mut file = File::open("input/1.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    println!("Trebuchtet Ergebnis 1: {}", cal_trebuchet(&buffer)?);
    println!("Trebuchtet Ergebnis 2: {}", cal_trebuchet_str(&buffer)?);

    Ok(())
}
