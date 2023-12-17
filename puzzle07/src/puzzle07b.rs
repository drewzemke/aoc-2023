use std::cmp::Ordering;

use common::puzzle::PuzzlePart;

use crate::{parser::parse_line, Hand, HandRep};

pub struct Puzzle07b {}

impl PuzzlePart for Puzzle07b {
    fn description() -> &'static str {
        "Sum the 'ranks' of a set of poker hands, treating Jacks as Jokers."
    }

    fn solve(input: &str) -> String {
        let mut hands: Vec<(Hand, HandRep, u32)> = input
            .lines()
            .map(parse_line)
            .map(|(hand, num)| {
                let rep = hand.rep();
                (hand, rep, num)
            })
            .collect();

        hands.sort_by(|(hand1, rep1, _), (hand2, rep2, _)| {
            let rank_ord = rep1.ranking_with_jokers().cmp(&rep2.ranking_with_jokers());

            // if the hands have the same rank, compare them card-by-card, treating Jacks as the lowest card
            // NOTE: it's possible that I'm supposed to swap `hand1` and `hand2` below
            match rank_ord {
                Ordering::Equal => hand1
                    .0
                    .iter()
                    .zip(&hand2.0)
                    .find_map(|(card1, card2)| {
                        let cmp = card1.cmp_with_jokers(card2);
                        if cmp != Ordering::Equal {
                            Some(cmp)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(Ordering::Equal),
                _ => rank_ord,
            }
        });

        hands
            .iter()
            .enumerate()
            .map(|(rank, (_, _, num))| (rank as u32 + 1) * *num)
            .sum::<u32>()
            .to_string()
    }
}
