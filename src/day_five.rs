use std::{error::Error, fmt::Display, ops::Range, str::FromStr};

use prettytable::{Table, row};

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_five.txt");

#[derive(Debug, Clone)]
struct SourceDestination {
    pub source_range: Range<u64>,
    pub destination_range: Range<u64>,
    pub length: u64,
}

impl SourceDestination {
    pub fn new(source_start: u64, dest_start: u64, length: u64) -> Self {
        Self {
            source_range: source_start..source_start + length,
            destination_range: dest_start..dest_start + length,
            length,
        }
    }

    pub fn get_destination_from_source(&self, source_index: u64) -> Option<u64> {
        if !self.source_range.contains(&source_index) {
            return None;
        }

        let destination_index = source_index - self.source_range.start;
        // TODO: Figure out how to get the nth index in the range without mutating.
        return self
            .destination_range
            .clone()
            .nth(destination_index as usize);
    }
}

#[derive(Default, Debug, Clone)]
struct SourceDestinationMap {
    /// Sorted by the start of the source range
    values: Vec<SourceDestination>,
}

impl SourceDestinationMap {
    pub fn get_destination_value(&self, source: u64) -> u64 {
        for range_map in &self.values {
            if let Some(destination) = range_map.get_destination_from_source(source) {
                return destination;
            }
        }

        return source;
    }
}

impl FromStr for SourceDestinationMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s
            .lines()
            .map(|line| {
                let numbers = line
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<_>>();
                let source = numbers[0];
                let dest = numbers[1];
                let length = numbers[2];
                SourceDestination::new(source, dest, length)
            })
            .collect::<Vec<_>>();

        values.sort_by(|a, b| a.source_range.start.cmp(&b.source_range.start));

        Ok(Self { values })
    }
}

#[derive(Default, Debug, Clone)]
struct DayFive {
    pub seeds: Vec<u64>,
    seed_to_soil: SourceDestinationMap,
    soil_to_fertilizer: SourceDestinationMap,
    fertilizer_to_water: SourceDestinationMap,
    water_to_light: SourceDestinationMap,
    light_to_temperature: SourceDestinationMap,
    temperature_to_humidity: SourceDestinationMap,
    humidity_to_location: SourceDestinationMap,
}

impl DayFive {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Display for DayFive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["seed", "soil", "fertilizer", "water", "light", "temperature", "humidity", "location"]);

        for seed in &self.seeds {
            let soil = self.seed_to_soil.get_destination_value(*seed);
            let fertilizer = self.soil_to_fertilizer.get_destination_value(soil);
            let water = self.fertilizer_to_water.get_destination_value(fertilizer);
            let light = self.water_to_light.get_destination_value(water);
            let temperature = self.light_to_temperature.get_destination_value(light);
            let humidity = self.temperature_to_humidity.get_destination_value(temperature);
            let location = self.humidity_to_location.get_destination_value(humidity);
            table.add_row(row![seed, soil, fertilizer, water, light, temperature, humidity, location]);
        }
        
        write!(f, "{}", table)
    }
}

impl FromStr for DayFive {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("\n\n");

        let mut day_five = DayFive::new();
        for part in parts {
            let mut map_parts = part.split(':');
            let map_name = map_parts
                .next()
                .ok_or_else(|| anyhow!("Invalid name format"))?;
            let rest = map_parts
                .next()
                .ok_or_else(|| anyhow!("Invalid value format"))?
                .trim();

            match map_name {
                "seeds" => {
                    let seeds: Vec<u64> = rest.split(' ').map(|s| s.parse().unwrap()).collect();
                    day_five.seeds = seeds;
                }
                "seed-to-soil map" => {
                    let map = rest.parse()?;
                    day_five.seed_to_soil = map;
                }
                "soil-to-fertilizer map" => {
                    let map = rest.parse()?;
                    day_five.soil_to_fertilizer = map;
                }
                "fertilizer-to-water map" => {
                    let map = rest.parse()?;
                    day_five.fertilizer_to_water = map;
                }
                "water-to-light map" => {
                    let map = rest.parse()?;
                    day_five.water_to_light = map;
                }
                "light-to-temperature map" => {
                    let map = rest.parse()?;
                    day_five.light_to_temperature = map;
                }
                "temperature-to-humidity map" => {
                    let map = rest.parse()?;
                    day_five.temperature_to_humidity = map;
                }
                "humidity-to-location map" => {
                    let map = rest.parse()?;
                    day_five.humidity_to_location = map;
                }
                _ => return Err(anyhow!("Unknown map found")),
            }
        }

        Ok(day_five)
    }
}

pub fn part_one(args: Args) {
    let day_five: DayFive = FILE_CONTENTS.parse().unwrap();
    println!("{}", day_five);
}

pub fn part_two(args: Args) {}
