use std::collections::HashMap;

fn main() {
    let day1_input = advent_of_code::read_input_string("day1.test");
    total_number(day1_input)
}

fn total_number(input: String) {
    let lines = input.lines();
    for line in lines {
        line_number(line)
    }
}

fn line_number(line: &str) {
    create_char_linker();
    let chars = line.chars();
    println!("{line}");
    let mut char_digit: Option<char> = None;
    for char in chars {
        let is_digit = char.is_ascii_digit();
        if is_digit {
            char_digit = Some(char);
            break;
        }
    }
    let char_digit = char_digit.unwrap();
    println!("{char_digit}");
    panic!("so")
}

// fn check_first_letter(char: char) {
//     let first_entry = NUMBERSTRING[0].bytes();
//     // let first_char = first_entry[0];
//     // if char == first_char {
//     //     println!("hm")
//     // }
// }

fn create_char_linker() {
    let mut number_char_map: HashMap<i32, &str> = HashMap::new();
    number_char_map.insert(1, "one");
    number_char_map.insert(2, "two");
    number_char_map.insert(3, "three");
    number_char_map.insert(4, "four");
    number_char_map.insert(5, "five");
    number_char_map.insert(6, "six");
    number_char_map.insert(7, "seven");
    number_char_map.insert(8, "eight");
    number_char_map.insert(9, "nine");

    let mut char_map: HashMap<char, Vec<i32>> = HashMap::new();
    for (key, value) in &number_char_map {
        let char = value.chars().next().unwrap();
        let char_vec = char_map.entry(char).or_default();
        char_vec.push(*key)
    }
    panic!("so")
}
