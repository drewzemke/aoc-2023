use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{map_res, value},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Game {
    id: u32,
    set: Vec<ColorSet>,
}

impl Game {
    fn color_maxes(&self) -> ColorSet {
        self.set.iter().fold(ColorSet(0, 0, 0), |set1, set2| {
            ColorSet(set1.0.max(set2.0), set1.1.max(set2.1), set1.2.max(set2.2))
        })
    }
}

#[derive(Debug, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy)]
struct ColorSet(u32, u32, u32);

impl std::ops::Add<ColorSet> for ColorSet {
    type Output = Self;

    fn add(self, rhs: ColorSet) -> Self::Output {
        ColorSet(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

// puzzle 1: constraints
const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut id_sum: u32 = 0;
    let mut power_sum: u32 = 0;

    for line in reader.lines().map(Result::unwrap) {
        let (_, game) = parse_game(&line).unwrap();

        // puzzle 1: compute maxes to find which games were possible given the constraints
        // puzzle 2: compute maxes to find the minimal number of cubes necessary to make each game possible
        let color_maxes = game.color_maxes();

        if color_maxes.0 <= MAX_RED && color_maxes.1 <= MAX_GREEN && color_maxes.2 <= MAX_BLUE {
            id_sum += game.id;
        }

        // puzzle 2: the "power" of a set of cubes is the product of the number of each cube
        let game_power = color_maxes.0 * color_maxes.1 * color_maxes.2;
        power_sum += game_power;
    }

    println!("puzzle 1 (sum of ids of possible games): {id_sum}");
    println!("puzzle 2 (sum of powers of minimal sets): {power_sum}");
}

fn parse_game(input: &str) -> IResult<&str, Game> {
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
