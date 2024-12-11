use std::io;

enum Operation {
    Add { x: f64, y: f64 },
    Subtract { x: f64, y: f64 },
    Multiply { x: f64, y: f64 },
    Divide { x: f64, y: f64 },
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add { x, y } => x + y,
        Operation::Subtract { x, y } => x - y,
        Operation::Multiply { x, y } => x * y,
        Operation::Divide { x, y } => {
            if y != 0.0 {
                x / y
            } else {
                panic!("Division by zero is not allowed.");
            }
        }
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut input).unwrap();
    let first_number: f64 = input.trim().parse().expect("Please enter a valid number.");
    input.clear();

    println!("Enter the operation (+, -, *, /):");
    io::stdin().read_line(&mut input).unwrap();
    let operation = input.trim().to_string();
    input.clear();

    println!("Enter the second number:");
    io::stdin().read_line(&mut input).unwrap();
    let second_number: f64 = input.trim().parse().expect("Please enter a valid number.");
    input.clear();

    let op = match operation.as_str() {
        "+" => Operation::Add {
            x: first_number,
            y: second_number,
        },
        "-" => Operation::Subtract {
            x: first_number,
            y: second_number,
        },
        "*" => Operation::Multiply {
            x: first_number,
            y: second_number,
        },
        "/" => Operation::Divide {
            x: first_number,
            y: second_number,
        },
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    let result = calculate(op);

    println!("The result is: {}", result);
}
