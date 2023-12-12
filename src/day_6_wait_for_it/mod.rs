use std::num::ParseIntError;

pub fn solve(input_path: String) -> String {
    match std::fs::read_to_string(input_path) {
        Ok(input) => {
            let race = get_race_from_input(&input);

            match race {
                Ok(race) => {
                    println!("{:?}", race);

                    let result = get_possible_race_beat_amount(&race);

                    format!("Result: {}", result)
                }
                Err(err) => {
                    format!("Error parsing races: {}", err)
                }
            }
        }
        Err(err) => {
            format!("Error reading input file: {}", err)
        }
    }
}

fn get_possible_race_beat_amount(race: &Race) -> u64 {
    race.get_beating_results().len() as u64
}

#[allow(dead_code)]
fn get_possible_races_beat_amount(races: &Vec<Race>) -> u64 {
    let mut result: u64 = 1;

    for race in races {
        result *= race.get_beating_results().len() as u64;
    }

    return result;
}

/// Part two - only one race
fn get_race_from_input(input: &String) -> Result<Race, ParseIntError> {
    let lines = input.lines().collect::<Vec<&str>>();
    let times_line: Vec<&str> = lines[0].split(":").collect();
    let distances_line: Vec<&str> = lines[1].split(":").collect();

    let time = get_parsed_number_for_race(times_line[1])?;
    let distance = get_parsed_number_for_race(distances_line[1])?;

    return Ok(Race::new(time, distance));
}

fn get_parsed_number_for_race(string: &str) -> Result<u64, ParseIntError> {
    let parse_result = string
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()?;

    Ok(parse_result)
}

/// Part one - multiple races
#[allow(dead_code)]
fn get_races_from_input(input: &String) -> Result<Vec<Race>, ParseIntError> {
    let lines = input.lines().collect::<Vec<&str>>();
    let times_line: Vec<&str> = lines[0].split(":").collect();
    let distances_line: Vec<&str> = lines[1].split(":").collect();

    let times = times_line[1].split_whitespace().collect::<Vec<&str>>();
    let distances = distances_line[1].split_whitespace().collect::<Vec<&str>>();

    let mut races: Vec<Race> = Vec::new();

    for (index, time) in times.iter().enumerate() {
        let parsed_time = time.parse::<u64>()?;
        let parsed_distance = distances[index].parse::<u64>()?;

        races.push(Race::new(parsed_time, parsed_distance))
    }

    return Ok(races);
}

#[derive(Debug)]
struct Race {
    time_in_ms: u64,
    record_distance_in_mm: u64,
}

impl Race {
    const SPEED_PER_MM_PER_MS_BUTTON_HELD: u64 = 1;

    pub fn new(time_in_ms: u64, record_distance_in_mm: u64) -> Self {
        Race {
            time_in_ms,
            record_distance_in_mm,
        }
    }

    fn get_beating_results(&self) -> Vec<u64> {
        let mut results: Vec<u64> = Vec::new();

        // We exclude 0 and the last value because if you hold the button for 0 seconds, you won't gain any speed
        // And if you hold for the whole duration of the race, you won't even start before the race ends
        for index in 1..self.time_in_ms {
            let speed = index * Self::SPEED_PER_MM_PER_MS_BUTTON_HELD;
            let remaining_time = self.time_in_ms - index;

            let distance_traveled = speed * remaining_time;

            if distance_traveled > self.record_distance_in_mm {
                results.push(distance_traveled);
            }
        }

        return results;
    }
}
