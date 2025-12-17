use std::io;

fn main() {
    println!("[*] Welcome, you must choose the operation you want to do!");
    println!("[*] Type: + to perform a addition.");
    println!("[*] Type: - to perform a subtraction.");
    println!("[*] Type: * to perform a multiplication.");
    println!("[*] Type: / to perform a division.");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");

    // Handle first number logic
    println!("[*] Type the first number: ");

    let mut first_number = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line.");
    let first_number: f64 = first_number.trim().parse().expect("Not a number!");

    // Handle second number logic
    println!("[*] Type the second number: ");

    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line.");
    let second_number: f64 = second_number.trim().parse().expect("Not a number!");

    calculator(user_input.trim(), first_number, second_number);
}

// @dev using float numbers here since they're signed.
fn calculator(op: &str, x: f64, y: f64) {
    if op == "+" {
        println!("Result {} + {} = {}", x, y, x + y);
    }
    if op == "-" {
        println!("Result {} - {} = {}", x, y, x - y);
    }
    if op == "*" {
        println!("Result {} * {} = {}", x, y, x * y);
    }
    if op == "/" {
        println!("Result {} / {} = {}", x, y, x / y);
    }
}
