use std::collections::{HashMap, VecDeque};

pub fn solve() -> String {
    let input = std::fs::read_to_string("src/day_8_haunted_wasteland/input.txt");

    if input.is_err() {
        return "Error reading input file".to_string();
    }

    let input = input.unwrap();

    let result = parse_instruction_set(&input);

    let steps = walk_instructions(result.unwrap());

    println!("Steps: {}", steps);

    return "D8".to_string();
}

fn walk_instructions(parse_result: ParseResult) -> u16 {
    // We always start with AAA
    let mut current_node = parse_result.instructions.get("AAA").unwrap();
    let mut steps = 0u16;

    loop {
        let mut next_instruction: &String;

        // We don't want to remove the directions from the original VecDeque, so iterate over it instead
        for (_, direction) in parse_result.directions.iter().enumerate() {
            println!("Current node: {:?}", current_node);
            println!("Direction: {}", direction);

            if *direction == 'L' {
                next_instruction = &current_node.left;
            } else {
                next_instruction = &current_node.right;
            }

            println!("Next instruction: {}", next_instruction);
            steps += 1;

            // We stop when we find ZZZ
            if next_instruction == "ZZZ" {
                println!("Next instruction is {:?}, stopping", next_instruction);
                return steps;
            }

            current_node = parse_result.instructions.get(next_instruction).unwrap();
        }
    }
}

fn parse_instruction_set(input: &String) -> Result<ParseResult, ParseResultError> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut directions: VecDeque<char> = VecDeque::new();
    let mut instructions: HashMap<String, Instruction> = HashMap::new();

    // The first line will be the instruction set consisting of a sequence of "LRLRRLRLRLR"
    for direction in lines[0].chars() {
        directions.push_back(direction);
    }

    for line in lines {
        // Line example: AAA = (BBB, CCC)
        let split_line = line.split(" = ").collect::<Vec<&str>>();

        if split_line.len() != 2 {
            continue;
        }

        println!("Line: {}", line);
        let node = split_line[0];

        let replaced = split_line[1].replace(&['(', ')'], "");
        let split_replaced: Vec<&str> = replaced.split(", ").collect();

        let left = split_replaced[0];
        let right = split_replaced[1];

        let instruction = Instruction {
            left: left.to_string(),
            right: right.to_string(),
        };

        instructions.insert(node.to_string(), instruction);
    }

    return Ok(ParseResult {
        directions,
        instructions: instructions,
    });
}

#[derive(Debug)]
enum ParseResultError {
    InvalidInput,
}

#[derive(Debug)]
struct ParseResult {
    directions: VecDeque<char>,
    instructions: HashMap<String, Instruction>,
}

#[derive(Debug)]
struct Instruction {
    left: String,
    right: String,
}
