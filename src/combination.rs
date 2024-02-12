use super::hand::Hand;
use darth_rust::DarthRust;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Combination {
    RoyalStraightFlush(Hand), // A Coroa Real
    StraightFlush(Hand),      // Escada Real
    FourOfAKind(Hand),        // Quadra ou Poker
    FullHouse(Hand),          // Full ou Casa Cheia
    Flush(Hand),              // Flush ou Cor
    Straight(Hand),           // Sequência ou Escada
    ThreeOfAKind(Hand),       // Trinca
    TwoPair(Hand),            // Dois Pares
    OnePair(Hand),            // Um Par
    HighCard(Hand),           // Carta Alta
}

#[derive(Clone, Debug, DarthRust, Default, PartialEq, Eq, Hash)]
pub struct Power {
    pub value: usize,
}

pub trait CombinationTrait {
    fn power(&self) -> Power;
    fn to_str(&self) -> &str;
}

impl CombinationTrait for Combination {
    fn power(&self) -> Power {
        let sum = |hand: &Hand| {
            hand.cards.iter().map(|c| c.number.value()).sum::<usize>()
        };
        match self {
            Self::RoyalStraightFlush(hand) => {
                let mut power = Power::new(10000);
                power.sum_value(sum(hand));
                power
            }
            Self::StraightFlush(hand) => {
                let mut power = Power::new(9000);
                power.sum_value(sum(hand));
                power
            }
            Self::FourOfAKind(hand) => {
                let mut power = Power::new(8000);
                power.sum_value(sum(hand));
                power
            }
            Self::FullHouse(hand) => {
                let mut power = Power::new(7000);
                power.sum_value(sum(hand));
                power
            }
            Self::Flush(hand) => {
                let mut power = Power::new(6000);
                power.sum_value(sum(hand));
                power
            }
            Self::Straight(hand) => {
                let mut power = Power::new(5000);
                power.sum_value(sum(hand));
                power
            }
            Self::ThreeOfAKind(hand) => {
                let mut power = Power::new(4000);
                power.sum_value(sum(hand));
                power
            }
            Self::TwoPair(hand) => {
                let mut power = Power::new(3000);
                power.sum_value(sum(hand));
                power
            }
            Self::OnePair(hand) => {
                let mut power = Power::new(2000);
                power.sum_value(sum(hand));
                power
            }
            Self::HighCard(hand) => {
                let mut power = Power::new(1000);
                power.sum_value(sum(hand));
                power
            }
        }
    }

    fn to_str(&self) -> &str {
        match *self {
            Self::RoyalStraightFlush(_) => "Royal Straight Flush",
            Self::StraightFlush(_) => "Straight Flush",
            Self::FourOfAKind(_) => "Four of a Kind",
            Self::FullHouse(_) => "Full House",
            Self::Flush(_) => "Flush",
            Self::Straight(_) => "Straight",
            Self::ThreeOfAKind(_) => "Three of a Kind",
            Self::TwoPair(_) => "Two Pair",
            Self::OnePair(_) => "One Pair",
            Self::HighCard(_) => "High Card",
        }
    }
}
