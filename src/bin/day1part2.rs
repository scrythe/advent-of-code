use std::{char, collections::HashMap};

fn main() {
    let day1_input = advent_of_code::read_input_string("day1.test");
    total_number(day1_input)
}

fn total_number(input: String) {
    let lines = input.lines();
    for line in lines {
        line_number(line);
    }
}

enum PossibleNumbers<'a> {
    Numbers(&'a Vec<i32>),
    Number(i32),
    None,
}

struct PossibleCharNumber {
    char: char,
    number: i32,
}

fn line_number(line: &str) -> i32 {
    let number_map = create_number_map();
    let char_map = create_char_map();
    let chars = line.chars();
    let mut char_digit: Option<char> = None;
    let mut char_number: Option<i32> = None;
    let mut possible_numbers = PossibleNumbers::None;

    let mut current_index = 0;
    for current_char in chars {
        match possible_numbers {
            PossibleNumbers::Numbers(numbers) => {
                let possible_chars = get_numbers_chars(numbers, current_index, &number_map);
                let possible_char = possible_chars
                    .into_iter()
                    .find(|possible_number| current_char == possible_number.char);
                if let Some(possible_char) = possible_char {
                    possible_numbers = PossibleNumbers::Number(possible_char.number);
                    current_index += 1;
                } else {
                    current_index = 0;
                    possible_numbers = PossibleNumbers::None;
                }
            }
            PossibleNumbers::Number(number) => {
                let possible_char = get_number_char(number, current_index, &number_map);
                if possible_char.char == current_char {
                    current_index += 1;
                } else {
                    current_index = 0;
                    possible_numbers = PossibleNumbers::None;
                }
                if number_finished(number, current_index, &number_map) {
                    char_number = Some(number);
                    break;
                }
            }
            PossibleNumbers::None => {
                let is_digit = current_char.is_ascii_digit();
                if is_digit {
                    char_digit = Some(current_char);
                    break;
                }
                let possible = char_map.get(&current_char);
                if let Some(numbers) = possible {
                    possible_numbers = PossibleNumbers::Numbers(numbers);
                    current_index += 1;
                }
            }
        }
    }
    let number = char_number.unwrap_or(0);
    match char_digit {
        Some(digit) => digit.to_digit(10).unwrap().try_into().unwrap(),
        None => number,
    }
}

fn get_numbers_chars(
    numbers: &Vec<i32>,
    index: i32,
    number_map: &HashMap<i32, String>,
) -> Vec<PossibleCharNumber> {
    let mut possible_chars: Vec<PossibleCharNumber> = vec![];
    for number in numbers {
        let possible_char = get_number_char(*number, index, number_map);
        possible_chars.push(possible_char)
    }
    possible_chars
}

fn number_finished(number: i32, index: i32, number_map: &HashMap<i32, String>) -> bool {
    let number_string = number_map.get(&number).unwrap();
    let length: i32 = number_string.len().try_into().unwrap();
    index >= length
}

fn get_number_char(
    number: i32,
    index: i32,
    number_map: &HashMap<i32, String>,
) -> PossibleCharNumber {
    let numb_string = number_map.get(&number).unwrap();
    let index: usize= index.try_into().unwrap();
    let char = numb_string.chars().nth(index).unwrap();
    PossibleCharNumber { char, number }
}

fn create_number_map() -> HashMap<i32, String> {
    let mut number_map: HashMap<i32, String> = HashMap::new();
    number_map.insert(1, "one".to_string());
    number_map.insert(2, "two".to_string());
    number_map.insert(3, "three".to_string());
    number_map.insert(4, "four".to_string());
    number_map.insert(5, "five".to_string());
    number_map.insert(6, "six".to_string());
    number_map.insert(7, "seven".to_string());
    number_map.insert(8, "eight".to_string());
    number_map.insert(9, "nine".to_string());
    number_map
}

fn create_char_map() -> HashMap<char, Vec<i32>> {
    let mut char_map: HashMap<char, Vec<i32>> = HashMap::new();
    char_map.insert('s', vec![7, 6]);
    char_map.insert('n', vec![9]);
    char_map.insert('f', vec![5, 4]);
    char_map.insert('o', vec![1]);
    char_map.insert('t', vec![2, 3]);
    char_map.insert('e', vec![8]);
    char_map
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_char() {
    //     let numb_map = create_number_map();
    //     let chars = get_numbers_chars(&vec![2, 3], 2, &numb_map);
    //     assert_eq!(chars, vec!['o', 'r']);
    //     let chars = get_numbers_chars(&vec![4, 5], 3, &numb_map);
    //     assert_eq!(chars, vec!['r', 'e']);
    // }

    // #[test]
    // fn test_numbers() {
    //     let mut total = 0;
    //     let number = line_number("two1nine");
    //     assert_eq!(number, 29);
    //     total += number;
    //     let number = line_number("eightwothree");
    //     assert_eq!(number, 83);
    //     total += number;
    //     let number = line_number("abcone2threexyz");
    //     assert_eq!(number, 13);
    //     total += number;
    //     let number = line_number("xtwone3four");
    //     assert_eq!(number, 24);
    //     total += number;
    //     let number = line_number("4nineeightseven2");
    //     assert_eq!(number, 42);
    //     total += number;
    //     let number = line_number("zoneight234");
    //     assert_eq!(number, 14);
    //     total += number;
    //     let number = line_number("7pqrstsixteen");
    //     assert_eq!(number, 76);
    //     total += number;
    //
    //     assert_eq!(total, 281);
    // }

    #[test]
    fn test_numbers_forward() {
        let number = line_number("two1nine");
        assert_eq!(number, 2);
        let number = line_number("eightwothree");
        assert_eq!(number, 8);
        let number = line_number("abcone2threexyz");
        assert_eq!(number, 1);
        let number = line_number("xtwone3four");
        assert_eq!(number, 2);
        let number = line_number("4nineeightseven2");
        assert_eq!(number, 4);
        let number = line_number("zoneight234");
        assert_eq!(number, 1);
        let number = line_number("7pqrstsixteen");
        assert_eq!(number, 7);
    }

    // #[test]
    // fn get_number() {
    //     let numb_map = create_number_map();
    //     //     let chars = get_numbers_chars(&vec![2, 3], 2, &numb_map);
    //     let char = get_number_char(5, 4, &numb_map);
    //     println!("5");
    // }
}
