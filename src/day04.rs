fn number_is_valid(number: i32) -> bool {
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

fn count_valid_passwords(start: i32, end: i32) -> i32 {
    let mut count = 0;
    for num in start..(end+1) {
        if number_is_valid(num) {
            count += 1;
        }
    }
    return count;
}

pub fn execute() {
    let count = count_valid_passwords(108457, 562041);
    println!("Number of valid passwords in range: {}", count);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_number_is_valid() {
        assert_eq!(crate::day04::number_is_valid(111111), true);
        assert_eq!(crate::day04::number_is_valid(223450), false);
        assert_eq!(crate::day04::number_is_valid(123789), false);
    }
}
