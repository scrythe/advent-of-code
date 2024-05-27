use std::collections::HashMap;

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

fn line_number(line: &str) -> char {
    let numb_map = create_numb_map();
    let char_map = create_char_map();
    let chars = line.chars();
    println!("{line}");
    let mut char_digit: Option<char> = None;
    let mut possible_numbs: Option<&Vec<i32>> = None;
    let mut current_index = 0;
    for char in chars {
        if let Some(numbs) = possible_numbs {
            let possible_chars = get_chars(numbs, current_index, &numb_map);
            if possible_chars.contains(&char) {
                current_index += 1;
                return char;
            }
        }
        let is_digit = char.is_ascii_digit();
        if is_digit {
            char_digit = Some(char);
            break;
        }
        let possible = char_map.get(&char);
        if let Some(numbs) = possible {
            possible_numbs = Some(numbs);
            current_index += 1;
        } else {
            current_index = 0;
            possible_numbs = None;
        }
    }
    char_digit.unwrap()
}

fn get_chars(numbs: &Vec<i32>, index: i32, number_map: &HashMap<i32, String>) -> Vec<char> {
    let mut possible_chars: Vec<char> = vec![];
    for numb in numbs {
        let numb_string = number_map.get(numb).unwrap();
        let char = numb_string.chars().nth(index.try_into().unwrap()).unwrap();
        possible_chars.push(char)
    }
    possible_chars
}

fn create_numb_map() -> HashMap<i32, String> {
    let mut number_char_map: HashMap<i32, String> = HashMap::new();
    number_char_map.insert(1, "one".to_string());
    number_char_map.insert(2, "two".to_string());
    number_char_map.insert(3, "three".to_string());
    number_char_map.insert(4, "four".to_string());
    number_char_map.insert(5, "five".to_string());
    number_char_map.insert(6, "six".to_string());
    number_char_map.insert(7, "seven".to_string());
    number_char_map.insert(8, "eight".to_string());
    number_char_map.insert(9, "nine".to_string());
    number_char_map
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

    #[test]
    fn test_char() {
        let numb_map = create_numb_map();
        let chars = get_chars(&vec![2, 3], 2, &numb_map);
        assert_eq!(chars, vec!['o', 'r']);
        let chars = get_chars(&vec![4, 5], 3, &numb_map);
        assert_eq!(chars, vec!['r', 'e']);
    }

    #[test]
    fn test_next_char() {
        // let char = line_number("hmons");
        // assert_eq!(char, 'n');
        let char = line_number("sadfib");
        assert_eq!(char, 'i');
    }
}
