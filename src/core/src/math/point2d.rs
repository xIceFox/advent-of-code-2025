use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point2D {
    pub x: i64,
    pub y: i64,
}

impl Point2D {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y}
    }

    pub fn parse(str: &str) -> Result<Point2D, String> {
        let split: Vec<&str> = str.split(',').collect();
        if let [x, y] = split[..] {
            Ok(Self {
                x: x.parse().map_err(|_| format!("Invalid x: {}", x))?,
                y: y.parse().map_err(|_| format!("Invalid y: {}", y))?,
            })
        } else {
            Err(format!("Wrong input format: {}, expected x,y,z", str))
        }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        let sum = (self - other).pow(2).sum();
        (sum as f64).sqrt()
    }

    pub fn pow(&self, exponent: u32) -> Self {
        Self {
            x: self.x.pow(exponent),
            y: self.y.pow(exponent),
        }
    }

    pub fn sum(&self) -> i64 {
        self.x + self.y
    }
}

impl Sub for Point2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub for &Point2D {
    type Output = Point2D;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for &Point2D {
    type Output = Point2D;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{x: {}, y: {}}}", self.x, self.y)
    }
}