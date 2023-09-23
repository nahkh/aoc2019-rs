use crate::input_files::read_content;
use crate::position3::Position3;
use gcd::Gcd;
use regex::Regex;
use std::collections::HashSet;

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
    fn new(moons: Vec<Moon>) -> Orbits {
        Orbits { moons }
    }

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

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Moon1d {
    position: i32,
    velocity: i32,
}

impl Moon1d {
    fn new(position: i32, velocity: i32) -> Moon1d {
        Self { position, velocity }
    }

    fn evolve(&self, acceleration: i32) -> Moon1d {
        let new_velocity = self.velocity + acceleration;
        Self {
            position: self.position + new_velocity,
            velocity: new_velocity,
        }
    }

    fn calcute_gravity(&self, other: &Self) -> i32 {
        if self.position > other.position {
            -1
        } else if self.position < other.position {
            1
        } else {
            0
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Orbits1d {
    moon1: Moon1d,
    moon2: Moon1d,
    moon3: Moon1d,
    moon4: Moon1d,
}

impl Orbits1d {
    fn new(moon1: Moon1d, moon2: Moon1d, moon3: Moon1d, moon4: Moon1d) -> Orbits1d {
        Self {
            moon1,
            moon2,
            moon3,
            moon4,
        }
    }

    fn evolve(&self) -> Orbits1d {
        let d1 = self.moon1.calcute_gravity(&self.moon2)
            + self.moon1.calcute_gravity(&self.moon3)
            + self.moon1.calcute_gravity(&self.moon4);
        let d2 = self.moon2.calcute_gravity(&self.moon1)
            + self.moon2.calcute_gravity(&self.moon3)
            + self.moon2.calcute_gravity(&self.moon4);
        let d3 = self.moon3.calcute_gravity(&self.moon1)
            + self.moon3.calcute_gravity(&self.moon2)
            + self.moon3.calcute_gravity(&self.moon4);
        let d4 = self.moon4.calcute_gravity(&self.moon1)
            + self.moon4.calcute_gravity(&self.moon2)
            + self.moon4.calcute_gravity(&self.moon3);
        Orbits1d::new(
            self.moon1.evolve(d1),
            self.moon2.evolve(d2),
            self.moon3.evolve(d3),
            self.moon4.evolve(d4),
        )
    }

    fn calculate_cycle(&self) -> u64 {
        let mut cycles = 0;
        let mut visited_states = HashSet::new();
        let mut current_state = self.clone();
        while !visited_states.contains(&current_state) {
            visited_states.insert(current_state);
            current_state = current_state.evolve();
            cycles += 1;
        }
        cycles
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct CompositeOrbits {
    orbits_x: Orbits1d,
    orbits_y: Orbits1d,
    orbits_z: Orbits1d,
}

impl CompositeOrbits {
    fn new(moons: Vec<Moon>) -> CompositeOrbits {
        assert!(moons.len() == 4);
        CompositeOrbits {
            orbits_x: Orbits1d {
                moon1: Moon1d::new(moons[0].position.x, moons[0].velocity.x),
                moon2: Moon1d::new(moons[1].position.x, moons[1].velocity.x),
                moon3: Moon1d::new(moons[2].position.x, moons[2].velocity.x),
                moon4: Moon1d::new(moons[3].position.x, moons[3].velocity.x),
            },
            orbits_y: Orbits1d {
                moon1: Moon1d::new(moons[0].position.y, moons[0].velocity.y),
                moon2: Moon1d::new(moons[1].position.y, moons[1].velocity.y),
                moon3: Moon1d::new(moons[2].position.y, moons[2].velocity.y),
                moon4: Moon1d::new(moons[3].position.y, moons[3].velocity.y),
            },
            orbits_z: Orbits1d {
                moon1: Moon1d::new(moons[0].position.z, moons[0].velocity.z),
                moon2: Moon1d::new(moons[1].position.z, moons[1].velocity.z),
                moon3: Moon1d::new(moons[2].position.z, moons[2].velocity.z),
                moon4: Moon1d::new(moons[3].position.z, moons[3].velocity.z),
            },
        }
    }

    fn calculate_cycle(&self) -> u64 {
        let cycle_x = self.orbits_x.calculate_cycle();
        let cycle_y = self.orbits_y.calculate_cycle();
        let cycle_yx = cycle_x * cycle_y / (cycle_x.gcd(cycle_y));
        let cycle_z = self.orbits_z.calculate_cycle();
        cycle_yx * cycle_z / (cycle_yx.gcd(cycle_z))
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

fn parse_orbits(content: &String) -> Vec<Moon> {
    let mut moons = Vec::new();
    for line in content.lines() {
        let position = parse_position(&line.to_string());
        if position.is_some() {
            moons.push(Moon::new(position.unwrap()));
        }
    }

    moons
}

fn part1(content: &String, print_steps: bool) {
    let mut orbits = Orbits::new(parse_orbits(content));
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

fn part2(content: &String) {
    let orbits = CompositeOrbits::new(parse_orbits(content));
    println!(
        "Part 2: The universe cycles in {:?} steps",
        orbits.calculate_cycle()
    );
}

pub fn execute() {
    let content = read_content(&"data/day12.txt".to_string());
    part1(&content, false);
    part2(&content);
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
