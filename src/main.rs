use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    let first_arg = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second_arg = args.nth(0).unwrap();

    println!("{:?} {} {}", first_arg, operator, second_arg);

    let first_number = first_arg.parse::<f32>().unwrap();
    let second_number = second_arg.parse::<f32>().unwrap();
    // println!("{:?}", args);
    // println!("First number = {}\nSecond number = {}", first_number, second_number);
    let result = calculate(first_number, operator, second_number);
    println!("{:?}", output(first_number, operator, second_number, result))
}

fn calculate(num1: f32, operator: char, num2: f32) -> f32 {
    match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '/' => {
            assert_ne!(num2, 0.0, "denominator must not be zero");
            num1 / num2
        },
        '*' | 'x' | 'X' => num1 * num2,
        _ => panic!("Invalid operator")
    }
}

fn output(num1: f32, operator: char, num2: f32, result: f32) -> String {
    format!("{} {} {} = {}", num1, operator, num2, result)
}

