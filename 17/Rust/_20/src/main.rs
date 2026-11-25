use std::{
    fs,
    ops::{Add, Sub},
};

struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

impl TryFrom<&str> for Vec3 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let trimmed = value.replace("<", "").replace(">", "");
        let parts: Vec<&str> = trimmed.split(",").collect();
        if parts.len() != 3 {
            return Err("Vec3 must have 3 elements");
        }
        let Ok(x) = parts[0].parse() else {
            return Err("Bad X value");
        };
        let Ok(y) = parts[1].parse() else {
            return Err("Bad Y value");
        };
        let Ok(z) = parts[2].parse() else {
            return Err("Bad Z value");
        };
        Ok(Self { x, y, z })
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

struct Particle {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
}

impl TryFrom<&str> for Particle {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {}
}

fn read_input(path: &str) -> Vec<Particle> {
    let s = fs::read_to_string(path).expect("Input file must exist");
    s.lines().into_iter().map(|line| line.into()).collect()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {}
}
