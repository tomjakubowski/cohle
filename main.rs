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

mod vector3d {
    #[deriving(Show,Eq)]
    pub struct Vector(f64, f64, f64);

    #[allow(dead_code)]
    pub static unit_x: Vector = Vector(1.0, 0.0, 0.0);
    #[allow(dead_code)]
    pub static unit_y: Vector = Vector(0.0, 1.0, 0.0);
    #[allow(dead_code)]
    pub static unit_z: Vector = Vector(0.0, 0.0, 1.0);
    #[allow(dead_code)]
    pub static zero: Vector = Vector(0.0, 0.0, 0.0);

    impl Neg<Vector> for Vector {
        fn neg(&self) -> Vector {
            let Vector(x, y, z) = *self;
            Vector(-x, -y, -z)
        }
    }

    impl Add<Vector, Vector> for Vector {
        fn add(&self, rhs: &Vector) -> Vector {
            let Vector(x1, y1, z1) = *self;
            let Vector(x2, y2, z2) = *rhs;

            Vector(x1 + x2, y1 + y2, z1 + z2)
        }
    }

    impl Sub<Vector, Vector> for Vector {
        fn sub(&self, rhs: &Vector) -> Vector {
            self + -rhs
        }
    }

    #[test]
    fn test_neg_vectors() {
        assert_eq!(-unit_x, Vector(-1.0, 0.0, 0.0))
    }

    #[test]
    fn test_add_vectors() {
        assert_eq!(unit_x + unit_y + unit_z, Vector(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_sub_vectors() {
        assert_eq!(unit_x - unit_y, Vector(1.0, -1.0, 0.0));
    }
}

pub fn main() {
    use point3d::{Point, dist};

    let p = Point(1.123f64, 1.542f64, 0f64);
    println!("the uhh distance between two points {} and {} \
              is {}", p, point3d::origin, dist(p, point3d::origin));
}
