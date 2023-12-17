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

/// A representation of a hand as a list of cards with multiplicity.
/// Cards with the same multiplicity are sorted in order
/// of _decreasing_ rank.
#[derive(Debug, PartialEq, Eq)]
pub struct HandRep(Vec<(Card, u32)>);

impl Hand {
    fn rep(&self) -> HandRep {
        let mut buckets: Vec<(Card, u32)> = vec![];

        for card in &self.0 {
            if let Some((_, count)) = buckets.iter_mut().find(|(other, _)| card == other) {
                *count += 1;
            } else {
                buckets.push((card.clone(), 1));
            }
        }

        buckets.sort_by(|(card1, count1), (card2, count2)| {
            if count1 == count2 {
                card2.cmp(card1)
            } else {
                count2.cmp(count1)
            }
        });

        HandRep(buckets)
    }
}

#[cfg(test)]
mod hand_rep_tests {
    use super::*;

    #[test]
    pub fn test_compute_hand_rep() {
        let hand = Hand([
            Card::Number(3),
            Card::Number(2),
            Card::Number(10),
            Card::Number(3),
            Card::King,
        ]);
        assert_eq!(
            hand.rep(),
            HandRep(vec![
                (Card::Number(3), 2),
                (Card::King, 1),
                (Card::Number(10), 1),
                (Card::Number(2), 1),
            ])
        );

        let hand = Hand([
            Card::Number(2),
            Card::Number(2),
            Card::Number(2),
            Card::Number(2),
            Card::Number(2),
        ]);
        assert_eq!(hand.rep(), HandRep(vec![(Card::Number(2), 5),]));

        let hand = Hand([
            Card::Number(4),
            Card::Number(2),
            Card::Number(2),
            Card::Ace,
            Card::Number(2),
        ]);
        assert_eq!(
            hand.rep(),
            HandRep(vec![
                (Card::Number(2), 3),
                (Card::Ace, 1),
                (Card::Number(4), 1),
            ])
        );
    }
}

impl HandRep {
    pub fn ranking(&self) -> HandRanking {
        match self.0.first().unwrap() {
            (_, 5) => HandRanking::FiveOfAKind,
            (_, 4) => HandRanking::FourOfAKind,
            (_, 3) => match self.0.get(1).unwrap() {
                (_, 2) => HandRanking::FullHouse,
                (_, 1) => HandRanking::ThreeOfAKind,
                _ => panic!("invalid hand rep"),
            },
            (_, 2) => match self.0.get(1).unwrap() {
                (_, 2) => HandRanking::TwoPair,
                (_, 1) => HandRanking::Pair,
                _ => panic!("invalid hand rep"),
            },
            (_, 1) => HandRanking::HighCard,
            _ => panic!("invalid hand rep"),
        }
    }
}

#[cfg(test)]
mod hand_rep_rank_tests {
    use super::*;

    #[test]
    pub fn test_hand_rep_ranking() {
        let hand = Hand([
            Card::Number(3),
            Card::Number(2),
            Card::Number(10),
            Card::Number(3),
            Card::King,
        ]);
        let rep = hand.rep();
        assert_eq!(rep.ranking(), HandRanking::Pair);

        let hand = Hand([
            Card::Number(3),
            Card::Number(2),
            Card::Number(10),
            Card::Queen,
            Card::King,
        ]);
        let rep = hand.rep();
        assert_eq!(rep.ranking(), HandRanking::HighCard);

        let hand = Hand([
            Card::Number(3),
            Card::Number(8),
            Card::Number(3),
            Card::Queen,
            Card::Queen,
        ]);
        let rep = hand.rep();
        assert_eq!(rep.ranking(), HandRanking::TwoPair);

        let hand = Hand([
            Card::Jack,
            Card::Number(10),
            Card::Jack,
            Card::Jack,
            Card::Number(10),
        ]);
        let rep = hand.rep();
        assert_eq!(rep.ranking(), HandRanking::FullHouse);

        let hand = Hand([
            Card::Number(10),
            Card::Number(5),
            Card::Number(5),
            Card::Jack,
            Card::Number(5),
        ]);
        let rep = hand.rep();
        assert_eq!(rep.ranking(), HandRanking::ThreeOfAKind);

        let hand = Hand([
            Card::Number(2),
            Card::Number(2),
            Card::Number(2),
            Card::Number(2),
            Card::Number(2),
        ]);
        let rep = hand.rep();
        assert_eq!(rep.ranking(), HandRanking::FiveOfAKind);

        let hand = Hand([
            Card::Number(8),
            Card::Number(8),
            Card::Number(8),
            Card::Number(3),
            Card::Number(8),
        ]);
        let rep = hand.rep();
        assert_eq!(rep.ranking(), HandRanking::FourOfAKind);
    }
}
