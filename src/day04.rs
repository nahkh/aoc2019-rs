fn number_is_valid_part1(number: i32) -> bool {
    let num1 = number / 100000;
    let num2 = (number % 100000) / 10000;
    let num3 = (number % 10000) / 1000;
    let num4 = (number % 1000) / 100;
    let num5 = (number % 100) / 10;
    let num6 = number % 10;
    if num1 > num2 || num2 > num3 || num3 > num4 || num4 > num5 || num5 > num6 {
        return false;
    }
    if num1 != num2 && num2 != num3 && num3 != num4 && num4 != num5 && num5 != num6 {
        return false;
    }

    true
}

fn number_is_valid_part2(number: i32) -> bool {
    let num1 = number / 100000;
    let num2 = (number % 100000) / 10000;
    let num3 = (number % 10000) / 1000;
    let num4 = (number % 1000) / 100;
    let num5 = (number % 100) / 10;
    let num6 = number % 10;
    if num1 > num2 || num2 > num3 || num3 > num4 || num4 > num5 || num5 > num6 {
        return false;
    }
    let numbers = [num1, num2, num3, num4, num5, num6];
    let mut current_streak = 0;
    let mut prev_number = -1;
    for num in numbers {
        if num == prev_number {
            current_streak += 1;
        } else {
            if current_streak == 2 {
                return true;
            }
            current_streak = 1;
        }
        prev_number = num;
    }
    
    current_streak == 2
}

fn count_valid_passwords_part1(start: i32, end: i32) -> i32 {
    let mut count = 0;
    for num in start..(end+1) {
        if number_is_valid_part1(num) {
            count += 1;
        }
    }
    return count;
}

fn count_valid_passwords_part2(start: i32, end: i32) -> i32 {
    let mut count = 0;
    for num in start..(end+1) {
        if number_is_valid_part2(num) {
            count += 1;
        }
    }
    return count;
}

pub fn execute() {
    let count1 = count_valid_passwords_part1(108457, 562041);
    println!("Part 1: Number of valid passwords in range: {}", count1);
    let count2 = count_valid_passwords_part2(108457, 562041);
    println!("Part 1: Number of valid passwords in range: {}", count2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_number_is_valid_part1() {
        assert_eq!(crate::day04::number_is_valid_part1(111111), true);
        assert_eq!(crate::day04::number_is_valid_part1(223450), false);
        assert_eq!(crate::day04::number_is_valid_part1(123789), false);
    }

    #[test]
    fn test_number_is_valid_part2() {
        assert_eq!(crate::day04::number_is_valid_part2(112233), true);
        assert_eq!(crate::day04::number_is_valid_part2(123444), false);
        assert_eq!(crate::day04::number_is_valid_part2(111122), true);
    }
}
