use super::shared::file_processing::read_lines;

pub struct PuzzleInputHandler {
    path: String,
}

impl PuzzleInputHandler {
    /// Returns a new PuzzleInputHandler instance for handling the given path
    pub fn new(path: String) -> PuzzleInputHandler {
        return PuzzleInputHandler { path: path };
    }

    /// Returns the calibration from the puzzle input
    pub fn get_calibration_from_puzzle_input(&self) -> i32 {
        let mut calibration: i32 = 0;
        match read_lines(self.path.to_string()) {
            Ok(lines) => {
                for line in lines {
                    match line {
                        Ok(l) => {
                            let line_calibration =
                                PuzzleInputHandler::get_calibration_from_line(&l);
                            calibration += line_calibration;
                            println!("Line Calibration: {}", line_calibration);
                        }
                        Err(error) => {
                            println!("Error reading line: {}", error);
                        }
                    }
                }
            }
            Err(error) => {
                println!("Error reading file: {}", error);
            }
        }

        return calibration;
    }

    /// Returns the calibration from the given line
    fn get_calibration_from_line(line: &str) -> i32 {
        let radix: u32 = 10; // Base 10
        let mut first: char = '0';
        let mut last: char = '0';

        // loop over every character of string
        for character in line.chars() {
            if character.is_digit(radix) {
                let digit: char = character;

                // Only set this when the first digit is found
                if first == '0' {
                    first = digit;
                }

                last = digit; // Set on every digit found
            }
        }

        let combined = format!("{}{}", first, last);
        println!("First: {}, Last: {}, Combined: {}", first, last, combined);

        // No need for error handling as we know the string is a number
        return combined.parse::<i32>().unwrap();
    }
}
