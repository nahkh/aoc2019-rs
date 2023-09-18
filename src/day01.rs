fn calculate_fuel(mass: i32) -> i32 {
    let needed_fuel = mass / 3 - 2;
    if needed_fuel <= 0 {
        return 0;
    }
    needed_fuel + calculate_fuel(needed_fuel)
}

pub fn execute() {
    let mut total_fuel: i32 = 0;
    for line in crate::input_files::read_content(&String::from("data/day01.txt")).lines() {
        let mass = line.parse::<i32>().unwrap();
        total_fuel += calculate_fuel(mass);
    }
    println!("Total fuel needed: {}", total_fuel);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_fuel() {
        assert_eq!(crate::day01::calculate_fuel(1969), 966);
        assert_eq!(crate::day01::calculate_fuel(100756), 50346);
    }
}
