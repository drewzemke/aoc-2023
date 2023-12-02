pub mod game {
    #[derive(Debug)]
    pub struct Game {
        pub id: u32,
        pub set: Vec<ColorSet>,
    }

    impl Game {
        pub fn color_maxes(&self) -> ColorSet {
            self.set.iter().fold(ColorSet(0, 0, 0), |set1, set2| {
                ColorSet(set1.0.max(set2.0), set1.1.max(set2.1), set1.2.max(set2.2))
            })
        }
    }

    #[derive(Debug, Clone)]
    pub enum Color {
        Red,
        Green,
        Blue,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct ColorSet(pub u32, pub u32, pub u32);

    impl std::ops::Add<ColorSet> for ColorSet {
        type Output = Self;

        fn add(self, rhs: ColorSet) -> Self::Output {
            ColorSet(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
        }
    }
}

pub mod parser {
    use crate::game::{Color, ColorSet, Game};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, space1},
        combinator::{map_res, value},
        multi::separated_list1,
        sequence::tuple,
        IResult,
    };

    pub fn parse_game(input: &str) -> IResult<&str, Game> {
        let (rem, (_, game_id, _)) = tuple((tag("Game "), parse_int, tag(": ")))(input)?;
        let (rem, color_sets) = separated_list1(tag("; "), parse_color_set)(rem)?;

        let game = Game {
            id: game_id,
            set: color_sets,
        };
        Ok((rem, game))
    }

    fn parse_color_set(input: &str) -> IResult<&str, ColorSet> {
        let (rem, color_amts) = separated_list1(tag(", "), parse_color_amount)(input)?;
        let color_set = color_amts
            .into_iter()
            .fold(ColorSet(0, 0, 0), |color_set, color_amt| {
                color_set
                    + match color_amt {
                        (Color::Red, amt) => ColorSet(amt, 0, 0),
                        (Color::Green, amt) => ColorSet(0, amt, 0),
                        (Color::Blue, amt) => ColorSet(0, 0, amt),
                    }
            });

        Ok((rem, color_set))
    }

    fn parse_color_amount(input: &str) -> IResult<&str, (Color, u32)> {
        let (rem, (amount, _, color)) = tuple((parse_int, space1, parse_color))(input)?;
        Ok((rem, (color, amount)))
    }

    fn parse_color(input: &str) -> IResult<&str, Color> {
        alt((
            value(Color::Red, tag("red")),
            value(Color::Green, tag("green")),
            value(Color::Blue, tag("blue")),
        ))(input)
    }

    fn parse_int(input: &str) -> IResult<&str, u32> {
        map_res(digit1, |s: &str| s.parse::<u32>())(input)
    }
}
