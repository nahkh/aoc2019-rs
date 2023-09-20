use std::collections::HashMap;
use crate::position::Position;
use crate::intcode::IntCodeComputer;
use crate::input_files::read_content;

enum Orientation {
    Up,
    Left,
    Right,
    Down,
}

impl Orientation {
    fn new() -> Orientation {
        Orientation::Up
    }

    fn rotate_left(&self) -> Orientation {
        match self {
            Orientation::Left => Orientation::Down,
            Orientation::Down => Orientation::Right,
            Orientation::Right => Orientation::Up,
            Orientation::Up => Orientation::Left,
        }
    }

    fn rotate_right(&self) -> Orientation {
        match self {
            Orientation::Left => Orientation::Up,
            Orientation::Down => Orientation::Left,
            Orientation::Right => Orientation::Down,
            Orientation::Up => Orientation::Right,
        }
    }

    fn get_move_offset(&self) -> Position {
        match self {
            Orientation::Left => Position::new(-1, 0),
            Orientation::Down => Position::new(0, 1),
            Orientation::Right => Position::new(1, 0),
            Orientation::Up => Position::new(0, -1),
        }
    }
}

enum Color {
    Black,
    White,
}

impl Color {
    fn from_number(number: i64) -> Color {
        match number {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Invalid color {}", number),
        }
    }

    fn to_number(&self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

struct HullSurface {
    paint: HashMap<Position, Color>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl HullSurface {
    fn new() -> HullSurface {
        HullSurface { paint: HashMap::new(), min_x: i32::MAX, max_x: i32::MIN, min_y: i32::MAX, max_y: i32::MIN}
    }
    fn get_color(&self, position: &Position) -> &Color {
        self.paint.get(position).unwrap_or(&Color::Black)
    }

    fn set_color(&mut self, position: &Position, color: Color) {
        self.min_x = self.min_x.min(position.x);
        self.max_x = self.max_x.max(position.x);
        self.min_y = self.min_y.min(position.y);
        self.max_y = self.max_y.max(position.y);
        self.paint.insert(*position, color);
    }

    fn render(&self) -> String {
        if self.paint.is_empty() {
            return "".to_string();
        }
        let mut output = String::new();
        for y in self.min_y..(self.max_y + 1) {
            for x in self.min_x..(self.max_x + 1) {
                let position = Position::new(x, y);
                match self.get_color(&position) {
                    Color::Black => output.push(' '),
                    Color::White => output.push('#'),
                }
            }
            output.push('\n');
        }
        output
    }
}

fn part1(content: &String) {
    let mut m = IntCodeComputer::read_program(&content);
    let mut surface = HullSurface::new();
    let mut orientation = Orientation::new();
    let mut robot = Position::new(0, 0);

    while !m.has_terminated() {
        let current_color = surface.get_color(&robot);
        m.add_input(current_color.to_number());
        m.execute_until_stopped();
        let output_size = m.get_output_size();
        let new_color = Color::from_number(m.get_output(output_size - 2).unwrap());
        let rotation_dir = m.get_output(output_size - 1).unwrap();
        orientation = match rotation_dir {
            0 => orientation.rotate_left(),
            1 => orientation.rotate_right(),
            _ => panic!("Invalid rotation direction {}", rotation_dir),
        };
        surface.set_color(&robot, new_color);
        robot = robot + orientation.get_move_offset();
    }

    println!("Part 1: Number of tiles painted {}", surface.paint.len());
}

fn part2(content: &String) {
    let mut m = IntCodeComputer::read_program(&content);
    let mut surface = HullSurface::new();
    let mut orientation = Orientation::new();
    let mut robot = Position::new(0, 0);
    surface.set_color(&robot, Color::White);

    while !m.has_terminated() {
        let current_color = surface.get_color(&robot);
        m.add_input(current_color.to_number());
        m.execute_until_stopped();
        let output_size = m.get_output_size();
        let new_color = Color::from_number(m.get_output(output_size - 2).unwrap());
        let rotation_dir = m.get_output(output_size - 1).unwrap();
        orientation = match rotation_dir {
            0 => orientation.rotate_left(),
            1 => orientation.rotate_right(),
            _ => panic!("Invalid rotation direction {}", rotation_dir),
        };
        surface.set_color(&robot, new_color);
        robot = robot + orientation.get_move_offset();
    }

    println!("Part 2:\n{}", surface.render());
}


pub fn execute() {
    let content = read_content(&String::from("data/day11.txt"));
    part1(&content);
    part2(&content);
}