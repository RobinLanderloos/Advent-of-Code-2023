use core::fmt;
use std::{num::ParseIntError, ops::Add};

const SEED_TO_SOIL: &str = "seed-to-soil";
const SOIL_TO_FERTILIZER: &str = "soil-to-fertilizer";
const FERTILIZER_TO_WATER: &str = "fertilizer-to-water";
const WATER_TO_LIGHT: &str = "water-to-light";
const LIGHT_TO_TEMPERATURE: &str = "light-to-temperature";
const TEMPERATURE_TO_HUMIDITY: &str = "temperature-to-humidity";
const HUMIDITY_TO_LOCATION: &str = "humidity-to-location";

pub fn solve(path: String) -> String {
    let mut input_string = std::fs::read_to_string(path).unwrap();
    input_string = input_string.add("\r\n"); // Add a newline to the end of the file to make parsing easier
    let parse_result = parse_input(&input_string);

    match parse_result {
        Ok(result) => {
            let mut lowest_location = u64::MAX;
            for seed in result.seeds.clone() {
                let location = result.find_location(seed);
                if location < lowest_location {
                    lowest_location = location;
                }
            }

            return lowest_location.to_string();
        }
        Err(error) => {
            return format!("Error: {}", error);
        }
    }
}

fn parse_input(input: &String) -> Result<InputParseResult, InputParseError> {
    let seeds: Vec<u64>;
    let mut seed_to_soil: Vec<MapDefinition> = Vec::new();
    let mut soil_to_fertilizer: Vec<MapDefinition> = Vec::new();
    let mut fertilizer_to_water: Vec<MapDefinition> = Vec::new();
    let mut water_to_light: Vec<MapDefinition> = Vec::new();
    let mut light_to_temperature: Vec<MapDefinition> = Vec::new();
    let mut temperature_to_humidity: Vec<MapDefinition> = Vec::new();
    let mut humidity_to_location: Vec<MapDefinition> = Vec::new();

    // We manually split the string instead of using .lines() becase .lines() ignores the last line if it is empty
    let mut lines = input.split("\r\n");

    // The first line should always contain the seeds
    seeds = get_seeds_from_line(lines.next().unwrap())?;

    let mut processing_map = false;
    let mut current_map: String;
    let mut current_map_definitions: &mut Vec<MapDefinition> = &mut Vec::new();

    let mut index = -1;
    for line in lines {
        index += 1;
        // This means we have reached the start of a map definition block
        println!("Started processing line: {}", index);
        if line.contains("map") {
            processing_map = true;
            current_map = line.replace(" map:", "");

            match current_map.as_str() {
                SEED_TO_SOIL => current_map_definitions = &mut seed_to_soil,
                SOIL_TO_FERTILIZER => current_map_definitions = &mut soil_to_fertilizer,
                FERTILIZER_TO_WATER => current_map_definitions = &mut fertilizer_to_water,
                WATER_TO_LIGHT => current_map_definitions = &mut water_to_light,
                LIGHT_TO_TEMPERATURE => current_map_definitions = &mut light_to_temperature,
                TEMPERATURE_TO_HUMIDITY => current_map_definitions = &mut temperature_to_humidity,
                HUMIDITY_TO_LOCATION => current_map_definitions = &mut humidity_to_location,
                _ => {
                    return Err(InputParseError::InvalidMap(String::from(
                        "Invalid map name",
                    )))
                }
            }

            continue;
        }

        if processing_map && !line.is_empty() {
            current_map_definitions.push(parse_map_definition_from_line(line)?);
            continue;
        }
    }

    let result: InputParseResult = InputParseResult {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    };

    return Ok(result);
}

fn parse_map_definition_from_line(line: &str) -> Result<MapDefinition, InputParseError> {
    println!("Parsing map definition: {}", line);
    let map_definition_split = line.split(" ").collect::<Vec<&str>>();

    if map_definition_split.len() != 3 {
        return Err(InputParseError::InvalidMap(String::from(
            "Invalid map definition",
        )));
    }

    println!("Map definition split: {:?}", map_definition_split);

    let destination = map_definition_split[0].parse::<u64>()?;
    let source = map_definition_split[1].parse::<u64>()?;
    let range = map_definition_split[2].parse::<u64>()?;

    let map_definition = MapDefinition::new(source, destination, range);

    return Ok(map_definition);
}

