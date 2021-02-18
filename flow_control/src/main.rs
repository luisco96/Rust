fn main() {
    let number = 5;

    if number < 5 {
    println!("condition was true");
    } else {
        println!("condition was false");
    }

    // this wont compile, only bools can be evaluated
    //if number {
    //    println!("number is not zero");
    //}
    // we can try this instead
    if number != 0 {
        println!("number was something other than zero");
    }

    // let if
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {}", number);

    // while, break and continue are the same than in c like languages

    // loop iterates forever till you reach a break statement
    // you can return something from the loop after the break
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // for loops 
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // with range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
