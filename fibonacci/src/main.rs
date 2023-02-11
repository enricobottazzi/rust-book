use std::io;

fn main() {

    println!("Please input the index of the fibonacci series you want to know the value of");
    let input = read_input();
    let input = parse_input(input);
    let fibonacci_number = get_nth_fibonacci_number(input);
    println!("The {}th fibonacci number is {}", input, fibonacci_number);
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
    input
}

fn parse_input(input : String) -> u32 {
    input.trim().parse().expect("Please type a number!")
}

fn get_nth_fibonacci_number(n: u32) -> u32 {
    let mut f1 = 0;
    let mut f2 = 1;

    for _ in 0..n {
        let f3 = f1 + f2;
        f1 = f2;
        f2 = f3;
    }
    f1
}