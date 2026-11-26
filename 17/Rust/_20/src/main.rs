use std::{
    collections::HashMap,
    fs,
    ops::{Add, Sub},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Vec3 {
    fn manhattan(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
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

#[derive(Clone, Copy)]
struct Particle {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
}

impl Particle {
    fn step(&mut self) {
        self.velocity = self.velocity + self.acceleration;
        self.position = self.position + self.velocity;
    }
}

impl TryFrom<&str> for Particle {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<String> = value
            .split(", ")
            .map(|part| part.replace("p=", "").replace("v=", "").replace("a=", ""))
            .collect();
        if parts.len() != 3 {
            return Err("Particle must have position, velocity, and acceleration");
        }

        let Ok(position) = parts[0].as_str().try_into() else {
            return Err("Bad Position");
        };
        let Ok(velocity) = parts[1].as_str().try_into() else {
            return Err("Bad Position");
        };
        let Ok(acceleration) = parts[2].as_str().try_into() else {
            return Err("Bad Position");
        };

        Ok(Self {
            position,
            velocity,
            acceleration,
        })
    }
}

fn read_input(path: &str) -> Vec<Particle> {
    let s = fs::read_to_string(path).expect("Input file must exist");
    s.lines()
        .into_iter()
        .map(|line| line.try_into().expect("Bad line"))
        .collect()
}

fn part_one(particles: Vec<Particle>) -> usize {
    let mut particles = particles;
    for i in 0..100000 {
        particles.iter_mut().for_each(|p| p.step());
    }
    particles
        .into_iter()
        .enumerate()
        .min_by(|(ia, a), (ib, b)| a.position.manhattan().cmp(&b.position.manhattan()))
        .map(|(i, p)| i)
        .unwrap_or(0)
}

fn part_two(particles: Vec<Particle>) -> usize {
    let mut particles: Vec<Option<Particle>> = particles.into_iter().map(|p| Some(p)).collect();
    for _ in 0..100000 {
        let mut visited = HashMap::new();
        particles
            .iter_mut()
            .for_each(|particle_option| match particle_option {
                Some(particle) => particle.step(),
                None => {}
            });
        for i in 0..particles.len() {
            let particle_option = particles[i];
            let Some(particle) = particle_option else {
                continue;
            };
            if let Some(existing_particle) = visited.get(&particle.position) {
                particles[i] = None;
                particles[*existing_particle] = None;
            };
            visited.insert(particle.position, i);
        }
    }
    particles.into_iter().filter(|p| p.is_some()).count()
}

fn main() {
    let particles = read_input("input.txt");
    println!("Part one: {}", part_one(particles.clone()));
    println!("Part two: {}", part_two(particles));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {}
}
