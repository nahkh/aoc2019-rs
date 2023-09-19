use crate::input_files::read_content;
use std::collections::HashSet;
use crate::position::Position;
use gcd::Gcd;

#[derive(PartialEq)]
enum Quarter {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Quarter {
    fn of(position: Position) -> Quarter {
        if position.x > 0 {
            if position.y > 0 {
                Quarter::BottomRight
            } else {
                Quarter::TopRight
            }
        } else {
            if position.y > 0 {
                Quarter::BottomLeft
            } else {
                Quarter::TopLeft
            }
        }
    }
}

#[derive(Debug)]
struct AsteroidField {
    asteroids: HashSet<Position>
}

impl AsteroidField {
    fn parse(content: &String) -> AsteroidField{
        let mut asteroids = HashSet::new();
        let mut y = 0;
        for line in content.lines() {
            let mut x = 0;
            for c in line.chars() {
                if c == '#' {
                    asteroids.insert(Position::new(x, y));
                }
                x += 1;
            }
            y += 1;
        }
        AsteroidField {
            asteroids
        }
    }

    fn centered_on(&self, asteroid: Position) -> AsteroidField {
        let mut asteroids = HashSet::new();
        for other in self.asteroids.iter() {
            if *other != asteroid {
                asteroids.insert(*other - asteroid);
            }
        }
        AsteroidField {
            asteroids
        }
    }

    fn asteroid_is_shaded_by(&self, asteroid: Position, offset: Position) -> bool {
        let mut shade = asteroid + offset;
        let initial_quarter = Quarter::of(asteroid);
        while shade != Position::new(0, 0) {
            if self.asteroids.contains(&shade) {
                return true;
            }
            shade = shade + offset;
            assert!(Quarter::of(shade) == initial_quarter || shade == Position::new(0, 0), "{:?} -> {:?} led to {:?}", asteroid, offset, shade);
        }
        return false;
    }

    fn create_offset(&self, asteroid: Position) -> Position {
        assert!(asteroid != Position::new(0, 0));
        if asteroid.x == 0 {
            if asteroid.y > 0 {
                Position::new(0, -1)
            } else {
                Position::new(0, 1)
            }
        } else if asteroid.y == 0 {
            if asteroid.x > 0 {
                Position::new(-1, 0)
            } else {
                Position::new(1, 0)
            }
        } else {
            let x_sign = -asteroid.x.signum();
            let y_sign = -asteroid.y.signum();
            let x_abs = asteroid.x.abs() as u32;
            let y_abs = asteroid.y.abs() as u32;
            let divisor = x_abs.gcd(y_abs);
            let x = x_sign * ((x_abs / divisor) as i32);
            let y = y_sign * ((y_abs / divisor) as i32);
            Position::new(x, y)
        }
    }

    fn asteroid_is_shaded(&self, asteroid: Position) -> bool {
        if asteroid == Position::new(0, 0) {
            return false;
        }
        let offset = self.create_offset(asteroid);
        self.asteroid_is_shaded_by(asteroid, offset)
    }

    fn without_blocked_asteroids(&self) -> AsteroidField {
        let mut asteroids = HashSet::new();
        for other in self.asteroids.iter() {
            if !self.asteroid_is_shaded(*other) {
                asteroids.insert(*other);
            }
        }
        AsteroidField {
            asteroids
        }
    }
}

fn find_best_asteroid(asteroid_field: &AsteroidField) -> usize {
    let mut best_count = 0;
    for &asteroid in asteroid_field.asteroids.iter() {
        let remapped = asteroid_field.centered_on(asteroid);
        let trimmed = remapped.without_blocked_asteroids();
        let seen_asteroids = trimmed.asteroids.len();
        if seen_asteroids > best_count {
            best_count = seen_asteroids;
        }
    }
    best_count
}


pub fn execute() {
    let content = read_content(&String::from("data/day10.txt"));
    let asteroid_field = AsteroidField::parse(&content);
    println!("Part 1: {}", find_best_asteroid(&asteroid_field));
}