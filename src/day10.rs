use crate::input_files::read_content;
use std::collections::HashSet;
use crate::position::Position;
use gcd::Gcd;
use std::f64::consts::FRAC_PI_2;
use std::f64::consts::PI;

#[derive(PartialEq, Debug)]
enum Quarter {
    Center,
    Top,
    Left,
    Right,
    Bottom,
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
            } else if position.y < 0 {
                Quarter::TopRight
            } else {
                Quarter::Right
            }
        } else if position.x < 0 {
            if position.y > 0 {
                Quarter::BottomLeft
            } else if position.y < 0 {
                Quarter::TopLeft
            } else {
                Quarter::Left
            }
        } else {
            if position.y > 0 {
                Quarter::Bottom
            } else if position.y < 0 {
                Quarter::Top
            } else {
                Quarter::Center
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
    
    
    fn get_angle(asteroid: Position) -> f64 {
        let angle = match Quarter::of(asteroid) {
            Quarter::Center => 0.0_f64,
            Quarter::Top => 0.0_f64,
            Quarter::Left => FRAC_PI_2 * 3.0_f64,
            Quarter::Right => FRAC_PI_2,
            Quarter::Bottom => PI,
            Quarter::TopLeft => 2.0 * PI - (asteroid.x as f64 / asteroid.y as f64).atan(),
            Quarter::TopRight => (asteroid.x as f64 / -asteroid.y as f64).atan(),
            Quarter::BottomLeft => PI + (-asteroid.x as f64 / asteroid.y as f64).atan(),
            Quarter::BottomRight => PI - (asteroid.x as f64 / asteroid.y as f64).atan(),
        };

        assert!(angle >= 0.0, "{:?} should've had a positive angle, had {:.3}", asteroid, angle);

        angle
    }

    fn get_rotation_angle(&self, asteroid: Position) -> f64 {
        let offset = self.create_offset(asteroid);
        let shade_count = self.asteroid_is_shaded_by(asteroid, offset);
        
        Self::get_angle(asteroid) + (shade_count as f64 * 2.0 * PI)
    }


    fn asteroid_is_shaded_by(&self, asteroid: Position, offset: Position) -> usize {
        let mut shading_count = 0;
        let mut shade = asteroid + offset;
        let initial_quarter = Quarter::of(asteroid);
        while shade != Position::new(0, 0) {
            if self.asteroids.contains(&shade) {
                shading_count += 1;
            }
            shade = shade + offset;
            assert!(Quarter::of(shade) == initial_quarter || shade == Position::new(0, 0), "{:?} -> {:?} led to {:?}", asteroid, offset, shade);
        }
        return shading_count;
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
        self.asteroid_is_shaded_by(asteroid, offset) > 0
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

fn find_best_asteroid(asteroid_field: &AsteroidField) -> (usize, Position) {
    let mut best_count = 0;
    let mut best_asteroid = Position::new(0, 0);
    for &asteroid in asteroid_field.asteroids.iter() {
        let remapped = asteroid_field.centered_on(asteroid);
        let trimmed = remapped.without_blocked_asteroids();
        let seen_asteroids = trimmed.asteroids.len();
        if seen_asteroids > best_count {
            best_count = seen_asteroids;
            best_asteroid = asteroid;
        }
    }
    (best_count, best_asteroid)
}

fn find_200th_asteroid(asteroid_field: &AsteroidField, center: Position) -> Position {
    let remapped = asteroid_field.centered_on(center);
    let mut asteroids = Vec::new();
    for &asteroid in remapped.asteroids.iter() {
        let angle = remapped.get_rotation_angle(asteroid);
        asteroids.push((angle, asteroid));
    }
    asteroids.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    asteroids.get(199).unwrap().1
}


pub fn execute() {
    let content = read_content(&String::from("data/day10.txt"));
    let asteroid_field = AsteroidField::parse(&content);
    let (count, best_asteroid) = find_best_asteroid(&asteroid_field);
    println!("Part 1: {} at {:?}", count, best_asteroid);
    let asteroid200 = find_200th_asteroid(&asteroid_field, best_asteroid);
    let original_asteroid200 = asteroid200 + best_asteroid;
    println!("Part 2: {:?} -> {}", original_asteroid200, original_asteroid200.x * 100 + original_asteroid200.y);
}

#[cfg(test)]
mod tests {
    use crate::day10::FRAC_PI_2;
    use crate::day10::PI;
    use crate::position::Position;
    use crate::input_files::read_content;
    use crate::day10::AsteroidField;
    use crate::day10::find_best_asteroid;
    use crate::day10::find_200th_asteroid;

    fn assert_near(expected: f64, actual: f64) {
        let abs_difference = (expected - actual).abs();
        assert!(abs_difference < 1e-10, "{:.3} should be near {:.3}", actual, expected);
    }

    #[test]
    fn evaluate_angles() {
        assert_near(0.0, AsteroidField::get_angle(Position::new(0, -1)));
        assert_near(FRAC_PI_2, AsteroidField::get_angle(Position::new(1, 0)));
        assert_near(PI, AsteroidField::get_angle(Position::new(0, 1)));
        assert_near(3.0 * FRAC_PI_2, AsteroidField::get_angle(Position::new(-1, 0)));
        assert_near(0.5 * FRAC_PI_2, AsteroidField::get_angle(Position::new(1, -1)));
        assert_near(1.5 * FRAC_PI_2, AsteroidField::get_angle(Position::new(1, 1)));
        assert_near(2.5 * FRAC_PI_2, AsteroidField::get_angle(Position::new(-1, 1)));
        assert_near(3.5 * FRAC_PI_2, AsteroidField::get_angle(Position::new(-1, -1)));
    }

    #[test]
    fn evalute_known_pattern() {
        let content = read_content(&String::from("data/day10_sample.txt"));
        let asteroid_field = AsteroidField::parse(&content);
        let (count, best_asteroid) = find_best_asteroid(&asteroid_field);
        assert_eq!(count, 210);
        assert_eq!(best_asteroid, Position::new(11, 13));
        let asteroid200 = find_200th_asteroid(&asteroid_field, best_asteroid);
        let original_asteroid200 = asteroid200 + best_asteroid;
        assert_eq!(original_asteroid200, Position::new(8, 2));
    }

}
