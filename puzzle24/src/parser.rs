use crate::{Path, Vector};

impl From<&str> for Vector {
    fn from(input: &str) -> Self {
        let mut values = input.split(',').map(|s| s.trim().parse::<i64>().unwrap());
        Self(
            values.next().unwrap() as f64,
            values.next().unwrap() as f64,
            values.next().unwrap() as f64,
        )
    }
}

impl From<&str> for Path {
    fn from(input: &str) -> Self {
        let (start, velocity) = input.split_once('@').unwrap();
        Self {
            start: start.into(),
            velocity: velocity.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vector() {
        let input = "1, -2, 3";

        let vector = Vector::from(input);
        assert_eq!(vector, Vector(1.0, -2.0, 3.0));
    }

    #[test]
    fn test_parse_path() {
        let input = "19, 13, 30 @ -2,  1, -2";

        let path = Path::from(input);
        assert_eq!(
            path,
            Path {
                start: Vector(19.0, 13.0, 30.0),
                velocity: Vector(-2.0, 1.0, -2.0),
            }
        );
    }
}
