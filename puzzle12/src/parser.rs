use crate::{DamagedGroups, Schematic, SpringRow, SpringState};

impl From<&str> for SpringRow {
    fn from(input: &str) -> Self {
        let states = input
            .chars()
            .map(|c| match c {
                '.' => SpringState::Operational,
                '#' => SpringState::Damaged,
                '?' => SpringState::Unknown,
                _ => panic!("unrecognized character"),
            })
            .collect();
        SpringRow(states)
    }
}

impl From<&str> for DamagedGroups {
    fn from(input: &str) -> Self {
        let groups = input
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        DamagedGroups(groups)
    }
}

impl From<&str> for Schematic {
    fn from(input: &str) -> Self {
        let (spring_row, damaged_groups) = input.split_once(' ').unwrap();
        Schematic(spring_row.into(), damaged_groups.into())
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn parse_spring_row() {
        let input = "???.###";
        let springs = SpringRow::from(input);

        assert_eq!(
            springs,
            SpringRow(vec![
                SpringState::Unknown,
                SpringState::Unknown,
                SpringState::Unknown,
                SpringState::Operational,
                SpringState::Damaged,
                SpringState::Damaged,
                SpringState::Damaged,
            ]),
        );
    }

    #[test]
    fn parse_damaged_groups() {
        let input = "1,1,3";
        let springs = DamagedGroups::from(input);

        assert_eq!(springs, DamagedGroups(vec![1, 1, 3]),);
    }

    #[test]
    fn parse_schematic() {
        let input = "????.#...#... 4,1,1";
        let schematic = Schematic::from(input);

        assert_eq!(
            schematic,
            Schematic(
                SpringRow(vec![
                    SpringState::Unknown,
                    SpringState::Unknown,
                    SpringState::Unknown,
                    SpringState::Unknown,
                    SpringState::Operational,
                    SpringState::Damaged,
                    SpringState::Operational,
                    SpringState::Operational,
                    SpringState::Operational,
                    SpringState::Damaged,
                    SpringState::Operational,
                    SpringState::Operational,
                    SpringState::Operational,
                ]),
                DamagedGroups(vec![4, 1, 1])
            )
        )
    }
}
