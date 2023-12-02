mod puzzle_input_handler;
use puzzle_input_handler::PuzzleInputHandler;

fn main() {
    println!("Trebuchet!?");

    let puzzle_input_handler = PuzzleInputHandler::new("./puzzle_input.txt".to_string());
    let calibration = puzzle_input_handler.get_calibration_from_puzzle_input();

    println!("Calibration: {}", calibration);
}
