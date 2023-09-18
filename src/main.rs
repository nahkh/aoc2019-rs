use std::env;

mod input_files;
mod intcode;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_arg = &args[1];
    let target_day = day_arg.parse::<usize>().unwrap();
    match target_day {
        1 => day01::execute(),
        2 => day02::execute(),
        3 => day03::execute(),
        4 => day04::execute(),
        5 => day05::execute(),
        6 => day06::execute(),
        7 => day07::execute(),
        _ => panic!("Day {} not implemented", target_day)
    }
}
