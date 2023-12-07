use std::str::FromStr;

use arrayvec::ArrayVec;
use itertools::Itertools;

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_seven.txt");

const HAND_ORDER: &'static [fn(&Hand) -> bool] = &[
    Hand::is_five_of_a_kind,
    Hand::is_four_of_a_kind,
    Hand::is_full_house,
    Hand::is_three_of_a_kind,
    Hand::is_two_pair,
    Hand::is_one_pair,
    Hand::is_high_card,
];

const HAND_ORDER_WITH_JOKERS: &'static [fn(&HandWithJokers, u8) -> bool] = &[
    HandWithJokers::is_five_of_a_kind,
    HandWithJokers::is_four_of_a_kind,
    HandWithJokers::is_full_house,
    HandWithJokers::is_three_of_a_kind,
    HandWithJokers::is_two_pair,
    HandWithJokers::is_one_pair,
    HandWithJokers::is_high_card,
];

#[derive(Debug, Clone)]
struct Hand {
    pub hand_string: String,
    pub cards: [u8; 5],
    pub bid: u32,
    pub grouped: Vec<(u8, u8)>,
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_string, bid) = s.split_whitespace().collect_tuple().unwrap();

        let cards = hand_string
            .chars()
            .map(|card| {
                return match card {
                    'A' => 13,
                    'K' => 12,
                    'Q' => 11,
                    'J' => 10,
                    'T' => 9,
                    _ => (card.to_digit(10).unwrap() - 1) as u8,
                };
            })
            .collect::<ArrayVec<_, 5>>()
            .into_inner()
            .unwrap();

        Ok(Self {
            hand_string: hand_string.to_string(),
            cards,
            bid: bid.parse().unwrap(),
            grouped: cards
                .into_iter()
                .sorted()
                .group_by(|c| *c)
                .into_iter()
                .map(|(cat, grp)| (cat, grp.count() as u8))
                .collect(),
        })
    }
}

impl Hand {
    // This is always a 4 bit unsigned integer.
    pub fn category(&self) -> u8 {
        let len = HAND_ORDER.len();
        for (i, order_func) in HAND_ORDER.iter().enumerate() {
            if order_func(&self) {
                return (len - i) as u8;
            }
        }

        unreachable!()
    }

    pub fn get_value(&self) -> u32 {
        let category = self.category();
        let mut value = category as _;

        for card in self.cards {
            value = (value << 4) | card as u32;
        }

        value
    }

    pub fn is_five_of_a_kind(&self) -> bool {
        self.grouped.len() == 1
    }

    pub fn is_four_of_a_kind(&self) -> bool {
        self.grouped.len() == 2 && self.grouped[0].1.abs_diff(self.grouped[1].1) == 3
    }

    pub fn is_full_house(&self) -> bool {
        self.grouped.len() == 2 && self.grouped[0].1.abs_diff(self.grouped[1].1) == 1
    }

    pub fn is_three_of_a_kind(&self) -> bool {
        self.grouped.len() == 3 && self.grouped.iter().any(|(_c, count)| *count > 2)
    }

    pub fn is_two_pair(&self) -> bool {
        self.grouped.len() == 3 && !self.grouped.iter().any(|(_c, count)| *count > 2)
    }

    pub fn is_one_pair(&self) -> bool {
        self.grouped.len() == 4
    }

    pub fn is_high_card(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
struct HandWithJokers {
    pub hand_string: String,
    pub cards: [u8; 5],
    pub bid: u32,
    pub grouped: Vec<(u8, u8)>,
}

impl FromStr for HandWithJokers {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_string, bid) = s.split_whitespace().collect_tuple().unwrap();

        let cards = hand_string
            .chars()
            .map(|card| {
                return match card {
                    'A' => 13,
                    'K' => 12,
                    'Q' => 11,
                    'J' => 1,
                    'T' => 10,
                    _ => (card.to_digit(10).unwrap()) as u8,
                };
            })
            .collect::<ArrayVec<_, 5>>()
            .into_inner()
            .unwrap();

        Ok(Self {
            hand_string: hand_string.to_string(),
            cards,
            bid: bid.parse().unwrap(),
            grouped: cards
                .into_iter()
                .sorted()
                .group_by(|c| *c)
                .into_iter()
                .map(|(cat, grp)| (cat, grp.count() as u8))
                .collect(),
        })
    }
}

