use crate::input_files::read_content;
use crate::intcode::IntCodeComputer;

pub fn execute() {
    let content = read_content(&String::from("data/day09.txt"));
    let mut m1 = IntCodeComputer::read_program_with_input(&content, 1);
    m1.execute_until_stopped();
    println!("Part 1: BOOST keycode: {}", m1.get_last_output().unwrap());
    let mut m2 = IntCodeComputer::read_program_with_input(&content, 2);
    m2.execute_until_stopped();
    println!("Part 2: Distress signal: {}", m2.get_last_output().unwrap());
}
