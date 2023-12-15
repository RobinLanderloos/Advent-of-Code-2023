mod day_1_trebuchet;
mod day_2_cube_conundrum;
mod day_3_gear_ratios;
mod day_4_scratchcards;
mod day_5_if_you_give_a_seed_a_fertilizer;
mod day_6_wait_for_it;
mod day_7_camel_cards;
mod day_8_haunted_wasteland;
mod shared;

fn main() {
    // day_1_trebuchet();
    // day_2_cube_conundrum();
    // day_3_gear_ratios();
    // day_4_scratchcards();
    // day_5_if_you_give_a_seed_a_fertilizer();
    // day_6_wait_for_it();
    // day_7_camel_cards();

    day_8_haunted_wasteland();
}

fn day_8_haunted_wasteland() {
    println!("{}", day_8_haunted_wasteland::solve());
}

fn day_7_camel_cards() {
    println!(
        "{}",
        day_7_camel_cards::solve("src/day_7_camel_cards/input.txt")
    );
}

#[allow(dead_code)]
fn day_6_wait_for_it() {
    println!(
        "{}",
        day_6_wait_for_it::solve("src/day_6_wait_for_it/input.txt".to_string())
    );
}

#[allow(dead_code)]
fn day_5_if_you_give_a_seed_a_fertilizer() {
    println!(
        "{}",
        day_5_if_you_give_a_seed_a_fertilizer::solve(
            "src/day_5_if_you_give_a_seed_a_fertilizer/input.txt".to_string()
        )
    );
}

#[allow(dead_code)]
fn day_4_scratchcards() {
    println!(
        "{}",
        day_4_scratchcards::solve("src/day_4_scratchcards/input.txt".to_string())
    );
}

#[allow(dead_code)]
fn day_3_gear_ratios() {
    println!(
        "{}",
        day_3_gear_ratios::solve("src/day_3_gear_ratios/puzzle_input.txt".to_string())
    );
}

#[allow(dead_code)]
fn day_2_cube_conundrum() {
    println!(
        "{}",
        day_2_cube_conundrum::solve(
            "src/day_2_cube_conundrum/puzzle_p2_input.txt".to_string(),
            12,
            13,
            14,
        )
    );
}

#[allow(dead_code)]
fn day_1_trebuchet() {
    println!(
        "{}",
        day_1_trebuchet::solve("src/day_1_trebuchet/puzzle_input.txt".to_string())
    );
}
