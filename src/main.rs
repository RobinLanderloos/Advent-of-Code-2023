mod day_1_trebuchet;
mod day_2_cube_conundrum;
mod shared;

fn main() {
    // day_1_trebuchet();
    day_2_cube_conundrum();
}

#[allow(dead_code)]
fn day_1_trebuchet() {
    day_1_trebuchet::solve("src/day_1_trebuchet/puzzle_input.txt".to_string());
}

#[allow(dead_code)]
fn day_2_cube_conundrum() {
    day_2_cube_conundrum::solve(
        "src/day_2_cube_conundrum/puzzle_input.txt".to_string(),
        12,
        13,
        14,
    );
}
