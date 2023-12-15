use std::{cmp::Ordering, collections::HashMap, num::ParseIntError};

static CARDS: phf::Map<char, u8> = phf::phf_map! {
    'A' => 13,
    'K' => 12,
    'Q' => 11,
    'J' => 10,
    'T' => 9,
    '9' => 8,
    '8' => 7,
    '7' => 6,
    '6' => 5,
    '5' => 4,
    '4' => 3,
    '3' => 2,
    '2' => 1,
};

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path);

    if input.is_err() {
        return "Error reading input file".to_string();
    }

    let input = input.unwrap();

    let hands = get_hands_from_input(input);

    match hands {
        Ok(mut hands) => {
            let winnings = get_winnings(&mut hands);

            return format!("Total winnings: {}", winnings.iter().sum::<u128>());
        }
        Err(error) => format!("Error parsing input: {}", error),
    }
}

fn get_winnings(hands: &mut Vec<Hand>) -> Vec<u128> {
    let mut winnings: Vec<u128> = Vec::new();

    hands.sort_by(|left, right| left.compare_hand_strength(right));

    // Index + 1 will be the "rank" of the hand
    for (index, hand) in hands.iter().enumerate() {
        winnings.push(hand.bid as u128 * (index as u128 + 1));
    }

    return winnings;
}

fn get_hands_from_input(input: String) -> Result<Vec<Hand>, ParseIntError> {
    let mut hands: Vec<Hand> = Vec::new();

    for line in input.lines() {
        let line_split = line.split(" ").collect::<Vec<&str>>();
        let mut cards_map: HashMap<Card, u8> = HashMap::new();
        let bid = line_split[1].parse::<u16>()?;
        let mut cards: [char; 5] = ['2'; 5];

        for (index, character) in line_split[0].chars().into_iter().enumerate() {
            let card_value = CARDS.get(&character).unwrap();
            let card = Card {
                key: character,
                value: *card_value,
            };
            cards[index] = character;
            *cards_map.entry(card).or_insert(0) += 1;
        }

        hands.push(Hand::new(cards, cards_map, bid));
    }

    Ok(hands)
}

#[derive(Debug)]
struct Hand {
    bid: u16,
    /// The 5 cards in the hand
    cards: [char; 5],
    /// How often every card appears in the hand
    best_combination: Combination,
}

impl Hand {
    fn new(cards: [char; 5], cards_map: HashMap<Card, u8>, bid: u16) -> Self {
        let combination = Combination::get_best_combination(&cards_map);
        Hand {
            cards,
            bid,
            best_combination: combination,
        }
    }

    fn compare_hand_strength(&self, other: &Hand) -> Ordering {
        let left_combination = self.best_combination as u8;
        let right_combination = other.best_combination as u8;

        // First we'll check if either of the combinations is greater than the other
        if left_combination > right_combination {
            return Ordering::Greater;
        } else if left_combination < right_combination {
            return Ordering::Less;
        }

        // Then we'll go over every character to find the first with a higher value
        for (index, card) in self.cards.iter().enumerate() {
            let left_value = CARDS.get(card).unwrap();
            let right_value = CARDS.get(&other.cards[index]).unwrap();

            if left_value == right_value {
                continue;
            }

            if left_value > right_value {
                return Ordering::Greater;
            } else if left_value < right_value {
                return Ordering::Less;
            }
        }

        return Ordering::Equal;
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Card {
    key: char,
    value: u8,
}

impl Default for Card {
    fn default() -> Self {
        Card { key: '2', value: 1 }
    }
}

#[derive(Debug, Clone, Copy)]
enum Combination {
    FiveOfAKind = 10,
    FourOfAKind = 9,
    FullHouse = 8,
    ThreeOfAKind = 7,
    TwoPair = 6,
    OnePair = 5,
    HighCard = 4,
}

impl Combination {
    fn get_best_combination(cards: &HashMap<Card, u8>) -> Self {
        let cards = cards;
        let mut combination = Combination::HighCard;

        if cards.len() == 1 {
            combination = Combination::FiveOfAKind;
        } else if cards.len() == 2 {
            for (_, count) in cards.iter() {
                if *count == 4 {
                    combination = Combination::FourOfAKind;
                } else if *count == 3 {
                    combination = Combination::FullHouse;
                }
            }
        } else if cards.len() == 3 {
            for (_, count) in cards.iter() {
                if *count == 3 {
                    combination = Combination::ThreeOfAKind;
                } else if *count == 2 {
                    combination = Combination::TwoPair;
                }
            }
        } else if cards.len() == 4 {
            combination = Combination::OnePair;
        }

        combination
    }
}
