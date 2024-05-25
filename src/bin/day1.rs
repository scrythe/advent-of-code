fn main() {
    let part1_input = advent_of_code::read_input_string("day1.input");
    let part1_val = part1(part1_input);
    println!("Part1: {part1_val}");
}

fn part1(input: String) -> u32 {
    let mut total = 0;
    let lines = input.lines();
    for line in lines {
        total += calculate_line(line);
    }
    total
}

fn calculate_line(line: &str) -> u32 {
    let chars = line.chars();
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;
    for char in chars {
        let is_digit = !char.is_ascii_digit();
        if is_digit {
            continue;
        }
        last_digit = char.to_digit(10);
        if first_digit.is_some() {
            continue;
        }
        if first_digit.is_none() {
            first_digit = last_digit;
        }
    }
    let first_digit = first_digit.unwrap();
    let last_digit = last_digit.unwrap();
    let number = format!("{first_digit}{last_digit}");
    number.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut total = 0;
        let result = calculate_line("1abc2");
        assert_eq!(result, 12);

        total += result;
        let result = calculate_line("pqr3stu8vwx");
        assert_eq!(result, 38);

        total += result;
        let result = calculate_line("a1b2c3d4e5f");
        assert_eq!(result, 15);

        total += result;
        let result = calculate_line("treb7uchet");
        assert_eq!(result, 77);

        total += result;
        assert_eq!(total, 142);
    }
}
