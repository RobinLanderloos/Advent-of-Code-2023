use std::num::{ParseFloatError, ParseIntError};

pub fn solve(input_path: String) -> String {
    let input = std::fs::read_to_string(input_path);

    if input.is_err() {
        return "Error reading input file".to_string();
    }

    let input = input.unwrap();

    let races = get_races_from_input(input);

    if races.is_err() {
        return format!("Error parsing races: {}", races.err().unwrap());
    }

    let races = races.unwrap();

    let result = get_possible_race_beat_amount(&races);

    return format!("Result: {}", result);
}

fn get_possible_race_beat_amount(races: &Vec<Race>) -> u64 {
    let mut result: u64 = 1;

    for race in races {
        result *= race.get_results().len() as u64;
    }

    return result;
}

fn get_races_from_input(input: String) -> Result<Vec<Race>, ParseIntError> {
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

    fn get_results(&self) -> Vec<u64> {
        let mut results: Vec<u64> = Vec::new();

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
