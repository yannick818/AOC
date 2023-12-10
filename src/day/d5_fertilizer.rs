use std::{marker::PhantomData, ops::Range};

use crate::prelude::*;

#[allow(dead_code)]
const TEST_INPUT: &str = 
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[test]
fn test_fertilizer() {
    assert_eq!(35, cal_lowest_location(TEST_INPUT).unwrap());
}

#[test]
fn test_fertilizer2() {
    assert_eq!(46, cal_lowest_loc_ranges(TEST_INPUT).unwrap());
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, PartialOrd, Ord)]
struct Id<T> {
    id: u64,
    _phantom: PhantomData<T>,
}

impl<T> From<u64> for Id<T> {
    fn from(id: u64) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }
}


#[derive(Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Seed;
#[derive(Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Soil;
#[derive(Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Fertilizer;
#[derive(Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Water;
#[derive(Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Light;
#[derive(Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Temperature;
#[derive(Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Humidity;
#[derive(Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Location;

struct Region<A, B> {
    dest: u64,
    src: u64, 
    len: u64,
    _phantom: PhantomData<(A, B)>,
}

impl<A, B> Region<Id<A>, Id<B>> {

    fn new(dest: u64, src: u64, len: u64) -> Self {
        Self {
            dest,
            src,
            len,
            _phantom: PhantomData,
        }
    }

    fn contains(&self, id: &Id<A>) -> bool {
        let range = self.src..(self.src+self.len);
        range.contains(&id.id)
    }

    fn convert(&self, id: &Id<A>) -> Id<B> {
        let delta = self.dest as i64 - self.src as i64;

        Id::<B>::from((id.id as i64 + delta) as u64)
    }

    fn map(ranges: &[Self], id: &Id<A>) -> Id<B> {
        match ranges.iter().find(|r| r.contains(id)) {
            Some(r) => r.convert(id),
            None => Id::<B>::from(id.id),
        }
    }

}

type IdMap<A, B> = Vec<Region<Id<A>, Id<B>>>;

struct Almanac {
    seeds: Vec<Range<Id<Seed>>>,
    seed_to_soil: IdMap<Seed, Soil>,
    soil_to_fertilizer: IdMap<Soil, Fertilizer>,
    fertilizer_to_water: IdMap<Fertilizer, Water>,
    water_to_light: IdMap<Water, Light>,
    light_to_temperature: IdMap<Light, Temperature>,
    temperature_to_humidity: IdMap<Temperature, Humidity>,
    humidity_to_location: IdMap<Humidity, Location>,
}

type SeedGenerator = fn(&str) -> Vec<Range<Id<Seed>>>;

impl Almanac {

    fn new(input: &str, seed_gen: SeedGenerator) -> Self {
        let seeds = seed_gen(input);
        let seed_to_soil = Almanac::cal_map(input, "seed-to-soil map:\n");
        let soil_to_fertilizer = Almanac::cal_map(input, "soil-to-fertilizer map:\n");
        let fertilizer_to_water = Almanac::cal_map(input, "fertilizer-to-water map:\n");
        let water_to_light = Almanac::cal_map(input, "water-to-light map:\n");
        let light_to_temperature = Almanac::cal_map(input, "light-to-temperature map:\n");
        let temperature_to_humidity = Almanac::cal_map(input, "temperature-to-humidity map:\n");
        let humidity_to_location = Almanac::cal_map(input, "humidity-to-location map:\n");

        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn cal_map<A, B>(input: &str, name: &str) -> IdMap<A, B>{
        input.split(name).nth(1).unwrap().split("\n\n").next().unwrap()
        .lines().map(|line| {
            let vec = line.split(' ').map(|s| s.parse().unwrap()).collect::<Vec<u64>>();
            (vec[0], vec[1], vec[2])
        }).map(|(dest, src, len)| {
            Region::new(dest, src, len)
        }).collect()
    }

    fn list_seeds(input: &str) -> Vec<Range<Id<Seed>>> {
        input.lines()
        .next().unwrap().split(": ").nth(1).unwrap()
        .split(' ').map(|s| {
            let num = s.parse::<u64>().unwrap();
            Id::<Seed>::from(num)
        }).map(|id| {
            id..Id::<Seed>::from(id.id+1)
        }).collect()
    }

    fn list_seed_ranges(input: &str) -> Vec<Range<Id<Seed>>> {
        let numbers = input.lines()
        .next().unwrap().split(": ").nth(1).unwrap()
        .split(' ').map(|s| {
            s.parse::<u64>().unwrap()
        }).map(|num| {
            Id::<Seed>::from(num)
        });

        let vec = numbers.clone().collect::<Vec<_>>();
        vec.chunks(2).map(|chunk| {
            (chunk[0], chunk[1].id)
        }).map(|(start, len)| {
            start..Id::<Seed>::from(start.id+len)
        }).collect()

    }


}

pub fn cal_lowest_location(input: &str) -> Result<u64> {

    let min = cal_location(input, Almanac::list_seeds);

    Ok(min)
}

pub fn cal_lowest_loc_ranges(input: &str) -> Result<u64> {
    let min = cal_location(input, Almanac::list_seed_ranges);
    Ok(min)
}

fn cal_location(input: &str, generator: SeedGenerator) -> u64 {
    
    let almanac = Almanac::new(input, generator);
    let min = almanac.seeds.iter().fold(u64::MAX, |min, range| {
        (range.start.id..range.end.id).map(|id|{
            Id::<Seed>::from(id)
        }).map(|seed| {
            let soil = Region::map(&almanac.seed_to_soil, &seed);
            let fertilizer = Region::map(&almanac.soil_to_fertilizer, &soil);
            let water = Region::map(&almanac.fertilizer_to_water, &fertilizer);
            let light = Region::map(&almanac.water_to_light, &water);
            let temperature = Region::map(&almanac.light_to_temperature, &light);
            let humidity = Region::map(&almanac.temperature_to_humidity, &temperature);
            let location = Region::map(&almanac.humidity_to_location, &humidity);
            location.id
        }).min().unwrap_or(u64::MAX).min(min)
    });
    min
}




