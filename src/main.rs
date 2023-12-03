mod day_1_trebuchet;
mod day_2_cube_conundrum;
mod day_3_gear_ratios;
mod shared;

fn main() {
    // day_1_trebuchet();
    // day_2_cube_conundrum();
    day_3_gear_ratios();
}

#[allow(dead_code)]
fn day_1_trebuchet() {
    let result = day_1_trebuchet::solve("src/day_1_trebuchet/puzzle_input.txt".to_string());

    println!("Result: {}", result);
}

#[allow(dead_code)]
fn day_2_cube_conundrum() {
    let result = day_2_cube_conundrum::solve(
        "src/day_2_cube_conundrum/puzzle_p2_input.txt".to_string(),
        12,
        13,
        14,
    );

    println!("Result: {}", result);
}

#[allow(dead_code)]
fn day_3_gear_ratios() {
    let result = day_3_gear_ratios::solve("src/day_3_gear_ratios/puzzle_input.txt".to_string());

    println!("Result: {}", result);
}