use itertools::Itertools;
use std::{cmp::Ordering, fs};

static CARD_ORDER: &'static str = "J23456789TQKA";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Card(char);

impl Card {
    fn new(c: char) -> Self {
        assert!(CARD_ORDER.contains(c));
        Self(c)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        CARD_ORDER.find(self.0).cmp(&CARD_ORDER.find(other.0))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand(Vec<Card>);

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        Self(cards)
    }

    pub fn groups(&self) -> Vec<usize> {
        let joker_count = self.0.iter().filter(|c| c.0 == 'J').count();
        let groups = self
            .0
            .iter()
            .filter(|c| c.0 != 'J')
            .sorted()
            .group_by(|card| *card);
        let mut group_counts = groups
            .into_iter()
            .map(|(_key, group)| group.count())
            .sorted()
            .rev()
            .collect_vec();
        if let Some(first) = group_counts.first_mut() {
            *first += joker_count;
        } else {
            group_counts.push(joker_count);
        }
        group_counts
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let group_cmp = self.groups().cmp(&other.groups());
        if group_cmp.is_eq() {
            self.0.cmp(&other.0)
        } else {
            group_cmp
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input/day7.txt").unwrap();
    let hands = contents
        .lines()
        .map(|line| {
            let (hand, wager) = line.split(" ").collect_tuple().unwrap();
            let cards = hand.chars().map(Card::new).collect::<Vec<_>>();
            (Hand::new(cards), wager.parse::<usize>().unwrap())
        })
        .sorted()
        .collect_vec();

    println!("hands: {hands:#?}");

    let value: usize = hands
        .iter()
        .enumerate()
        .map(|(index, (_hand, wager))| (index + 1) * wager)
        .sum();

    println!("{value}")
}
