use std::borrow::Borrow;
use std::fs;

use std::cmp::Ordering;
use itertools::Itertools;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Card {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn new(c: &char) -> Self {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::_9,
            '8' => Card::_8,
            '7' => Card::_7,
            '6' => Card::_6,
            '5' => Card::_5,
            '4' => Card::_4,
            '3' => Card::_3,
            '2' => Card::_2,
            &_ => panic!()
        }
    }
    fn simple_score(&self) -> u8 {
        match self {
            Card::_2 => 2,
            Card::_3 => 3,
            Card::_4 => 4,
            Card::_5 => 5,
            Card::_6 => 6,
            Card::_7 => 7,
            Card::_8 => 8,
            Card::_9 => 9,
            Card::T => 10,
            Card::J => 11,
            Card::Q => 12,
            Card::K => 13,
            Card::A => 14
        }
    }
    fn joker_score(&self) -> u8 {
        match self {
            Card::J => 1,
            _ => self.simple_score()
        }
    }
}


#[derive(PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

fn simple_hand_type(cards: &Vec<Card>) -> HandType {
    if cards.len() == 0 {
        return HandType::HighCard;
    }
    let mut counter: HashMap<&Card, u64> = HashMap::new();
    for card in cards {
        if !counter.contains_key(card) {
            counter.insert(card, 0);
        }
        counter.insert(card, counter.get(card).unwrap() + 1);
    }
    let counts: Vec<&u64> = counter.values().sorted().rev().collect();
    if *counts[0] == 5 {
        HandType::Five
    } else if *counts[0] == 4 {
        HandType::Four
    } else if *counts[0] == 3 && counts.len() > 1 && *counts[1] == 2 {
        HandType::Full
    } else if *counts[0] == 3 {
        HandType::Three
    } else if *counts[0] == 2 && counts.len() > 1 && *counts[1] == 2 {
        HandType::TwoPair
    } else if *counts[0] == 2 {
        HandType::Pair
    } else {
        HandType::HighCard
    }
}

fn wildcard_hand_type(cards: &Vec<Card>) -> HandType {
    let cards_without_jokers: Vec<Card> = cards
        .iter()
        .filter(|&x| x.ne(&Card::J))
        .map(|x| x.clone())
        .collect();
    let jokers = cards.len() - cards_without_jokers.len();
    let hand = simple_hand_type(&cards_without_jokers);
    match jokers {
        5 | 4 => HandType::Five,
        3 =>
            match hand {
                HandType::Pair => HandType::Five,
                HandType::HighCard => HandType::Four,
                _ => panic!()
            },
        2 => match hand {
            HandType::Three => HandType::Five,
            HandType::Pair => HandType::Four,
            HandType::HighCard => HandType::Three,
            _ => panic!()
        },
        1 => match hand {
            HandType::Four => HandType::Five,
            HandType::Three => HandType::Four,
            HandType::TwoPair => HandType::Full,
            HandType::Pair => HandType::Three,
            HandType::HighCard => HandType::Pair,
            _ => panic!()
        },
        _ => hand
    }
}


struct Hand {
    cards: Vec<Card>,
    card_scores: Vec<u8>,
    bid: u64,
    hand_type: HandType,
}

impl Hand {
    fn new(line: &str, card_score: &impl Fn(&Card) -> u8, hand_creator: &impl Fn(&Vec<Card>) -> HandType) -> Self {
        let (c, m) = line.split(" ").collect_tuple().unwrap();
        let bid = m.parse().unwrap();
        let cards: Vec<Card> = c.chars()
            .map(|c| Card::new(&c))
            .collect();
        let card_scores = cards
            .iter()
            .map(|c| card_score(c))
            .collect();
        let hand_type = hand_creator(&cards);
        Hand {
            cards,
            card_scores,
            bid,
            hand_type,
        }
    }

    fn cmp_high_card(&self, other: &Hand) -> Ordering {
        self.card_scores
            .iter()
            .zip(&other.card_scores)
            .map(|(a, b)| a.cmp(b))
            .find_or_first(|ordering| ordering.is_ne())
            .unwrap()
    }
}

impl Eq for Hand {}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cmp_high_card(other).is_eq()
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        return if self.hand_type > other.hand_type {
            Ordering::Greater
        } else if self.hand_type < other.hand_type {
            Ordering::Less
        } else {
            self.cmp_high_card(other)
        };
    }
}

fn parse(data: &str, card_score: &impl Fn(&Card) -> u8, hand_creator: &impl Fn(&Vec<Card>) -> HandType) -> Vec<Hand> {
    data.lines()
        .map(|line| Hand::new(line, card_score, hand_creator))
        .collect()
}

fn score(hands: &Vec<Hand>) -> u64 {
    hands.iter()
        .sorted()
        .enumerate()
        .map(|(i, h)| ((i + 1) as u64) * h.bid)
        .sum()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("7.txt").unwrap();
    let hands = parse(&contents, &|c: &Card| c.simple_score(), &|v| simple_hand_type(v));
    println!("{}", score(&hands));
    let hands_with_joker = parse(&contents, &|c| c.joker_score(), &|v| wildcard_hand_type(v));
    println!("{}", score(&hands_with_joker));
}