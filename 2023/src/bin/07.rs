use anyhow::anyhow;
use itertools::Itertools;
use std::{collections::HashMap, io, str::FromStr};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

const CARD_ORDER: &str = "23456789TJQKA";
const CARD_ORDER_WITH_JOKER: &str = "J23456789TQKA";

#[derive(Eq, PartialEq)]
struct Card {
    hand: String,
    bid: usize,
    card_counts: HashMap<char, usize>,
    joker: bool,
}

impl Card {
    fn new(hand: impl Into<String>, bid: usize) -> Self {
        let hand: String = hand.into();
        let card_counts = hand.chars().counts();
        Card {
            hand,
            bid,
            card_counts,
            joker: false,
        }
    }

    fn joker(mut self) -> Self {
        self.joker = true;
        self
    }

    fn hand_type(&self) -> HandType {
        if self.card_counts.len() == 1 {
            HandType::FiveOfAKind
        } else if self.card_counts.len() == 2 {
            if self.card_counts.values().contains(&4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        } else if self.card_counts.len() == 3 {
            if self.card_counts.values().contains(&3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPairs
            }
        } else if self.card_counts.len() == 4 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn hand_type_with_joker(&self) -> HandType {
        if !self.card_counts.contains_key(&'J') {
            return self.hand_type();
        }

        let non_jokers = self.card_counts.keys().copied().filter(|c| *c != 'J');
        non_jokers
            .map(|c| Card::new(self.hand.replace('J', &c.to_string()), self.bid).hand_type())
            .max()
            .unwrap_or(HandType::FiveOfAKind)
    }

    fn get_card_indices(&self) -> Vec<u8> {
        self.hand
            .chars()
            .map(|c| {
                if self.joker {
                    CARD_ORDER_WITH_JOKER
                } else {
                    CARD_ORDER
                }
                .find(c)
                .unwrap_or_else(|| panic!("Char {c} not found")) as u8
            })
            .collect()
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = aoclib::split_to_tuple2(s, " ").ok_or(anyhow!("Invalid input row"))?;
        let bid = bid.parse()?;
        Ok(Card::new(hand, bid))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        assert_eq!(&self.joker, &other.joker);
        let hand_type_order = if self.joker {
            self.hand_type_with_joker()
                .cmp(&other.hand_type_with_joker())
        } else {
            self.hand_type().cmp(&other.hand_type())
        };
        if hand_type_order.is_eq() {
            let a = self.get_card_indices();
            let b = other.get_card_indices();
            a.cmp(&b)
        } else {
            hand_type_order
        }
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &[String]) -> usize {
    let mut cards: Vec<Card> = input.iter().map(|l| l.parse().unwrap()).collect();
    cards.sort();
    cards
        .into_iter()
        .enumerate()
        .map(|(i, c)| (i + 1) * c.bid)
        .sum()
}

fn part2(input: &[String]) -> usize {
    let mut cards: Vec<_> = input
        .iter()
        .map(|l| l.parse::<Card>().unwrap().joker())
        .collect();
    cards.sort();
    cards
        .into_iter()
        .enumerate()
        .map(|(i, c)| (i + 1) * c.bid)
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(7)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 252052080);

        let p2 = part2(&input);
        assert_eq!(p2, 252898370);
    }
}
