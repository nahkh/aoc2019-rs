use crate::position::Position;
use crate::intcode::IntCodeComputer;
use crate::input_files::read_content;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::slice::Iter;

#[derive(Hash, PartialEq, Clone)]
enum Tile {
    Start,
    Floor,
    Wall,
    Oxygen,
    Empty,
}

impl Tile {
    fn render(&self) -> char {
        match self {
            Tile::Start => 'S',
            Tile::Floor => '.',
            Tile::Wall => '#',
            Tile::Oxygen => 'O',
            Tile::Empty => ' ',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn as_position(&self) -> Position {
        match self {
            Direction::North => Position::new(0, -1),
            Direction::South => Position::new(0, 1),
            Direction::West => Position::new(-1, 0),
            Direction::East => Position::new(1, 0),
        }
    }

    fn as_machine_api(&self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::West => '<',
            Direction::East => '>',
        }
    }

    fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];
        DIRECTIONS.iter()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Route {
    directions: Vec<Direction>,
}

impl Route {
    fn new() -> Route {
        Route {
            directions: Vec::new(),
        }
    }

    fn append(&self, direction: &Direction) -> Route {
        let mut new_directions = self.directions.clone();
        new_directions.push(direction.clone());
        Route {
            directions: new_directions,
        }
    }

    fn len(&self) -> usize {
        self.directions.len()
    } 
}


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct PositionNode {
    cost: usize,
    position: Position,
    route: Route,
}

impl PositionNode {
    fn new(position: Position) -> PositionNode {
        PositionNode {
            cost: 0,
            position: position,
            route: Route::new(),
        }
    }

    fn apply_direction(&self, direction: &Direction) -> PositionNode {
        PositionNode {
            cost: self.cost + 1, 
            position: self.position + direction.as_position(),
            route: self.route.append(direction)
        }
    }

    fn neighbors(&self) -> Vec<PositionNode> {
        Direction::iterator().map(|x| self.apply_direction(x)).collect::<Vec<_>>()
    }
}

impl Ord for PositionNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for PositionNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


enum RoutingError {
    InvalidStartPosition,
    InvalidEndPosition,
    StartEqualsEnd,
    NoPathFound,
}

struct RobotMap {
    tiles: HashMap<Position, Tile>,
    frontier: HashSet<Position>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    oxygen_position: Option<Position>,
    annotations: HashMap<Position, char>,
}


impl RobotMap {
    fn new() -> RobotMap {
        RobotMap {
            tiles: HashMap::new(),
            frontier: HashSet::new(),
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
            oxygen_position: None,
            annotations: HashMap::new(),
        }
    }

    fn try_adding_to_frontier(&mut self, position: Position) {
        if self.frontier.contains(&position) || self.tiles.contains_key(&position) {
            return;
        }
        self.frontier.insert(position);
    }

    fn put(&mut self, position: Position, value: Tile) {
        self.min_x = self.min_x.min(position.x);
        self.max_x = self.max_x.max(position.x + 1);
        self.min_y = self.min_y.min(position.y);
        self.max_y = self.max_y.max(position.y + 1);
        self.tiles.insert(position, value.clone());
        if value == Tile::Oxygen {
            self.oxygen_position = Some(position);
        } 
        self.frontier.remove(&position);
        if value != Tile::Wall {
            for neighbor in position.neighbors() {
                self.try_adding_to_frontier(neighbor);
            }
        }
    }

    fn render(&self) -> String {
        let mut output = String::new();
        for y in (self.min_y - 2)..(self.max_y + 2) {
            for x in (self.min_x - 2)..(self.max_x + 2) {
                let pos = Position::new(x, y);
                if self.annotations.contains_key(&pos) {
                    output.push(
                        *self.annotations
                            .get(&pos)
                            .unwrap(),
                    );
                } else {
                    output.push(
                        self.tiles
                            .get(&pos)
                            .unwrap_or(&Tile::Empty)
                            .render(),
                    );
                }
                
            }
            output.push('\n');
        }
        output
    }

    fn get_nearest_frontier(&self, position: Position) -> Option<Position> {
        let mut closest_position = None;
        let mut closest_distance = u64::MAX;
        for frontier_position in self.frontier.iter() {
            let distance = frontier_position.manhattan_distance(position);
            if distance < closest_distance {
                closest_distance = distance;
                closest_position = Some(*frontier_position);
            }
        }
        
        closest_position
    }

