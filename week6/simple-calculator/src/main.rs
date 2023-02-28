use std::io;

fn main() {
    println!("Simple Calculator in Rust");
    println!("-------------------------");

    loop {
        println!("Please enter the operation you want to perform:");
        println!("1. Addition (+)");
        println!("2. Subtraction (-)");
        println!("3. Multiplication (*)");
        println!("4. Division (/)");
        println!("5. Exit");

        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("Failed to read line");
        let operation: u32 = operation.trim().parse().expect("Please enter a valid number");

        if operation == 5 {
            println!("Exiting...");
            break;
        }

        println!("Please enter the first number:");
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Failed to read line");
        let num1: f64 = num1.trim().parse().expect("Please enter a valid number");

        println!("Please enter the second number:");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Failed to read line");
        let num2: f64 = num2.trim().parse().expect("Please enter a valid number");

        let result = match operation {
            1 => num1 + num2,
            2 => num1 - num2,
            3 => num1 * num2,
            4 => num1 / num2,
            _ => {
                println!("Invalid operation selected");
                continue;
            }
        };

        println!("Result: {}", result);
    }
}
