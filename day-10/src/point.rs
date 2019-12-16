#[derive(Debug, PartialEq, Clone, Copy)]
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
