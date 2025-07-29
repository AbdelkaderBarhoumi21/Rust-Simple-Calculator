use std::io;
fn main() {
    println!("------Simple Calculator------");
    println!("Available operations : + ,- ,* ,/");
    println!("Enter your expressions (eg, 5+3)");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();
    if tokens.len() != 3 {
        println!("Invalid input please follow the format :  number operator number");
        return;
    }
    let num1: f64 = match tokens[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Inavlid first number.");
            return;
        }
    };
    let operator = tokens[1];
    let num2: f64 = match tokens[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Inavlid first number.");
            return;
        }
    };
    let result = match operator {
        "+" => add(num1, num2),
        "-" => substract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => divide(num1, num2),
        _ => {
            println!("Invalid operator use + ,- ,* ,/");
            return;
        }
    };
    println!("Result = {:.2}", result);
}

fn add(x: f64, y: f64) -> f64 {
    x + y
}
fn substract(x: f64, y: f64) -> f64 {
    x - y
}
fn multiply(x: f64, y: f64) -> f64 {
    x * y
}
fn divide(x: f64, y: f64) -> f64 {
    if y == 0.0 {
        println!("Division by zero is not allowed");
        std::process::exit(1);
    }
    x / y
}
