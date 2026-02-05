use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::{fmt, fs};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Card(char);

impl Card {
    fn new(c: char) -> Self {
        match c {
            '2' => Self('2'),
            '3' => Self('3'),
            '4' => Self('4'),
            '5' => Self('5'),
            '6' => Self('6'),
            '7' => Self('7'),
            '8' => Self('8'),
            '9' => Self('9'),
            'T' => Self('T'),
            'J' => Self('J'),
            'Q' => Self('Q'),
            'K' => Self('K'),
            'A' => Self('A'),
            _ => panic!("Invalid card: {}", c),
        }
    }

    fn rank(&self) -> usize {
        match self.0 {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => 9,
            'T' => 8,
            '9' => 7,
            '8' => 6,
            '7' => 5,
            '6' => 4,
            '5' => 3,
            '4' => 2,
            '3' => 1,
            '2' => 0,
            _ => panic!("Invalid card: {}", self.0),
        }
    }
}

#[derive(Clone)]
struct Hand {
    cards: [Card; 5],
    rank: HandRank,
    bet: usize,
}

impl Hand {
    fn new(cards: [char; 5], bet: usize) -> Self {
        let cards = cards.map(Card::new);
        let hand_rank = HandRank::from_cards(&cards);
        Self {
            cards,
            rank: hand_rank,
            bet,
        }
    }

    fn from_string(s: &str) -> Self {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let bet = parts[1].parse::<usize>().unwrap();
        let cards = parts[0].chars().collect::<Vec<char>>().try_into().unwrap();
        Self::new(cards, bet)
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} -> {}",
            self.cards.iter().map(|card| card.0).collect::<String>(),
            self.bet,
            self.rank
        )
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} -> {}",
            self.cards.iter().map(|card| card.0).collect::<String>(),
            self.bet,
            self.rank
        )
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rank != other.rank {
            return self.rank.cmp(&other.rank);
        }
        let mut ordering = Ordering::Equal;
        self.cards
            .iter()
            .zip(other.cards.iter())
            .for_each(|(self_card, other_card)| {
                if ordering != Ordering::Equal {
                    return;
                }
                let self_rank = self_card.rank();
                let other_rank = other_card.rank();
                if self_rank != other_rank {
                    ordering = self_rank.cmp(&other_rank);
                    return;
                }
            });
        ordering
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hand {}

#[derive(Debug, Clone)]
struct HandWithJokers {
  hand: Hand,
  upgraded_hand: Hand,
}

#[derive(Debug, Clone, Copy)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandRank {
    fn to_int(&self) -> usize {
        match self {
            Self::HighCard => 0,
            Self::OnePair => 1,
            Self::TwoPair => 2,
            Self::ThreeOfAKind => 3,
            Self::FullHouse => 4,
            Self::FourOfAKind => 5,
            Self::FiveOfAKind => 6,
        }
    }

    fn from_cards(cards: &[Card]) -> Self {
        let mut counts = [0usize; 13];
        for card in cards.iter() {
            counts[card.rank()] += 1;
        }
        let max_count = counts.iter().max().unwrap();
        if *max_count == 5 {
            return Self::FiveOfAKind;
        } else if *max_count == 4 {
            return Self::FourOfAKind;
        }

        if *max_count == 3 && counts.iter().any(|&count| count == 2) {
            return Self::FullHouse;
        }

        if counts.iter().filter(|&count| *count == 2).count() == 2 {
            return Self::TwoPair;
        }

        if *max_count == 3 {
            return Self::ThreeOfAKind;
        }

        if counts.iter().filter(|&count| *count == 2).count() == 1 {
            return Self::OnePair;
        }

        Self::HighCard
    }
}

