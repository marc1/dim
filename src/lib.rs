#[cfg(test)]
mod tests {
    use crate::vec_3::Unit;
    use crate::vec_3::Vec3;

    #[test]
    fn unit_magnitude_1() {
        let _u = Unit::I(1.5923);

        assert_eq!(_u.mag(), 1.5923);
    }

    #[test]
    fn unit_magnitude_2() {
        let _u = Unit::I(3.52);

        assert_eq!(_u.mag(), 3.52);
    }

    #[test]
    fn unit_magnitude_3() {
        let _u = Unit::I(99.2452);

        assert_eq!(_u.mag(), 99.2452);
    }

    #[test]
    fn unit_add_1() {
        let _u = Unit::I(1.5);
        let _v = Unit::I(5.5);

        assert_eq!(_u + _v, Unit::I(7.0));
    }

    #[test]
    fn unit_add_2() {
        let _u = Unit::I(1.5);
        let _v = Unit::J(5.5);

        assert_eq!(_u + _v, _u);
    }

    #[test]
    fn unit_add_3() {
        let _u = Unit::K(9.25);
        let _v = Unit::K(11.75);

        assert_eq!(_u + _v, Unit::K(21.0));
    }

    #[test]
    fn unit_sub_1() {
        let _u = Unit::K(9.25);
        let _v = Unit::K(11.75);

        assert_eq!(_u - _v, Unit::K(-2.5));
    }

    #[test]
    fn unit_sub_2() {
        let _u = Unit::I(10.0);
        let _v = Unit::I(2.5);

        assert_eq!(_u - _v, Unit::I(7.5));
    }

    #[test]
    fn unit_sub_3() {
        let _u = Unit::I(10.0);
        let _v = Unit::K(5.0);

        assert_eq!(_u - _v, _u);
    }

    #[test]
    fn unit_mul_1() {
        let _u = Unit::I(3.0);
        let _v = Unit::J(9.5);

        assert_eq!(_u * _v, Unit::K(28.5));
    }

    #[test]
    fn unit_mul_2() {
        let _u = Unit::K(7.0);
        let _v = Unit::J(3.0);

        assert_eq!(_u * _v, Unit::I(-21.0));
    }

    #[test]
    fn unit_mul_3() {
        let _u = Unit::I(3.0);
        let _v = Unit::I(9.5);

        assert_eq!(_u * _v, Unit::I(0.0));
    }

    #[test]
    fn vec_add_1() {
        let _u = Vec3::create(1.0, 2.0, 3.0);
        let _v = Vec3::create(4.0, 5.0, 6.0);

        let r = Vec3::create(5.0, 7.0, 9.0);

        assert_eq!(_u + _v, r);
    }

    #[test]
    fn vec_add_2() {
        let _u = Vec3::create(-1.0, 50.0, -27.5);
        let _v = Vec3::create(77.0, -423.3, 99.21);

        let r = Vec3::create(76.0, -373.3, 71.71);

        assert_eq!(_u + _v, r);
    }

    #[test]
    fn vec_sub_1() {
        let _u = Vec3::create(1.0, 2.0, 3.0);
        let _v = Vec3::create(4.0, 5.0, 6.0);

        let r = Vec3::create(-3.0, -3.0, -3.0);

        assert_eq!(_u - _v, r);
    }

    #[test]
    fn vec_sub_2() {
        let _u = Vec3::create(10.0, 10.0, 10.0);
        let _v = Vec3::create(10.0, 5.0, 0.0);

        let r = Vec3::create(0.0, 5.0, 10.0);

        assert_eq!(_u - _v, r);
    }

    #[test]
    fn vec_mul_1() {
        let _u = Vec3::create(1.0, 2.0, 3.0);
        let _v = Vec3::create(4.0, 5.0, 6.0);

        let r = Vec3::create(-3.0, 6.0, -3.0);

        assert_eq!(_u * _v, r);
    }

    #[test]
    fn vec_mul_2() {
        let _u = Vec3::create(1.0, 0.0, 0.0);
        let _v = Vec3::create(0.0, 1.0, 0.0);

        let r = Vec3::create(0.0, 0.0, 1.0);

        assert_eq!(_u * _v, r);
    }
}

pub mod vec_3 {
    use std::fmt;
    use std::ops;

    pub const UNIT_I: Vec3 = Vec3 {
        x: Unit::I(1.0),
        y: Unit::J(0.0),
        z: Unit::K(0.0),
    };
    pub const UNIT_J: Vec3 = Vec3 {
        x: Unit::I(0.0),
        y: Unit::J(1.0),
        z: Unit::K(0.0),
    };
    pub const UNIT_K: Vec3 = Vec3 {
        x: Unit::I(0.0),
        y: Unit::J(0.0),
        z: Unit::K(1.0),
    };

    #[derive(Copy, Clone)]
    pub enum Unit {
        I(f64),
        J(f64),
        K(f64),
    }

