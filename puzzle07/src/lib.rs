pub mod parser;
pub mod puzzle07a;
pub mod puzzle07b;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Card {
    Number(u32),
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand([Card; 5]);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRanking {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

impl Hand {
    pub fn ranking(&self) -> HandRanking {
        let [c1, c2, c3, c4, c5] = {
            let mut cards = self.0.clone();
            cards.sort();
            cards
        };

        if c1 == c5 {
            HandRanking::FiveOfAKind
        } else if c1 == c4 || c2 == c5 {
            HandRanking::FourOfAKind
        } else if (c1 == c3 && c4 == c5) || (c3 == c5 && c1 == c2) {
            HandRanking::FullHouse
        } else if c1 == c3 || c2 == c4 || c3 == c5 {
            HandRanking::ThreeOfAKind
        } else if (c1 == c2 && (c3 == c4 || c4 == c5)) || (c2 == c3 && c4 == c5) {
            HandRanking::TwoPair
        } else if c1 == c2 || c2 == c3 || c3 == c4 || c4 == c5 {
            HandRanking::Pair
        } else {
            HandRanking::HighCard
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let rank_ord = self.ranking().cmp(&other.ranking());
        match rank_ord {
            std::cmp::Ordering::Equal => other.0.cmp(&self.0),
            _ => rank_ord,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_hand_ranking() {
        let hand = Hand([
            Card::Number(3),
            Card::Number(2),
            Card::Number(10),
            Card::Number(3),
            Card::King,
        ]);
        assert_eq!(hand.ranking(), HandRanking::Pair);

        let hand = Hand([
            Card::Number(3),
            Card::Number(2),
            Card::Number(10),
            Card::Queen,
            Card::King,
        ]);
        assert_eq!(hand.ranking(), HandRanking::HighCard);

        let hand = Hand([
            Card::Number(3),
            Card::Number(8),
            Card::Number(3),
            Card::Queen,
            Card::Queen,
        ]);
        assert_eq!(hand.ranking(), HandRanking::TwoPair);

        let hand = Hand([
            Card::Jack,
            Card::Number(10),
            Card::Jack,
            Card::Jack,
            Card::Number(10),
        ]);
        assert_eq!(hand.ranking(), HandRanking::FullHouse);

        let hand = Hand([
            Card::Number(10),
            Card::Number(5),
            Card::Number(5),
            Card::Jack,
            Card::Number(5),
        ]);
        assert_eq!(hand.ranking(), HandRanking::ThreeOfAKind);

        let hand = Hand([
            Card::Number(2),
            Card::Number(2),
            Card::Number(2),
            Card::Number(2),
            Card::Number(2),
        ]);
        assert_eq!(hand.ranking(), HandRanking::FiveOfAKind);

        let hand = Hand([
            Card::Number(8),
            Card::Number(8),
            Card::Number(8),
            Card::Number(3),
            Card::Number(8),
        ]);
        assert_eq!(hand.ranking(), HandRanking::FourOfAKind);
    }
}
