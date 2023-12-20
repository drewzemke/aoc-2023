pub mod puzzle15a;
pub mod puzzle15b;

pub struct Hash;

impl Hash {
    pub fn hash(input: &str) -> u32 {
        let mut value = 0;
        for c in input.chars() {
            value += c as u32;
            value *= 17;
            value %= 256;
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(Hash::hash("HASH"), 52);
    }
}
