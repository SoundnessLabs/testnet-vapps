use crate::types::card::{Card, Rank};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HandRank {
    HighCard = 1,
    Pair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    Straight = 5,
    Flush = 6,
    FullHouse = 7,
    FourOfAKind = 8,
    StraightFlush = 9,
    RoyalFlush = 10,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub rank: HandRank,
    pub score: u32,
    pub kickers: Vec<Rank>,
}

impl Hand {
    pub fn evaluate(mut cards: Vec<Card>) -> Self {
        assert_eq!(cards.len(), 7, "Hand evaluation requires exactly 7 cards");
        
        // Sort cards by rank (descending)
        cards.sort_by(|a, b| b.rank.cmp(&a.rank));
        
        // Find best 5-card hand from 7 cards
        let best_hand = Self::find_best_hand(&cards);
        best_hand
    }

    fn find_best_hand(cards: &[Card]) -> Hand {
        // Generate all possible 5-card combinations
        let combinations = Self::generate_combinations(cards, 5);
        
        let mut best_hand = None;
        let mut best_score = 0;

        for combo in combinations {
            let hand = Self::evaluate_five_cards(combo);
            if hand.score > best_score {
                best_score = hand.score;
                best_hand = Some(hand);
            }
        }

        best_hand.expect("Should always find a valid hand")
    }

    fn evaluate_five_cards(mut cards: Vec<Card>) -> Hand {
        cards.sort_by(|a, b| b.rank.cmp(&a.rank));
        
        // Check for flush
        let is_flush = cards.iter().all(|c| c.suit == cards[0].suit);
        
        // Check for straight
        let is_straight = Self::is_straight(&cards);
        
        // Count ranks
        let mut rank_counts: HashMap<Rank, usize> = HashMap::new();
        for card in &cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
        }

        let mut counts: Vec<usize> = rank_counts.values().cloned().collect();
        counts.sort_by(|a, b| b.cmp(a));

        // Determine hand rank and calculate score
        let (hand_rank, kickers) = match (is_flush, is_straight, &counts[..]) {
            (true, true, _) => {
                if cards[0].rank == Rank::Ace && cards[4].rank == Rank::Ten {
                    (HandRank::RoyalFlush, vec![])
                } else {
                    (HandRank::StraightFlush, vec![cards[0].rank])
                }
            },
            (false, false, [4, 1]) => {
                let four_rank = Self::get_rank_with_count(&rank_counts, 4);
                let kicker = Self::get_rank_with_count(&rank_counts, 1);
                (HandRank::FourOfAKind, vec![four_rank, kicker])
            },
            (false, false, [3, 2]) => {
                let three_rank = Self::get_rank_with_count(&rank_counts, 3);
                let pair_rank = Self::get_rank_with_count(&rank_counts, 2);
                (HandRank::FullHouse, vec![three_rank, pair_rank])
            },
            (true, false, _) => {
                let kickers = cards.iter().map(|c| c.rank).collect();
                (HandRank::Flush, kickers)
            },
            (false, true, _) => {
                (HandRank::Straight, vec![cards[0].rank])
            },
            (false, false, [3, 1, 1]) => {
                let three_rank = Self::get_rank_with_count(&rank_counts, 3);
                let mut kickers: Vec<Rank> = rank_counts.iter()
                    .filter(|(_, &count)| count == 1)
                    .map(|(&rank, _)| rank)
                    .collect();
                kickers.sort_by(|a, b| b.cmp(a));
                kickers.insert(0, three_rank);
                (HandRank::ThreeOfAKind, kickers)
            },
            (false, false, [2, 2, 1]) => {
                let mut pairs: Vec<Rank> = rank_counts.iter()
                    .filter(|(_, &count)| count == 2)
                    .map(|(&rank, _)| rank)
                    .collect();
                pairs.sort_by(|a, b| b.cmp(a));
                let kicker = Self::get_rank_with_count(&rank_counts, 1);
                pairs.push(kicker);
                (HandRank::TwoPair, pairs)
            },
            (false, false, [2, 1, 1, 1]) => {
                let pair_rank = Self::get_rank_with_count(&rank_counts, 2);
                let mut kickers: Vec<Rank> = rank_counts.iter()
                    .filter(|(_, &count)| count == 1)
                    .map(|(&rank, _)| rank)
                    .collect();
                kickers.sort_by(|a, b| b.cmp(a));
                kickers.insert(0, pair_rank);
                (HandRank::Pair, kickers)
            },
            _ => {
                let kickers = cards.iter().map(|c| c.rank).collect();
                (HandRank::HighCard, kickers)
            }
        };

