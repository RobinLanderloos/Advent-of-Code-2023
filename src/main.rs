mod day_1_trebuchet;
mod shared;

fn main() {
    day_1_trebuchet();
}

fn day_1_trebuchet() {
    let path = String::from("src/day_1_trebuchet/puzzle_input.txt");
    let puzzle_input_handler = day_1_trebuchet::PuzzleInputHandler::new(path);
    let calibration = puzzle_input_handler.get_calibration_from_puzzle_input();
    println!("Calibration: {}", calibration);
}
