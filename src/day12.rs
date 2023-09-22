use crate::input_files::read_content;
use crate::position3::Position3;
use regex::Regex;

#[derive(Debug, Clone)]
struct Moon {
    position: Position3,
    velocity: Position3,
}

impl Moon {
    fn new(initial_position: Position3) -> Moon {
        Moon {
            position: initial_position,
            velocity: Position3::new(0, 0, 0),
        }
    }

    fn accumulate(&mut self, delta: Position3) {
        self.velocity += delta;
    }

    fn apply(&mut self) {
        self.position += self.velocity;
    }

    fn render(&self) -> String {
        format!(
            "pos=<x={:3}, y={:3}, z={:3}>, vel=<x={:3}, y={:3}, z={:3}>",
            self.position.x,
            self.position.y,
            self.position.z,
            self.velocity.x,
            self.velocity.y,
            self.velocity.z
        )
    }

    fn calcute_gravity(&self, other: &Self) -> Position3 {
        let mut dx = 0;
        let mut dy = 0;
        let mut dz = 0;
        if self.position.x > other.position.x {
            dx = -1;
        } else if self.position.x < other.position.x {
            dx = 1;
        }
        if self.position.y > other.position.y {
            dy = -1;
        } else if self.position.y < other.position.y {
            dy = 1;
        }
        if self.position.z > other.position.z {
            dz = -1;
        } else if self.position.z < other.position.z {
            dz = 1;
        }

        Position3::new(dx, dy, dz)
    }

    fn potential_energy(&self) -> u64 {
        self.position.x.abs() as u64 + self.position.y.abs() as u64 + self.position.z.abs() as u64
    }

    fn kinetic_energy(&self) -> u64 {
        self.velocity.x.abs() as u64 + self.velocity.y.abs() as u64 + self.velocity.z.abs() as u64
    }

    fn energy(&self) -> u64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[derive(Debug)]
struct Orbits {
    moons: Vec<Moon>,
}

impl Orbits {
    fn simulate_step(&mut self) {
        let moon_copy = self.moons.to_vec();
        for i in 0..self.moons.len() {
            for j in 0..self.moons.len() {
                if i == j {
                    continue;
                }
                let moon1 = &mut self.moons[i];
                let moon2 = &moon_copy[j];
                moon1.accumulate(moon1.calcute_gravity(&moon2));
            }
        }
        for moon in self.moons.iter_mut() {
            moon.apply();
        }
    }

    fn total_energy(&self) -> u64 {
        let mut energy = 0;
        for moon in self.moons.iter() {
            energy += moon.energy();
        }

        energy
    }
}

fn parse_position(line: &String) -> Option<Position3> {
    let re = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
    let matched = re.captures(line)?;
    return Some(Position3::new(
        matched[1].parse::<i32>().ok()?,
        matched[2].parse::<i32>().ok()?,
        matched[3].parse::<i32>().ok()?,
    ));
}

fn parse_orbits(content: &String) -> Orbits {
    let mut moons = Vec::new();
    for line in content.lines() {
        let position = parse_position(&line.to_string());
        if position.is_some() {
            moons.push(Moon::new(position.unwrap()));
        }
    }

    Orbits { moons }
}

fn part1(content: &String, print_steps: bool) {
    let mut orbits = parse_orbits(content);
    for step in 0..1000 {
        if print_steps {
            println!("After {} steps:", step);
            for moon in orbits.moons.iter() {
                println!("{}", moon.render());
            }
            println!("");
        }

        orbits.simulate_step();
    }
    println!(
        "Part 1: Total energy after 1000 steps is {}",
        orbits.total_energy()
    );
}

pub fn execute() {
    let content = read_content(&"data/day12.txt".to_string());
    part1(&content, false);
}

#[cfg(test)]
mod tests {
    use crate::day12::parse_position;
    use crate::position3::Position3;

    #[test]
    fn test_parse_position() {
        assert_eq!(
            parse_position(&("<x=-15, y=-14, z=12>".to_string())).unwrap(),
            Position3::new(-15, -14, 12)
        )
    }
}