impl Display for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Ord for HandRank {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_int = self.to_int();
        let other_int = other.to_int();
        if self_int != other_int {
            return self_int.cmp(&other_int);
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandRank {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for HandRank {}

fn read_input(filename: &str) -> Vec<Hand> {
    let mut hands = Vec::new();
    let input = fs::read_to_string(filename).unwrap();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let hand = Hand::from_string(line);
        hands.push(hand);
    }
    hands
}

fn part_one(hands: Vec<Hand>) -> usize {
    let mut hands = hands;
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bet * (i + 1))
        .sum()
}

fn upgrade_hand(hand: &Hand) -> Hand {
    let counts = hand.cards.iter().fold(HashMap::new(), |mut acc, card| {
        *acc.entry(card.clone()).or_insert(0) += 1;
        acc
    });
    let num_jokers = counts.get(&Card('J')).unwrap_or(&0);
    if *num_jokers == 0 {
        return hand.clone();
    }
    let max_count_result = counts
        .iter()
        .filter(|(card, _)| card.0 != 'J')
        .max_by_key(|(_, count)| **count);
    if max_count_result.is_none() {
        return Hand::new(['A'; 5], hand.bet);
    }
    let max_count = max_count_result.unwrap().1;
    let mut cards_with_max_count = counts
        .iter()
        .filter(|(_, count)| **count == *max_count)
        .map(|(card, _)| card.clone())
        .collect::<Vec<Card>>();
    cards_with_max_count.sort_by(|a, b| {
        if a.0 == 'J' {
            Ordering::Greater
        } else if b.0 == 'J' {
            Ordering::Less
        } else {
            b.rank().cmp(&a.rank())
        }
    });

    let best_card = cards_with_max_count[0];
    let mut new_cards = ['A'; 5];
    for i in 0..5 {
        if hand.cards[i] == best_card || hand.cards[i] == Card('J') {
            new_cards[i] = best_card.0;
        } else {
            new_cards[i] = hand.cards[i].0;
        }
    }
    Hand::new(new_cards, hand.bet)
}

fn compare_hands_with_jokers(a: &Hand, b: &Hand) -> Ordering {
  if a.rank != b.rank {
    return a.rank.cmp(&b.rank);
  }
  let mut ordering = Ordering::Equal;
  a.cards.iter().zip(b.cards.iter()).for_each(|(a_card, b_card)| {
    if ordering != Ordering::Equal {
      return;
    }
    let a_rank = if a_card.0 == 'J' { 0 } else { a_card.rank() };
    let b_rank = if b_card.0 == 'J' { 0 } else { b_card.rank() };
    if a_rank != b_rank {
      ordering = a_rank.cmp(&b_rank);
      return;
    }
  });
  ordering
}

fn part_two(hands: Vec<Hand>) -> usize {
    let hands_with_jokers = hands.iter().map(|hand| HandWithJokers{ hand: hand.clone(), upgraded_hand: upgrade_hand(hand) }).collect::<Vec<HandWithJokers>>();
    let mut hands_with_adjusted_ranks = hands_with_jokers.iter().map(|hand| Hand { cards: hand.hand.cards, rank: hand.upgraded_hand.rank, bet: hand.upgraded_hand.bet }).collect::<Vec<Hand>>();
    hands_with_adjusted_ranks.sort_by(|a, b| compare_hands_with_jokers(a, b));
    hands_with_adjusted_ranks
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bet * (i + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_comparison() {
        let hand1 = Hand::from_string("KK677 28");
        let hand2 = Hand::from_string("KTJJT 220");
        assert!(hand1 > hand2);
    }

    #[test]
    fn test_part_one() {
        let hands = read_input("test.txt");
        assert_eq!(part_one(hands), 6440);
    }

    #[test]
    fn test_upgrade_hand() {
        let hand = Hand::from_string("KK677 28");
        let upgraded_hand = upgrade_hand(&hand);
        assert_eq!(upgraded_hand, Hand::from_string("KK677 28"));

        let hand = Hand::from_string("KTJJT 220");
        let upgraded_hand = upgrade_hand(&hand);
        assert_eq!(upgraded_hand, Hand::from_string("KTTTT 220"));

        let hand = Hand::from_string("T55J5 483");
        let upgraded_hand = upgrade_hand(&hand);
        assert_eq!(upgraded_hand, Hand::from_string("T5555 483"));

        let hand = Hand::from_string("QQQJA 483");
        let upgraded_hand = upgrade_hand(&hand);
        assert_eq!(upgraded_hand, Hand::from_string("QQQQA 483"));

        let hand = Hand::from_string("JJJJJ 120");
        let upgraded_hand = upgrade_hand(&hand);
        assert_eq!(upgraded_hand, Hand::from_string("AAAAA 120"));
    }

    #[test]
    fn test_upgrade_hand_no_jokers() {
        let hands = read_input("input.txt");
        let upgraded_hands = hands.iter().map(upgrade_hand).collect::<Vec<Hand>>();
        for hand in upgraded_hands {
            let hand_has_jokers = hand.cards.iter().any(|card| card.0 == 'J');
            assert!(!hand_has_jokers);
        }
    }

    #[test]
    fn test_part_two() {
        let hands = read_input("test.txt");
        assert_eq!(part_two(hands), 5905);
    }
}

fn main() {
    let hands = read_input("input.txt");
    println!("{}", part_one(hands.clone()));
    println!("{}", part_two(hands));
}
