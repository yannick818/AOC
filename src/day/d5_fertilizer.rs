use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use crate::prelude::*;

#[test]
fn test_fertilizer() {
    let input = "seeds: 79 14 55 13

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

    assert_eq!(35, cal_lowest_location(input).unwrap());
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Id<T> {
    id: u32,
    _phantom: PhantomData<T>,
}

impl<T> From<u32> for Id<T> {
    fn from(id: u32) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct SeedId;
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct SoilId;
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct FertilizerId;
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct WaterId;
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct LightId;
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct TemperatureId;
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct HumidityId;
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct LocationId;

type Seed = Id<SeedId>;
type Soil = Id<SoilId>;
type Fertilizer = Id<FertilizerId>;
type Water = Id<WaterId>;
type Light = Id<LightId>;
type Temperature = Id<TemperatureId>;
type Humidity = Id<HumidityId>;
type Location = Id<LocationId>;

struct Almanac {
    seeds: Vec<Seed>,
    seed_to_soil: HashMap<Seed, Soil>,
    soil_to_fertilizer: HashMap<Soil, Fertilizer>,
    fertilizer_to_water: HashMap<Fertilizer, Water>,
    water_to_light: HashMap<Water, Light>,
    light_to_temperature: HashMap<Light, Temperature>,
    temperature_to_humidity: HashMap<Temperature, Humidity>,
    humidity_to_location: HashMap<Humidity, Location>,    
}

impl Almanac {
    fn new(input: &str) -> Self {
        let seeds = input.lines()
        .next().unwrap().split(": ").nth(1).unwrap()
        .split(' ').map(|s| {
            let num = s.parse::<u32>().unwrap();
            Seed::from(num)
        }).collect::<Vec<_>>();

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

    fn cal_map<K: From<u32> + Hash + Eq + PartialEq, V: From<u32>>(input: &str, name: &str) -> HashMap<K, V> {
        input.split(name).nth(1).unwrap().split("\n\n").next().unwrap()
        .lines().map(|line| {
            let vec = line.split(' ').map(|s| s.parse().unwrap()).collect::<Vec<u32>>();
            (vec[0], vec[1], vec[2])
        }).fold(HashMap::new(), |mut map, (dest, src, len)| {
            for i in 0..len {
                map.insert(K::from(src+i), V::from(dest+i));
            }
            map
        })
    }

}

pub fn cal_lowest_location(input: &str) -> Result<u32> {
    let almanac = Almanac::new(input);
    let min = almanac.seeds.iter().map(|seed| {
        let soil = almanac.seed_to_soil.get(seed).copied().unwrap_or(Soil::from(seed.id));
        let fertilizer = almanac.soil_to_fertilizer.get(&soil).copied().unwrap_or(Fertilizer::from(soil.id));
        let water = almanac.fertilizer_to_water.get(&fertilizer).copied().unwrap_or(Water::from(fertilizer.id));
        let light = almanac.water_to_light.get(&water).copied().unwrap_or(Light::from(water.id));
        let temperature = almanac.light_to_temperature.get(&light).copied().unwrap_or(Temperature::from(light.id));
        let humidity = almanac.temperature_to_humidity.get(&temperature).copied().unwrap_or(Humidity::from(temperature.id));
        let location = almanac.humidity_to_location.get(&humidity).copied().unwrap_or(Location::from(humidity.id));
        location.id
    }).min().unwrap();

    Ok(min)
}

