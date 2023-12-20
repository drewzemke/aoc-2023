use crate::InitializationStep;

impl<'a> From<&'a str> for InitializationStep<'a> {
    fn from(input: &'a str) -> Self {
        if let Some((label, focal_length)) = input.split_once('=') {
            // label followed by '=' followed by a number
            let focal_length = focal_length.parse::<u32>().unwrap();
            Self::Set {
                label,
                value: focal_length,
            }
        } else {
            // label followed by a hyphen
            let label = input.split('-').next().unwrap();
            Self::Unset { label }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_set_step() {
        let input = "ddtp=6";
        let step = InitializationStep::from(input);

        assert_eq!(
            step,
            InitializationStep::Set {
                label: "ddtp",
                value: 6
            }
        )
    }

    #[test]
    fn test_parse_unset_step() {
        let input = "dcb-";
        let step = InitializationStep::from(input);

        assert_eq!(step, InitializationStep::Unset { label: "dcb" })
    }
}
