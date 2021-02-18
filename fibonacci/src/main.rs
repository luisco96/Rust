use std::io;

fn main() {
    println!("Enter a number:");
    
    let mut initial = String::new();
    io::stdin()
        .read_line(&mut initial)
        .expect("Failed to read from input");

    let initial: u64 = match initial.trim().parse() {
        Ok(num) => num,
        Err(_) => { 
            println!("Enter a valid number");
            panic!();
        }
    };

    let result = fibonacci(initial);
    println!("The fibonacci number for {} is {}", initial, result);
}

fn fibonacci(x: u64) -> u64 {
    if x == 0 {
        0
    } else if x == 1 {
        1
    } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
}
