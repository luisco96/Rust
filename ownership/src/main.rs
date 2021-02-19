fn main() {
    { // example 1, string literals vs String
        let s = "hello"; // s is valid from this point forward

        // do stuff with s

        // String type
        let s2 = String::from("hello");
        let mut s2 = String::from("hello");

        s2.push_str(", world!"); // push_str() appends a literal to a String
        println!("{}", s2);
    }   
    // this scope is now over, and s is no longer valid
    // s2 its also out of scope and its automatically dealocated 
    // in Rust dealocation is made through the function drop()

    { // example 2, move operation 
        // being integers with a know size, here the x value its copied 
        // to y
        let x = 5;
        let y = x;

        // Here "hello" is not copied to s2, intead only the s1 information
        // is copied, that is, the pointer to "hello", its lenght and capacity
        let s1 = String::from("hello");
        let s2 = s1;

        // this next line is invalid, the info from s1 is not copied
        // its moved, meaning s1 is now invalid and s2 is the owner 
        // of the String 
        //println!("{}, world!", s1);

    }
    // only s2 its dropped

    { // example 3, clone operation
        let s1 = String::from("hello");
        let s2 = s1.clone();

        // now "hello" its really compied to s2
        // and s1 and s2 are valid, pointing to diferent addresses in the heap
        println!("s1 = {}, s2 = {}", s1, s2);
    }
    // both s1 and s2 are dropped after going out of scope
    
    { // example 4, functions
        let s = String::from("hello"); // s comes into scope

        takes_ownership(s);             // s's valies moves into the fucntion...
                                        // ... abd so is no longer valid here
        
        let x = 5;                      // x comes into scope

        makes_copy(x);                  // x would move into the fucntion,
                                        // but i32 is Copy, so it;s okay to still
                                        // use x afterward
    
    } // Here, x goes out of scope, then s. But because s;s values was moved, nothign
      // special happens.

    { // exaple 5, function return values
        let s1 = gives_ownership();     // gives_ownership moves its return
                                        // value into s1
        
        let s2 = String::from("hello"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2);  // s2 is moves into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3
        
    } // Here, s3 goes out of scope and is dropped, s2 goes out of scope but was
      // moved, so nohting happens. s1 goes out of scope and is dropped.

    { // example 6, references
        let s1 = String::from("hello");

        let len = calculate_lenght(&s1);

        println!("The lenght of '{}' is {}.", s1, len);
    }

    { // example 7, mutable reference
        let mut s = String::from("hello");

        change(&mut s);

        let r1 = &mut s;
        //let r2 = &mut s; // this is invalid
        // you can only have one mutable reference to a particualar piece of 
        // data in a particular scope
    }

    { // example 8, slices 
        let s = String::from("hello world");
        
        let hello = first_word(&s);

        //s.clear(); // error, compiler error
        println!("the first word is: {}", hello);
    }
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and 'drop' is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {        // gives_ownership will move its
                                        // return value into the function 
                                        // that calls it
    let some_string = String::from("hello");    // some_string comes into scope

    some_string                         // some_string is returned and
                                        // moves out to the calling
                                        // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {   // a_string comes into
                                                        // scope
    
    a_string    // a_string is returned and moved out to the calling function
}

fn calculate_lenght(s: &String) -> usize {  // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}