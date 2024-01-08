// WRite a method for classifying hands (count each type of card)
// Implement the Compare trait for a Hand struct

use std::fs;
use std::io;
use std::collections::HashMap;
use std::cmp::Ordering;
use crate::day1::to_u32;


fn count_cards(cs: Vec<Card>) -> (Vec<Card>, HashMap<Card, u32>) {
    let mut cards_count: HashMap<Card, u32> = HashMap::new();
    for c in &cs {
        if cards_count.contains_key(&c) {
            cards_count.insert(*c, cards_count[&c] + 1);
        } else {
            cards_count.insert(*c, 1);
        }
    }
    (cs, cards_count)
}


fn arg_max_no_j(cards_count: &HashMap<Card, u32>) -> Card {
    let filtered_card_counts: Vec<(Card, u32)> = cards_count
        .iter()
        .filter(|(x, _)| *x != &Card::J)
        .map(|(x, i)| (*x, *i))
        .collect();
    let mut most: u32 = 0;
    let mut most_card: Card = Card::One; // placeholder
    for (card, count) in filtered_card_counts {
        if count > most {
            most = count;
            most_card = card;
        } else if count == most && card > most_card {
            most_card = card;
        }
    }
    most_card
}


fn count_cards_2(cs: Vec<Card>) -> (Vec<Card>, HashMap<Card, u32>) {
    let (cs, mut cards_count): (Vec<Card>, HashMap<Card, u32>) = count_cards(cs);
    let mut new_cards: Vec<Card> = cs.clone();
    if cards_count.contains_key(&Card::J) {
        new_cards = Vec::new();
        if cards_count.len() == 1 {  // Only J available
            cards_count = HashMap::from([(Card::One, 5)]);
            new_cards = Vec::from([Card::One, Card::One, Card::One, Card::One, Card::One])
        } else {
            let most_card: Card = arg_max_no_j(&cards_count);
            cards_count.insert(most_card, cards_count[&most_card] + cards_count[&Card::J]);
            cards_count.remove_entry(&Card::J);
            for card in cs {
                if card == Card::J {
                    new_cards.push(Card::One);
                } else {
                    new_cards.push(card);
                }
            }
        }
    }
    (new_cards, cards_count)
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
enum Card {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}


fn type_of_card(c: char) -> Card {
    match c {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::T,
        'J' => Card::J,
        'Q' => Card::Q,
        'K' => Card::K,
        'A' => Card::A,
        _ => panic!("Card not found"),
    }
}


struct Hand {
    hand_type: HandType,
    hand: Vec<Card>,
}


impl Hand {
    fn from(hand_line: &str, count_fn: &dyn Fn(Vec<Card>) -> (Vec<Card>, HashMap<Card, u32>)) -> Hand {
        let hand: Vec<Card> = hand_line.chars().map(|x| type_of_card(x)).collect();
        let (new_hand, cards_count): (Vec<Card>, HashMap<Card, u32>) = count_fn(hand);
        let hand_type: HandType = type_of_hand(cards_count);

        Hand {
            hand_type: hand_type,
            hand: new_hand,
        }
    }
}


impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        (self.hand_type, self.hand.clone()).cmp(&(other.hand_type, other.hand.clone()))
    }
}


impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}


impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        (self.hand_type, self.hand.clone()) == (other.hand_type, other.hand.clone())
    }
}


impl Eq for Hand {  }


#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}


fn type_of_hand(cards_count: HashMap<Card, u32>) -> HandType {
    match cards_count.len() {
        5 => HandType::High,
        4 => HandType::OnePair,
        3 => match cards_count.values().max().expect("Unknown error") {
            3 => HandType::ThreeOfAKind,
            2 => HandType::TwoPair,
            _ => panic!("Invalid number of cards"),
        },
        2 => match cards_count.values().max().expect("Unknown error") {
            4 => HandType::FourOfAKind,
            3 => HandType::FullHouse,
            _ => panic!("Invalid number of cards"),
        },
        1 => HandType::FiveOfAKind,
        _ => panic!("Invalid number of cards"),
    }
}


pub fn day7_p1(input_file: &str) -> Result<u32, io::Error> {
    let mut hands: Vec<(Hand, u32)> = Vec::new();
    for line in fs::read_to_string(input_file)?.lines() {
        let line_split: Vec<&str> = line.split_whitespace().collect();
        let (hand_line, count): (&str, &str) = (line_split[0], line_split[1]);
        hands.push((Hand::from(hand_line, &count_cards), count.parse::<u32>().expect("Not a valid count value {count}")));
    }

    hands.sort();

    let points: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, c))| c * (to_u32(i)+1))
        .sum();
        
    Ok(points)
}


pub fn day7_p2(input_file: &str) -> Result<u32, io::Error> {
    let mut hands: Vec<(Hand, u32)> = Vec::new();
    for line in fs::read_to_string(input_file)?.lines() {
        let line_split: Vec<&str> = line.split_whitespace().collect();
        let (hand_line, count): (&str, &str) = (line_split[0], line_split[1]);
        hands.push((Hand::from(hand_line, &count_cards_2), count.parse::<u32>().expect("Not a valid count value {count}")));
    }

    //upgrade_j(&mut hands);
    hands.sort();
    println!();

    let points: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, c))| c * (to_u32(i)+1))
        .sum();
        
    Ok(points)
}

