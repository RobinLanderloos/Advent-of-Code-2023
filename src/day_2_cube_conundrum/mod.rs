use super::shared::file_processing::read_lines;

/// Solves the puzzle using the input at the given path
pub fn solve(path: String, red_cubes_count: i32, green_cubes_count: i32, blue_cubes_count: i32) {
    let lines = read_lines(path);

    if lines.is_err() {
        println!("Error reading file: {}", lines.err().unwrap());
        return;
    }

    let mut games: Vec<Game> = Vec::new();

    // Line example: "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
    for line in lines.unwrap() {
        let line = line.unwrap();
        let line_parts = line.split(": ").collect::<Vec<&str>>();
        let game_id = line_parts[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        let game_subsets = line_parts[1].split("; ").collect::<Vec<&str>>();

        let mut game = Game::new(game_id);

        for game_subset in game_subsets {
            game.add_game_subset(GameSubset::new(String::from(game_subset)));
        }

        games.push(game);
    }

    let mut possible_id_sum = 0;
    let mut sum_of_powers = 0;
    let mut sum_of_powers_optimized = 0;
    for game in games {
        if game.is_possible(green_cubes_count, blue_cubes_count, red_cubes_count) {
            possible_id_sum += game.id;
        }
        sum_of_powers += game.get_power_of_minimum_pulls();
        sum_of_powers_optimized += game.get_power_of_minimum_pulls_optimized();
    }

    println!("Possible id sum: {}", possible_id_sum);
    println!("Sum of powers: {}", sum_of_powers);
    println!("Sum of powers (optimized): {}", sum_of_powers_optimized);
}

#[derive(Debug)]
pub struct Game {
    id: i32,
    green_count: i32,
    blue_count: i32,
    red_count: i32,
    reds_pulled: Vec<i32>,
    greens_pulled: Vec<i32>,
    blues_pulled: Vec<i32>,
    min_red: i32,
    min_green: i32,
    min_blue: i32,
}

impl Game {
    /// Initializes a new game instance with the given id
    pub fn new(id: i32) -> Game {
        return Game {
            id: id,
            red_count: 0,
            blue_count: 0,
            green_count: 0,
            greens_pulled: Vec::new(),
            blues_pulled: Vec::new(),
            reds_pulled: Vec::new(),
            min_green: 1,
            min_blue: 1,
            min_red: 1,
        };
    }

    /// Adds a game subset to the game
    pub fn add_game_subset(&mut self, game_subset: GameSubset) {
        self.green_count = Game::set_if_higher(self.green_count, game_subset.green_count);
        self.min_green = Game::set_lowest_possible(self.min_green, game_subset.green_count);
        if game_subset.green_count > 0 {
            self.greens_pulled.push(game_subset.green_count);
        }

        self.blue_count = Game::set_if_higher(self.blue_count, game_subset.blue_count);
        self.min_blue = Game::set_lowest_possible(self.min_blue, game_subset.blue_count);
        if game_subset.blue_count > 0 {
            self.blues_pulled.push(game_subset.blue_count);
        }

        self.red_count = Game::set_if_higher(self.red_count, game_subset.red_count);
        self.min_red = Game::set_lowest_possible(self.min_red, game_subset.red_count);
        if game_subset.red_count > 0 {
            self.reds_pulled.push(game_subset.red_count);
        }
    }

    /// Returns true if the game is possible, false otherwise
    pub fn is_possible(&self, green_cubes: i32, blue_cubes: i32, red_cubes: i32) -> bool {
        let mut is_possible = true;

        if self.green_count > green_cubes {
            is_possible = false;
        }

        if self.blue_count > blue_cubes {
            is_possible = false;
        }

        if self.red_count > red_cubes {
            is_possible = false;
        }

        println!("{:?} Is possible: {}", self, is_possible);

        return is_possible;
    }

    fn set_if_higher(current_value: i32, new_value: i32) -> i32 {
        if new_value > current_value {
            return new_value;
        } else {
            return current_value;
        }
    }

    fn set_lowest_possible(current_value: i32, new_value: i32) -> i32 {
        if new_value == 0 {
            return current_value;
        }

        if new_value > current_value {
            return new_value;
        }

        return current_value;
    }

    pub fn get_power_of_minimum_pulls(&self) -> i32 {
        let highest_red = self.reds_pulled.iter().max().unwrap();
        let highest_blue = self.blues_pulled.iter().max().unwrap();
        let highest_green = self.greens_pulled.iter().max().unwrap();

        let power = highest_red * highest_blue * highest_green;

        println!(
            "Game {} power: {} ({} * {} * {})",
            self.id, power, highest_red, highest_green, highest_blue
        );

        return power;
    }

    pub fn get_power_of_minimum_pulls_optimized(&self) -> i32 {
        let power = self.min_red * self.min_blue * self.min_green;
        println!(
            "Game {} power: {} ({} * {} * {})",
            self.id, power, self.min_red, self.min_green, self.min_blue
        );

        return power;
    }
}

#[derive(Debug)]
pub struct GameSubset {
    green_count: i32,
    blue_count: i32,
    red_count: i32,
}

impl GameSubset {
    /// Initializes a new game subset instance
    ///
    /// Input must be in the following format: "3 green, 4 blue, 1 red"
    ///
    /// The order of the colors does not matter
    pub fn new(line_subset: String) -> GameSubset {
        let mut green_count = 0;
        let mut blue_count = 0;
        let mut red_count = 0;

        let line_subset_trimmed = line_subset.trim();
        let line_subset_parts = line_subset_trimmed.split(", ");

        // Example: "3 green" or "1 red"
        for line_part in line_subset_parts {
            let split_line_parts = line_part.split(" ").collect::<Vec<&str>>();
            let color = split_line_parts[1];
            let count = split_line_parts[0].parse::<i32>().unwrap();

            match color {
                "green" => {
                    green_count = count;
                }
                "blue" => {
                    blue_count = count;
                }
                "red" => {
                    red_count = count;
                }
                _ => {
                    panic!("Invalid color: {}", color);
                }
            }
        }

        return GameSubset {
            green_count,
            blue_count,
            red_count,
        };
    }
}
