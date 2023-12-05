use common::puzzle::PuzzlePart;

pub struct Puzzle04a {}

impl PuzzlePart for Puzzle04a {
    fn description() -> &'static str {
        "Sum the scores of scratchy cards."
    }

    fn solve(input: &str) -> String {
        input
            .lines()
            .map(ScratchCard::parse_from_str)
            .map(|card| card.score())
            .sum::<u32>()
            .to_string()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ScratchCard {
    winning_nums: Vec<u32>,
    player_nums: Vec<u32>,
}

impl ScratchCard {
    fn parse_from_str(input: &str) -> Self {
        let (_, input) = input.split_once(": ").unwrap();
        let (winning_num_str, player_num_str) = input.split_once(" | ").unwrap();

        let winning_nums: Vec<_> = winning_num_str
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let player_nums: Vec<_> = player_num_str
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        Self {
            winning_nums,
            player_nums,
        }
    }

    fn matches(&self) -> u32 {
        self.player_nums
            .iter()
            .filter(|n| self.winning_nums.contains(n))
            .count() as u32
    }

    fn score(&self) -> u32 {
        match self.matches() {
            0 => 0,
            n => 2u32.pow(n - 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_smol_card_from_str() {
        let input = "Card 11: 1 2 | 3 4";
        let card = ScratchCard::parse_from_str(input);

        assert_eq!(
            card,
            ScratchCard {
                winning_nums: vec![1, 2],
                player_nums: vec![3, 4],
            }
        )
    }

    #[test]
    fn test_parse_card_from_str() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = ScratchCard::parse_from_str(input);

        assert_eq!(
            card,
            ScratchCard {
                winning_nums: vec![41, 48, 83, 86, 17,],
                player_nums: vec![83, 86, 6, 31, 17, 9, 48, 53],
            }
        )
    }

    #[test]
    fn test_compute_matches() {
        let card = ScratchCard {
            winning_nums: vec![41, 48, 83, 86, 17],
            player_nums: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        assert_eq!(card.matches(), 4)
    }

    #[test]
    fn test_compute_score() {
        let card = ScratchCard {
            winning_nums: vec![41, 48, 83, 86, 17],
            player_nums: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        assert_eq!(card.score(), 8)
    }

    #[test]
    fn test_compute_score_no_matches() {
        let card = ScratchCard {
            winning_nums: vec![1, 2],
            player_nums: vec![3, 4],
        };

        assert_eq!(card.score(), 0)
    }
}