impl HandWithJokers {
    // This is always a 4 bit unsigned integer.
    pub fn category(&self) -> u8 {
        let len = HAND_ORDER_WITH_JOKERS.len();
        let joker_count = self.joker_count();
        for (i, order_func) in HAND_ORDER_WITH_JOKERS.iter().enumerate() {
            if order_func(&self, joker_count) {
                return (len - i) as u8;
            }
        }

        unreachable!()
    }

    pub fn get_value(&self) -> u32 {
        let category = self.category();
        let mut value = category as _;

        for card in self.cards {
            value = (value << 4) | card as u32;
        }

        value
    }

    pub fn joker_count(&self) -> u8 {
        self.grouped
            .iter()
            .find_map(|(c, count)| {
                if *c == 1 {
                    return Some(*count);
                }
                None
            })
            .unwrap_or(0)
    }

    pub fn is_five_of_a_kind(&self, joker_count: u8) -> bool {
        self.grouped.len() == 1
            || self
                .grouped
                .iter()
                .any(|(_, count)| (*count + joker_count as u8) == 5)
    }

    pub fn is_four_of_a_kind(&self, joker_count: u8) -> bool {
        (self.grouped.len() == 2 && self.grouped[0].1.abs_diff(self.grouped[1].1) == 3)
            || self
                .grouped
                .iter()
                .filter(|(c, _)| *c != 1)
                .any(|(_, count)| (*count + joker_count as u8) == 4)
    }

    pub fn is_full_house(&self, joker_count: u8) -> bool {
        self.grouped.len() == 2 && self.grouped[0].1.abs_diff(self.grouped[1].1) == 1
            || (self.grouped.len() == 3 && joker_count == 1)
    }

    pub fn is_three_of_a_kind(&self, joker_count: u8) -> bool {
        (self.grouped.len() == 3 && self.grouped.iter().any(|(_c, count)| *count > 2))
            || self
                .grouped
                .iter()
                .filter(|(c, _)| *c != 1)
                .any(|(_, count)| (*count + joker_count as u8) == 3)
    }

    pub fn is_two_pair(&self, joker_count: u8) -> bool {
        (self.grouped.len() == 3 && !self.grouped.iter().any(|(_c, count)| *count > 2))
            || (joker_count == 1 && self.grouped.len() == 4)
    }

    pub fn is_one_pair(&self, joker_count: u8) -> bool {
        self.grouped.len() == 4 || (joker_count == 1 && self.grouped.len() == 5)
    }

    pub fn is_high_card(&self, _joker_count: u8) -> bool {
        true
    }
}

pub fn part_one(_args: Args) {
    let hands = FILE_CONTENTS
        .lines()
        .map(|l| l.parse::<Hand>().unwrap())
        .collect_vec();

    let scored = hands
        .iter()
        .map(|h| (h, h.get_value()))
        .sorted_by(|(_hand, score), (_hand_right, score_right)| {
            score.partial_cmp(score_right).unwrap()
        })
        .map(|(hand, _score)| hand)
        .collect_vec();

    for (i, hand) in scored.iter().enumerate() {
        println!(
            "{}:\t{}\t{}\t(value: {:#b})",
            i + 1,
            hand.hand_string,
            hand.bid,
            hand.get_value()
        );
    }

    let total = scored
        .iter()
        .enumerate()
        .map(|(i, s)| s.bid as usize * (i + 1))
        .sum::<usize>();

    println!("Total: {}", total);
}

pub fn part_two(_args: Args) {
    let hands = FILE_CONTENTS
        .lines()
        .map(|l| l.parse::<HandWithJokers>().unwrap())
        .collect_vec();

    let scored = hands
        .iter()
        .map(|h| (h, h.get_value()))
        .sorted_by(|(_hand, score), (_hand_right, score_right)| {
            score.partial_cmp(score_right).unwrap()
        })
        .map(|(hand, _score)| hand)
        .collect_vec();

    for (i, hand) in scored.iter().enumerate() {
        println!(
            "{}:\t{}\t{}\t(value: {:#b}, category: {})",
            i + 1,
            hand.hand_string,
            hand.bid,
            hand.get_value(),
            hand.category()
        );
    }

    let total = scored
        .iter()
        .enumerate()
        .map(|(i, s)| s.bid as usize * (i + 1))
        .sum::<usize>();

    println!("Total: {}", total);
}
