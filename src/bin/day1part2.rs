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

struct CharCalculations {
    number_map: HashMap<i32, String>,
    char_map: HashMap<char, Vec<i32>>,
}

impl CharCalculations {
    fn new() -> Self {
        let number_map = Self::create_number_map();
        let char_map = Self::create_char_map();
        Self {
            number_map,
            char_map,
        }
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
}
impl CharCalculations {
    fn get_chars(&self, numbers: &Vec<i32>, index: i32) -> Vec<PossibleCharNumber> {
        let mut possible_chars: Vec<PossibleCharNumber> = vec![];
        for number in numbers {
            let possible_char = self.get_char(*number, index);
            possible_chars.push(possible_char)
        }
        possible_chars
    }

    fn get_char(&self, number: i32, index: i32) -> PossibleCharNumber {
        let numb_string = self.number_map.get(&number).unwrap();
        let index: usize = index.try_into().unwrap();
        let char = numb_string.chars().nth(index).unwrap();
        PossibleCharNumber { char, number }
    }

    fn check_finished(&self, number: i32, index: i32) -> bool {
        let number_string = self.number_map.get(&number).unwrap();
        let length: i32 = number_string.len().try_into().unwrap();
        index >= length
    }

    fn get_possible_numbers(&self, current_char: char) -> Option<&Vec<i32>> {
        self.char_map.get(&current_char)
    }
}

fn line_number(line: &str) -> i32 {
    let char_calculations = CharCalculations::new();
    let chars = line.chars();
    let mut possible_numbers = PossibleNumbers::None;

    let mut current_index = 0;
    for current_char in chars {
        match possible_numbers {
            PossibleNumbers::Numbers(numbers) => {
                let possible_chars = char_calculations.get_chars(numbers, current_index);
                let possible_char = possible_chars
                    .into_iter()
                    .find(|possible_number| current_char == possible_number.char);
                possible_numbers = match possible_char {
                    Some(possible_char) => {
                        current_index += 1;
                        PossibleNumbers::Number(possible_char.number)
                    }
                    None => {
                        current_index = 0;
                        PossibleNumbers::None
                    }
                };
            }
            PossibleNumbers::Number(number) => {
                let possible_char = char_calculations.get_char(number, current_index);
                if possible_char.char == current_char {
                    current_index += 1;
                } else {
                    current_index = 0;
                    possible_numbers = PossibleNumbers::None;
                }
                if char_calculations.check_finished(number, current_index) {
                    return number;
                }
            }
            PossibleNumbers::None => {
                let is_digit = current_char.is_ascii_digit();
                if is_digit {
                    let number: i32 = current_char.to_digit(10).unwrap().try_into().unwrap();
                    return number;
                }
                let possible = char_calculations.get_possible_numbers(current_char);
                if let Some(numbers) = possible {
                    possible_numbers = PossibleNumbers::Numbers(numbers);
                    current_index += 1;
                }
            }
        }
    }
    panic!("no number found");
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
