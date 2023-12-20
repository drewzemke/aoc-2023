use crate::{Element, Platform, PlatformRep, PlatformRows};

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

impl From<&str> for Platform {
    fn from(input: &str) -> Self {
        Self(PlatformRep::Rows(PlatformRows(
            input
                .lines()
                .map(|line| line.chars().map(Element::from).collect())
                .collect(),
        )))
    }
}
