use crate::position::Position;
use crate::intcode::IntCodeComputer;
use crate::input_files::read_content;
use std::collections::HashMap;


#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl Tile {
    fn from_intcode(value: i64) -> Tile {
        match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => panic!("Invalid output value {}", value),
        }
    }

    fn render(&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Block => '+',
            Tile::HorizontalPaddle => '-',
            Tile::Ball => '*',
        }
    }
}

#[derive(Clone)]
struct Display {
    data: HashMap<Position, Tile>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    ball_position: Option<Position>,
    paddle_position: Option<Position>,
}

impl Display {
    fn new() -> Display {
        Display {
            data: HashMap::new(),
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
            ball_position: None,
            paddle_position: None,
        }
    }

    fn put(&mut self, position: Position, value: Tile) {
        self.min_x = self.min_x.min(position.x);
        self.max_x = self.max_x.max(position.x + 1);
        self.min_y = self.min_y.min(position.y);
        self.max_y = self.max_y.max(position.y + 1);
        self.data.insert(position, value);
        if value == Tile::Ball {
            self.ball_position = Some(position);
        } else if value == Tile::HorizontalPaddle {
            self.paddle_position = Some(position);
        }
    }

    fn render(&self) -> String {
        let mut output = String::new();
        for y in self.min_y..self.max_y {
            for x in self.min_x..self.max_x {
                output.push(
                    self.data
                        .get(&Position::new(x, y))
                        .unwrap_or(&Tile::Empty)
                        .render(),
                );
            }
            output.push('\n');
        }
        output
    }

    fn find_tile(&self, wanted_tile: Tile) -> Option<Position> {
        if wanted_tile == Tile::HorizontalPaddle {
            return self.paddle_position;
        }
        if wanted_tile == Tile::Ball {
            return self.ball_position;
        }
        for (pos, tile) in self.data.clone().into_iter() {
            if tile == wanted_tile {
                return Some(pos);
            }
        }

        None
    }
}


#[derive(Clone)]
struct ArcadeCabinet {
    display: Display,
    computer: IntCodeComputer,
    score: u64,
    read_output: usize,
    tick: usize,
}

impl ArcadeCabinet {
    fn new(program: &String) -> ArcadeCabinet {
        let mut cabinet = ArcadeCabinet {
            display: Display::new(),
            computer: IntCodeComputer::read_program(program),
            score: 0,
            read_output: 0,
            tick: 0,
        };
        cabinet.computer.set_value(0, 2);
        cabinet.execute(0);

        cabinet
    }

    fn execute(&mut self, input: i64) {
        self.computer.add_input(input);
        self.computer.execute_until_stopped();
        for i in (self.read_output / 3)..(self.computer.get_output_size() / 3) {
            let x = self.computer.get_output(i * 3).unwrap();
            let y = self.computer.get_output(i * 3 + 1).unwrap();
            let tile_id = self.computer.get_output(i * 3 + 2).unwrap();
            if x == -1 && y == 0 {
                self.score = tile_id as u64;
            } else {
                self.display.put(Position::new(x as i32, y as i32), Tile::from_intcode(tile_id));
            }
        }
        self.read_output = self.computer.get_output_size();
        self.tick += 1;
    }

    fn get_block_count(&self) -> usize {
        let mut count = 0;
        for (_, value) in self.display.data.clone().into_iter() {
            if value == Tile::Block {
                count += 1;
            }
        }
        count
    }

    fn play(&mut self, render_game: bool) -> u64 {
        loop {
            if render_game {
                println!("{}", self.render());
            }
            if self.get_block_count() == 0 {
                return self.score;
            }

            let ball_position = self.display.find_tile(Tile::Ball).unwrap();
            let paddle_position = self.display.find_tile(Tile::HorizontalPaddle).unwrap();
            let paddle_direction = if paddle_position.x < ball_position.x {
                1
            } else if paddle_position.x > ball_position.x {
                -1
            } else {
                0
            };
            self.execute(paddle_direction);
        }
    }

    fn render(&self) -> String {
        let mut output = format!("Score {} - Tick {}\n", self.score, self.tick);
        output.push_str(&self.display.render());
        output
    }
}

fn part1(program: &String, render_game: bool) {
    let mut arcade = ArcadeCabinet::new(program);
    arcade.execute(0);
    println!("Part 1: Rendered {} blocks", arcade.get_block_count());
    if render_game {
        println!("{}", arcade.render());
    }
}

fn part2(program: &String, render_game: bool) {
    let mut arcade = ArcadeCabinet::new(program);
    let score = arcade.play(render_game);
    println!("Part 2: Score: {}\n", score);
}

pub fn execute() {
    let content = read_content(&"data/day13.txt".to_string());
    part1(&content, false);
    part2(&content, false);
}
