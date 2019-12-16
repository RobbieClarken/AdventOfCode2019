const TWO_PI: f64 = 2.0 * std::f64::consts::PI;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point(i32, i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    pub fn get_to(self, other: Self) -> (Self, i32) {
        let step = other - self;
        let common_divisor = gcd(step.0, step.1);
        let dir = step / common_divisor;
        (dir, common_divisor)
    }

    pub fn squared_distance_to(self, other: Self) -> i32 {
        (other.0 - self.0).pow(2) + (other.1 - self.1).pow(2)
    }

    /// Calculates angle θ as illustrated:
    ///              θ = 0
    ///               -y
    ///               ↑
    /// θ = 3π/4 -x ←  → +x θ = π / 2
    ///               ↓
    ///               +x
    ///             θ = π
    pub fn angle(self) -> f64 {
        ((self.0 as f64).atan2(-self.1 as f64) + TWO_PI) % TWO_PI
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl std::ops::Div<i32> for Point {
    type Output = Self;
    fn div(self, denominator: i32) -> Self {
        Self(self.0 / denominator, self.1 / denominator)
    }
}

impl std::ops::Mul<i32> for Point {
    type Output = Self;
    fn mul(self, factor: i32) -> Self {
        Self(self.0 * factor, self.1 * factor)
    }
}

fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x.abs()
}

#[cfg(test)]
mod test_point {
    use super::*;

    #[test]
    fn calculates_angle() {
        assert_eq!(Point::new(0, -1).angle(), 0.0);
        assert_eq!(Point::new(1, 0).angle(), std::f64::consts::FRAC_PI_2);
        assert_eq!(Point::new(0, 1).angle(), std::f64::consts::PI);
        assert_eq!(Point::new(-1, 0).angle(), 3.0 * std::f64::consts::FRAC_PI_2);
    }
}
