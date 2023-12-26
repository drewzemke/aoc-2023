pub mod parser;
pub mod puzzle24a;
pub mod puzzle24b;

#[derive(Debug)]
pub struct Vector(f64, f64, f64);

impl From<(i32, i32, i32)> for Vector {
    fn from(tuple: (i32, i32, i32)) -> Self {
        Self(tuple.0 as f64, tuple.1 as f64, tuple.2 as f64)
    }
}

const TOLERANCE: f64 = 0.000001;

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < TOLERANCE
            && (self.1 - other.1).abs() < TOLERANCE
            && (self.2 - other.2).abs() < TOLERANCE
    }
}

impl Vector {
    pub fn has_xy_in(&self, min: f64, max: f64) -> bool {
        self.0 >= min && self.0 <= max && self.1 >= min && self.1 <= max
    }
}

#[derive(Debug, PartialEq)]
pub struct Path {
    start: Vector,
    velocity: Vector,
}

impl Path {
    pub fn at(&self, time: f64) -> Vector {
        Vector(
            self.start.0 + time * self.velocity.0,
            self.start.1 + time * self.velocity.1,
            self.start.2 + time * self.velocity.2,
        )
    }

    // So here's how you compute intersections. Consider these paths:
    //   p1 = a + tb
    //   p2 = c + td
    // The paths "intersect" if
    //   a + tb = c + sd
    // for some scalars t and s.
    // Let's turn that into a matrix equation! It's the same as:
    //   tb - sd = c - a
    // or, if you don't mind some iffy matrix notation:
    //   | b.x d.x | |  t |   | c.x - a.x |
    //   | b.y d.y | | -s | = | c.y - a.y |
    //   | b.z d.z |          | c.z - a.z |
    // There's no solution if the deteminant D = (b.x)(d.y) - (d.x)(b.y) is zero --
    // This is an over-constrained system, and since (by assumption in this puzzle)
    // none of the paths are parallel to the z-axis, so we can restrict to
    // the x- and y- coordinates to find t and s.
    //   |  t | = (1/D) | d.y -d.x | | c.x - a.x |
    //   | - s|         | -b.y b.x | | c.y - a.y |
    // Now just expand that stuff to find t and s:
    //   t = (1/D) ( (d.y)(c.x - a.x) - (d.x)(c.y - b.y) )
    //   s = (1/D) ( (b.y)(c.x - a.x) - (b.x)(c.y - b.y) )
    // Tada! Plug t into the equation for the path p1 to find the intersection location, if desired.
    pub fn intersection_times(&self, other: &Path) -> Option<(f64, f64)> {
        let det = self.velocity.0 * other.velocity.1 - other.velocity.0 * self.velocity.1;
        if det == 0.0 {
            None
        } else {
            let t = (1.0 / det)
                * ((other.velocity.1) * (other.start.0 - self.start.0)
                    - (other.velocity.0) * (other.start.1 - self.start.1));
            let s = (1.0 / det)
                * ((self.velocity.1) * (other.start.0 - self.start.0)
                    - (self.velocity.0) * (other.start.1 - self.start.1));
            Some((t, s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_intersection() {
        let path1 = Path {
            start: (19, 13, 30).into(),
            velocity: (-2, 1, -2).into(),
        };
        let path2 = Path {
            start: (12, 31, 28).into(),
            velocity: (-1, -2, -1).into(),
        };

        let (t1, t2) = path1.intersection_times(&path2).unwrap();

        let p1 = path1.at(t1);
        let p2 = path2.at(t2);

        assert!((p1.0 - p2.0).abs() < TOLERANCE);
        assert!((p1.1 - p2.1).abs() < TOLERANCE);
    }

    #[test]
    fn test_path_non_intersection() {
        let path1 = Path {
            start: (19, 13, 30).into(),
            velocity: (-2, 1, -2).into(),
        };
        let path2 = Path {
            start: (12, 31, 28).into(),
            velocity: (4, -2, 4).into(),
        };

        let intersection = path1.intersection_times(&path2);
        assert!(intersection.is_none());
    }
}
