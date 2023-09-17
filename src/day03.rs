use std::result::Result;
use std::option::Option;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Add;
use std::cmp::min;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y
            }
    }
}

trait ManhattanDistance {
    fn distance(self, other: &Self) -> i32;
}

impl ManhattanDistance for Position {
    fn distance(self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    LEFT(i32),
    RIGHT(i32),
    UP(i32),
    DOWN(i32),
}

#[derive(Debug)]
struct FuelManagementSystem {
    current_position: Position,
    current_length: i32,
    known_wires: HashSet<Position>,
    known_lengths: HashMap<Position, i32>,
}

trait InstructionConsumer {
    fn apply(&mut self, instruction: Instruction);
}

impl InstructionConsumer for FuelManagementSystem {
    fn apply(&mut self, instruction: Instruction) {
        let (offset, distance) = match instruction {
            Instruction::LEFT(x) => (Position {x: -1, y: 0}, x),
            Instruction::RIGHT(x) => (Position {x: 1, y: 0}, x),
            Instruction::UP(x) => (Position {x: 0, y: -1}, x),
            Instruction::DOWN(x) => (Position {x: 0, y: 1}, x),
        };
        // println!("From {:?} <- {:?} x {}", self.current_position, offset, distance);
        for _i in 0..distance {
            // println!("At {:?}", self.current_position);
            self.current_position = self.current_position + offset;
            self.current_length += 1;
            self.known_wires.insert(self.current_position);
            self.known_lengths.insert(self.current_position, self.current_length);
        }
        // println!("reached {:?}", self.current_position);
    }
}

fn parse_instruction(content: String) -> Result<Instruction, &'static str> {
    // TODO: This is likely not the idiomatic way of translating errors,
    // Find a better way.
    let distance_result = content[1..].trim().parse::<i32>();
    if distance_result.is_err() {
        return Err("Invalid distance");
    }
    
    let distance = distance_result.unwrap();
    match &content[0..1] {
        "R" => Ok(Instruction::RIGHT(distance)),
        "L" => Ok(Instruction::LEFT(distance)),
        "U" => Ok(Instruction::UP(distance)),
        "D" => Ok(Instruction::DOWN(distance)),
        _ => Err("Invalid leading character"),
    }
}

fn parse_instructions(content: String) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for part in content.split(',') {
        let parse_result = parse_instruction(part.to_string());
        match parse_result {
            Ok(instruction) => instructions.push(instruction),
            Err(_) => panic!("Could not parse instruction {}", part),
        };
    }
    return instructions;
}

fn find_nearest_crossing(first_wire: String, second_wire: String) -> Option<i32> {
    let instructions1 = parse_instructions(first_wire);
    let instructions2 = parse_instructions(second_wire);
    let origin = Position {x: 0, y: 0};
    let mut fms1 = FuelManagementSystem { 
        current_position: origin, 
        current_length: 0,
        known_wires: HashSet::new(),
        known_lengths: HashMap::new(),
    };
    let mut fms2 = FuelManagementSystem { 
        current_position: origin, 
        current_length: 0,
        known_wires: HashSet::new(),
        known_lengths: HashMap::new(),
    };
    for instruction in instructions1 {
        fms1.apply(instruction);
    }
    for instruction in instructions2 {
        fms2.apply(instruction);
    }
    let mut smallest_distance = None;
    for crossing in fms1.known_wires.intersection(&fms2.known_wires) {
        smallest_distance = match smallest_distance {
            None => Some(origin.distance(crossing)),
            Some(x) => Some(min(x, origin.distance(crossing))),
        }
    }
    return smallest_distance;
}

fn find_shortest_crossing(first_wire: String, second_wire: String) -> Option<i32> {
    let instructions1 = parse_instructions(first_wire);
    let instructions2 = parse_instructions(second_wire);
    let origin = Position {x: 0, y: 0};
    let mut fms1 = FuelManagementSystem { 
        current_position: origin, 
        current_length: 0,
        known_wires: HashSet::new(),
        known_lengths: HashMap::new(),
    };
    let mut fms2 = FuelManagementSystem { 
        current_position: origin, 
        current_length: 0,
        known_wires: HashSet::new(),
        known_lengths: HashMap::new(),
    };
    for instruction in instructions1 {
        fms1.apply(instruction);
    }
    for instruction in instructions2 {
        fms2.apply(instruction);
    }
    let mut smallest_distance = None;
    for crossing in fms1.known_wires.intersection(&fms2.known_wires) {
        smallest_distance = match smallest_distance {
            None => Some(fms1.known_lengths[crossing] + fms2.known_lengths[crossing]),
            Some(x) => Some(min(x, fms1.known_lengths[crossing] + fms2.known_lengths[crossing])),
        }
    }
    return smallest_distance;
}

