use std::io;
use std::collections::HashMap;


fn main() {

    println!("Please enter a list of integers separated by spaces");
    let input = read_input();
    let mut parsed_input = parse_input(input);
    println!("The median is {}", get_median(&mut parsed_input));
    println!("The mode is {}", get_mode(&mut parsed_input));
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
    input
}

fn parse_input(input : String) -> Vec<u32> {
    let numbers = input.trim().split_whitespace().map(|s| s.parse().unwrap());

    let mut vec = Vec::new();
    for num in numbers {
        vec.push(num);
    }
    vec
}

fn get_median(input: &mut Vec<u32>) -> u32 {
    input.sort();
    let length = input.len();
    if length % 2 == 0 {
        (input[length / 2] + input[length / 2 - 1]) / 2
    } else {
        input[length / 2]
    }
}

// create an hash map with number as key and count as value
fn get_mode(input: &mut Vec<u32>) -> u32 {

    let mut map = HashMap::new();
    for num in input {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut mode = 0;
    for (key, value) in map {
        if value > max {
            max = value;
            mode = *key;
        }
    }
    mode
}
