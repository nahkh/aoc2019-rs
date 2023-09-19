use crate::input_files::read_content;

fn calculate_fuel_atomic(mass: i32) -> i32 {
    mass / 3 - 2
}

fn calculate_fuel(mass: i32) -> i32 {
    let needed_fuel = calculate_fuel_atomic(mass);
    if needed_fuel <= 0 {
        return 0;
    }
    needed_fuel + calculate_fuel(needed_fuel)
}

pub fn execute() {
    let mut atomic_fuel: i32 = 0;
    let mut total_fuel: i32 = 0;
    for line in read_content(&String::from("data/day01.txt")).lines() {
        let mass = line.parse::<i32>().unwrap();
        total_fuel += calculate_fuel(mass);
        atomic_fuel += calculate_fuel_atomic(mass);
    }
    println!("Part 1: Total fuel needed: {}", atomic_fuel);
    println!("Part 2: Total fuel needed: {}", total_fuel);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_fuel() {
        assert_eq!(crate::day01::calculate_fuel(1969), 966);
        assert_eq!(crate::day01::calculate_fuel(100756), 50346);
    }
}
