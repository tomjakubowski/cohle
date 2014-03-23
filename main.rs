#[ crate_type = "bin" ];

#[cfg(test)]
mod test_helpers {
    pub fn assert_float_eq<T: Float>(a: T, b: T) {
        assert!((a - b).abs() < Float::epsilon())
    }
}

mod point3d {
    use std::f64::{pow, sqrt};

    #[deriving(Show,Eq)]
    pub struct Point(f64, f64, f64);
    pub static origin: Point = Point(0f64, 0f64, 0f64);

    pub fn dist(p1: Point, p2: Point) -> f64 {
        match (p1, p2) {
            (Point(x1, y1, z1), Point(x2, y2, z2)) => {
                sqrt(pow(x2 - x1, 2.0) + pow(y2 - y1, 2.0) + pow(z2 - z1, 2.0))
            }
        }
    }

    #[test]
    fn test_dist() {
        use test_helpers::assert_float_eq;

        let (p1, p2) = (Point(0f64, 1f64, 1f64), Point(1f64, 1f64, 0f64));
        assert_float_eq(dist(origin, p1), sqrt(2.0));
        assert_float_eq(dist(origin, p2), sqrt(2.0));
    }
}

pub fn main() {
    use point3d::{Point, dist};

    let p = Point(1.123f64, 1.542f64, 0f64);
    println!("the uhh distance between two points {} and {} \
              is {}", p, point3d::origin, dist(p, point3d::origin));
}
