use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

pub struct Point3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3D {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn parse(str: &str) -> Result<Point3D, String> {
        let split: Vec<&str> = str.split(',').collect();
        if let [x, y, z] = split[..] {
            Ok(Self {
                x: x.parse().map_err(|_| format!("Invalid x: {}", x))?,
                y: y.parse().map_err(|_| format!("Invalid y: {}", y))?,
                z: z.parse().map_err(|_| format!("Invalid z: {}", z))?,
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
            z: self.z.pow(exponent),
        }
    }

    pub fn sum(&self) -> i64 {
        self.x + self.y + self.z
    }
}

impl Sub for Point3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for &Point3D {
    type Output = Point3D;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Point3D {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add for &Point3D {
    type Output = Point3D;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{x: {}, y: {}, z: {}}}", self.x, self.y, self.z)
    }
}

impl PartialEq<Self> for Point3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Point3D{

}
