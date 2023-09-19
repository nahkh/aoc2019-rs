use crate::input_files::read_content;
use crate::intcode::IntCodeComputer;

pub fn execute() {
    let content = read_content(&String::from("data/day05.txt"));
    let mut m1 = IntCodeComputer::read_program_with_input(&content, 1);
    m1.execute_until_stopped();
    println!(
        "Part 1: Diagnostic output: {}",
        m1.get_last_output().unwrap()
    );
    let mut m2 = IntCodeComputer::read_program_with_input(&content, 5);
    m2.execute_until_stopped();
    println!(
        "Part 2: Diagnostic output: {}",
        m2.get_last_output().unwrap()
    );
}