pub fn execute() {
    let content = crate::input_files::read_content(&String::from("data/day03.txt"));
    let lines: Vec<&str> = content.lines().collect();
    let nearest_crossing = find_nearest_crossing(lines[0].to_string(), lines[1].to_string());
    println!("Part 1: Nearest crossing at {}", nearest_crossing.unwrap_or(-1));
    let shortest_crossing = find_shortest_crossing(lines[0].to_string(), lines[1].to_string());
    println!("Part 2: Shortest crossing at {}", shortest_crossing.unwrap_or(-1));
}



#[cfg(test)]
mod tests {
    use crate::day03::Instruction;
    use crate::day03::parse_instruction;
    use crate::day03::parse_instructions;
    use crate::day03::find_nearest_crossing;
    use crate::day03::find_shortest_crossing;
    use crate::day03::Position;
    use crate::day03::FuelManagementSystem;
    use crate::day03::HashSet;
    use crate::day03::HashMap;
    use crate::day03::InstructionConsumer;

    #[test]
    fn test_parsing() {
        let l34 = parse_instruction(String::from("L34")).unwrap();
        assert_eq!(l34, Instruction::LEFT(34));
        let r32 = parse_instruction(String::from("R 32 ")).unwrap();
        assert_eq!(r32, Instruction::RIGHT(32));
        let u12 = parse_instruction(String::from("U12 ")).unwrap();
        assert_eq!(u12, Instruction::UP(12));
        let d17 = parse_instruction(String::from("D17\n")).unwrap();
        assert_eq!(d17, Instruction::DOWN(17));
    }

    #[test]
    fn test_multi_parsing() {
        let instructions = parse_instructions(String::from("L34,R32,U12,D17\n"));
        assert_eq!(instructions.len(), 4);
        assert_eq!(instructions[0], Instruction::LEFT(34));
        assert_eq!(instructions[1], Instruction::RIGHT(32));
        assert_eq!(instructions[2], Instruction::UP(12));
        assert_eq!(instructions[3], Instruction::DOWN(17));
    }

    #[test]
    fn test_instruction() {
        let origin = Position {x: 0, y: 0};
        let mut fms = FuelManagementSystem { 
            current_position: origin, 
            current_length: 0,
            known_wires: HashSet::new(),
            known_lengths: HashMap::new(),
        };
        fms.apply(Instruction::LEFT(1));
        assert_eq!(fms.current_position, Position {x: -1, y: 0});
        assert_eq!(fms.known_wires, HashSet::from([Position {x: -1, y: 0}]));
        fms.apply(Instruction::UP(2));
        assert_eq!(fms.current_position, Position {x: -1, y: -2});
        assert_eq!(fms.known_wires, HashSet::from([
            Position {x: -1, y: 0},
            Position {x: -1, y: -1},
            Position {x: -1, y: -2}
        ]));
        
    }

    #[test]
    fn test_nearest_crossing_execution() {
        let distance1 = find_nearest_crossing("R8,U5,L5,D3".to_string(), "U7,R6,D4,L4".to_string()).unwrap();
        assert_eq!(distance1, 6);
        let distance2 = find_nearest_crossing("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(), "U62,R66,U55,R34,D71,R55,D58,R83".to_string()).unwrap();
        assert_eq!(distance2, 159);
        let distance3 = find_nearest_crossing("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(), "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string()).unwrap();
        assert_eq!(distance3, 135);
    }

    #[test]
    fn test_shortest_crossing_execution() {
        let distance1 = find_shortest_crossing("R8,U5,L5,D3".to_string(), "U7,R6,D4,L4".to_string()).unwrap();
        assert_eq!(distance1, 30);
        let distance2 = find_shortest_crossing("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(), "U62,R66,U55,R34,D71,R55,D58,R83".to_string()).unwrap();
        assert_eq!(distance2, 610);
        let distance3 = find_shortest_crossing("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(), "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string()).unwrap();
        assert_eq!(distance3, 410);
    }
}
