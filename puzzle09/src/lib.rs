pub mod parser;
pub mod puzzle09a;
pub mod puzzle09b;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DiscreteFn(Vec<i32>);

impl DiscreteFn {
    pub fn difference(&self) -> DiscreteFn {
        let differences = self
            .0
            .windows(2)
            .map(|window| {
                let [v1, v2] = window else {
                    panic!("whoops?");
                };
                v2 - v1
            })
            .collect();
        DiscreteFn(differences)
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|x| *x == 0)
    }

    fn all_differences(&self) -> Vec<DiscreteFn> {
        let mut differences = vec![self.clone()];

        while !differences.last().unwrap().is_zero() {
            let diff = differences.last().unwrap().difference();
            differences.push(diff);
        }

        differences
    }

    pub fn extrapolate(&self) -> i32 {
        self.all_differences()
            .iter()
            .rfold(0, |value, func| value + func.0.last().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difference() {
        let func = DiscreteFn(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(func.difference(), DiscreteFn(vec![3, 3, 3, 3, 3,]))
    }

    #[test]
    fn test_is_zero() {
        let func1 = DiscreteFn(vec![0, 0, 0, 0, 0, 0]);
        assert!(func1.is_zero());

        let func2 = DiscreteFn(vec![0, 3, 6, 9, 12, 15]);
        assert!(!func2.is_zero());
    }

    #[test]
    fn test_all_differences() {
        let func = DiscreteFn(vec![0, 3, 6, 9, 12, 15]);

        let differences = func.all_differences();

        assert_eq!(
            differences,
            vec![
                DiscreteFn(vec![0, 3, 6, 9, 12, 15]),
                DiscreteFn(vec![3, 3, 3, 3, 3]),
                DiscreteFn(vec![0, 0, 0, 0]),
            ]
        )
    }

    #[test]
    fn test_extrapolate() {
        let func = DiscreteFn(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(func.extrapolate(), 18);

        let func = DiscreteFn(vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(func.extrapolate(), 28);

        let func = DiscreteFn(vec![10, 13, 16, 21, 30, 45]);
        assert_eq!(func.extrapolate(), 68);
    }
}