    impl Unit {
        pub fn mag(self) -> f64 {
            if let Unit::I(_v) | Unit::J(_v) | Unit::K(_v) = self {
                return _v;
            }

            0.0
        }
    }

    impl fmt::Display for Unit {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Unit::I(_v) => write!(f, "{:.3}i", self.mag()),
                Unit::J(_v) => write!(f, "{:.3}j", self.mag()),
                Unit::K(_v) => write!(f, "{:.3}k", self.mag()),
            }
        }
    }

    impl ops::Neg for Unit {
        type Output = Unit;

        fn neg(self) -> Self {
            match self {
                Unit::I(_v) => Unit::I(-_v),
                Unit::J(_v) => Unit::J(-_v),
                Unit::K(_v) => Unit::K(-_v),
            }
        }
    }

    impl ops::Add for Unit {
        type Output = Unit;

        fn add(self, other: Self) -> Self {
            match self {
                Unit::I(_v) => match other {
                    Unit::I(_w) => Unit::I(_v + _w),
                    _ => self,
                },
                Unit::J(_v) => match other {
                    Unit::J(_w) => Unit::J(_v + _w),
                    _ => self,
                },
                Unit::K(_v) => match other {
                    Unit::K(_w) => Unit::K(_v + _w),
                    _ => self,
                },
            }
        }
    }

    impl ops::Sub for Unit {
        type Output = Unit;

        fn sub(self, other: Self) -> Self {
            self + (-other)
        }
    }

    impl ops::Mul for Unit {
        type Output = Unit;

        fn mul(self, other: Self) -> Self {
            match self {
                Unit::I(_v) => match other {
                    Unit::I(_w) => Unit::I(0f64),
                    Unit::J(_w) => Unit::K(_v * _w),
                    Unit::K(_w) => Unit::J(-(_v * _w)),
                },
                Unit::J(_v) => match other {
                    Unit::I(_w) => Unit::K(-(_v * _w)),
                    Unit::J(_w) => Unit::J(0f64),
                    Unit::K(_w) => Unit::I(_v * _w),
                },
                Unit::K(_v) => match other {
                    Unit::I(_w) => Unit::K(-(_v * _w)),
                    Unit::J(_w) => Unit::I(-(_v * _w)),
                    Unit::K(_w) => Unit::K(0f64),
                },
            }
        }
    }

    impl PartialEq for Unit {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Unit::I(_v) => match other {
                    Unit::I(_w) => _v == _w,
                    _ => false,
                },
                Unit::J(_v) => match other {
                    Unit::J(_w) => _v == _w,
                    _ => false,
                },
                Unit::K(_v) => match other {
                    Unit::K(_w) => _v == _w,
                    _ => false,
                },
            }
        }
    }
    pub struct Vec3 {
        pub x: Unit,
        pub y: Unit,
        pub z: Unit,
    }

    impl Vec3 {
        pub fn create(x: f64, y: f64, z: f64) -> Self {
            Vec3 {
                x: Unit::I(x),
                y: Unit::J(y),
                z: Unit::K(z),
            }
        }

        pub fn dot(&self, other: &Self) -> f64 {
            self.x.mag() * other.x.mag()
                + self.y.mag() * other.y.mag()
                + self.z.mag() * other.z.mag()
        }

        pub fn mag(&self) -> f64 {
            self.dot(self).sqrt()
        }

        pub fn get_unit(&self, u: &Unit) -> Unit {
            match u {
                Unit::I(_) => self.x,
                Unit::J(_) => self.y,
                Unit::K(_) => self.z,
            }
        }

        pub fn angle_r(&self, other: &Vec3) -> f64 {
            (self.dot(&other) / (self.mag() * other.mag())).acos()
        }

        pub fn angle_d(&self, other: &Vec3) -> f64 {
            self.angle_r(other).to_degrees()
        }
    }

    impl fmt::Display for Vec3 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} {} {}", self.x, self.y, self.z)
        }
    }

    impl PartialEq for Vec3 {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y && self.z == other.z
        }
    }

    impl ops::Neg for Vec3 {
        type Output = Vec3;

        fn neg(self) -> Vec3 {
            Vec3 {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }
    }

    impl ops::Add for Vec3 {
        type Output = Vec3;

        fn add(self, other: Self) -> Self {
            Vec3 {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    impl ops::Sub for Vec3 {
        type Output = Vec3;

        fn sub(self, other: Self) -> Self {
            self + (-other)
        }
    }

    impl ops::Mul for Vec3 {
        type Output = Vec3;

        fn mul(self, other: Self) -> Self {
            Vec3 {
                x: Unit::I(self.y.mag() * other.z.mag() - self.z.mag() * other.y.mag()),
                y: Unit::J(self.z.mag() * other.x.mag() - self.x.mag() * other.z.mag()),
                z: Unit::K(self.x.mag() * other.y.mag() - self.y.mag() * other.x.mag()),
            }
        }
    }
}
