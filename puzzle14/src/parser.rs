use crate::{Element, PlatformRows};

impl From<char> for Element {
    fn from(c: char) -> Self {
        match c {
            'O' => Self::RollingRock,
            '#' => Self::StationaryRock,
            '.' => Element::Nothing,
            _ => panic!("unreconized character: {c}"),
        }
    }
}

impl From<&str> for PlatformRows {
    fn from(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| line.chars().map(Element::from).collect())
                .collect(),
        )
    }
}
