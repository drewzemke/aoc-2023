use crate::{Card, Hand};
fn parse_card(ch: char) -> Card {
    if let Some(digit) = ch.to_digit(10) {
        Card::Number(digit)
    } else {
        match ch {
            'T' => Card::Number(10),
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Unrecognized card character"),
        }
    }
}

fn parse_hand(input: &str) -> Hand {
    let mut cards = input.chars().map(parse_card);
    Hand([
        cards.next().unwrap(),
        cards.next().unwrap(),
        cards.next().unwrap(),
        cards.next().unwrap(),
        cards.next().unwrap(),
    ])
}

pub fn parse_line(input: &str) -> (Hand, u32) {
    let (hand_str, num_str) = input.split_once(' ').unwrap();
    (parse_hand(hand_str), num_str.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hand() {
        assert_eq!(
            parse_hand("32T3K"),
            Hand([
                Card::Number(3),
                Card::Number(2),
                Card::Number(10),
                Card::Number(3),
                Card::King,
            ])
        );
        assert_eq!(
            parse_hand("KTJJT"),
            Hand([
                Card::King,
                Card::Number(10),
                Card::Jack,
                Card::Jack,
                Card::Number(10),
            ])
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("T55J5 684"),
            (
                Hand([
                    Card::Number(10),
                    Card::Number(5),
                    Card::Number(5),
                    Card::Jack,
                    Card::Number(5),
                ]),
                684
            )
        )
    }
}