fn get_seeds_from_line(line: &str) -> Result<Vec<u64>, InputParseError> {
    let mut seeds_result: Vec<u64> = Vec::new();
    let seeds_line_split = line.split(": ").collect::<Vec<&str>>();

    if seeds_line_split.len() != 2 {
        return Err(InputParseError::InvalidSeeds(String::from(
            "Invalid seed line",
        )));
    }

    let seeds = seeds_line_split[1].split(" ").collect::<Vec<&str>>();

    if seeds.len() < 1 {
        return Err(InputParseError::InvalidSeeds(String::from(
            "No seeds found on line",
        )));
    }

    for seed in seeds {
        let parsed_seed = seed.parse::<u64>();
        if parsed_seed.is_err() {
            return Err(InputParseError::InvalidSeeds(String::from(
                "An error occurred while parsing a seed",
            )));
        }
        seeds_result.push(parsed_seed.unwrap());
    }

    return Ok(seeds_result);
}

struct InputParseResult {
    seeds: Vec<u64>,
    seed_to_soil: Vec<MapDefinition>,
    soil_to_fertilizer: Vec<MapDefinition>,
    fertilizer_to_water: Vec<MapDefinition>,
    water_to_light: Vec<MapDefinition>,
    light_to_temperature: Vec<MapDefinition>,
    temperature_to_humidity: Vec<MapDefinition>,
    humidity_to_location: Vec<MapDefinition>,
}

impl InputParseResult {
    pub fn find_location(&self, seed_number: u64) -> u64 {
        let soil_number =
            InputParseResult::get_destination_from_map_definitions(&self.seed_to_soil, seed_number);
        let fertilizer_number = InputParseResult::get_destination_from_map_definitions(
            &self.soil_to_fertilizer,
            soil_number,
        );
        let water_number = InputParseResult::get_destination_from_map_definitions(
            &self.fertilizer_to_water,
            fertilizer_number,
        );
        let light_number = InputParseResult::get_destination_from_map_definitions(
            &self.water_to_light,
            water_number,
        );
        let temperature_number = InputParseResult::get_destination_from_map_definitions(
            &self.light_to_temperature,
            light_number,
        );
        let humidity_number = InputParseResult::get_destination_from_map_definitions(
            &self.temperature_to_humidity,
            temperature_number,
        );
        let location_number = InputParseResult::get_destination_from_map_definitions(
            &self.humidity_to_location,
            humidity_number,
        );

        return location_number;
    }

    fn get_destination_from_map_definitions(
        map_definitions: &Vec<MapDefinition>,
        source_value: u64,
    ) -> u64 {
        let mut destination = source_value;
        for map_definition in map_definitions {
            if map_definition.is_mapped(destination) {
                destination = map_definition.get_mapped_value(destination);
                return destination; // Immediatly return the first mapped value
            }
        }

        return destination;
    }
}

impl fmt::Display for InputParseResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Seeds: {:?}\n", self.seeds)?;
        write!(f, "Seed to soil: {:?}\n", self.seed_to_soil)?;
        write!(f, "Soil to fertilizer: {:?}\n", self.soil_to_fertilizer)?;
        write!(f, "Fertilizer to water: {:?}\n", self.fertilizer_to_water)?;
        write!(f, "Water to light: {:?}\n", self.water_to_light)?;
        write!(f, "Light to temperature: {:?}\n", self.light_to_temperature)?;
        write!(
            f,
            "Temperature to humidity: {:?}\n",
            self.temperature_to_humidity
        )?;
        write!(f, "Humidity to location: {:?}", self.humidity_to_location)
    }
}

#[derive(Debug)]
struct MapDefinition {
    destination: u64,
    source: u64,
    range: u64,
}

impl MapDefinition {
    fn new(source: u64, destination: u64, range: u64) -> Self {
        Self {
            destination,
            source,
            range,
        }
    }

    /// Returns the mapped value if found, else returns the source_value
    pub fn get_mapped_value(&self, source_value: u64) -> u64 {
        if !self.is_mapped(source_value) {
            return source_value;
        }

        let diff = source_value - self.source;
        let mapped_value = self.destination + diff;

        return mapped_value;
    }

    fn is_mapped(&self, source_value: u64) -> bool {
        return source_value >= self.source && source_value <= self.source + self.range;
    }
}

enum InputParseError {
    InvalidSeeds(String),
    InvalidMap(String),
    ParseIntError(std::num::ParseIntError),
}

impl fmt::Display for InputParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputParseError::InvalidSeeds(string) => write!(f, "Invalid seed, {}", string),
            InputParseError::InvalidMap(map) => write!(f, "Invalid map: {}", map),
            InputParseError::ParseIntError(parse_error) => {
                write!(f, "Parse error: {}", parse_error)
            }
        }
    }
}

impl From<ParseIntError> for InputParseError {
    fn from(error: ParseIntError) -> Self {
        InputParseError::ParseIntError(error)
    }
}
