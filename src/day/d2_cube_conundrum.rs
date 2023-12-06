use crate::prelude::*;

#[test]
fn test_cube() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 3: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(8, cal_cubes(input).unwrap());
}

#[test]
fn test_cube2() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 3: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(2286, cal_cubes2(input).unwrap());
}

pub fn cal_cubes(input: &str) -> Result<usize> {
    let sum = input
        .lines()
        .enumerate()
        .filter_map(|(id, line)| match is_possible(line) {
            Ok(true) => Some(id + 1),
            _ => None,
        })
        .sum();

    Ok(sum)
}

fn is_possible(game: &str) -> Result<bool> {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let max_cubes = cal_max(game)?;

    match (
        max_cubes.0 > MAX_RED,
        max_cubes.1 > MAX_GREEN,
        max_cubes.2 > MAX_BLUE,
    ) {
        (false, false, false) => Ok(true),
        _ => Ok(false),
    }
}

fn cal_max(game: &str) -> Result<(u32, u32, u32)> {
    let games = match game.split_once(':') {
        Some((_, games)) => games,
        None => return Err("no colon".into()),
    };

    let max_cubes = games
        .trim()
        .split(';')
        .fold((0_u32, 0_u32, 0_u32), |mut max, round| {
            round
                .split(',')
                .map(|r| r.trim())
                .map(|r: &str| r.split(' ').collect::<Vec<_>>())
                .for_each(|cnt_color| {
                    let cnt = cnt_color.first().unwrap().parse::<u32>().unwrap();
                    let color = *cnt_color.last().unwrap();
                    match color {
                        "red" => max.0 = max.0.max(cnt),
                        "green" => max.1 = max.1.max(cnt),
                        "blue" => max.2 = max.2.max(cnt),
                        _ => panic!("unknown color"),
                    }
                });
            max
        });

    Ok(max_cubes)
}

pub fn cal_cubes2(input: &str) -> Result<u32> {
    let sum = input
        .lines()
        .map(|line| cal_max(line).unwrap())
        .fold(0, |sum, (r, g, b)| sum + r * g * b);

    Ok(sum)
}
