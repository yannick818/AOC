use std::ops::Not;

use crate::prelude::*;

#[allow(dead_code)]
const INPUT: &str = 
"Time:      7  15   30
Distance:  9  40  200";

#[test]
fn test_way_to_win() {
    assert_eq!(288, cal_ways_to_win(INPUT).unwrap());
}

type Duration = u64;
type Speed = u64;
type Distance = u64;

struct Race {
    duration: Duration,
    distance: Distance,
}

struct Boat {
    speed: Speed,
}

impl Boat {
    fn start(push_time: Duration) -> Self {
        Self {
            speed: push_time,
        }
    
    } 

    fn cal_distance(&self, time: Duration) -> Distance {
        self.speed * time
    }
}

pub fn cal_ways_to_win(input: &str) -> Result<u64> {
    let races = parse_races(input);
    let mult: usize = races.iter()
    .map(|race| {
        (1..race.duration)
        .map(|push_time| {
            let boat = Boat::start(push_time);
            let distance = boat.cal_distance(race.duration - push_time);
            distance > race.distance
        })
        .filter(|&win| win)
        .count()
        // println!("ways_to_win: {}", ways_to_win);
        // ways_to_win
    }).product();
    
    Ok(mult as u64)
}

fn parse_races(input: &str) -> Vec<Race> {

    let mut lines = input.lines();
    let time = lines.next().unwrap()
    .split_once("Time: ").unwrap().1.split(' ')
    .filter_map(|s| {
        s.is_empty().not().then(|| {
            s.parse::<Duration>().unwrap()
        })
    });
    let distance = lines.next().unwrap()
    .split_once("Distance: ").unwrap().1.split(' ')
    .filter_map(|s| {
        s.is_empty().not().then(|| {
            s.parse::<Distance>().unwrap()
        })
    });

    time.zip(distance)
    .map(|(duration, distance)| {
        Race { duration, distance}
    }).collect()
    
}