use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Trebuchet!?");

    let calibration = get_calibration_from_puzzle_input();

    println!("Calibration: {}", calibration);
}

fn get_calibration_from_puzzle_input() -> i32 {
    let mut calibration: i32 = 0;
    match read_lines("./puzzle_input.txt") {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(l) => {
                        let line_calibration = get_calibration_from_line(&l);
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