        let score = Self::calculate_score(&hand_rank, &kickers);

        Hand {
            cards,
            rank: hand_rank,
            score,
            kickers,
        }
    }

    fn is_straight(cards: &[Card]) -> bool {
        if cards.len() != 5 {
            return false;
        }

        // Check for regular straight
        for i in 0..4 {
            if cards[i].rank as u8 != cards[i + 1].rank as u8 + 1 {
                // Check for wheel straight (A-2-3-4-5)
                if i == 0 && cards[0].rank == Rank::Ace && cards[1].rank == Rank::Five {
                    continue;
                }
                return false;
            }
        }
        true
    }

    fn get_rank_with_count(rank_counts: &HashMap<Rank, usize>, count: usize) -> Rank {
        *rank_counts.iter()
            .find(|(_, &c)| c == count)
            .map(|(rank, _)| rank)
            .expect("Rank with count should exist")
    }

    fn calculate_score(hand_rank: &HandRank, kickers: &[Rank]) -> u32 {
        let base_score = (*hand_rank as u32) * 1_000_000;
        let mut kicker_score = 0;
        
        for (i, &kicker) in kickers.iter().enumerate() {
            kicker_score += (kicker as u32) * (100_u32.pow(4 - i as u32));
        }
        
        base_score + kicker_score
    }

    fn generate_combinations(cards: &[Card], k: usize) -> Vec<Vec<Card>> {
        if k == 0 {
            return vec![vec![]];
        }
        if cards.is_empty() {
            return vec![];
        }

        let mut result = Vec::new();
        
        // Include first card
        let with_first = Self::generate_combinations(&cards[1..], k - 1);
        for mut combo in with_first {
            combo.insert(0, cards[0]);
            result.push(combo);
        }
        
        // Exclude first card
        let without_first = Self::generate_combinations(&cards[1..], k);
        result.extend(without_first);
        
        result
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::card::{Card, Rank, Suit};

    #[test]
    fn test_royal_flush() {
        let cards = vec![
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
            Card::new(Suit::Spades, Rank::Queen),
            Card::new(Suit::Spades, Rank::Jack),
            Card::new(Suit::Spades, Rank::Ten),
            Card::new(Suit::Hearts, Rank::Two),
            Card::new(Suit::Clubs, Rank::Three),
        ];

        let hand = Hand::evaluate(cards);
        assert_eq!(hand.rank, HandRank::RoyalFlush);
    }

    #[test]
    fn test_straight_flush() {
        let cards = vec![
            Card::new(Suit::Hearts, Rank::Nine),
            Card::new(Suit::Hearts, Rank::Eight),
            Card::new(Suit::Hearts, Rank::Seven),
            Card::new(Suit::Hearts, Rank::Six),
            Card::new(Suit::Hearts, Rank::Five),
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Clubs, Rank::King),
        ];

        let hand = Hand::evaluate(cards);
        assert_eq!(hand.rank, HandRank::StraightFlush);
    }

    #[test]
    fn test_four_of_a_kind() {
        let cards = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Diamonds, Rank::Ace),
            Card::new(Suit::Clubs, Rank::Ace),
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Diamonds, Rank::Queen),
            Card::new(Suit::Clubs, Rank::Jack),
        ];

        let hand = Hand::evaluate(cards);
        assert_eq!(hand.rank, HandRank::FourOfAKind);
        assert_eq!(hand.kickers[0], Rank::Ace);
        assert_eq!(hand.kickers[1], Rank::King);
    }
}