    fn find_route(&self, start: Position, end: Position) -> Result<Route, RoutingError> {
        if start == end {
            return Err(RoutingError::StartEqualsEnd);
        }
        if self.tiles.get(&start).unwrap_or(&Tile::Wall) == &Tile::Wall {
            return Err(RoutingError::InvalidStartPosition);
        }
        if self.tiles.get(&end).unwrap_or(&Tile::Empty) == &Tile::Wall && !self.frontier.contains(&end) {
            return Err(RoutingError::InvalidEndPosition);
        }
        let mut visited_positions = HashSet::new();
        let mut frontier = BinaryHeap::new();
        frontier.push(PositionNode::new(start));
        while let Some(node) = frontier.pop() {
            if node.position == end {
                return Ok(node.route);
            }
            visited_positions.insert(node.position.clone());
            for neighbor in node.neighbors() {
                if visited_positions.contains(&neighbor.position) {
                    continue;
                }
                let tile = self.tiles.get(&neighbor.position).unwrap_or(&Tile::Empty);
                if tile != &Tile::Wall && (tile != &Tile::Empty || (self.frontier.contains(&neighbor.position) && neighbor.position == end)) {
                    frontier.push(neighbor.clone());
                }
            }
        }

        Err(RoutingError::NoPathFound)
    }

    fn annotate_route(&mut self, route: &Route, starting_position: &Position, current_position: &Position) {
        let mut position = starting_position.clone();
        self.annotations.clear();
        for direction in route.clone().directions {
            self.annotations.insert(position.clone(), direction.as_char());
            position = position + direction.as_position();
        }
        self.annotations.insert(position, 'G');
        self.annotations.insert(*current_position, 'R');
    }

    fn get_max_distance_from(&self, start: Position) -> usize {
        let mut max_distance = 0;
        let mut visited_positions = HashSet::new();
        let mut frontier = BinaryHeap::new();
        frontier.push(PositionNode::new(start));
        while let Some(node) = frontier.pop() {
            max_distance = max_distance.max(node.cost);
            visited_positions.insert(node.position.clone());
            for neighbor in node.neighbors() {
                if visited_positions.contains(&neighbor.position) {
                    continue;
                }
                let tile = self.tiles.get(&neighbor.position).unwrap();
                if tile != &Tile::Wall {
                    frontier.push(neighbor.clone());
                }
            }
        }
        return max_distance;
    }
}

struct RobotEnvironment {
    robot_position: Position,
    map: RobotMap,
    computer: IntCodeComputer,
}

impl RobotEnvironment {
    fn new(content: &String) -> RobotEnvironment {
        let mut environment = RobotEnvironment {
            robot_position: Position::new(0, 0),
            map: RobotMap::new(),
            computer: IntCodeComputer::read_program(content),
        };
        environment.map.put(environment.robot_position, Tile::Start);

        environment
    }

    fn attempt_movement(&mut self, direction: &Direction) -> bool {
        let new_position = self.robot_position + direction.as_position();
        self.computer.add_input(direction.as_machine_api());
        self.computer.execute_until_stopped();
        let response_code = self.computer.get_last_output();
        match response_code {
            Some(0) => {
                // Hit a wall, could not move
                self.map.put(new_position, Tile::Wall);
                false
            },
            Some(1) => {
                // Moved
                self.map.put(new_position, Tile::Floor);
                self.robot_position = new_position;
                true
            },
            Some(2) => {
                 // Moved, found oxygen system
                 self.map.put(new_position, Tile::Oxygen);
                 self.robot_position = new_position;
                 true
            },
            _ => panic!("Unexpected response code {:?}", response_code),
        }
    }

    fn attempt_route(&mut self, route: Route) {
        let mut failed_previous_move = false;
        let initial_position = self.robot_position;
        for direction in &route.directions {
            if failed_previous_move {
                println!("{}", self.map.render());
                self.map.annotate_route(&route, &initial_position, &self.robot_position);
                panic!("Attempting to move along a route, but a middle step failed.\nInitial position: {:?}\nCurrent position{:?}\nRoute {:?}\nMap: {}", initial_position, self.robot_position, route, self.map.render());
            }
            failed_previous_move = !self.attempt_movement(direction);
        }
    }

    fn explore(&mut self) {
        while let Some(frontier_position) = self.map.get_nearest_frontier(self.robot_position) {
            let route = self.map.find_route(self.robot_position, frontier_position).ok().unwrap();
            self.attempt_route(route);
        }
    }

    fn get_shorted_path_length_to_oxygen(&self) -> Option<usize> {
        Some(self.map.find_route(Position::new(0, 0), self.map.oxygen_position?).ok()?.len())
    }

    fn get_oxygen_fill_time(&self) -> Option<usize> {
        Some(self.map.get_max_distance_from(self.map.oxygen_position?))
    }
}


fn part1(content: &String) {
    let mut robot_environment = RobotEnvironment::new(content);
    robot_environment.explore();
    println!("Part 1: Shortest route to oxygen system is {}", robot_environment.get_shorted_path_length_to_oxygen().unwrap_or(usize::MAX));
}

fn part2(content: &String) {
    let mut robot_environment = RobotEnvironment::new(content);
    robot_environment.explore();
    println!("Part 2: Time taken to fill with oxygen is {} minutes", robot_environment.get_oxygen_fill_time().unwrap_or(usize::MAX));
}

pub fn execute() {
    let content = read_content(&"data/day15.txt".to_string());
    part1(&content);
    part2(&content);
}