pub fn solve(path: String) -> String {
    let input = std::fs::read_to_string(path).expect("Unable to read file");

    let mut total_winnings = 0;
    for line in input.lines() {
        let card = Card::new(line);
        // First store cards in a list
        // Then iterate over every card and depending on winnings_power, add copies of the next x amount of cards to the list
        total_winnings += card.get_winnings().0;

        println!();
    }

    println!("------------------------------------------");
    println!("Total winnings: {}", total_winnings);

    return total_winnings.to_string();
}

struct Card {
    card_number: u32,
    winning_numbers: Vec<u32>,
    scratched_numbers: Vec<u32>,
}

impl Card {
    /// Parses a line of text into a Card struct
    /// The line must be in the following format "Card n: n n n n n | x x x x x" where n is a number on the card and x is a scratched number
    pub fn new(line: &str) -> Card {
        println!("Started parsing: {}", line);
        let card_and_numbers_split: Vec<&str> = line.split(":").collect();
        let card_number = card_and_numbers_split[0]
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .expect("Unable to parse card number");
        println!("Card number: {}", card_number);

        let numbers_split: Vec<&str> = card_and_numbers_split[1].split("|").collect();
        let numbers: Vec<&str> = numbers_split[0].trim().split_whitespace().collect();
        let scratched: Vec<&str> = numbers_split[1].trim().split_whitespace().collect();

        println!("Numbers: {:?}", numbers);
        println!("Scratched: {:?}", scratched);

        return Card {
            card_number,
            winning_numbers: numbers
                .iter()
                .map(|x| x.parse::<u32>().expect("Unable to parse winning number"))
                .collect(),
            scratched_numbers: scratched
                .iter()
                .map(|x| x.parse::<u32>().expect("Unable to parse winning number"))
                .collect(),
        };
    }

    pub fn get_winnings(&self) -> (u32, u32) {
        let mut winnings_power = 0;
        let mut winnings = 0;

        for number in self.scratched_numbers.iter() {
            if self.winning_numbers.contains(&number) {
                winnings_power += 1;
            }
        }

        if winnings_power > 0 {
            winnings = 1;
            for _ in 1..winnings_power {
                winnings *= 2;
            }
        }

        println!("Winnings: {}", winnings);
        return (winnings, winnings_power);
    }
}
