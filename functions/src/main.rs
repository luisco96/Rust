fn main() {
    println!("Hello, world!");
    another_function();
    function_parameters(28, 12);
    println!("function_return returns {}", function_return());

    let number = 7;
    println!("{} plus one is {}", number, plus_one(number));
}

fn another_function() {
    println!("Another function.");
}

fn function_parameters(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn function_return() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}