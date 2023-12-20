pub mod parse;
pub mod puzzle15a;
pub mod puzzle15b;

pub struct Hash;

impl Hash {
    pub fn hash(input: &str) -> usize {
        let mut value = 0;
        for c in input.chars() {
            value += c as usize;
            value *= 17;
            value %= 256;
        }

        value
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InitializationStep<'a> {
    Set { label: &'a str, value: u32 },
    Unset { label: &'a str },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Lens<'a> {
    label: &'a str,
    value: u32,
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct LensBox<'a>(Vec<Lens<'a>>);

#[derive(Debug, PartialEq, Eq)]
pub struct LensArray<'a>(Vec<LensBox<'a>>);

impl<'a> Default for LensArray<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> LensArray<'a> {
    pub fn new() -> Self {
        Self((0..256).map(|_| LensBox::default()).collect())
    }

    pub fn add(&mut self, lens: Lens<'a>) {
        let box_idx = Hash::hash(lens.label);
        let lens_box = &mut self.0[box_idx];

        if let Some(other) = lens_box
            .0
            .iter_mut()
            .find(|other_lens| other_lens.label == lens.label)
        {
            *other = lens;
        } else {
            lens_box.0.push(lens)
        }
    }

    pub fn remove(&mut self, label: &str) {
        let box_idx = Hash::hash(label);
        let lens_box = &mut self.0[box_idx];

        if let Some(index) = lens_box.0.iter().position(|lens| lens.label == label) {
            lens_box.0.remove(index);
        }
    }

    pub fn focusing_power(&self) -> u32 {
        self.0
            .iter()
            .enumerate()
            .map(|(box_idx, lens_box)| {
                (box_idx as u32 + 1)
                    * lens_box
                        .0
                        .iter()
                        .enumerate()
                        .map(|(lens_idx, Lens { value, .. })| (lens_idx as u32 + 1) * *value)
                        .sum::<u32>()
            })
            .sum::<u32>()
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
